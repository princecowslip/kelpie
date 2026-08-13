//! `Progress` repository (kelpie.md §136 Phase 2). Backs migration `0009_progress.sql`.
//!
//! Functions take `&rusqlite::Connection` per call, matching the connection lifecycle
//! established by [`crate::bootstrap`] / [`crate::open_at`].

use rusqlite::{params, Connection, OptionalExtension};

use kelpie_core::domain::progress::Progress;

use crate::Result;

/// Insert a new progress record, or replace the existing one for the same
/// `item_uid` (kelpie.md §84).
pub fn upsert_progress(conn: &Connection, progress: &Progress) -> Result<()> {
    conn.execute(
        "INSERT INTO progress
            (item_uid, series_uid, page_index, scroll_fraction, position_seconds, updated_at, completed)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
         ON CONFLICT(item_uid) DO UPDATE SET
            series_uid = excluded.series_uid,
            page_index = excluded.page_index,
            scroll_fraction = excluded.scroll_fraction,
            position_seconds = excluded.position_seconds,
            updated_at = excluded.updated_at,
            completed = excluded.completed",
        params![
            progress.item_uid,
            progress.series_uid,
            progress.page_index,
            progress.scroll_fraction,
            progress.position_seconds,
            progress.updated_at,
            progress.completed,
        ],
    )?;
    Ok(())
}

/// Fetch the progress record for a given item/chapter uid, if one exists.
pub fn get_progress(conn: &Connection, item_uid: &str) -> Result<Option<Progress>> {
    conn.query_row(
        "SELECT item_uid, series_uid, page_index, scroll_fraction, position_seconds,
                updated_at, completed
         FROM progress WHERE item_uid = ?1",
        params![item_uid],
        row_to_progress,
    )
    .optional()
    .map_err(Into::into)
}

/// List progress records that are not yet completed, most recently updated
/// first, up to `limit` rows.
pub fn list_in_progress(conn: &Connection, limit: u32) -> Result<Vec<Progress>> {
    let mut stmt = conn.prepare(
        "SELECT item_uid, series_uid, page_index, scroll_fraction, position_seconds,
                updated_at, completed
         FROM progress
         WHERE completed = 0
         ORDER BY updated_at DESC
         LIMIT ?1",
    )?;
    let rows = stmt
        .query_map(params![limit], row_to_progress)?
        .collect::<rusqlite::Result<Vec<Progress>>>()?;
    Ok(rows)
}

/// Mark an existing progress record completed or not. No-op (zero rows
/// affected) if no record exists for `item_uid`.
pub fn set_completed(conn: &Connection, item_uid: &str, completed: bool) -> Result<()> {
    conn.execute(
        "UPDATE progress SET completed = ?2 WHERE item_uid = ?1",
        params![item_uid, completed],
    )?;
    Ok(())
}

/// Delete the progress record for a given item/chapter uid, if one exists.
pub fn delete(conn: &Connection, item_uid: &str) -> Result<()> {
    conn.execute(
        "DELETE FROM progress WHERE item_uid = ?1",
        params![item_uid],
    )?;
    Ok(())
}

