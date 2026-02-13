# Quickstart: yt-dlp GUI

**Branch**: `002-yt-dlp-gui` | **Date**: 2026-02-13

## Prerequisites

- Rust (최신 stable)
- Node.js 18+ / Bun
- Tauri CLI (`bun run tauri`)

## New Dependencies

### Rust (Cargo.toml)

```toml
# HTTP client for downloading yt-dlp/ffmpeg binaries
reqwest = { version = "0.12", features = ["rustls-tls", "stream"] }

# SQLite for download history
rusqlite = { version = "0.32", features = ["bundled"] }

# Regex for progress parsing
regex = "1"
```

### Frontend (package.json)

추가 프론트엔드 의존성 없음. 기존 `@tauri-apps/api`, `@tauri-apps/plugin-store`, Skeleton UI 활용.

## Development Setup

```bash
# 1. 의존성 설치
bun install

# 2. 개발 서버 실행
bun run tauri dev

# 3. 첫 실행 시 yt-dlp/ffmpeg 자동 다운로드됨
#    앱 데이터 디렉토리: %APPDATA%/vibe-tinker (Windows)
```

## Key Implementation Order

1. **yt-dlp 바이너리 관리자** - 자동 다운로드/업데이트 (FR-012)
2. **URL 검증 및 메타데이터 조회** - `--dump-json` 파싱 (FR-001, FR-002)
3. **단일 비디오 다운로드** - 진행률 스트리밍 (FR-003, FR-004, FR-005)
4. **다운로드 큐** - 동시 다운로드 관리 (FR-008)
5. **채널/재생목록 탐색** - `--flat-playlist` (FR-006, FR-007, FR-011)
6. **설정 관리** - tauri-plugin-store (FR-009)
7. **다운로드 이력** - SQLite (FR-014)
8. **브라우저 쿠키** - `--cookies-from-browser` (FR-013)

## Architecture Overview

```
┌─────────────────────────────────┐
│       SvelteKit Frontend        │
│  (Skeleton UI + Tailwind CSS)   │
│                                 │
│  Routes:                        │
│  /                 → 메인 (URL 입력)│
│  /tools/ytdlp/     → 다운로드 화면  │
│  /tools/ytdlp/queue → 큐 관리     │
│  /tools/ytdlp/history → 이력     │
│  /tools/ytdlp/settings → 설정   │
├─────────────────────────────────┤
│     Tauri Commands + Events     │
│  (tauri-specta 자동 바인딩)      │
├─────────────────────────────────┤
│        Rust Backend             │
│                                 │
│  Modules:                       │
│  ytdlp/binary.rs  → 바이너리 관리 │
│  ytdlp/metadata.rs → 메타데이터  │
│  ytdlp/download.rs → 다운로드    │
│  ytdlp/progress.rs → 진행률 파싱 │
│  db.rs            → SQLite      │
│  settings.rs      → 설정        │
└─────────────────────────────────┘
         │
         ▼
    yt-dlp / ffmpeg
    (앱 데이터 디렉토리)
```

## Tauri Command → Frontend Binding Flow

```
1. Rust: #[tauri::command] #[specta::specta] fn my_cmd() → Result<T, AppError>
2. lib.rs: collect_commands![my_cmd]
3. bun run tauri dev → 자동 bindings.ts 생성
4. Svelte: import { commands } from "$lib/bindings"
5. Svelte: const result = await commands.myCmd()
```
