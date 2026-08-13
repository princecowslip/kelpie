//! `Collection` repository (kelpie.md §136 Phase 2, §94-95 Library / Collections).
//! Backs migration `0007_collections.sql`.
//!
//! SQLite's `PRAGMA foreign_keys` enforcement is not guaranteed on for every
//! connection this repository might be handed (the shared connection setup in
//! `crate::open_at` does not toggle it), so [`delete_collection`] cleans up
//! `collection_items` rows explicitly rather than relying solely on the schema's
//! `ON DELETE CASCADE` to fire.

use rusqlite::{params, Connection, OptionalExtension};

use kelpie_core::domain::collection::{Collection, CollectionItem, CollectionSortMode};

use crate::{Error, Result};

fn sort_mode_as_db_str(sort_mode: CollectionSortMode) -> &'static str {
    match sort_mode {
        CollectionSortMode::Manual => "manual",
        CollectionSortMode::DateAdded => "date_added",
        CollectionSortMode::Published => "published",
        CollectionSortMode::Title => "title",
        CollectionSortMode::Creator => "creator",
    }
}

fn sort_mode_from_db_str(value: &str) -> Result<CollectionSortMode> {
    match value {
        "manual" => Ok(CollectionSortMode::Manual),
        "date_added" => Ok(CollectionSortMode::DateAdded),
        "published" => Ok(CollectionSortMode::Published),
        "title" => Ok(CollectionSortMode::Title),
        "creator" => Ok(CollectionSortMode::Creator),
        other => Err(Error::IntegrityCheck(format!(
            "unrecognized collection sort_mode in database: {other}"
        ))),
    }
}

fn row_to_collection(
    row: &rusqlite::Row<'_>,
) -> rusqlite::Result<(String, String, String, String, String)> {
    Ok((
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
        row.get(4)?,
    ))
}

/// Create a new collection.
pub fn create_collection(conn: &Connection, collection: &Collection) -> Result<()> {
    conn.execute(
        "INSERT INTO collections (uid, name, sort_mode, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            collection.uid,
            collection.name,
            sort_mode_as_db_str(collection.sort_mode),
            collection.created_at,
            collection.updated_at,
        ],
    )?;
    Ok(())
}

/// Fetch a collection by its uid. Returns `Ok(None)` if no such collection exists.
pub fn get_collection_by_uid(conn: &Connection, uid: &str) -> Result<Option<Collection>> {
    let result = conn
        .query_row(
            "SELECT uid, name, sort_mode, created_at, updated_at
             FROM collections WHERE uid = ?1",
            params![uid],
            row_to_collection,
        )
        .optional()?;

    match result {
        Some((uid, name, sort_mode, created_at, updated_at)) => Ok(Some(Collection {
            uid,
            name,
            sort_mode: sort_mode_from_db_str(&sort_mode)?,
            created_at,
            updated_at,
        })),
        None => Ok(None),
    }
}

/// Update a collection's mutable fields (name, sort mode, updated_at). Leaves `uid`
/// and `created_at` untouched.
pub fn update_collection(
    conn: &Connection,
    uid: &str,
    name: &str,
    sort_mode: CollectionSortMode,
    updated_at: &str,
) -> Result<()> {
    conn.execute(
        "UPDATE collections SET name = ?1, sort_mode = ?2, updated_at = ?3 WHERE uid = ?4",
        params![name, sort_mode_as_db_str(sort_mode), updated_at, uid],
    )?;
    Ok(())
}

/// Delete a collection and all of its `collection_items` rows.
pub fn delete_collection(conn: &Connection, uid: &str) -> Result<()> {
    conn.execute(
        "DELETE FROM collection_items WHERE collection_uid = ?1",
        params![uid],
    )?;
    conn.execute("DELETE FROM collections WHERE uid = ?1", params![uid])?;
    Ok(())
}

/// Add an item to a collection at the given manual `position`.
pub fn add_item(
    conn: &Connection,
    collection_uid: &str,
    item_uid: &str,
    position: i64,
    added_at: &str,
) -> Result<()> {
    conn.execute(
        "INSERT INTO collection_items (collection_uid, item_uid, position, added_at)
         VALUES (?1, ?2, ?3, ?4)",
        params![collection_uid, item_uid, position, added_at],
    )?;
    Ok(())
}

/// Remove an item from a collection.
pub fn remove_item(conn: &Connection, collection_uid: &str, item_uid: &str) -> Result<()> {
    conn.execute(
        "DELETE FROM collection_items WHERE collection_uid = ?1 AND item_uid = ?2",
        params![collection_uid, item_uid],
    )?;
    Ok(())
}

/// Rewrite `position` for every item in `ordered_item_uids`, in the given order,
/// starting at 0. Items not present in `collection_uid` are silently skipped.
pub fn reorder_items(
    conn: &Connection,
    collection_uid: &str,
    ordered_item_uids: &[String],
) -> Result<()> {
    for (position, item_uid) in ordered_item_uids.iter().enumerate() {
        conn.execute(
            "UPDATE collection_items SET position = ?1 WHERE collection_uid = ?2 AND item_uid = ?3",
            params![position as i64, collection_uid, item_uid],
        )?;
    }
    Ok(())
}

