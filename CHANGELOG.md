# Changelog


## v1.1.28 (2026-02-15)

### Features
- add automatic changelog generation to release workflow

### Bug Fixes
- harden Rust backend security and fix critical bugs
- improve error handling and safety across backend
- prevent stale DB connections after factory reset
- add missing CommandExt imports for Windows builds

### Refactoring
- remove unused AppState and increment_counter command
- split large Rust backend files into single-responsibility modules

## v1.1.27 (2026-02-15)

### Features
- add automatic changelog generation to release workflow

### Refactoring
- split large Rust backend files into single-responsibility modules
All notable changes to this project will be documented in this file.
