# Tauri Commands Contract: yt-dlp GUI

**Branch**: `002-yt-dlp-gui` | **Date**: 2026-02-13

All commands use `#[tauri::command]` + `#[specta::specta]` and return `Result<T, AppError>`.
TypeScript bindings are auto-generated via tauri-specta.

## Binary Management

### check_dependencies
시스템에 yt-dlp와 ffmpeg가 설치되어 있는지 확인한다.

```
Input: None
Output: DependencyStatus { ytdlp_installed: bool, ytdlp_version: Option<String>, ffmpeg_installed: bool, ffmpeg_version: Option<String> }
```

### install_dependencies
yt-dlp와 ffmpeg 바이너리를 다운로드하여 앱 데이터 디렉토리에 설치한다. Channel로 설치 진행률을 스트리밍한다.

```
Input: on_event: Channel<InstallEvent>
Output: Result<(), AppError>

InstallEvent:
  | { event: "progress", data: { component: "ytdlp" | "ffmpeg", percent: f32, status: String } }
  | { event: "completed", data: { component: "ytdlp" | "ffmpeg" } }
  | { event: "error", data: { component: "ytdlp" | "ffmpeg", message: String } }
```

### update_ytdlp
yt-dlp를 최신 버전으로 업데이트한다.

```
Input: None
Output: Result<String, AppError>  // 새 버전 문자열
```

## Video Metadata

### fetch_video_info
YouTube URL에서 비디오 메타데이터를 조회한다.

```
Input: url: String
Output: Result<VideoInfo, AppError>

VideoInfo: {
  url: String, videoId: String, title: String, thumbnail: String,
  duration: u64, uploadDate: String, channel: String, channelUrl: String,
  formats: Vec<FormatInfo>, filesizeApprox: Option<u64>
}

FormatInfo: {
  formatId: String, ext: String, resolution: Option<String>,
  qualityLabel: Option<String>, filesize: Option<u64>,
  vcodec: Option<String>, acodec: Option<String>,
  hasVideo: bool, hasAudio: bool
}
```

### fetch_playlist_info
채널 또는 재생목록 URL에서 비디오 목록을 조회한다 (flat-playlist).

```
Input: url: String, page: u32, page_size: u32
Output: Result<PlaylistResult, AppError>

PlaylistResult: {
  playlistId: String, title: String, url: String,
  videoCount: Option<u64>, channelName: Option<String>,
  entries: Vec<PlaylistEntry>
}

PlaylistEntry: {
  url: String, videoId: String, title: Option<String>,
  duration: Option<u64>, thumbnail: Option<String>
}
```

### validate_url
URL이 유효한 YouTube URL인지 검증한다.

```
Input: url: String
Output: Result<UrlValidation, AppError>

UrlValidation: {
  valid: bool, urlType: "video" | "channel" | "playlist" | "unknown",
  normalizedUrl: Option<String>
}
```

## Download

### start_download
비디오 다운로드를 시작하고 Channel로 진행률을 스트리밍한다.

```
Input: request: DownloadRequest, on_event: Channel<DownloadEvent>
Output: Result<u64, AppError>  // download task ID

DownloadRequest: {
  videoUrl: String, videoId: String, title: String,
  formatId: String, qualityLabel: String,
  outputDir: Option<String>, cookieBrowser: Option<String>
}

DownloadEvent:
  | { event: "started", data: { taskId: u64 } }
  | { event: "progress", data: { taskId: u64, percent: f32, speed: String, eta: String } }
  | { event: "postprocessing", data: { taskId: u64, status: String } }
  | { event: "completed", data: { taskId: u64, filePath: String, fileSize: u64 } }
  | { event: "error", data: { taskId: u64, message: String } }
```

### pause_download
진행 중인 다운로드를 일시정지한다.

```
Input: task_id: u64
Output: Result<(), AppError>
```

### resume_download
일시정지된 다운로드를 재개한다. Channel로 진행률을 다시 스트리밍한다.

```
Input: task_id: u64, on_event: Channel<DownloadEvent>
Output: Result<(), AppError>
```

### cancel_download
다운로드를 취소하고 불완전한 파일을 정리한다.

```
Input: task_id: u64
Output: Result<(), AppError>
```

### retry_download
실패한 다운로드를 재시도한다.

```
Input: task_id: u64, on_event: Channel<DownloadEvent>
Output: Result<(), AppError>
```

## Queue Management

### get_download_queue
현재 다운로드 큐 상태를 조회한다.

```
Input: None
Output: Vec<DownloadTaskInfo>

DownloadTaskInfo: {
  id: u64, videoUrl: String, videoId: String, title: String,
  qualityLabel: String, outputPath: String,
  status: "pending" | "downloading" | "paused" | "completed" | "failed" | "cancelled",
  progress: f32, speed: Option<String>, eta: Option<String>,
  errorMessage: Option<String>, createdAt: i64, completedAt: Option<i64>
}
```

### clear_completed
완료된 항목을 큐에서 제거한다.

```
Input: None
Output: Result<u32, AppError>  // 제거된 항목 수
```

## History

### get_download_history
다운로드 이력을 조회한다.

```
Input: page: u32, page_size: u32, search: Option<String>
Output: Result<HistoryResult, AppError>

HistoryResult: {
  items: Vec<HistoryItem>, totalCount: u64, page: u32, pageSize: u32
}

HistoryItem: {
  id: u64, videoUrl: String, videoId: String, title: String,
  qualityLabel: String, format: String, filePath: String,
  fileSize: Option<u64>, downloadedAt: i64
}
```

### check_duplicate
비디오 ID로 이전 다운로드 여부를 확인한다.

```
Input: video_id: String
Output: Result<Option<HistoryItem>, AppError>
```

### delete_history_item
이력 항목을 삭제한다.

```
Input: id: u64
Output: Result<(), AppError>
```

## Settings

### get_settings
현재 앱 설정을 조회한다.

```
Input: None
Output: AppSettings

AppSettings: {
  downloadPath: String, defaultQuality: String,
  maxConcurrent: u32, filenameTemplate: String,
  cookieBrowser: Option<String>, autoUpdateYtdlp: bool
}
```

### update_settings
앱 설정을 저장한다.

```
Input: settings: AppSettings
Output: Result<(), AppError>
```

### get_available_browsers
쿠키 추출에 사용할 수 있는 브라우저 목록을 반환한다.

```
Input: None
Output: Vec<String>  // e.g., ["chrome", "firefox", "edge"]
```

### select_download_directory
디렉토리 선택 다이얼로그를 열어 다운로드 경로를 선택한다.

```
Input: None
Output: Result<Option<String>, AppError>
```
