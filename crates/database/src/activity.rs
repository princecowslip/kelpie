//! History + Progress repository (kelpie.md §136 Core Data Layer): History
//! (§98) and Reading Progress (§84), the `history` and `progress` tables from
//! `migrations/0002_core_data_layer.sql`.
//!
//! Owned by a Stage B unit — repository code and comprehensive unit tests
//! land here.
//!
//! Timestamps: every `*_at` column in this module is generated in SQL via
//! `datetime('now')` (UTC, `YYYY-MM-DD HH:MM:SS` text) rather than in Rust,
//! so `record_open`'s atomic upsert (insert-or-bump in one statement) doesn't
//! need a value threaded in from the caller and every timestamp in a given
//! table comes from the same clock source.
//!
//! Retention policy (§98's Forever/90 days/30 days/7 days/Session only/Never)
//! is a user setting enforced by a future background sweep job — out of
//! scope here. This module only provides the plain CRUD primitives
//! (`clear_history`/`clear_all_history`) that sweep would eventually call.

use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};

use crate::Result;

/// A row of the `history` table (§98 History): per-item open tracking.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub item_uid: String,
    pub first_opened_at: String,
    pub last_opened_at: String,
    pub open_count: i64,
    pub completed: bool,
}

impl HistoryEntry {
    fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            item_uid: row.get("item_uid")?,
            first_opened_at: row.get("first_opened_at")?,
            last_opened_at: row.get("last_opened_at")?,
            open_count: row.get("open_count")?,
            completed: row.get::<_, i64>("completed")? != 0,
        })
    }
}

/// A row of the `progress` table (§84 Reading Progress).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Progress {
    pub id: i64,
    pub item_uid: String,
    pub series_uid: Option<String>,
    pub chapter_uid: Option<String>,
    pub page_index: Option<i64>,
    pub scroll_fraction: Option<f64>,
    pub updated_at: String,
    pub completed: bool,
}

impl Progress {
    fn from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            item_uid: row.get("item_uid")?,
            series_uid: row.get("series_uid")?,
            chapter_uid: row.get("chapter_uid")?,
            page_index: row.get("page_index")?,
            scroll_fraction: row.get("scroll_fraction")?,
            updated_at: row.get("updated_at")?,
            completed: row.get::<_, i64>("completed")? != 0,
        })
    }
}

/// Input for [`upsert_progress`]. Deliberately excludes `id` and `updated_at`
/// — both are owned by the repository (`id` is assigned by SQLite on first
/// insert and kept stable across updates via `ON CONFLICT`; `updated_at` is
/// always set to `datetime('now')` at write time), so there's no struct shape
/// in which a caller could pass a stale or spoofed value for either.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewProgress {
    pub item_uid: String,
    pub series_uid: Option<String>,
    pub chapter_uid: Option<String>,
    pub page_index: Option<i64>,
    pub scroll_fraction: Option<f64>,
    pub completed: bool,
}

/// Record that `item_uid` was opened: the core §98 operation. Inserts a new
/// `history` row (`first_opened_at = last_opened_at = now`, `open_count = 1`)
/// if none exists yet, otherwise bumps `last_opened_at` to now and increments
/// `open_count` — all in one atomic `INSERT ... ON CONFLICT DO UPDATE`
/// statement, avoiding a read-then-write race between concurrent callers.
pub fn record_open(conn: &Connection, item_uid: &str) -> Result<HistoryEntry> {
    conn.execute(
        "INSERT INTO history (item_uid, first_opened_at, last_opened_at, open_count, completed)
         VALUES (?1, datetime('now'), datetime('now'), 1, 0)
         ON CONFLICT(item_uid) DO UPDATE SET
             last_opened_at = excluded.last_opened_at,
             open_count = open_count + 1",
        params![item_uid],
    )?;

    get_history(conn, item_uid)?.ok_or_else(|| rusqlite::Error::QueryReturnedNoRows.into())
}

/// Update the `completed` flag on an existing `history` row for `item_uid`.
///
/// Interpretive decision: if no history row exists yet for this item, this
/// is a silent no-op rather than an error — marking something "completed"
/// implies it was opened at some point, but callers that only care about the
/// completed flag (e.g. "mark as read" from a series list, without the item
/// necessarily having been opened via the reader) shouldn't have to
/// special-case calling `record_open` first just to avoid an error. Callers
/// that need the row to definitely exist should call `record_open` first
/// themselves.
pub fn mark_completed(conn: &Connection, item_uid: &str, completed: bool) -> Result<()> {
    conn.execute(
        "UPDATE history SET completed = ?1 WHERE item_uid = ?2",
        params![completed as i64, item_uid],
    )?;
    Ok(())
}