fn row_to_progress(row: &rusqlite::Row<'_>) -> rusqlite::Result<Progress> {
    Ok(Progress {
        item_uid: row.get(0)?,
        series_uid: row.get(1)?,
        page_index: row.get(2)?,
        scroll_fraction: row.get(3)?,
        position_seconds: row.get(4)?,
        updated_at: row.get(5)?,
        completed: row.get(6)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_conn() -> (tempfile::TempDir, Connection) {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = dir.path().join("progress.sqlite");
        let conn = crate::open_at(&path).expect("open_at should apply all migrations");
        (dir, conn)
    }

    fn sample_progress(item_uid: &str) -> Progress {
        Progress {
            item_uid: item_uid.to_string(),
            series_uid: Some("series-xyz".to_string()),
            page_index: Some(14),
            scroll_fraction: Some(0.42),
            position_seconds: None,
            updated_at: "2026-08-13T12:00:00Z".to_string(),
            completed: false,
        }
    }

    #[test]
    fn upsert_then_get_round_trips() {
        let (_dir, conn) = temp_conn();
        let progress = sample_progress("chapter-20-uid");

        upsert_progress(&conn, &progress).expect("upsert should succeed");

        let fetched = get_progress(&conn, "chapter-20-uid")
            .expect("get_progress should succeed")
            .expect("progress row should exist");

        assert_eq!(fetched.item_uid, progress.item_uid);
        assert_eq!(fetched.series_uid, progress.series_uid);
        assert_eq!(fetched.page_index, progress.page_index);
        assert_eq!(fetched.scroll_fraction, progress.scroll_fraction);
        assert_eq!(fetched.position_seconds, progress.position_seconds);
        assert_eq!(fetched.updated_at, progress.updated_at);
        assert_eq!(fetched.completed, progress.completed);
    }

    #[test]
    fn upsert_replaces_rather_than_duplicates() {
        let (_dir, conn) = temp_conn();
        let mut progress = sample_progress("chapter-20-uid");
        upsert_progress(&conn, &progress).expect("first upsert should succeed");

        progress.page_index = Some(15);
        progress.scroll_fraction = Some(0.9);
        progress.updated_at = "2026-08-13T13:00:00Z".to_string();
        upsert_progress(&conn, &progress).expect("second upsert should succeed");

        let fetched = get_progress(&conn, "chapter-20-uid")
            .expect("get_progress should succeed")
            .expect("progress row should exist");
        assert_eq!(fetched.page_index, Some(15));
        assert_eq!(fetched.scroll_fraction, Some(0.9));
        assert_eq!(fetched.updated_at, "2026-08-13T13:00:00Z");

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM progress", [], |row| row.get(0))
            .expect("count rows");
        assert_eq!(count, 1, "upsert must replace, not duplicate, the row");
    }

    #[test]
    fn full_lifecycle_in_progress_complete_delete() {
        let (_dir, conn) = temp_conn();
        let progress = sample_progress("chapter-20-uid");
        upsert_progress(&conn, &progress).expect("upsert should succeed");

        let in_progress = list_in_progress(&conn, 10).expect("list_in_progress should succeed");
        assert!(
            in_progress.iter().any(|p| p.item_uid == "chapter-20-uid"),
            "uncompleted progress should show up in list_in_progress"
        );

        set_completed(&conn, "chapter-20-uid", true).expect("set_completed should succeed");

        let fetched = get_progress(&conn, "chapter-20-uid")
            .expect("get_progress should succeed")
            .expect("progress row should still exist");
        assert!(fetched.completed);

        let in_progress = list_in_progress(&conn, 10).expect("list_in_progress should succeed");
        assert!(
            !in_progress.iter().any(|p| p.item_uid == "chapter-20-uid"),
            "completed progress should no longer show up in list_in_progress"
        );

        delete(&conn, "chapter-20-uid").expect("delete should succeed");
        let fetched = get_progress(&conn, "chapter-20-uid").expect("get_progress should succeed");
        assert!(
            fetched.is_none(),
            "progress row should be gone after delete"
        );
    }

    #[test]
    fn list_in_progress_orders_by_updated_at_descending_and_respects_limit() {
        let (_dir, conn) = temp_conn();

        let mut older = sample_progress("item-a");
        older.series_uid = None;
        older.updated_at = "2026-08-10T00:00:00Z".to_string();
        upsert_progress(&conn, &older).expect("upsert should succeed");

        let mut newer = sample_progress("item-b");
        newer.series_uid = None;
        newer.updated_at = "2026-08-12T00:00:00Z".to_string();
        upsert_progress(&conn, &newer).expect("upsert should succeed");

        let results = list_in_progress(&conn, 1).expect("list_in_progress should succeed");
        assert_eq!(results.len(), 1, "limit should be respected");
        assert_eq!(
            results[0].item_uid, "item-b",
            "most recently updated row should come first"
        );
    }

    #[test]
    fn plain_item_progress_uses_position_seconds_without_series() {
        let (_dir, conn) = temp_conn();
        let progress = Progress {
            item_uid: "video-item-uid".to_string(),
            series_uid: None,
            page_index: None,
            scroll_fraction: None,
            position_seconds: Some(842.5),
            updated_at: "2026-08-13T12:00:00Z".to_string(),
            completed: false,
        };
        upsert_progress(&conn, &progress).expect("upsert should succeed");

        let fetched = get_progress(&conn, "video-item-uid")
            .expect("get_progress should succeed")
            .expect("progress row should exist");
        assert_eq!(fetched.series_uid, None);
        assert_eq!(fetched.position_seconds, Some(842.5));
    }
}
