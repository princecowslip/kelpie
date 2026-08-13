//! `History` repository (kelpie.md §136 Phase 2). Backs migration `0008_history.sql`.
//!
//! Storage primitive for kelpie.md §98 History: tracks per-item first/last opened,
//! open count, and completion. Functions take `&rusqlite::Connection` per call,
//! matching the connection lifecycle established by [`crate::bootstrap`] /
//! [`crate::open_at`] rather than owning a connection or pool.
//!
//! `prune_before` is the delete-before(cutoff) primitive that the later-phase §99
//! Private Mode retention policy (Forever/90 days/30 days/7 days/Session only/Never)
//! will eventually be built on; wiring an actual policy to it is not this unit's job.

use rusqlite::{params, OptionalExtension, Row};

use kelpie_core::domain::history::History;

use crate::Result;

/// SQLite expression for "now" as an RFC3339 string with millisecond precision
/// (e.g. `2026-08-13T12:34:56.789Z`).
const NOW_RFC3339: &str = "strftime('%Y-%m-%dT%H:%M:%fZ', 'now')";

fn row_to_history(row: &Row<'_>) -> rusqlite::Result<History> {
    let open_count: i64 = row.get("open_count")?;
    let completed: i64 = row.get("completed")?;
    Ok(History {
        item_uid: row.get("item_uid")?,
        first_opened_at: row.get("first_opened_at")?,
        last_opened_at: row.get("last_opened_at")?,
        open_count: open_count as u32,
        completed: completed != 0,
    })
}

/// Record that `item_uid` was opened.
///
/// Upserts: if no history row exists yet for `item_uid`, creates one with
/// `open_count = 1` and `first_opened_at = last_opened_at = now`. If a row already
/// exists, increments `open_count` and updates `last_opened_at`, leaving
/// `first_opened_at` untouched.
pub fn record_open(conn: &rusqlite::Connection, item_uid: &str) -> Result<()> {
    let sql = format!(
        "INSERT INTO history (item_uid, first_opened_at, last_opened_at, open_count, completed)
         VALUES (?1, {now}, {now}, 1, 0)
         ON CONFLICT(item_uid) DO UPDATE SET
             open_count = open_count + 1,
             last_opened_at = {now}",
        now = NOW_RFC3339
    );
    conn.execute(&sql, params![item_uid])?;
    Ok(())
}

/// Fetch the history row for `item_uid`, if one exists.
pub fn get_by_item(conn: &rusqlite::Connection, item_uid: &str) -> Result<Option<History>> {
    conn.query_row(
        "SELECT item_uid, first_opened_at, last_opened_at, open_count, completed
         FROM history WHERE item_uid = ?1",
        params![item_uid],
        row_to_history,
    )
    .optional()
    .map_err(Into::into)
}

/// Mark (or unmark) `item_uid` as completed. No-op if the item has no history row.
pub fn set_completed(conn: &rusqlite::Connection, item_uid: &str, completed: bool) -> Result<()> {
    conn.execute(
        "UPDATE history SET completed = ?2 WHERE item_uid = ?1",
        params![item_uid, completed],
    )?;
    Ok(())
}

/// The most recently opened items, ordered by `last_opened_at` descending, capped
/// at `limit` rows.
pub fn list_recent(conn: &rusqlite::Connection, limit: u32) -> Result<Vec<History>> {
    let mut stmt = conn.prepare(
        "SELECT item_uid, first_opened_at, last_opened_at, open_count, completed
         FROM history ORDER BY last_opened_at DESC LIMIT ?1",
    )?;
    let rows = stmt
        .query_map(params![limit], row_to_history)?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(rows)
}

/// Delete every history row whose `last_opened_at` is strictly older than `cutoff`
/// (an RFC3339 string; lexicographic comparison is safe because RFC3339 timestamps
/// sort the same way lexically and chronologically). Storage primitive for the
/// later-phase §99 Private Mode retention policy. Returns the number of rows
/// deleted.
pub fn prune_before(conn: &rusqlite::Connection, cutoff: &str) -> Result<usize> {
    let deleted = conn.execute(
        "DELETE FROM history WHERE last_opened_at < ?1",
        params![cutoff],
    )?;
    Ok(deleted)
}