/// Fetch the `history` row for `item_uid`, if any.
pub fn get_history(conn: &Connection, item_uid: &str) -> Result<Option<HistoryEntry>> {
    conn.query_row(
        "SELECT item_uid, first_opened_at, last_opened_at, open_count, completed
         FROM history WHERE item_uid = ?1",
        params![item_uid],
        HistoryEntry::from_row,
    )
    .optional()
    .map_err(Into::into)
}

/// The most recently opened items, most recent first.
pub fn list_recent_history(conn: &Connection, limit: u32) -> Result<Vec<HistoryEntry>> {
    let mut stmt = conn.prepare(
        "SELECT item_uid, first_opened_at, last_opened_at, open_count, completed
         FROM history ORDER BY last_opened_at DESC LIMIT ?1",
    )?;
    let rows = stmt
        .query_map(params![limit], HistoryEntry::from_row)?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(rows)
}

/// Delete the `history` row for a single item, if any (no-op if absent).
pub fn clear_history(conn: &Connection, item_uid: &str) -> Result<()> {
    conn.execute("DELETE FROM history WHERE item_uid = ?1", params![item_uid])?;
    Ok(())
}

/// Delete every `history` row. The retention-policy sweep job this enables
/// (§98: Forever/90/30/7/Session only/Never) is out of scope for this module
/// — this is just the bulk-delete primitive it would call.
pub fn clear_all_history(conn: &Connection) -> Result<()> {
    conn.execute("DELETE FROM history", [])?;
    Ok(())
}

/// Insert or update the single `progress` row for `progress.item_uid`
/// (enforced by the table's `UNIQUE(item_uid)` constraint) in one upsert
/// statement. `updated_at` is always set to `datetime('now')`; `id` is
/// assigned by SQLite on first insert and left untouched on update.
pub fn upsert_progress(conn: &Connection, progress: &NewProgress) -> Result<Progress> {
    conn.execute(
        "INSERT INTO progress (item_uid, series_uid, chapter_uid, page_index, scroll_fraction, updated_at, completed)
         VALUES (?1, ?2, ?3, ?4, ?5, datetime('now'), ?6)
         ON CONFLICT(item_uid) DO UPDATE SET
             series_uid = excluded.series_uid,
             chapter_uid = excluded.chapter_uid,
             page_index = excluded.page_index,
             scroll_fraction = excluded.scroll_fraction,
             updated_at = excluded.updated_at,
             completed = excluded.completed",
        params![
            progress.item_uid,
            progress.series_uid,
            progress.chapter_uid,
            progress.page_index,
            progress.scroll_fraction,
            progress.completed as i64,
        ],
    )?;

    get_progress(conn, &progress.item_uid)?
        .ok_or_else(|| rusqlite::Error::QueryReturnedNoRows.into())
}

/// Fetch the `progress` row for `item_uid`, if any.
pub fn get_progress(conn: &Connection, item_uid: &str) -> Result<Option<Progress>> {
    conn.query_row(
        "SELECT id, item_uid, series_uid, chapter_uid, page_index, scroll_fraction, updated_at, completed
         FROM progress WHERE item_uid = ?1",
        params![item_uid],
        Progress::from_row,
    )
    .optional()
    .map_err(Into::into)
}

