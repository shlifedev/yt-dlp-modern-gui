# Research: yt-dlp GUI

**Branch**: `002-yt-dlp-gui` | **Date**: 2026-02-13

## 1. yt-dlp 프로세스 통합 (Rust)

**Decision**: `tokio::process::Command`로 yt-dlp를 비동기 서브프로세스로 실행하고, stdout/stderr를 라인 단위로 스트리밍 파싱한다.

**Rationale**: 직접 프로세스 제어가 가능하고, yt-dlp의 모든 플래그를 활용할 수 있다. 러스트 래퍼 크레이트(youtube_dl, ytdlp-rs)는 기능이 제한적이고 업데이트가 느리다.

**Alternatives considered**:
- `youtube_dl` 크레이트: JSON 파싱은 편리하나 progress streaming 미지원
- `ytdlp-rs` 크레이트: 얇은 래퍼지만 비동기 progress 처리 부족
- Python yt-dlp 라이브러리 임베딩: Python 런타임 의존성 추가로 복잡도 증가

**Key patterns**:
- 메타데이터 조회: `yt-dlp --dump-json <url>` → JSON stdout 파싱
- 채널/재생목록 목록: `yt-dlp --flat-playlist --dump-json <url>` → 비디오별 JSON 라인
- 다운로드: `yt-dlp --newline --progress-template "download:%(progress._percent_str)s|%(progress._speed_str)s|%(progress._eta_str)s" -f <format> -o <path> <url>`
- 취소: `child.kill()` via tokio

## 2. yt-dlp/ffmpeg 자동 다운로드

**Decision**: 첫 실행 시 GitHub Releases API에서 플랫폼별 바이너리를 다운로드하여 앱 데이터 디렉토리에 저장한다.

**Rationale**: 사용자 진입 장벽을 없애고, 앱이 자체적으로 의존성을 관리할 수 있다. `reqwest`로 HTTP 다운로드, `std::env::consts::OS`로 플랫폼 감지.

**Download URLs**:
- yt-dlp Windows: `https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe`
- yt-dlp macOS: `https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_macos`
- yt-dlp Linux: `https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp`
- ffmpeg: BtbN/FFmpeg-Builds (Windows/Linux), martin-riedl.de (macOS)

**Update mechanism**: `yt-dlp --update` 내장 명령 활용, 주기적 또는 수동 업데이트 트리거.

**Alternatives considered**:
- 앱에 바이너리 번들링: 앱 크기 증가, 빠르게 구버전화
- 시스템 패키지 매니저: 플랫폼마다 다르고 관리자 권한 필요
- npm ffmpeg-static: Node.js 의존성 추가

## 3. 다운로드 진행률 파싱

**Decision**: `--progress-template` 플래그로 구조화된 출력을 사용하고, `--newline`으로 라인 단위 파싱한다.

**Rationale**: yt-dlp 공식 문서에서 기본 stdout 파싱은 비권장하고 `--progress-template` 사용을 권장한다. 포맷 변경에 안전하다.

**Template format**:
```
--progress-template "download:%(progress._percent_str)s|%(progress._speed_str)s|%(progress._eta_str)s|%(progress._total_bytes_str)s"
```

**Available variables**: `_percent_str`, `_speed_str`, `_eta_str`, `_total_bytes_str`, `downloaded_bytes`, `total_bytes`

**Alternatives considered**:
- 기본 출력 정규식 파싱: 향후 포맷 변경 시 깨짐
- 파일 크기 폴링: 속도/ETA 정보 부재

## 4. 브라우저 쿠키 추출

**Decision**: `--cookies-from-browser <browser>` 플래그 사용. 설정에서 브라우저 선택 가능.

**Rationale**: yt-dlp 내장 기능으로 Chrome, Firefox, Edge, Brave 등 주요 브라우저 지원. 별도 구현 불필요.

**Supported browsers**: Chrome, Chromium, Firefox, Edge, Opera, Brave, Vivaldi, Safari (macOS)

**Limitations**:
- 일부 환경에서 브라우저가 실행 중이면 쿠키 DB 잠금 발생 가능
- Linux Flatpak 브라우저는 커스텀 경로 필요
- Edge에서 간헐적 문제 보고

**Alternatives considered**:
- 쿠키 파일 수동 지정 (`--cookies`): 사용자 번거로움
- 자체 쿠키 추출 구현: yt-dlp가 이미 지원하므로 불필요

## 5. Tauri 이벤트 시스템 (실시간 진행률)

**Decision**: Tauri `Channel<T>`을 사용하여 다운로드 진행률을 커맨드 스코프로 스트리밍한다. 글로벌 이벤트(`app.emit()`)는 다운로드 완료/오류 알림에 사용한다.

**Rationale**: Channel은 순서 보장과 커맨드별 격리를 제공하여 여러 동시 다운로드의 진행률을 독립적으로 전달할 수 있다.

**Key patterns**:
- Rust: `on_event: Channel<DownloadEvent>` → `on_event.send(progress)`
- Frontend: `new Channel<DownloadEvent>()` → `onmessage` 핸들러
- 이벤트 타입: `#[serde(tag = "event", content = "data")]` enum 사용

**Alternatives considered**:
- 프론트엔드 폴링: 레이턴시 높음, 비효율적
- WebSocket: 로컬 앱에서는 과도한 복잡성

## 6. 데이터 저장

**Decision**: tauri-plugin-store (이미 설치됨)로 설정 저장, rusqlite로 다운로드 이력/큐 관리.

**Rationale**: plugin-store는 키-값 설정에 적합하고 이미 프로젝트에 포함. SQLite는 다운로드 이력 조회, 중복 감지 등 구조화된 쿼리에 적합하다.

**New dependencies needed**:
- `reqwest` (with `rustls-tls`): HTTP 다운로드 (yt-dlp/ffmpeg 바이너리)
- `rusqlite` (with `bundled`): SQLite DB
- `regex`: 진행률 파싱 보조

**Alternatives considered**:
- JSON 파일 직접 관리: 쿼리 기능 부재, 대량 이력 시 성능 문제
- tauri-plugin-sql: sqlx 기반이라 무거움, rusqlite가 더 가벼움