/// Delete the history row for `item_uid` directly, independent of any retention
/// policy.
pub fn delete(conn: &rusqlite::Connection, item_uid: &str) -> Result<()> {
    conn.execute("DELETE FROM history WHERE item_uid = ?1", params![item_uid])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::Duration;

    use tempfile::tempdir;

    use super::*;

    fn temp_conn() -> (tempfile::TempDir, rusqlite::Connection) {
        let dir = tempdir().expect("create temp dir");
        let path = dir.path().join("history.sqlite");
        let conn = crate::open_at(&path).expect("open_at should succeed");
        (dir, conn)
    }

    /// End-to-end: migration + repository against a real SQLite database, exercising
    /// every function in this module.
    #[test]
    fn history_lifecycle_end_to_end() {
        let (_dir, conn) = temp_conn();
        let item_uid = "provider-x:media_item:test-item-1";

        record_open(&conn, item_uid).expect("first record_open");
        sleep(Duration::from_millis(20));
        record_open(&conn, item_uid).expect("second record_open");
        sleep(Duration::from_millis(20));
        record_open(&conn, item_uid).expect("third record_open");

        let history = get_by_item(&conn, item_uid)
            .expect("get_by_item")
            .expect("history row should exist after record_open");

        assert_eq!(history.item_uid, item_uid);
        assert_eq!(
            history.open_count, 3,
            "open_count should be 3 after three opens"
        );
        assert!(!history.completed);
        assert!(
            history.last_opened_at > history.first_opened_at,
            "last_opened_at ({}) should have advanced past first_opened_at ({})",
            history.last_opened_at,
            history.first_opened_at
        );
        let first_opened_at = history.first_opened_at.clone();

        // Mark completed and verify via get_by_item.
        set_completed(&conn, item_uid, true).expect("set_completed");
        let history = get_by_item(&conn, item_uid)
            .expect("get_by_item")
            .expect("history row should still exist");
        assert!(history.completed, "history should be marked completed");
        assert_eq!(
            history.first_opened_at, first_opened_at,
            "set_completed must not touch first_opened_at"
        );

        // Should show up in list_recent.
        let recent = list_recent(&conn, 10).expect("list_recent");
        assert!(
            recent.iter().any(|h| h.item_uid == item_uid),
            "item should appear in list_recent"
        );

        // prune_before with a cutoff in the future removes it.
        let future_cutoff = "9999-01-01T00:00:00.000Z";
        let pruned = prune_before(&conn, future_cutoff).expect("prune_before");
        assert_eq!(pruned, 1, "prune_before(future) should delete the one row");
        assert!(
            get_by_item(&conn, item_uid).expect("get_by_item").is_none(),
            "row should be gone after prune_before(future)"
        );

        // Re-insert and test delete removes it directly.
        record_open(&conn, item_uid).expect("re-insert via record_open");
        assert!(get_by_item(&conn, item_uid).expect("get_by_item").is_some());
        delete(&conn, item_uid).expect("delete");
        assert!(
            get_by_item(&conn, item_uid).expect("get_by_item").is_none(),
            "row should be gone after delete"
        );
    }

    #[test]
    fn get_by_item_returns_none_for_missing_item() {
        let (_dir, conn) = temp_conn();
        assert!(get_by_item(&conn, "does-not-exist")
            .expect("get_by_item")
            .is_none());
    }

    #[test]
    fn list_recent_orders_by_last_opened_at_descending() {
        let (_dir, conn) = temp_conn();
        record_open(&conn, "item-a").expect("record_open item-a");
        sleep(Duration::from_millis(20));
        record_open(&conn, "item-b").expect("record_open item-b");

        let recent = list_recent(&conn, 10).expect("list_recent");
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0].item_uid, "item-b", "most recently opened first");
        assert_eq!(recent[1].item_uid, "item-a");
    }

    #[test]
    fn list_recent_respects_limit() {
        let (_dir, conn) = temp_conn();
        record_open(&conn, "item-a").expect("record_open item-a");
        record_open(&conn, "item-b").expect("record_open item-b");
        record_open(&conn, "item-c").expect("record_open item-c");

        let recent = list_recent(&conn, 2).expect("list_recent");
        assert_eq!(recent.len(), 2);
    }

    #[test]
    fn prune_before_keeps_rows_not_older_than_cutoff() {
        let (_dir, conn) = temp_conn();
        record_open(&conn, "keep-me").expect("record_open");

        // Cutoff in the past: nothing should be pruned.
        let past_cutoff = "2000-01-01T00:00:00.000Z";
        let pruned = prune_before(&conn, past_cutoff).expect("prune_before");
        assert_eq!(pruned, 0);
        assert!(get_by_item(&conn, "keep-me")
            .expect("get_by_item")
            .is_some());
    }

    #[test]
    fn set_completed_is_a_no_op_for_missing_item() {
        let (_dir, conn) = temp_conn();
        // Should not error even though there is no such row.
        set_completed(&conn, "does-not-exist", true).expect("set_completed");
        assert!(get_by_item(&conn, "does-not-exist")
            .expect("get_by_item")
            .is_none());
    }

    #[test]
    fn delete_is_a_no_op_for_missing_item() {
        let (_dir, conn) = temp_conn();
        // Should not error even though there is no such row.
        delete(&conn, "does-not-exist").expect("delete");
    }
}
