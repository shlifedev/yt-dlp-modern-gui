#[tauri::command]
#[specta::specta]
pub fn increment_counter(current: i32) -> i32 {
    current + 1
}