/// Delete the `progress` row for a single item, if any (no-op if absent).
pub fn delete_progress(conn: &Connection, item_uid: &str) -> Result<()> {
    conn.execute(
        "DELETE FROM progress WHERE item_uid = ?1",
        params![item_uid],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn temp_db_path(name: &str) -> (tempfile::TempDir, PathBuf) {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = dir.path().join(name);
        (dir, path)
    }

    /// Insert a minimal, obviously-synthetic `items` row directly via raw SQL
    /// so `history`/`progress` foreign keys have something to point at.
    fn insert_item(conn: &Connection, uid: &str) {
        conn.execute(
            "INSERT INTO items (uid, provider_id, canonical_url, kind, title, classification_json, availability, created_at, updated_locally_at)
             VALUES (?1, 'provider-x', ?2, 'video', 'Test Item', '{}', 'available', datetime('now'), datetime('now'))",
            params![uid, format!("https://example.test/items/{uid}")],
        )
        .expect("insert test item");
    }

    fn insert_series(conn: &Connection, uid: &str) {
        conn.execute(
            "INSERT INTO series (uid, provider_id, canonical_url, title)
             VALUES (?1, 'provider-x', ?2, 'Test Series')",
            params![uid, format!("https://example.test/series/{uid}")],
        )
        .expect("insert test series");
    }

    fn insert_chapter(conn: &Connection, uid: &str, series_uid: &str) {
        conn.execute(
            "INSERT INTO chapters (uid, series_uid, provider_id, canonical_url)
             VALUES (?1, ?2, 'provider-x', ?3)",
            params![
                uid,
                series_uid,
                format!("https://example.test/chapters/{uid}")
            ],
        )
        .expect("insert test chapter");
    }

    fn setup() -> (tempfile::TempDir, Connection) {
        let (dir, path) = temp_db_path("activity.sqlite");
        let conn = crate::open_at(&path).expect("bootstrap");
        (dir, conn)
    }

    #[test]
    fn record_open_on_fresh_item_creates_row_with_open_count_one() {
        let (_dir, conn) = setup();
        insert_item(&conn, "item-1");

        let entry = record_open(&conn, "item-1").expect("record_open");

        assert_eq!(entry.item_uid, "item-1");
        assert_eq!(entry.open_count, 1);
        assert_eq!(entry.first_opened_at, entry.last_opened_at);
        assert!(!entry.completed);
    }

    #[test]
    fn record_open_again_increments_count_and_advances_last_opened_but_not_first() {
        let (_dir, conn) = setup();
        insert_item(&conn, "item-1");

        let first = record_open(&conn, "item-1").expect("first record_open");
        // Force a distinct timestamp so last_opened_at is verifiably advanced
        // rather than merely equal (SQLite's datetime('now') has 1s resolution).
        conn.execute(
            "UPDATE history SET first_opened_at = '2020-01-01 00:00:00', last_opened_at = '2020-01-01 00:00:00' WHERE item_uid = 'item-1'",
            [],
        )
        .unwrap();

        let second = record_open(&conn, "item-1").expect("second record_open");

        assert_eq!(second.open_count, 2);
        assert_eq!(second.first_opened_at, "2020-01-01 00:00:00");
        assert_ne!(second.last_opened_at, "2020-01-01 00:00:00");
        assert_eq!(first.item_uid, second.item_uid);
    }

    #[test]
    fn mark_completed_toggles_flag_on_existing_row() {
        let (_dir, conn) = setup();
        insert_item(&conn, "item-1");
        record_open(&conn, "item-1").unwrap();

        mark_completed(&conn, "item-1", true).unwrap();
        assert!(get_history(&conn, "item-1").unwrap().unwrap().completed);

        mark_completed(&conn, "item-1", false).unwrap();
        assert!(!get_history(&conn, "item-1").unwrap().unwrap().completed);
    }

    #[test]
    fn mark_completed_on_missing_row_is_a_silent_no_op() {
        let (_dir, conn) = setup();
        insert_item(&conn, "item-1");

        // No record_open call first -- no history row exists yet.
        let result = mark_completed(&conn, "item-1", true);
        assert!(result.is_ok());
        assert!(get_history(&conn, "item-1").unwrap().is_none());
    }

    #[test]
    fn get_history_returns_none_for_unknown_item() {
        let (_dir, conn) = setup();
        assert!(get_history(&conn, "does-not-exist").unwrap().is_none());
    }

    #[test]
    fn list_recent_history_orders_by_last_opened_at_desc() {
        let (_dir, conn) = setup();
        insert_item(&conn, "item-1");
        insert_item(&conn, "item-2");
        insert_item(&conn, "item-3");

        conn.execute(
            "INSERT INTO history (item_uid, first_opened_at, last_opened_at, open_count, completed) VALUES
             ('item-1', '2020-01-01 00:00:00', '2020-01-01 00:00:00', 1, 0),
             ('item-2', '2020-01-03 00:00:00', '2020-01-03 00:00:00', 1, 0),
             ('item-3', '2020-01-02 00:00:00', '2020-01-02 00:00:00', 1, 0)",
            [],
        )
        .unwrap();

        let recent = list_recent_history(&conn, 10).unwrap();
        let uids: Vec<&str> = recent.iter().map(|e| e.item_uid.as_str()).collect();
        assert_eq!(uids, vec!["item-2", "item-3", "item-1"]);
    }

    #[test]
    fn list_recent_history_respects_limit() {
        let (_dir, conn) = setup();
        for i in 0..5 {
            let uid = format!("item-{i}");
            insert_item(&conn, &uid);
            record_open(&conn, &uid).unwrap();
        }

        let recent = list_recent_history(&conn, 2).unwrap();
        assert_eq!(recent.len(), 2);
    }

    #[test]
    fn clear_history_removes_single_row_only() {
        let (_dir, conn) = setup();
        insert_item(&conn, "item-1");
        insert_item(&conn, "item-2");
        record_open(&conn, "item-1").unwrap();
        record_open(&conn, "item-2").unwrap();

        clear_history(&conn, "item-1").unwrap();

        assert!(get_history(&conn, "item-1").unwrap().is_none());
        assert!(get_history(&conn, "item-2").unwrap().is_some());
    }

    #[test]
    fn clear_history_on_missing_row_is_a_no_op() {
        let (_dir, conn) = setup();
        insert_item(&conn, "item-1");
        assert!(clear_history(&conn, "item-1").is_ok());
    }

    #[test]
    fn clear_all_history_removes_every_row() {
        let (_dir, conn) = setup();
        insert_item(&conn, "item-1");
        insert_item(&conn, "item-2");
        record_open(&conn, "item-1").unwrap();
        record_open(&conn, "item-2").unwrap();

        clear_all_history(&conn).unwrap();

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM history", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn upsert_progress_inserts_then_updates_in_place() {
        let (_dir, conn) = setup();
        insert_item(&conn, "item-1");
        insert_series(&conn, "series-1");
        insert_chapter(&conn, "chapter-1", "series-1");

        let inserted = upsert_progress(
            &conn,
            &NewProgress {
                item_uid: "item-1".to_string(),
                series_uid: Some("series-1".to_string()),
                chapter_uid: Some("chapter-1".to_string()),
                page_index: Some(3),
                scroll_fraction: Some(0.25),
                completed: false,
            },
        )
        .unwrap();

        assert_eq!(inserted.item_uid, "item-1");
        assert_eq!(inserted.page_index, Some(3));
        assert!(!inserted.completed);

        let updated = upsert_progress(
            &conn,
            &NewProgress {
                item_uid: "item-1".to_string(),
                series_uid: Some("series-1".to_string()),
                chapter_uid: Some("chapter-1".to_string()),
                page_index: Some(9),
                scroll_fraction: Some(0.75),
                completed: true,
            },
        )
        .unwrap();

        assert_eq!(
            updated.id, inserted.id,
            "same row should be reused, not duplicated"
        );
        assert_eq!(updated.page_index, Some(9));
        assert_eq!(updated.scroll_fraction, Some(0.75));
        assert!(updated.completed);

        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM progress WHERE item_uid = 'item-1'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1, "exactly one row per item_uid");
    }

    #[test]
    fn upsert_progress_allows_null_series_and_chapter() {
        let (_dir, conn) = setup();
        insert_item(&conn, "item-1");

        let progress = upsert_progress(
            &conn,
            &NewProgress {
                item_uid: "item-1".to_string(),
                series_uid: None,
                chapter_uid: None,
                page_index: None,
                scroll_fraction: Some(0.5),
                completed: false,
            },
        )
        .unwrap();

        assert!(progress.series_uid.is_none());
        assert!(progress.chapter_uid.is_none());
    }

    #[test]
    fn get_progress_hit_and_miss() {
        let (_dir, conn) = setup();
        insert_item(&conn, "item-1");
        insert_item(&conn, "item-2");

        upsert_progress(
            &conn,
            &NewProgress {
                item_uid: "item-1".to_string(),
                series_uid: None,
                chapter_uid: None,
                page_index: Some(1),
                scroll_fraction: None,
                completed: false,
            },
        )
        .unwrap();

        assert!(get_progress(&conn, "item-1").unwrap().is_some());
        assert!(get_progress(&conn, "item-2").unwrap().is_none());
    }

    #[test]
    fn delete_progress_removes_row() {
        let (_dir, conn) = setup();
        insert_item(&conn, "item-1");
        upsert_progress(
            &conn,
            &NewProgress {
                item_uid: "item-1".to_string(),
                series_uid: None,
                chapter_uid: None,
                page_index: Some(1),
                scroll_fraction: None,
                completed: false,
            },
        )
        .unwrap();

        delete_progress(&conn, "item-1").unwrap();

        assert!(get_progress(&conn, "item-1").unwrap().is_none());
    }

    #[test]
    fn delete_progress_on_missing_row_is_a_no_op() {
        let (_dir, conn) = setup();
        insert_item(&conn, "item-1");
        assert!(delete_progress(&conn, "item-1").is_ok());
    }

    #[test]
    fn deleting_item_cascades_to_history_and_progress() {
        let (_dir, conn) = setup();
        insert_item(&conn, "item-1");
        record_open(&conn, "item-1").unwrap();
        upsert_progress(
            &conn,
            &NewProgress {
                item_uid: "item-1".to_string(),
                series_uid: None,
                chapter_uid: None,
                page_index: Some(1),
                scroll_fraction: None,
                completed: false,
            },
        )
        .unwrap();

        conn.execute("PRAGMA foreign_keys = ON", []).unwrap();
        conn.execute("DELETE FROM items WHERE uid = 'item-1'", [])
            .unwrap();

        assert!(get_history(&conn, "item-1").unwrap().is_none());
        assert!(get_progress(&conn, "item-1").unwrap().is_none());
    }
}
