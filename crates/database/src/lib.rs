//! kelpie-database: SQLite persistence layer (kelpie.md §104 Database Domains,
//! §106 Migration Policy). The migration mechanism (numbered / immutable /
//! transactional / tested, per §106) bootstraps the database file, and the
//! Phase 2 (§136) domain schema and repositories build on top of it.
//!
//! [`bootstrap`] opens (creating if necessary) the SQLite database at the XDG
//! data directory, enables WAL journaling, applies any outstanding migrations
//! from [`MIGRATIONS`] each inside its own transaction, records each applied
//! version in `schema_migrations`, and runs `PRAGMA integrity_check` before
//! handing back the connection.
//!
//! Repository modules (one per Phase 2 domain group) each own their own table
//! set from the `0002_core_data_layer` migration; this module and the
//! migration files are frozen once Phase 2 repository work starts — new
//! repository code lives in its own module file, not here.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use rusqlite::{params, Connection};

pub mod activity;
pub mod collections;
pub mod creators_tags;
pub mod follows;
pub mod items;
pub mod series;

/// Errors that can occur while locating, opening, or migrating the database.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to resolve or create the Kelpie data directory: {0}")]
    Io(#[from] std::io::Error),

    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("database integrity check failed: {0}")]
    IntegrityCheck(String),
}

pub type Result<T> = std::result::Result<T, Error>;

/// A single numbered migration. Per kelpie.md §106, migrations are numbered,
/// immutable once merged, transactional, and tested.
struct Migration {
    version: i64,
    sql: &'static str,
}

/// All migrations, in ascending version order. `0001_init` (Phase 1, §135)
/// creates the `schema_migrations` bookkeeping table itself; `0002_core_data_layer`
/// (Phase 2, §136) adds the full domain schema (§104 Database Domains).
const MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        sql: include_str!("../../../migrations/0001_init.sql"),
    },
    Migration {
        version: 2,
        sql: include_str!("../../../migrations/0002_core_data_layer.sql"),
    },
];

/// The on-disk path of the Kelpie SQLite database: `<XDG data dir>/kelpie.sqlite`
/// (kelpie.md §121-122). Creates the containing directory if it does not exist.
pub fn database_path() -> Result<PathBuf> {
    Ok(kelpie_core::paths::data_dir()?.join("kelpie.sqlite"))
}

/// Open (creating if necessary) the Kelpie SQLite database at its standard XDG
/// location, enable WAL journaling, apply any outstanding migrations, and verify
/// integrity. Safe to call repeatedly (e.g. on every app startup) — already-applied
/// migrations are skipped.
pub fn bootstrap() -> Result<Connection> {
    open_at(database_path()?)
}

/// Same as [`bootstrap`] but against an explicit path, so tests (and anything else
/// that wants a non-default location) don't have to touch the real XDG data dir.
pub fn open_at(path: impl AsRef<Path>) -> Result<Connection> {
    let conn = Connection::open(path)?;
    conn.pragma_update(None, "journal_mode", "WAL")?;
    run_migrations(&conn)?;
    integrity_check(&conn)?;
    Ok(conn)
}

/// Number of migrations recorded as applied in `schema_migrations`.
pub fn applied_migration_count(conn: &Connection) -> Result<u32> {
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM schema_migrations", [], |row| {
        row.get(0)
    })?;
    Ok(count as u32)
}

/// Versions already recorded in `schema_migrations`. Returns an empty set if the
/// table does not exist yet (i.e. no migration has ever run against this database).
fn applied_versions(conn: &Connection) -> Result<HashSet<i64>> {
    let table_exists: bool = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = 'schema_migrations'",
        [],
        |row| row.get::<_, i64>(0),
    )? > 0;

    if !table_exists {
        return Ok(HashSet::new());
    }

    let mut stmt = conn.prepare("SELECT version FROM schema_migrations")?;
    let versions = stmt
        .query_map([], |row| row.get::<_, i64>(0))?
        .collect::<rusqlite::Result<HashSet<i64>>>()?;
    Ok(versions)
}

/// Apply every migration in [`MIGRATIONS`] that isn't already recorded in
/// `schema_migrations`, each inside its own transaction (numbered / immutable /
/// transactional, per kelpie.md §106).
fn run_migrations(conn: &Connection) -> Result<()> {
    let applied = applied_versions(conn)?;

    for migration in MIGRATIONS {
        if applied.contains(&migration.version) {
            continue;
        }

        let tx = conn.unchecked_transaction()?;
        tx.execute_batch(migration.sql)?;
        tx.execute(
            "INSERT INTO schema_migrations (version, applied_at) VALUES (?1, datetime('now'))",
            params![migration.version],
        )?;
        tx.commit()?;
    }

    Ok(())
}

