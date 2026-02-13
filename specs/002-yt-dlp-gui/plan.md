# Implementation Plan: yt-dlp GUI

**Branch**: `002-yt-dlp-gui` | **Date**: 2026-02-13 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/002-yt-dlp-gui/spec.md`

## Summary

YouTube 비디오 및 채널의 비디오를 다운로드하는 yt-dlp GUI 데스크톱 앱. Tauri 2.0 (Rust 백엔드) + SvelteKit (프론트엔드) 기반의 기존 프로젝트에 yt-dlp 래핑 기능을 추가한다. Rust에서 yt-dlp를 비동기 서브프로세스로 실행하고, Tauri Channel을 통해 실시간 진행률을 프론트엔드로 스트리밍한다. 설정은 tauri-plugin-store, 다운로드 이력은 SQLite로 관리한다.

## Technical Context

**Language/Version**: Rust 2021 edition + TypeScript 5.6 + Svelte 5
**Primary Dependencies**: Tauri 2.0, tauri-specta, tokio, reqwest, rusqlite, serde, Skeleton UI 4, Tailwind CSS 4
**Storage**: tauri-plugin-store (설정), SQLite via rusqlite (다운로드 이력/큐)
**Testing**: `cargo test` (Rust), `bun run check` (TypeScript)
**Target Platform**: Windows, macOS, Linux (데스크톱)
**Project Type**: Desktop app (Tauri = Rust backend + SvelteKit frontend)
**Performance Goals**: 메타데이터 로딩 5초 이내, 채널 목록 첫 페이지 10초 이내, 진행률 1초 간격 업데이트
**Constraints**: yt-dlp/ffmpeg 외부 바이너리 의존, YouTube URL만 지원
**Scale/Scope**: 단일 사용자, 동시 다운로드 최대 3개 (기본값), 이력 무제한

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

Constitution이 프로젝트별로 정의되지 않음 (템플릿 상태). 게이트 위반 없음. 기존 프로젝트 컨벤션(CLAUDE.md) 준수:
- [x] tauri-specta로 타입 안전한 커맨드/이벤트 바인딩
- [x] thiserror로 에러 타입 정의
- [x] snake_case (Rust) / camelCase (TS) 명명 규칙
- [x] cargo fmt + clippy 린팅

## Project Structure

### Documentation (this feature)

```text
specs/002-yt-dlp-gui/
├── plan.md              # This file
├── spec.md              # Feature specification
├── research.md          # Phase 0 research findings
├── data-model.md        # Phase 1 data model
├── quickstart.md        # Phase 1 quickstart guide
├── contracts/
│   └── tauri-commands.md # Phase 1 Tauri command contracts
└── tasks.md             # Phase 2 output (via /speckit.tasks)
```

### Source Code (repository root)

```text
/src                              # Frontend (SvelteKit)
├── routes/
│   ├── tools/
│   │   └── ytdlp/
│   │       ├── +page.svelte      # 메인 화면 (URL 입력 + 비디오 정보)
│   │       ├── +layout.svelte    # ytdlp 도구 레이아웃
│   │       ├── queue/
│   │       │   └── +page.svelte  # 다운로드 큐 관리
│   │       ├── history/
│   │       │   └── +page.svelte  # 다운로드 이력
│   │       └── settings/
│   │           └── +page.svelte  # 다운로드 설정
│   ├── +page.svelte              # 앱 홈
│   └── +layout.svelte            # 전역 레이아웃 (사이드바)
├── lib/
│   ├── bindings.ts               # Auto-generated (DO NOT EDIT)
│   ├── components/
│   │   └── ytdlp/
│   │       ├── UrlInput.svelte       # URL 입력 컴포넌트
│   │       ├── VideoPreview.svelte   # 비디오 정보 미리보기
│   │       ├── FormatSelector.svelte # 화질/포맷 선택
│   │       ├── DownloadItem.svelte   # 큐 내 다운로드 항목
│   │       ├── ProgressBar.svelte    # 진행률 바
│   │       └── VideoListItem.svelte  # 채널/재생목록 비디오 항목
│   └── stores/
│       └── ytdlp/
│           ├── download.ts       # 다운로드 상태 스토어
│           └── settings.ts       # 설정 스토어

/src-tauri                        # Backend (Rust)
├── src/
│   ├── lib.rs                    # App 초기화, 상태, 커맨드 등록
│   ├── main.rs                   # Entry point
│   ├── command.rs                # 기존 커맨드
│   ├── modules/
│   │   ├── types.rs              # 공유 타입, 이벤트
│   │   └── logger.rs             # 로깅
│   └── ytdlp/
│       ├── mod.rs                # 모듈 선언
│       ├── binary.rs             # yt-dlp/ffmpeg 바이너리 관리 (다운로드, 경로, 버전)
│       ├── metadata.rs           # 메타데이터 조회 (--dump-json, --flat-playlist)
│       ├── download.rs           # 다운로드 실행 + 진행률 스트리밍
│       ├── progress.rs           # 진행률 파싱 (--progress-template)
│       ├── commands.rs           # Tauri 커맨드 정의
│       ├── types.rs              # yt-dlp 관련 타입 (VideoInfo, FormatInfo 등)
│       ├── db.rs                 # SQLite DB 초기화, 마이그레이션, CRUD
│       └── settings.rs           # 설정 읽기/쓰기 (tauri-plugin-store 래퍼)
└── Cargo.toml
```

**Structure Decision**: 기존 프로젝트 구조(Tauri + SvelteKit)를 유지하며, `src-tauri/src/ytdlp/` 모듈과 `src/routes/tools/ytdlp/` 라우트를 추가하여 기존 도구들과 동일한 패턴으로 통합한다. 기존의 사이드바 네비게이션 UI에 자연스럽게 추가된다.

## Complexity Tracking

위반 사항 없음. 기존 프로젝트 패턴을 따르며, 새로운 아키텍처 패턴 도입 없이 yt-dlp 서브프로세스 래핑에 집중한다.
