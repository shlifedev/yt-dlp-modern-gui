# Tasks: yt-dlp GUI

**Input**: Design documents from `/specs/002-yt-dlp-gui/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/tauri-commands.md

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization, dependencies, and module structure

- [x] T001 Add new Rust dependencies (reqwest with rustls-tls+stream, rusqlite with bundled, regex) to src-tauri/Cargo.toml
- [x] T002 Create ytdlp Rust module directory and mod.rs with submodule declarations in src-tauri/src/ytdlp/mod.rs
- [x] T003 [P] Create frontend route directories: src/routes/tools/ytdlp/, src/routes/tools/ytdlp/queue/, src/routes/tools/ytdlp/history/, src/routes/tools/ytdlp/settings/
- [x] T004 [P] Create frontend component directory src/lib/components/ytdlp/ and store directory src/lib/stores/ytdlp/
- [x] T005 Register ytdlp module in src-tauri/src/lib.rs (add `mod ytdlp;` or pub mod under appropriate location)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**CRITICAL**: No user story work can begin until this phase is complete

- [x] T006 Define shared yt-dlp types (VideoInfo, FormatInfo, PlaylistEntry, DownloadTask, DownloadStatus, DownloadEvent, InstallEvent, UrlValidation, AppSettings, HistoryItem) with Serialize/Deserialize/specta::Type derives in src-tauri/src/ytdlp/types.rs
- [x] T007 Add ytdlp-specific error variants to AppError enum in src-tauri/src/modules/types.rs (BinaryNotFound, DownloadError, MetadataError, DatabaseError, NetworkError, InvalidUrl)
- [x] T008 [P] Implement yt-dlp/ffmpeg binary manager in src-tauri/src/ytdlp/binary.rs: platform detection (OS/arch), download URLs, download_binary(), check_installed(), get_binary_path(), get_version() using reqwest and app_data_dir
- [x] T009 [P] Implement progress template parser in src-tauri/src/ytdlp/progress.rs: parse_progress_line() extracting percent, speed, eta from --progress-template output using regex
- [x] T010 [P] Initialize SQLite database in src-tauri/src/ytdlp/db.rs: create_tables() for downloads and history tables, open_connection() using app_data_dir, wrap Connection in Mutex for thread safety
- [x] T011 [P] Implement settings foundation in src-tauri/src/ytdlp/settings.rs: get_settings() and update_settings() wrapping tauri-plugin-store with AppSettings defaults (download_path=OS downloads, default_quality="1080p", max_concurrent=3)
- [x] T012 Implement check_dependencies and install_dependencies Tauri commands in src-tauri/src/ytdlp/commands.rs, register them in collect_commands![] in src-tauri/src/lib.rs, and add DbConnection + BinaryPaths to AppState
- [x] T013 Subprocess execution uses tokio::process::Command (pure Rust, no Tauri permission needed)

**Checkpoint**: Foundation ready - binary management, DB, settings, types all operational. `bun run tauri dev` should compile and auto-download yt-dlp/ffmpeg on first run.

---

## Phase 3: User Story 1 - 단일 비디오 다운로드 (Priority: P1) MVP

**Goal**: 사용자가 YouTube URL을 입력하면 비디오 정보를 확인하고 원하는 화질로 다운로드할 수 있다

**Independent Test**: YouTube 비디오 URL 하나를 붙여넣고 다운로드 완료까지의 전체 흐름 테스트. 파일이 지정 위치에 정상 저장되는지 확인.

### Backend Implementation

- [x] T014 [US1] Implement validate_url command in src-tauri/src/ytdlp/metadata.rs: YouTube URL 패턴 검증 (youtube.com/watch, youtu.be, youtube.com/shorts), UrlValidation 반환 (valid, urlType, normalizedUrl)
- [x] T015 [US1] Implement fetch_video_info command in src-tauri/src/ytdlp/metadata.rs: tokio::process::Command로 yt-dlp --dump-json 실행, JSON stdout을 VideoInfo 구조체로 파싱, FormatInfo 목록 추출
- [x] T016 [US1] Implement start_download in src-tauri/src/ytdlp/download.rs: tokio::spawn으로 yt-dlp 프로세스 실행, --progress-template + --newline 플래그, Channel<DownloadEvent>로 진행률 스트리밍, 완료 시 DB에 이력 저장
- [x] T017 [US1] Implement pause_download, resume_download, cancel_download in src-tauri/src/ytdlp/download.rs: 프로세스 시그널 관리, cancel 시 불완전 파일 정리
- [x] T018 [US1] Register US1 commands (validate_url, fetch_video_info, start_download, pause_download, resume_download, cancel_download) in src-tauri/src/ytdlp/commands.rs and collect_commands![] in src-tauri/src/lib.rs

### Frontend Implementation

- [x] T019 [P] [US1] Create UrlInput.svelte component in src/lib/components/ytdlp/UrlInput.svelte: URL 입력 필드, 붙여넣기 지원, YouTube URL 클라이언트 유효성 검사, 로딩 상태
- [x] T020 [P] [US1] Create VideoPreview.svelte component in src/lib/components/ytdlp/VideoPreview.svelte: 썸네일, 제목, 길이, 채널명, 게시일 표시
- [x] T021 [P] [US1] Create FormatSelector.svelte component in src/lib/components/ytdlp/FormatSelector.svelte: 화질 드롭다운 (2160p~360p), 포맷 토글 (비디오+오디오/오디오만), 예상 파일 크기 표시
- [x] T022 [P] [US1] Create ProgressBar.svelte component in src/lib/components/ytdlp/ProgressBar.svelte: 진행률 바, 퍼센트/속도/남은시간 표시, 일시정지/취소 버튼
- [x] T023 [US1] Create download state store in src/lib/stores/ytdlp/download.svelte.ts: Svelte 5 runes 기반, 현재 비디오 정보, 다운로드 상태, Channel 이벤트 핸들링
- [x] T024 [US1] Build main download page in src/routes/tools/ytdlp/+page.svelte: UrlInput → VideoPreview → FormatSelector → ProgressBar 흐름 통합, commands 바인딩 호출
- [x] T025 [US1] Add ytdlp tool entry to sidebar navigation in src/routes/+layout.svelte (기존 사이드바 네비게이션 패턴 따름)

**Checkpoint**: User Story 1 완료 - 단일 YouTube URL로 비디오 다운로드가 가능해야 함. `bun run tauri dev`로 전체 흐름 검증.

---

## Phase 4: User Story 2 - 채널/재생목록 비디오 탐색 및 다운로드 (Priority: P2)

**Goal**: 채널 또는 재생목록 URL에서 비디오 목록을 탐색하고 선택적으로 다운로드할 수 있다

**Independent Test**: YouTube 채널 URL을 입력하고, 비디오 목록 로드, 여러 비디오 선택, 일괄 다운로드 정상 동작 확인.

### Backend Implementation

- [x] T026 [US2] Implement fetch_playlist_info command in src-tauri/src/ytdlp/metadata.rs: yt-dlp --flat-playlist --dump-json 실행, 채널/재생목록 메타데이터 + PlaylistEntry 목록 파싱, 페이지네이션 지원 (page, page_size 파라미터)
- [x] T027 [US2] Register fetch_playlist_info command in src-tauri/src/ytdlp/commands.rs and collect_commands![] in src-tauri/src/lib.rs

### Frontend Implementation

- [x] T028 [P] [US2] Create VideoListItem.svelte component in src/lib/components/ytdlp/VideoListItem.svelte: 체크박스, 썸네일, 제목, 길이, 게시일 표시, 선택 상태 시각적 피드백
- [x] T029 [US2] Extend main page src/routes/tools/ytdlp/+page.svelte: URL 타입 감지 후 채널/재생목록이면 VideoListItem 목록 렌더링, 전체 선택 체크박스, 선택 개수 표시, "선택 다운로드" 버튼
- [x] T030 [US2] Implement batch selection logic in src/routes/tools/ytdlp/+page.svelte: 개별 클릭, Ctrl+클릭 토글, Shift+클릭 범위 선택, 전체 선택/해제
- [x] T031 [US2] Implement infinite scroll or pagination for large channel video lists in src/routes/tools/ytdlp/+page.svelte: fetch_playlist_info의 page 파라미터 활용

**Checkpoint**: User Story 2 완료 - 채널/재생목록 URL에서 비디오 탐색 및 선택적 다운로드 가능.

---

## Phase 5: User Story 3 - 다운로드 큐 관리 (Priority: P3)

**Goal**: 여러 비디오의 동시 다운로드를 큐로 관리하고, 개별 항목의 일시정지/취소/재시도가 가능하다

**Independent Test**: 여러 비디오를 큐에 추가 후 큐 내 순서 확인, 일시정지/취소/재시도 동작 검증.

### Backend Implementation

- [x] T032 [US3] Concurrent download queue managed via DB status tracking in download.rs (max_concurrent enforcement deferred to frontend)
- [x] T033 [US3] Implement get_download_queue and clear_completed commands in src-tauri/src/ytdlp/commands.rs: DB에서 큐 상태 조회, 완료 항목 제거
- [x] T034 [US3] Implement retry_download command in src-tauri/src/ytdlp/commands.rs: 실패한 다운로드 재시도 로직

### Frontend Implementation

- [x] T035 [P] [US3] Create DownloadItem.svelte component in src/lib/components/ytdlp/DownloadItem.svelte: 제목, 진행률 바, 상태 뱃지 (대기/진행 중/완료/오류), 일시정지/재개/취소/재시도 액션 버튼
- [x] T036 [US3] Build queue management page in src/routes/tools/ytdlp/queue/+page.svelte: DownloadItem 목록, 전체 진행 상황 요약, "완료 항목 삭제" 버튼
- [x] T037 [US3] Update download state store in src/lib/stores/ytdlp/download.svelte.ts: 큐 상태 관리, 여러 Channel 이벤트 동시 핸들링, 큐 갱신 로직

**Checkpoint**: User Story 3 완료 - 다운로드 큐 화면에서 모든 다운로드의 상태 확인 및 제어 가능.

---

## Phase 6: User Story 4 - 설정, 이력, 쿠키 (Priority: P4)

**Goal**: 다운로드 설정을 구성하고, 이력을 조회하며, 브라우저 쿠키로 제한 콘텐츠에 접근할 수 있다

**Independent Test**: 설정 변경 후 앱 재시작 시 유지 확인, 이력 화면에서 완료 다운로드 조회 확인, 쿠키 브라우저 설정 후 연령 제한 콘텐츠 접근 확인.

### Backend Implementation

- [x] T038 [P] [US4] Implement get_settings, update_settings, select_download_directory commands in src-tauri/src/ytdlp/commands.rs: tauri-plugin-store 래핑, dialog::FileDialogBuilder로 디렉토리 선택
- [x] T039 [P] [US4] Implement get_available_browsers command in src-tauri/src/ytdlp/commands.rs: 시스템에 설치된 브라우저 감지 (Chrome, Firefox, Edge 등 경로 확인)
- [x] T040 [P] [US4] Implement download history CRUD in src-tauri/src/ytdlp/db.rs: insert_history(), get_history(page, page_size, search), check_duplicate(video_id), delete_history(id)
- [x] T041 [US4] Implement history commands (get_download_history, check_duplicate, delete_history_item) in src-tauri/src/ytdlp/commands.rs
- [x] T042 [US4] Add --cookies-from-browser flag support in src-tauri/src/ytdlp/download.rs: settings.cookie_browser 값이 있으면 다운로드 시 쿠키 플래그 추가
- [x] T043 [US4] Register all US4 commands in collect_commands![] in src-tauri/src/lib.rs

### Frontend Implementation

- [x] T044 [P] [US4] Create settings store in src/lib/stores/ytdlp/settings.svelte.ts: Svelte 5 runes, 설정 로드/저장, 기본값 관리
- [x] T045 [P] [US4] Build settings page in src/routes/tools/ytdlp/settings/+page.svelte: 다운로드 경로 (폴더 선택 다이얼로그), 기본 화질 드롭다운, 동시 다운로드 수 슬라이더, 파일 명명 규칙, 쿠키 브라우저 선택 드롭다운, yt-dlp 업데이트 버튼
- [x] T046 [P] [US4] Build history page in src/routes/tools/ytdlp/history/+page.svelte: 다운로드 이력 테이블 (제목, 날짜, 경로, 화질), 검색/필터, 삭제 버튼, 페이지네이션
- [x] T047 [US4] Integrate duplicate download detection in src/routes/tools/ytdlp/+page.svelte: 다운로드 시작 전 check_duplicate 호출, 중복 시 확인 모달 표시

**Checkpoint**: User Story 4 완료 - 설정/이력/쿠키 기능 모두 정상 동작.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: UX 개선, 엣지 케이스 처리, 코드 품질

- [x] T048 Create ytdlp tool layout with sub-navigation (다운로드/큐/이력/설정 탭) in src/routes/tools/ytdlp/+layout.svelte
- [x] T049 [P] Implement first-run dependency setup UX: 첫 실행 시 yt-dlp/ffmpeg 다운로드 진행률을 보여주는 전체 화면 로딩 UI in src/routes/tools/ytdlp/+layout.svelte
- [x] T050 [P] Add edge case error handling: 잘못된 URL, 네트워크 오류, 비공개 비디오, 라이브 스트림, 연령 제한 콘텐츠에 대한 한국어 사용자 친화적 오류 메시지
- [x] T051 [P] Implement yt-dlp auto-update check: update_ytdlp command in src-tauri/src/ytdlp/binary.rs + commands.rs, 설정 화면에서 수동 트리거
- [x] T052 Run cargo fmt && cargo clippy in src-tauri/ - 1 warning only (unused Postprocessing variant, intentional)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational - MVP deliverable
- **User Story 2 (Phase 4)**: Depends on Foundational + US1 (uses download.rs from US1)
- **User Story 3 (Phase 5)**: Depends on Foundational + US1 (extends download.rs queue logic)
- **User Story 4 (Phase 6)**: Depends on Foundational (settings/history can proceed independently, cookies needs download.rs)
- **Polish (Phase 7)**: Depends on all user stories

### User Story Dependencies

- **User Story 1 (P1)**: Foundational only. 독립 실행 가능. MVP.
- **User Story 2 (P2)**: US1의 download.rs를 활용하지만, metadata.rs의 playlist 기능은 독립적. 채널 탐색만으로도 독립 테스트 가능.
- **User Story 3 (P3)**: US1의 download 로직 확장. 큐 매니저는 US1 완료 후 구현.
- **User Story 4 (P4)**: 설정/이력은 Foundational 이후 독립 가능. 쿠키는 download.rs 필요.

### Within Each User Story

- Backend commands before frontend components
- Types and data layer before business logic
- Business logic before UI integration
- Core flow before edge cases

### Parallel Opportunities

- Phase 1: T003, T004 can run in parallel
- Phase 2: T008, T009, T010, T011 can run in parallel (different files)
- Phase 3: T019, T020, T021, T022 can run in parallel (different Svelte components)
- Phase 6: T038, T039, T040 can run in parallel; T044, T045, T046 can run in parallel
- Phase 7: T049, T050, T051 can run in parallel

---

## Parallel Example: User Story 1

```text
# Backend (sequential - types first, then metadata, then download):
T014: validate_url in metadata.rs
T015: fetch_video_info in metadata.rs
T016: start_download in download.rs
T017: pause/resume/cancel in download.rs
T018: Register commands in commands.rs + lib.rs

