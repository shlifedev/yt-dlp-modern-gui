# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

Tauri 2.0 desktop application with SvelteKit frontend and Rust backend. A collection of developer tools with a sidebar navigation UI.

## Principles

1. **Lint before commit**: Always run `cargo fmt && cargo clippy` before committing
2. **Type-safe errors**: Use `thiserror` for custom error types in Rust
3. **Keep CLAUDE.md updated**: Update this file when adding new features or patterns
4. **KISS**: Keep code simple, avoid unnecessary abstraction

## Build Commands

```bash
# Development
bun run tauri dev          # Run app in dev mode

# Frontend only
bun run dev                # Vite dev server
bun run check              # TypeScript type check

# Backend only
cd src-tauri
cargo check                # Check compilation
cargo fmt                  # Format code
cargo clippy               # Lint code

# Production
bun run tauri build        # Build release
```

## Project Structure

```
/src                          # Frontend (SvelteKit)
├── routes/
│   ├── tools/
│   │   ├── encoding/
│   │   │   └── base64/         # Base64 encoder/decoder
│   │   └── llm-toolset/
│   │       ├── motion-descriptor/      # Mouse motion to LLM prompt
│   │       └── sprite-sheet-describer/ # Sprite sheet grid annotator
│   ├── +page.svelte         # Pages (+ prefix = SvelteKit special file)
│   └── +layout.svelte       # Layouts
├── lib/
│   └── bindings.ts          # Auto-generated (DO NOT EDIT)

/src-tauri                    # Backend (Rust)
├── src/
│   ├── lib.rs               # App initialization, state management
│   ├── main.rs              # Entry point
│   ├── command.rs           # Tauri commands
│   └── modules/
│       ├── types.rs         # Shared types and events
│       └── logger.rs        # Logging utility
└── Cargo.toml
```

## TypeScript/Svelte Guidelines

### Imports
```typescript
import { invoke } from "@tauri-apps/api/core"
import { onMount } from "svelte"
import { commands } from "$lib/bindings"
```

### Svelte 5 Runes
```typescript
let count = $state(0)
let doubled = $derived(count * 2)
```

### Formatting
- No semicolons
- Double quotes for strings
- 2 spaces indentation

## Rust Guidelines

### Tauri Commands
```rust
#[tauri::command]
#[specta::specta]
pub fn my_command(state: State<'_, Mutex<AppState>>) -> Result<String, AppError> {
    // implementation
}
```

After creating a command:
1. Add to `collect_commands![]` in `lib.rs`
2. Run app to regenerate TypeScript bindings

### Error Handling
Use thiserror for typed errors:
```rust
#[derive(Debug, thiserror::Error, specta::Type, serde::Serialize)]
pub enum AppError {
    #[error("File error: {0}")]
    FileError(String),
    #[error("{0}")]
    Custom(String),
}
```

### Types for Frontend
```rust
#[derive(Serialize, Deserialize, specta::Type, Clone)]
pub struct MyData {
    pub id: u32,
    #[serde(rename = "userName")]
    pub user_name: String,
}
```

### Tauri Events
```rust
#[derive(Clone, specta::Type, tauri_specta::Event, Serialize)]
pub struct MyEvent {
    pub message: String,
}
```
Register in `lib.rs`: `collect_events![MyEvent]`

### Naming Conventions
| Type | Convention | Example |
|------|------------|---------|
| Files | snake_case | `my_module.rs` |
| Functions | snake_case | `get_user()` |
| Types/Structs | PascalCase | `AppState` |
| Constants | UPPER_SNAKE | `MAX_SIZE` |

### State Management
```rust
pub struct AppState {
    pub count: u32,
}

// In commands:
fn my_cmd(state: State<'_, Mutex<AppState>>) -> Result<(), AppError> {
    let mut state = state.lock().unwrap();
    state.count += 1;
    Ok(())
}
```

## Before Committing

```bash
cd src-tauri && cargo fmt && cargo clippy
bun run check
```

## Common Tasks

### Adding a New Command
1. Define in `src-tauri/src/command.rs` with `#[tauri::command]` + `#[specta::specta]`
2. Add to `collect_commands![]` in `lib.rs`
3. Run `bun run tauri dev` to regenerate bindings
4. Use via `commands.myCommand()` in frontend

### Adding a New Route
1. Create `src/routes/my-route/+page.svelte`
2. Available at `/my-route`

### Adding State Fields
1. Add to `AppState` struct in `lib.rs`
2. Update initialization in `.manage()` call

## Tools

### Sprite Sheet Describer

스프라이트 시트를 n×n 그리드로 나누고 각 셀에 설명을 추가하는 도구입니다.

**Features:**
- 이미지 파일 로드 (drag & drop or file picker)
- 사용자 지정 그리드 크기 (1×1 ~ 50×50)
- 단일/멀티 셀 선택 모드
- 프리셋 태그 (지형, 구조물, 캐릭터 등)
- 커스텀 태그 (localStorage 저장)
- 출력 형식: JSON, 텍스트 리스트
- 인덱스 형식: Row/Col 또는 Index
- 프로젝트 저장/불러오기

**Usage:**
1. 이미지 불러오기
2. 그리드 크기 설정
3. 셀 클릭하여 설명 입력 또는 태그 사용
4. 출력 복사

**Selection Modes:**
- Single mode: Click cell → modal opens for description
- Multi mode: Click (single), Ctrl+click (toggle), Shift+click (rectangle)

## Keeping This File Updated

When you add:
- New command → Document pattern if non-trivial
- New module → Add to project structure
- New dependency → Note if it changes patterns
- New convention → Add to guidelines