/// List a collection's items, ordered by `position` ascending.
pub fn list_items(conn: &Connection, collection_uid: &str) -> Result<Vec<CollectionItem>> {
    let mut stmt = conn.prepare(
        "SELECT collection_uid, item_uid, position, added_at
         FROM collection_items WHERE collection_uid = ?1 ORDER BY position ASC",
    )?;
    let items = stmt
        .query_map(params![collection_uid], |row| {
            Ok(CollectionItem {
                collection_uid: row.get(0)?,
                item_uid: row.get(1)?,
                position: row.get(2)?,
                added_at: row.get(3)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_conn() -> (tempfile::TempDir, Connection) {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = dir.path().join("collections.sqlite");
        let conn = crate::open_at(&path).expect("open_at should apply all migrations");
        (dir, conn)
    }

    fn sample_collection(uid: &str) -> Collection {
        Collection {
            uid: uid.to_string(),
            name: "Favorites".to_string(),
            sort_mode: CollectionSortMode::Manual,
            created_at: "2026-08-13T00:00:00Z".to_string(),
            updated_at: "2026-08-13T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn create_and_get_round_trip() {
        let (_dir, conn) = temp_conn();
        let collection = sample_collection("col-1");
        create_collection(&conn, &collection).expect("create_collection");

        let fetched = get_collection_by_uid(&conn, "col-1")
            .expect("get_collection_by_uid")
            .expect("collection should exist");
        assert_eq!(fetched.uid, "col-1");
        assert_eq!(fetched.name, "Favorites");
        assert_eq!(fetched.sort_mode, CollectionSortMode::Manual);
    }

    #[test]
    fn get_missing_collection_returns_none() {
        let (_dir, conn) = temp_conn();
        assert!(get_collection_by_uid(&conn, "nope")
            .expect("query should succeed")
            .is_none());
    }

    #[test]
    fn update_collection_changes_name_and_sort_mode() {
        let (_dir, conn) = temp_conn();
        create_collection(&conn, &sample_collection("col-1")).expect("create_collection");

        update_collection(
            &conn,
            "col-1",
            "Renamed",
            CollectionSortMode::Title,
            "2026-08-13T01:00:00Z",
        )
        .expect("update_collection");

        let fetched = get_collection_by_uid(&conn, "col-1")
            .expect("get_collection_by_uid")
            .expect("collection should exist");
        assert_eq!(fetched.name, "Renamed");
        assert_eq!(fetched.sort_mode, CollectionSortMode::Title);
        assert_eq!(fetched.updated_at, "2026-08-13T01:00:00Z");
    }

    /// End-to-end: real migrated database, real repository calls, covering add /
    /// list / reorder / remove / delete-cascades-items.
    #[test]
    fn collection_lifecycle_end_to_end() {
        let (_dir, conn) = temp_conn();
        let collection = sample_collection("col-lifecycle");
        create_collection(&conn, &collection).expect("create_collection");

        add_item(&conn, "col-lifecycle", "item-a", 0, "2026-08-13T00:00:00Z").expect("add item-a");
        add_item(&conn, "col-lifecycle", "item-b", 1, "2026-08-13T00:01:00Z").expect("add item-b");
        add_item(&conn, "col-lifecycle", "item-c", 2, "2026-08-13T00:02:00Z").expect("add item-c");

        let items = list_items(&conn, "col-lifecycle").expect("list_items");
        let uids: Vec<&str> = items.iter().map(|i| i.item_uid.as_str()).collect();
        assert_eq!(uids, vec!["item-a", "item-b", "item-c"]);

        reorder_items(
            &conn,
            "col-lifecycle",
            &[
                "item-c".to_string(),
                "item-a".to_string(),
                "item-b".to_string(),
            ],
        )
        .expect("reorder_items");

        let reordered = list_items(&conn, "col-lifecycle").expect("list_items after reorder");
        let reordered_uids: Vec<&str> = reordered.iter().map(|i| i.item_uid.as_str()).collect();
        assert_eq!(reordered_uids, vec!["item-c", "item-a", "item-b"]);
        assert_eq!(reordered[0].position, 0);
        assert_eq!(reordered[1].position, 1);
        assert_eq!(reordered[2].position, 2);

        remove_item(&conn, "col-lifecycle", "item-a").expect("remove_item");
        let after_remove = list_items(&conn, "col-lifecycle").expect("list_items after remove");
        let after_remove_uids: Vec<&str> =
            after_remove.iter().map(|i| i.item_uid.as_str()).collect();
        assert_eq!(after_remove_uids, vec!["item-c", "item-b"]);

        delete_collection(&conn, "col-lifecycle").expect("delete_collection");
        assert!(get_collection_by_uid(&conn, "col-lifecycle")
            .expect("query should succeed")
            .is_none());

        let remaining_items = list_items(&conn, "col-lifecycle").expect("list_items after delete");
        assert!(
            remaining_items.is_empty(),
            "collection_items rows for a deleted collection must also be gone"
        );
    }

    #[test]
    fn unique_constraint_prevents_duplicate_item_membership() {
        let (_dir, conn) = temp_conn();
        create_collection(&conn, &sample_collection("col-dupe")).expect("create_collection");
        add_item(&conn, "col-dupe", "item-a", 0, "2026-08-13T00:00:00Z").expect("first add");

        let result = add_item(&conn, "col-dupe", "item-a", 1, "2026-08-13T00:00:01Z");
        assert!(
            result.is_err(),
            "adding the same item_uid to a collection twice should violate the UNIQUE constraint"
        );
    }
}