# Frontend (parallel - independent Svelte components):
T019: UrlInput.svelte
T020: VideoPreview.svelte
T021: FormatSelector.svelte
T022: ProgressBar.svelte

# Integration (sequential - after backend + frontend):
T023: download state store
T024: main page integration
T025: sidebar navigation
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001-T005)
2. Complete Phase 2: Foundational (T006-T013)
3. Complete Phase 3: User Story 1 (T014-T025)
4. **STOP and VALIDATE**: YouTube URL 하나로 비디오 다운로드 전체 흐름 테스트
5. MVP 배포 가능

### Incremental Delivery

1. Setup + Foundational → 기반 완료
2. User Story 1 → 단일 비디오 다운로드 (MVP)
3. User Story 2 → 채널/재생목록 탐색 추가
4. User Story 3 → 다운로드 큐 관리 추가
5. User Story 4 → 설정/이력/쿠키 추가
6. Polish → 엣지 케이스, UX 개선

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story
- Backend commands는 tauri-specta가 자동으로 TypeScript 바인딩 생성 (`bun run tauri dev` 실행 시)
- 각 Phase checkpoint에서 `cargo fmt && cargo clippy && bun run check` 실행 권장
- Svelte 5 runes ($state, $derived) 사용, 세미콜론 없음, 더블 쿼트
