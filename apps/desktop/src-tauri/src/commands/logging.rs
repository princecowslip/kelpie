//! Logging setup command handlers (kelpie.md §115 Diagnostics, §121-122 Linux
//! Integration/Filesystem Layout).

use std::sync::OnceLock;

/// The directory logs are written to, populated once by [`setup`]. `get_log_dir`
/// reads this rather than re-deriving the path, so it always reflects what the
/// running app actually initialized (and stays empty if `setup` failed).
static LOG_DIR: OnceLock<String> = OnceLock::new();

/// Called once from `run()`'s `.setup()` hook. Initializes `kelpie-core`'s
/// structured logging (rotating file under the XDG state dir, plus stderr in debug
/// builds) and records the resulting log directory for `get_log_dir`.
pub fn setup(_app: &tauri::App) -> tauri::Result<()> {
    let log_dir = kelpie_core::logging::init()
        .map_err(|e| tauri::Error::Io(std::io::Error::new(e.kind(), e.to_string())))?;
    let _ = LOG_DIR.set(log_dir.display().to_string());
    Ok(())
}

/// Returns the directory Kelpie writes its log files to, or an empty string if
/// logging has not been initialized yet (`setup` failed or hasn't run).
#[tauri::command]
pub fn get_log_dir() -> String {
    LOG_DIR.get().cloned().unwrap_or_default()
}
