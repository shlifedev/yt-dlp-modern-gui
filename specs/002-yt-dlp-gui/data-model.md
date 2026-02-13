# Data Model: yt-dlp GUI

**Branch**: `002-yt-dlp-gui` | **Date**: 2026-02-13

## Entities

### VideoInfo

YouTube 비디오의 메타데이터. yt-dlp `--dump-json` 출력에서 파싱.

| Field | Type | Description |
|-------|------|-------------|
| url | String | YouTube 비디오 URL |
| video_id | String | YouTube 비디오 고유 ID |
| title | String | 비디오 제목 |
| thumbnail | String | 썸네일 이미지 URL |
| duration | u64 | 비디오 길이 (초) |
| upload_date | String | 게시일 (YYYYMMDD) |
| channel | String | 채널명 |
| channel_url | String | 채널 URL |
| formats | Vec\<FormatInfo\> | 사용 가능한 포맷/화질 목록 |
| filesize_approx | Option\<u64\> | 예상 파일 크기 (바이트) |

### FormatInfo

비디오의 개별 다운로드 포맷.

| Field | Type | Description |
|-------|------|-------------|
| format_id | String | yt-dlp 포맷 식별자 |
| ext | String | 파일 확장자 (mp4, webm, m4a 등) |
| resolution | Option\<String\> | 해상도 (e.g., "1920x1080") |
| quality_label | Option\<String\> | 화질 라벨 (e.g., "1080p") |
| filesize | Option\<u64\> | 파일 크기 (바이트) |
| vcodec | Option\<String\> | 비디오 코덱 |
| acodec | Option\<String\> | 오디오 코덱 |
| has_video | bool | 비디오 포함 여부 |
| has_audio | bool | 오디오 포함 여부 |

### ChannelInfo

YouTube 채널 정보.

| Field | Type | Description |
|-------|------|-------------|
| channel_id | String | 채널 고유 ID |
| channel_name | String | 채널명 |
| channel_url | String | 채널 URL |
| thumbnail | Option\<String\> | 채널 아이콘 URL |
| video_count | Option\<u64\> | 총 비디오 수 |

### PlaylistInfo

YouTube 재생목록 정보.

| Field | Type | Description |
|-------|------|-------------|
| playlist_id | String | 재생목록 고유 ID |
| title | String | 재생목록 제목 |
| url | String | 재생목록 URL |
| video_count | Option\<u64\> | 비디오 수 |
| entries | Vec\<PlaylistEntry\> | 비디오 목록 (flat-playlist) |

### PlaylistEntry

재생목록 내 개별 비디오 엔트리 (경량). `--flat-playlist --dump-json` 결과.

| Field | Type | Description |
|-------|------|-------------|
| url | String | 비디오 URL |
| video_id | String | 비디오 고유 ID |
| title | Option\<String\> | 비디오 제목 |
| duration | Option\<u64\> | 비디오 길이 (초) |
| thumbnail | Option\<String\> | 썸네일 URL |

### DownloadTask

다운로드 작업 (런타임 + DB 저장).

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 고유 ID (auto-increment) |
| video_url | String | 대상 비디오 URL |
| video_id | String | YouTube 비디오 ID |
| title | String | 비디오 제목 |
| format_id | String | 선택된 포맷 ID |
| quality_label | String | 화질 라벨 (e.g., "1080p") |
| output_path | String | 저장 파일 경로 |
| status | DownloadStatus | 현재 상태 |
| progress | f32 | 진행률 (0.0 ~ 100.0) |
| speed | Option\<String\> | 다운로드 속도 |
| eta | Option\<String\> | 예상 남은 시간 |
| error_message | Option\<String\> | 오류 메시지 |
| created_at | i64 | 생성 시각 (Unix timestamp) |
| completed_at | Option\<i64\> | 완료 시각 |

### DownloadStatus (Enum)

```
Pending → Downloading → Completed
    ↓         ↓
  Cancelled  Paused → Downloading
              ↓
            Failed → Downloading (retry)
```

| Variant | Description |
|---------|-------------|
| Pending | 큐에서 대기 중 |
| Downloading | 다운로드 진행 중 |
| Paused | 일시정지 |
| Completed | 완료 |
| Failed | 오류로 실패 |
| Cancelled | 사용자가 취소 |

### DownloadHistory

완료된 다운로드 이력 (DB 저장).

| Field | Type | Description |
|-------|------|-------------|
| id | u64 | 고유 ID |
| video_url | String | 비디오 URL |
| video_id | String | YouTube 비디오 ID |
| title | String | 비디오 제목 |
| quality_label | String | 다운로드 화질 |
| format | String | 파일 포맷 (mp4, mp3 등) |
| file_path | String | 저장된 파일 경로 |
| file_size | Option\<u64\> | 파일 크기 (바이트) |
| downloaded_at | i64 | 다운로드 완료 시각 |

**Uniqueness**: `video_id`로 중복 다운로드 감지.

### AppSettings

앱 설정 (tauri-plugin-store로 저장).

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| download_path | String | OS 다운로드 폴더 | 기본 저장 경로 |
| default_quality | String | "1080p" | 기본 화질 |
| max_concurrent | u32 | 3 | 동시 다운로드 수 |
| filename_template | String | "%(title)s.%(ext)s" | 파일 명명 규칙 |
| cookie_browser | Option\<String\> | None | 쿠키 브라우저 |
| auto_update_ytdlp | bool | true | yt-dlp 자동 업데이트 |

## Relationships

```
Channel 1──* PlaylistEntry (채널의 비디오 목록)
Playlist 1──* PlaylistEntry (재생목록의 비디오 목록)
PlaylistEntry *──1 VideoInfo (상세 정보 조회 시)
VideoInfo 1──* FormatInfo (사용 가능한 포맷)
DownloadTask *──1 VideoInfo (다운로드 대상)
DownloadTask → DownloadHistory (완료 시 이력 생성)
```

## Storage Strategy

| Data | Storage | Reason |
|------|---------|--------|
| AppSettings | tauri-plugin-store (`settings.json`) | 키-값 저장, 이미 프로젝트에 포함 |
| DownloadTask | SQLite (`downloads` table) + 인메모리 상태 | 구조화된 쿼리, 앱 재시작 후 큐 복원 |
| DownloadHistory | SQLite (`history` table) | 이력 조회, 중복 감지 쿼리 |
| VideoInfo, FormatInfo | 인메모리 only | 임시 데이터, API 호출 시마다 새로 조회 |
| yt-dlp/ffmpeg 바이너리 | 앱 데이터 디렉토리 | OS별 표준 경로 |