/// Run `PRAGMA integrity_check` and turn anything other than `ok` into an error.
fn integrity_check(conn: &Connection) -> Result<()> {
    let result: String = conn.query_row("PRAGMA integrity_check", [], |row| row.get(0))?;
    if result != "ok" {
        return Err(Error::IntegrityCheck(result));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_db_path(name: &str) -> (tempfile::TempDir, PathBuf) {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = dir.path().join(name);
        (dir, path)
    }

    /// Number of migrations in [`MIGRATIONS`] — tests assert against this
    /// rather than a hardcoded literal so adding a migration doesn't require
    /// hunting down every count assertion.
    const MIGRATION_COUNT: i64 = MIGRATIONS.len() as i64;

    #[test]
    fn bootstrap_creates_schema_migrations_table() {
        let (_dir, path) = temp_db_path("bootstrap.sqlite");
        let conn = open_at(&path).expect("bootstrap should succeed");

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM schema_migrations", [], |row| {
                row.get(0)
            })
            .expect("schema_migrations table should exist");
        assert_eq!(
            count, MIGRATION_COUNT,
            "every migration in MIGRATIONS should be recorded"
        );

        let applied = applied_migration_count(&conn).expect("count applied migrations");
        assert_eq!(applied as i64, MIGRATION_COUNT);
    }

    #[test]
    fn bootstrap_records_migration_version_and_timestamp() {
        let (_dir, path) = temp_db_path("versions.sqlite");
        let conn = open_at(&path).expect("bootstrap should succeed");

        let (version, applied_at): (i64, String) = conn
            .query_row(
                "SELECT version, applied_at FROM schema_migrations WHERE version = 1",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .expect("version 1 should be recorded");
        assert_eq!(version, 1);
        assert!(!applied_at.is_empty());
    }

    #[test]
    fn reopening_is_idempotent() {
        let (_dir, path) = temp_db_path("idempotent.sqlite");

        // First open applies every migration.
        {
            let conn = open_at(&path).expect("first bootstrap should succeed");
            assert_eq!(
                applied_migration_count(&conn).unwrap() as i64,
                MIGRATION_COUNT
            );
        }

        // Reopening must not re-apply (and must not error on) an already-applied
        // migration, and the row count must stay the same.
        let conn = open_at(&path).expect("second bootstrap should succeed");
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM schema_migrations", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(
            count, MIGRATION_COUNT,
            "migrations must not be applied twice"
        );
    }

    #[test]
    fn wal_journal_mode_is_enabled() {
        let (_dir, path) = temp_db_path("wal.sqlite");
        let conn = open_at(&path).expect("bootstrap should succeed");

        let mode: String = conn
            .query_row("PRAGMA journal_mode", [], |row| row.get(0))
            .expect("read journal_mode");
        assert_eq!(mode.to_lowercase(), "wal");
    }

    #[test]
    fn integrity_check_passes_after_bootstrap() {
        let (_dir, path) = temp_db_path("integrity.sqlite");
        let conn = open_at(&path).expect("bootstrap should succeed");

        let result: String = conn
            .query_row("PRAGMA integrity_check", [], |row| row.get(0))
            .expect("run integrity_check");
        assert_eq!(result, "ok");
    }

    /// `0002_core_data_layer` should create every Phase 2 domain table (§104
    /// Database Domains subset covered by §136).
    #[test]
    fn core_data_layer_migration_creates_all_domain_tables() {
        let (_dir, path) = temp_db_path("core_data_layer.sqlite");
        let conn = open_at(&path).expect("bootstrap should succeed");

        const EXPECTED_TABLES: &[&str] = &[
            "items",
            "media_sources",
            "creators",
            "tags",
            "item_tags",
            "series",
            "chapters",
            "collections",
            "collection_items",
            "history",
            "progress",
            "follows",
            "favorites",
        ];

        for table in EXPECTED_TABLES {
            let exists: bool = conn
                .query_row(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = ?1",
                    params![table],
                    |row| row.get::<_, i64>(0),
                )
                .unwrap()
                > 0;
            assert!(exists, "expected table `{table}` to exist after migrations");
        }
    }

    #[test]
    fn database_path_lives_under_xdg_data_dir() {
        let expected = kelpie_core::paths::data_dir()
            .expect("resolve data dir")
            .join("kelpie.sqlite");
        assert_eq!(database_path().expect("database_path"), expected);
    }
}
