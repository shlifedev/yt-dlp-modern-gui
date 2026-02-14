use super::types::DependencyStatus;
use crate::modules::types::AppError;
use std::path::Path;
use std::time::Duration;

/// Build an augmented PATH that includes common package manager locations.
/// macOS .app bundles don't inherit the user's shell PATH, so we must add
/// directories like /opt/homebrew/bin (brew on Apple Silicon) explicitly.
fn augmented_path() -> String {
    let current = std::env::var("PATH").unwrap_or_default();
    let extra_dirs: &[&str] = if cfg!(target_os = "windows") {
        &[]
    } else {
        &[
            "/opt/homebrew/bin", // brew (Apple Silicon)
            "/usr/local/bin",    // brew (Intel Mac) / common Linux
            "/usr/bin",
            "/bin",
        ]
    };

    // Also add ~/.local/bin (pip install --user)
    let home_local = std::env::var("HOME")
        .ok()
        .map(|h| format!("{}/.local/bin", h));

    let mut parts: Vec<String> = Vec::new();
    for dir in extra_dirs {
        parts.push(dir.to_string());
    }
    if let Some(ref hl) = home_local {
        parts.push(hl.clone());
    }
    // Append the original PATH so user-custom paths are still searched
    if !current.is_empty() {
        parts.push(current);
    }
    parts.join(":")
}

/// Create a Command with augmented PATH environment variable.
pub fn command_with_path(program: &str) -> tokio::process::Command {
    let mut cmd = tokio::process::Command::new(program);
    cmd.env("PATH", augmented_path());
    cmd
}

/// Resolve the yt-dlp binary from system PATH (augmented).
pub async fn resolve_ytdlp_path() -> Result<String, AppError> {
    if try_get_version(Path::new("yt-dlp")).await.is_ok() {
        return Ok("yt-dlp".to_string());
    }
    Err(AppError::BinaryNotFound(
        "yt-dlp not found. Please install via your package manager (e.g. brew install yt-dlp)."
            .to_string(),
    ))
}

/// Check if yt-dlp is installed, return (version, debug_info).
pub async fn check_ytdlp() -> (Option<String>, Vec<String>) {
    let mut debug_lines: Vec<String> = Vec::new();
    let path_env = augmented_path();
    debug_lines.push(format!("PATH: {}", path_env));

    debug_lines.push("checking: yt-dlp --version".to_string());
    match try_get_version(Path::new("yt-dlp")).await {
        Ok(version) => {
            debug_lines.push(format!("  OK: {}", version));
            (Some(version), debug_lines)
        }
        Err(reason) => {
            debug_lines.push(format!("  FAIL: {}", reason));

            // Also try full paths as a diagnostic hint
            for p in &["/opt/homebrew/bin/yt-dlp", "/usr/local/bin/yt-dlp"] {
                let exists = std::path::Path::new(p).exists();
                debug_lines.push(format!("  {} exists={}", p, exists));
            }

            (None, debug_lines)
        }
    }
}

/// Try to get version from a binary. Returns Ok(version) or Err(reason).
async fn try_get_version(binary_path: &Path) -> Result<String, String> {
    let mut cmd = command_with_path(binary_path.to_str().unwrap_or("yt-dlp"));
    cmd.arg("--version");

    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }

    // PyInstaller binaries (yt-dlp_macos) need time to extract on first run
    let timeout_result = tokio::time::timeout(Duration::from_secs(30), cmd.output()).await;

    let cmd_result = match timeout_result {
        Ok(result) => result,
        Err(_) => {
            return Err(format!("timeout (30s) executing {}", binary_path.display()));
        }
    };

    let output = match cmd_result {
        Ok(output) => output,
        Err(e) => {
            return Err(format!("exec error: {} ({})", e, e.kind()));
        }
    };

    if output.status.success() {
        String::from_utf8(output.stdout)
            .map(|s| s.trim().to_string())
            .map_err(|e| format!("invalid utf8 in stdout: {}", e))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!(
            "exit code={}, stderr={}",
            output.status,
            stderr.trim()
        ))
    }
}

/// Check if ffmpeg is installed on system PATH (augmented), return version if so.
pub async fn check_ffmpeg() -> Option<String> {
    let mut cmd = command_with_path("ffmpeg");
    cmd.arg("-version");

    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }

    let output = tokio::time::timeout(Duration::from_secs(5), cmd.output())
        .await
        .ok()?
        .ok()?;

    if output.status.success() {
        String::from_utf8(output.stdout)
            .ok()
            .and_then(|s| s.lines().next().map(|line| line.to_string()))
    } else {
        None
    }
}

/// Get full dependency status
pub async fn check_dependencies() -> DependencyStatus {
    let (ytdlp_version, debug_lines) = check_ytdlp().await;
    let ffmpeg_version = check_ffmpeg().await;

    let debug_text = if debug_lines.is_empty() {
        None
    } else {
        Some(debug_lines.join("\n"))
    };

    DependencyStatus {
        ytdlp_installed: ytdlp_version.is_some(),
        ytdlp_version,
        ffmpeg_installed: ffmpeg_version.is_some(),
        ffmpeg_version,
        ytdlp_debug: debug_text,
    }
}

/// Update yt-dlp using --update flag
pub async fn update_ytdlp() -> Result<String, AppError> {
    let ytdlp_path = resolve_ytdlp_path().await?;

    let mut cmd = command_with_path(&ytdlp_path);
    cmd.arg("--update");

    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
    }

    let output = cmd
        .output()
        .await
        .map_err(|e| AppError::Custom(format!("Failed to update yt-dlp: {}", e)))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.trim().to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(AppError::Custom(format!("Update failed: {}", stderr)))
    }
}
