//! SQLite bootstrap command handlers (kelpie.md §104 Database Domains, §106 Migration
//! Policy). Owned by Stage B unit 2 (SQLite bootstrap).
//!
//! `setup` bootstraps the database once at app startup (creates the file if
//! missing, enables WAL, applies outstanding migrations, verifies integrity) so
//! any failure surfaces immediately rather than lazily on first command. Because
//! `kelpie_database::bootstrap` is idempotent and SQLite's WAL mode supports
//! multiple connections to the same file, `health_check` simply re-opens the
//! database per call instead of threading a shared connection through Tauri's
//! managed state — deliberately the simpler of the two options blessed for
//! Phase 1.

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DbHealth {
    pub ok: bool,
    pub path: String,
    pub applied_migrations: u32,
}

fn database_path_string() -> String {
    kelpie_database::database_path()
        .map(|p| p.display().to_string())
        .unwrap_or_default()
}

/// Called once from `run()`'s `.setup()` hook. Bootstraps the SQLite database so
/// startup fails fast (surfaced via the setup hook's error path) if the data
/// directory, migrations, or integrity check are broken.
pub fn setup(_app: &tauri::App) -> tauri::Result<()> {
    kelpie_database::bootstrap().map_err(|err| {
        let boxed: Box<dyn std::error::Error> = Box::new(err);
        tauri::Error::Setup(boxed.into())
    })?;
    Ok(())
}

#[tauri::command]
pub fn health_check() -> DbHealth {
    match kelpie_database::bootstrap() {
        Ok(conn) => {
            let applied_migrations = kelpie_database::applied_migration_count(&conn).unwrap_or(0);
            DbHealth {
                ok: true,
                path: database_path_string(),
                applied_migrations,
            }
        }
        Err(err) => {
            tracing::error!("database health check failed: {err}");
            DbHealth {
                ok: false,
                path: database_path_string(),
                applied_migrations: 0,
            }
        }
    }
}
