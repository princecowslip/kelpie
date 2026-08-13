//! SQLite bootstrap command handlers (kelpie.md §104 Database Domains, §106 Migration
//! Policy). Owned by Stage B unit 2 (SQLite bootstrap) — this stub is a no-op.

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DbHealth {
    pub ok: bool,
    pub path: String,
    pub applied_migrations: u32,
}

/// Called once from `run()`'s `.setup()` hook. No-op until Stage B unit 2 lands.
pub fn setup(_app: &tauri::App) -> tauri::Result<()> {
    Ok(())
}

#[tauri::command]
pub fn health_check() -> DbHealth {
    DbHealth {
        ok: false,
        path: String::new(),
        applied_migrations: 0,
    }
}
