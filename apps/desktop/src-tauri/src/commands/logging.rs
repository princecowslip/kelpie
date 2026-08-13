//! Logging setup command handlers (kelpie.md §115 Diagnostics, §121-122 Linux
//! Integration/Filesystem Layout). Owned by Stage B unit 3 (logging infrastructure) —
//! this stub is a no-op.

/// Called once from `run()`'s `.setup()` hook. No-op until Stage B unit 3 lands.
pub fn setup(_app: &tauri::App) -> tauri::Result<()> {
    Ok(())
}

#[tauri::command]
pub fn get_log_dir() -> String {
    String::new()
}
