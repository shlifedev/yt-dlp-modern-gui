use super::binary;
use super::progress;
use super::settings;
use super::types::*;
use crate::modules::types::AppError;
use std::process::Stdio;
use tauri::ipc::Channel;
use tauri::AppHandle;
use tauri::Manager;
use tokio::io::{AsyncBufReadExt, BufReader};

#[tauri::command]
#[specta::specta]
pub async fn start_download(
    app: AppHandle,
    request: DownloadRequest,
    on_event: Channel<DownloadEvent>,
) -> Result<u64, AppError> {
    // Get app data directory
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| AppError::Custom(format!("Failed to get app data dir: {}", e)))?;

    // Get binary paths
    let ytdlp_path = binary::get_ytdlp_path(&app_data_dir);
    let ffmpeg_path = binary::get_ffmpeg_path(&app_data_dir);

    // Check yt-dlp binary exists
    if !ytdlp_path.exists() {
        return Err(AppError::Custom(
            "yt-dlp binary not found. Please install dependencies first.".to_string(),
        ));
    }

    // Get settings for download path and filename template
    let settings = settings::get_settings(&app)?;

    // Determine output directory
    let output_dir = request
        .output_dir
        .as_deref()
        .unwrap_or(&settings.download_path);

    // Build output template
    let output_template = format!("{}/{}", output_dir, settings.filename_template);

    // Get database from state
    let db_state = app.state::<crate::DbState>();

    // Insert download record into DB
    let task_id = db_state.insert_download(&request, &output_template)?;

    // Update status to Downloading
    db_state.update_download_status(task_id, &DownloadStatus::Downloading, None)?;

    // Send Started event
    let _ = on_event.send(DownloadEvent::Started { task_id });

    // Build yt-dlp command
    let mut cmd = tokio::process::Command::new(&ytdlp_path);

    cmd.arg("--format").arg(&request.format_id);
    cmd.arg("--output").arg(&output_template);

    // Get ffmpeg directory (parent of ffmpeg binary)
    if let Some(ffmpeg_dir) = ffmpeg_path.parent() {
        cmd.arg("--ffmpeg-location").arg(ffmpeg_dir);
    }

    cmd.arg("--progress-template")
        .arg(progress::progress_template());
    cmd.arg("--newline");
    cmd.arg("--no-playlist");

    // Add cookie browser if specified
    if let Some(browser) = &request.cookie_browser {
        cmd.arg("--cookies-from-browser").arg(browser);
    }

    // Add video URL
    cmd.arg(&request.video_url);

    // Capture stderr for progress
    cmd.stderr(Stdio::piped());
    cmd.stdout(Stdio::null());

    // Spawn process
    let mut child = cmd
        .spawn()
        .map_err(|e| AppError::Custom(format!("Failed to spawn yt-dlp: {}", e)))?;

    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| AppError::Custom("Failed to capture stderr".to_string()))?;

    // Clone necessary data for the async task
    let db_state_clone = db_state.inner().clone();
    let on_event_clone = on_event.clone();

    // Spawn task to read progress
    tokio::spawn(async move {
        let reader = BufReader::new(stderr);
        let mut lines = reader.lines();

        while let Ok(Some(line)) = lines.next_line().await {
            if let Some(progress_info) = progress::parse_progress_line(&line) {
                let speed = progress_info.speed.as_deref().unwrap_or("...").to_string();
                let eta = progress_info.eta.as_deref().unwrap_or("...").to_string();

                // Send progress event
                let _ = on_event_clone.send(DownloadEvent::Progress {
                    task_id,
                    percent: progress_info.percent,
                    speed: speed.clone(),
                    eta: eta.clone(),
                });

                // Update DB progress
                let _ = db_state_clone.update_download_progress(
                    task_id,
                    progress_info.percent,
                    Some(&speed),
                    Some(&eta),
                );
            }
        }
    });

    // Wait for process to complete
    let status = child
        .wait()
        .await
        .map_err(|e| AppError::Custom(format!("Failed to wait for process: {}", e)))?;

    if status.success() {
        // Download completed successfully

        // Try to determine actual file path and size
        // For now, use the output template as the file path
        let file_path = output_template.clone();
        let file_size = std::fs::metadata(&file_path)
            .ok()
            .map(|m| m.len())
            .unwrap_or(0);

        // Mark as completed in DB
        let completed_at = chrono::Utc::now().timestamp();
        db_state.mark_completed(task_id, completed_at)?;

        // Insert into history
        let history_item = HistoryItem {
            id: 0, // Will be set by DB
            video_url: request.video_url.clone(),
            video_id: request.video_id.clone(),
            title: request.title.clone(),
            quality_label: request.quality_label.clone(),
            format: request.format_id.clone(),
            file_path: file_path.clone(),
            file_size: Some(file_size),
            downloaded_at: completed_at,
        };

        db_state.insert_history(&history_item)?;

        // Send completion event
        let _ = on_event.send(DownloadEvent::Completed {
            task_id,
            file_path,
            file_size,
        });
    } else {
        // Download failed - parse error for user-friendly message
        let raw_error = format!("yt-dlp exited with code: {:?}", status.code());

        let error_message = if let Some(code) = status.code() {
            match code {
                1 => "다운로드 중 오류가 발생했습니다.".to_string(),
                2 => "네트워크 연결 문제입니다. 인터넷 연결을 확인하세요.".to_string(),
                _ => raw_error,
            }
        } else {
            "다운로드 프로세스가 예기치 않게 종료되었습니다.".to_string()
        };

        db_state.update_download_status(task_id, &DownloadStatus::Failed, Some(&error_message))?;

        let _ = on_event.send(DownloadEvent::Error {
            task_id,
            message: error_message,
        });
    }

    Ok(task_id)
}

#[tauri::command]
#[specta::specta]
pub async fn cancel_download(app: AppHandle, task_id: u64) -> Result<(), AppError> {
    let db_state = app.state::<crate::DbState>();

    // Update status to Cancelled in DB
    db_state.update_download_status(task_id, &DownloadStatus::Cancelled, None)?;

    // TODO: Implement process killing in future enhancement
    // For now, just update the status

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn pause_download(_app: AppHandle, _task_id: u64) -> Result<(), AppError> {
    Err(AppError::Custom("Not yet implemented".to_string()))
}

#[tauri::command]
#[specta::specta]
pub async fn resume_download(_app: AppHandle, _task_id: u64) -> Result<(), AppError> {
    Err(AppError::Custom("Not yet implemented".to_string()))
}
