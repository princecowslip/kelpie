//! Collections repository (kelpie.md §136 Core Data Layer): manual Collection
//! (§95), the `collections` and `collection_items` tables from
//! `migrations/0002_core_data_layer.sql`.
//!
//! Scope: manual, locally-owned collections only. Smart/query-backed
//! collections (§96) are explicitly "Future" work and are out of scope here.
//!
//! Timestamp policy: `created_at`/`updated_at`/`added_at` are stamped by
//! SQLite itself via `datetime('now')` (UTC, `YYYY-MM-DD HH:MM:SS` text) at
//! INSERT/UPDATE time rather than computed in Rust, so every writer — this
//! module, `sqlite3` shell access, future migrations — gets a consistent,
//! single source of truth for "now". `create_collection` uses a `RETURNING`
//! clause (SQLite 3.35+, available via the `bundled` rusqlite feature) to
//! read back the stamped row in one round trip instead of INSERT-then-SELECT.
//!
//! Foreign keys note: `collection_items` declares `ON DELETE CASCADE` on both
//! its `collection_uid` and `item_uid` references in the migration. SQLite's
//! `foreign_keys` pragma defaults to on in this build (verified empirically
//! against a connection opened via this crate's `open_at`), so both
//! directions of cascade are live: deleting a collection removes its
//! `collection_items` rows, and deleting an item (the items repository's
//! responsibility) removes it from any collection it belonged to.
//! `delete_collection` still deletes `collection_items` explicitly inside
//! its transaction rather than relying solely on the cascade — harmless
//! belt-and-suspenders that keeps this function correct even if the pragma
//! default is ever revisited.

use rusqlite::{params, Connection, Row};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Result;

/// Collection sort order (kelpie.md §95). `Manual` is the default, matching
/// the `collections.sort_mode` column's SQL default of `'manual'`.
///
/// Only `Manual` order is actually applied by this module (via
/// `collection_items.position`, see [`list_collection_items`]); the other
/// modes describe how a *display layer* should re-sort the resolved `items`
/// rows and are not implemented as a query here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortMode {
    #[default]
    Manual,
    DateAdded,
    Published,
    Title,
    Creator,
}

impl SortMode {
    /// The exact TEXT value stored in `collections.sort_mode`.
    pub fn as_str(self) -> &'static str {
        match self {
            SortMode::Manual => "manual",
            SortMode::DateAdded => "date_added",
            SortMode::Published => "published",
            SortMode::Title => "title",
            SortMode::Creator => "creator",
        }
    }

    /// Parse a `collections.sort_mode` column value. Returns `None` for
    /// anything other than the five known values.
    pub fn from_str_opt(s: &str) -> Option<Self> {
        match s {
            "manual" => Some(SortMode::Manual),
            "date_added" => Some(SortMode::DateAdded),
            "published" => Some(SortMode::Published),
            "title" => Some(SortMode::Title),
            "creator" => Some(SortMode::Creator),
            _ => None,
        }
    }
}

impl rusqlite::types::ToSql for SortMode {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::from(self.as_str()))
    }
}

impl rusqlite::types::FromSql for SortMode {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let s = value.as_str()?;
        SortMode::from_str_opt(s).ok_or_else(|| {
            rusqlite::types::FromSqlError::Other(
                format!("unknown collections.sort_mode value: {s:?}").into(),
            )
        })
    }
}

/// A manual collection (kelpie.md §95): a user-named, locally-ordered
/// grouping of items that may mix any item kind.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Collection {
    pub uid: String,
    pub name: String,
    pub sort_mode: SortMode,
    pub created_at: String,
    pub updated_at: String,
}

/// One row of the `collection_items` join table: an item's membership in and
/// position within a collection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectionItem {
    pub collection_uid: String,
    pub item_uid: String,
    pub position: i64,
    pub added_at: String,
}

fn row_to_collection(row: &Row<'_>) -> rusqlite::Result<Collection> {
    Ok(Collection {
        uid: row.get(0)?,
        name: row.get(1)?,
        sort_mode: row.get(2)?,
        created_at: row.get(3)?,
        updated_at: row.get(4)?,
    })
}

fn row_to_collection_item(row: &Row<'_>) -> rusqlite::Result<CollectionItem> {
    Ok(CollectionItem {
        collection_uid: row.get(0)?,
        item_uid: row.get(1)?,
        position: row.get(2)?,
        added_at: row.get(3)?,
    })
}

/// Create a new manual collection. Generates a fresh `uuid::Uuid::new_v4()`
/// uid (collections have no provider-issued identity, unlike items/series)
/// and lets SQLite stamp `created_at`/`updated_at` via `datetime('now')`,
/// reading the inserted row back with `RETURNING`.
pub fn create_collection(conn: &Connection, name: &str, sort_mode: SortMode) -> Result<Collection> {
    let uid = Uuid::new_v4().to_string();
    let collection = conn.query_row(
        "INSERT INTO collections (uid, name, sort_mode, created_at, updated_at)
         VALUES (?1, ?2, ?3, datetime('now'), datetime('now'))
         RETURNING uid, name, sort_mode, created_at, updated_at",
        params![uid, name, sort_mode],
        row_to_collection,
    )?;
    Ok(collection)
}

/// Fetch a single collection by uid, or `None` if it doesn't exist.
pub fn get_collection(conn: &Connection, uid: &str) -> Result<Option<Collection>> {
    match conn.query_row(
        "SELECT uid, name, sort_mode, created_at, updated_at FROM collections WHERE uid = ?1",
        params![uid],
        row_to_collection,
    ) {
        Ok(collection) => Ok(Some(collection)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(err) => Err(err.into()),
    }
}

/// List every collection, ordered by creation time (then uid, as a tiebreaker
/// for collections created within the same `datetime('now')` second).
pub fn list_collections(conn: &Connection) -> Result<Vec<Collection>> {
    let mut stmt = conn.prepare(
        "SELECT uid, name, sort_mode, created_at, updated_at
         FROM collections
         ORDER BY created_at ASC, uid ASC",
    )?;
    let rows = stmt.query_map([], row_to_collection)?;
    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
}

/// Rename a collection and bump its `updated_at`. A no-op (not an error) if
/// `uid` doesn't exist.
pub fn rename_collection(conn: &Connection, uid: &str, name: &str) -> Result<()> {
    conn.execute(
        "UPDATE collections SET name = ?2, updated_at = datetime('now') WHERE uid = ?1",
        params![uid, name],
    )?;
    Ok(())
}

/// Change a collection's sort mode and bump its `updated_at`. A no-op (not an
/// error) if `uid` doesn't exist.
pub fn set_sort_mode(conn: &Connection, uid: &str, sort_mode: SortMode) -> Result<()> {
    conn.execute(
        "UPDATE collections SET sort_mode = ?2, updated_at = datetime('now') WHERE uid = ?1",
        params![uid, sort_mode],
    )?;
    Ok(())
}

/// Delete a collection and every `collection_items` row that references it.
/// Explicitly deletes the join rows first rather than relying on the
/// schema's `ON DELETE CASCADE` (see the module-level foreign keys note),
/// so this cascades correctly even on a connection without
/// `PRAGMA foreign_keys = ON`. A no-op (not an error) if `uid` doesn't exist.
pub fn delete_collection(conn: &Connection, uid: &str) -> Result<()> {
    let tx = conn.unchecked_transaction()?;
    tx.execute(
        "DELETE FROM collection_items WHERE collection_uid = ?1",
        params![uid],
    )?;
    tx.execute("DELETE FROM collections WHERE uid = ?1", params![uid])?;
    tx.commit()?;
    Ok(())
}

/// Add an item to a collection, appending it at the end (`position` =
/// `1 + MAX(position)` for that collection, or `0` if it's currently empty).
///
/// Idempotent: if the item is already in the collection, this is a no-op —
/// its existing `position` and `added_at` are left untouched rather than
/// moving it to the end again. Relies on `collection_items`'s
/// `(collection_uid, item_uid)` primary key via `ON CONFLICT DO NOTHING`.
pub fn add_item_to_collection(
    conn: &Connection,
    collection_uid: &str,
    item_uid: &str,
) -> Result<()> {
    let next_position: i64 = conn.query_row(
        "SELECT COALESCE(MAX(position) + 1, 0) FROM collection_items WHERE collection_uid = ?1",
        params![collection_uid],
        |row| row.get(0),
    )?;
    conn.execute(
        "INSERT INTO collection_items (collection_uid, item_uid, position, added_at)
         VALUES (?1, ?2, ?3, datetime('now'))
         ON CONFLICT (collection_uid, item_uid) DO NOTHING",
        params![collection_uid, item_uid, next_position],
    )?;
    Ok(())
}

/// Remove an item from a collection. A no-op (not an error) if it wasn't a
/// member.
pub fn remove_item_from_collection(
    conn: &Connection,
    collection_uid: &str,
    item_uid: &str,
) -> Result<()> {
    conn.execute(
        "DELETE FROM collection_items WHERE collection_uid = ?1 AND item_uid = ?2",
        params![collection_uid, item_uid],
    )?;
    Ok(())
}

/// Move an item to `new_position` within its collection's manual order
/// (kelpie.md §95), shifting the other items to keep a contiguous,
/// zero-based `position` sequence with no gaps or duplicates.
///
/// `new_position` is clamped into `[0, item_count - 1]` (moving past either
/// end just moves the item to that end). A no-op (not an error) if
/// `item_uid` isn't a member of `collection_uid`.
pub fn reorder_collection_item(
    conn: &Connection,
    collection_uid: &str,
    item_uid: &str,
    new_position: i64,
) -> Result<()> {
    let tx = conn.unchecked_transaction()?;

    let mut item_uids: Vec<String> = {
        let mut stmt = tx.prepare(
            "SELECT item_uid FROM collection_items
             WHERE collection_uid = ?1
             ORDER BY position ASC, item_uid ASC",
        )?;
        let rows = stmt
            .query_map(params![collection_uid], |row| row.get(0))?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        rows
    };

    let Some(current_index) = item_uids.iter().position(|uid| uid == item_uid) else {
        // Not a member of this collection: nothing to reorder.
        return Ok(());
    };
    item_uids.remove(current_index);

    let target_index = new_position.clamp(0, item_uids.len() as i64) as usize;
    item_uids.insert(target_index, item_uid.to_string());

    for (position, uid) in item_uids.iter().enumerate() {
        tx.execute(
            "UPDATE collection_items SET position = ?3 WHERE collection_uid = ?1 AND item_uid = ?2",
            params![collection_uid, uid, position as i64],
        )?;
    }

    tx.commit()?;
    Ok(())
}

/// List a collection's items in manual/position order (ascending). This is
/// the only ordering this module applies; `sort_mode` values other than
/// `Manual` are a display-layer concern for whoever resolves these
/// `item_uid`s against the `items` table, not something implemented here.
pub fn list_collection_items(
    conn: &Connection,
    collection_uid: &str,
) -> Result<Vec<CollectionItem>> {
    let mut stmt = conn.prepare(
        "SELECT collection_uid, item_uid, position, added_at
         FROM collection_items
         WHERE collection_uid = ?1
         ORDER BY position ASC, item_uid ASC",
    )?;
    let rows = stmt.query_map(params![collection_uid], row_to_collection_item)?;
    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
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

    /// Insert a minimal, obviously-synthetic `items` row directly via raw
    /// SQL so `collection_items`'s foreign key to `items(uid)` is satisfied.
    /// Placeholder values only — not representative of real provider data.
    fn insert_test_item(conn: &Connection, uid: &str) {
        conn.execute(
            "INSERT INTO items (
                uid, provider_id, canonical_url, kind, title,
                classification_json, availability, created_at, updated_locally_at
             ) VALUES (
                ?1, 'provider-x', 'https://example.test/items/' || ?1, 'video', 'Test Item',
                '{}', 'available', datetime('now'), datetime('now')
             )",
            params![uid],
        )
        .expect("insert test item");
    }

    fn insert_test_collection(conn: &Connection, name: &str) -> Collection {
        create_collection(conn, name, SortMode::Manual).expect("create collection")
    }

    // -- SortMode -------------------------------------------------------

    #[test]
    fn sort_mode_as_str_matches_column_values() {
        assert_eq!(SortMode::Manual.as_str(), "manual");
        assert_eq!(SortMode::DateAdded.as_str(), "date_added");
        assert_eq!(SortMode::Published.as_str(), "published");
        assert_eq!(SortMode::Title.as_str(), "title");
        assert_eq!(SortMode::Creator.as_str(), "creator");
    }

    #[test]
    fn sort_mode_default_is_manual() {
        assert_eq!(SortMode::default(), SortMode::Manual);
    }

    #[test]
    fn sort_mode_from_str_opt_round_trips_all_variants() {
        for mode in [
            SortMode::Manual,
            SortMode::DateAdded,
            SortMode::Published,
            SortMode::Title,
            SortMode::Creator,
        ] {
            assert_eq!(SortMode::from_str_opt(mode.as_str()), Some(mode));
        }
        assert_eq!(SortMode::from_str_opt("not-a-real-mode"), None);
    }

    // -- create / get / list ---------------------------------------------

    #[test]
    fn create_collection_round_trips_via_get() {
        let (_dir, path) = temp_db_path("create.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        let created = create_collection(&conn, "Test Collection", SortMode::Manual)
            .expect("create collection");
        assert!(!created.uid.is_empty());
        assert!(
            Uuid::parse_str(&created.uid).is_ok(),
            "uid should be a uuid"
        );
        assert_eq!(created.name, "Test Collection");
        assert_eq!(created.sort_mode, SortMode::Manual);
        assert!(!created.created_at.is_empty());
        assert_eq!(created.created_at, created.updated_at);

        let fetched = get_collection(&conn, &created.uid)
            .expect("get collection")
            .expect("collection should exist");
        assert_eq!(fetched, created);
    }

    #[test]
    fn create_collection_defaults_column_default_is_manual() {
        // Bypass the repository function to confirm the SQL column default
        // itself (not just Rust's Default impl) is 'manual', per the
        // migration's `DEFAULT 'manual'`.
        let (_dir, path) = temp_db_path("column_default.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        conn.execute(
            "INSERT INTO collections (uid, name, created_at, updated_at)
             VALUES ('col-1', 'Test Collection', datetime('now'), datetime('now'))",
            [],
        )
        .expect("insert without sort_mode");

        let fetched = get_collection(&conn, "col-1")
            .expect("get collection")
            .expect("collection should exist");
        assert_eq!(fetched.sort_mode, SortMode::Manual);
    }

    #[test]
    fn sort_mode_round_trips_through_all_variants() {
        let (_dir, path) = temp_db_path("sort_modes.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        for mode in [
            SortMode::Manual,
            SortMode::DateAdded,
            SortMode::Published,
            SortMode::Title,
            SortMode::Creator,
        ] {
            let created =
                create_collection(&conn, "Test Collection", mode).expect("create collection");
            let fetched = get_collection(&conn, &created.uid)
                .expect("get collection")
                .expect("collection should exist");
            assert_eq!(fetched.sort_mode, mode);
        }
    }

    #[test]
    fn get_collection_returns_none_for_missing_uid() {
        let (_dir, path) = temp_db_path("missing.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        assert_eq!(get_collection(&conn, "does-not-exist").unwrap(), None);
    }

    #[test]
    fn list_collections_returns_all_created_collections() {
        let (_dir, path) = temp_db_path("list.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        let a = insert_test_collection(&conn, "Collection A");
        let b = insert_test_collection(&conn, "Collection B");
        let c = insert_test_collection(&conn, "Collection C");

        let listed = list_collections(&conn).expect("list collections");
        let listed_uids: Vec<&str> = listed.iter().map(|c| c.uid.as_str()).collect();
        assert_eq!(listed.len(), 3);
        assert!(listed_uids.contains(&a.uid.as_str()));
        assert!(listed_uids.contains(&b.uid.as_str()));
        assert!(listed_uids.contains(&c.uid.as_str()));
    }

    #[test]
    fn list_collections_on_empty_database_is_empty() {
        let (_dir, path) = temp_db_path("list_empty.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        assert_eq!(list_collections(&conn).unwrap(), Vec::new());
    }

    // -- rename / set_sort_mode -------------------------------------------

    #[test]
    fn rename_collection_updates_name() {
        let (_dir, path) = temp_db_path("rename.sqlite");
        let conn = crate::open_at(&path).expect("open db");
        let created = insert_test_collection(&conn, "Old Name");

        rename_collection(&conn, &created.uid, "New Name").expect("rename collection");

        let fetched = get_collection(&conn, &created.uid).unwrap().unwrap();
        assert_eq!(fetched.name, "New Name");
    }

    #[test]
    fn rename_collection_missing_uid_is_noop() {
        let (_dir, path) = temp_db_path("rename_missing.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        rename_collection(&conn, "does-not-exist", "New Name").expect("should not error");
        assert_eq!(list_collections(&conn).unwrap().len(), 0);
    }

    #[test]
    fn set_sort_mode_updates_value() {
        let (_dir, path) = temp_db_path("set_sort_mode.sqlite");
        let conn = crate::open_at(&path).expect("open db");
        let created = insert_test_collection(&conn, "Test Collection");
        assert_eq!(created.sort_mode, SortMode::Manual);

        set_sort_mode(&conn, &created.uid, SortMode::Title).expect("set sort mode");

        let fetched = get_collection(&conn, &created.uid).unwrap().unwrap();
        assert_eq!(fetched.sort_mode, SortMode::Title);
    }

    // -- delete -------------------------------------------------------------

    #[test]
    fn delete_collection_removes_it() {
        let (_dir, path) = temp_db_path("delete.sqlite");
        let conn = crate::open_at(&path).expect("open db");
        let created = insert_test_collection(&conn, "Test Collection");

        delete_collection(&conn, &created.uid).expect("delete collection");

        assert_eq!(get_collection(&conn, &created.uid).unwrap(), None);
    }

    #[test]
    fn delete_collection_missing_uid_is_noop() {
        let (_dir, path) = temp_db_path("delete_missing.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        delete_collection(&conn, "does-not-exist").expect("should not error");
    }

    #[test]
    fn delete_collection_cascades_to_collection_items() {
        let (_dir, path) = temp_db_path("delete_cascade.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        let collection = insert_test_collection(&conn, "Test Collection");
        insert_test_item(&conn, "item-1");
        insert_test_item(&conn, "item-2");
        add_item_to_collection(&conn, &collection.uid, "item-1").unwrap();
        add_item_to_collection(&conn, &collection.uid, "item-2").unwrap();
        assert_eq!(
            list_collection_items(&conn, &collection.uid).unwrap().len(),
            2
        );

        delete_collection(&conn, &collection.uid).expect("delete collection");

        assert_eq!(
            list_collection_items(&conn, &collection.uid).unwrap(),
            Vec::new()
        );
        let raw_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM collection_items WHERE collection_uid = ?1",
                params![collection.uid],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(raw_count, 0);
    }

    #[test]
    fn deleting_an_item_cascades_out_of_its_collections() {
        // This exercises the schema-level `ON DELETE CASCADE` on
        // collection_items.item_uid declared in the migration. That cascade
        // only fires when the connection has foreign key enforcement on,
        // which `open_at` does not currently set (see module docs) — so this
        // test enables it explicitly to verify the schema itself is correct
        // for a caller (e.g. the items repository) that does.
        let (_dir, path) = temp_db_path("delete_item_cascade.sqlite");
        let conn = crate::open_at(&path).expect("open db");
        conn.execute_batch("PRAGMA foreign_keys = ON;").unwrap();

        let collection = insert_test_collection(&conn, "Test Collection");
        insert_test_item(&conn, "item-1");
        add_item_to_collection(&conn, &collection.uid, "item-1").unwrap();
        assert_eq!(
            list_collection_items(&conn, &collection.uid).unwrap().len(),
            1
        );

        conn.execute("DELETE FROM items WHERE uid = ?1", params!["item-1"])
            .expect("delete item");

        assert_eq!(
            list_collection_items(&conn, &collection.uid).unwrap(),
            Vec::new()
        );
    }

    // -- add / remove items ------------------------------------------------

    #[test]
    fn add_item_to_collection_assigns_sequential_positions() {
        let (_dir, path) = temp_db_path("add_positions.sqlite");
        let conn = crate::open_at(&path).expect("open db");
        let collection = insert_test_collection(&conn, "Test Collection");
        insert_test_item(&conn, "item-1");
        insert_test_item(&conn, "item-2");
        insert_test_item(&conn, "item-3");

        add_item_to_collection(&conn, &collection.uid, "item-1").unwrap();
        add_item_to_collection(&conn, &collection.uid, "item-2").unwrap();
        add_item_to_collection(&conn, &collection.uid, "item-3").unwrap();

        let items = list_collection_items(&conn, &collection.uid).unwrap();
        let ordered: Vec<(&str, i64)> = items
            .iter()
            .map(|i| (i.item_uid.as_str(), i.position))
            .collect();
        assert_eq!(ordered, vec![("item-1", 0), ("item-2", 1), ("item-3", 2)]);
    }

    #[test]
    fn add_item_to_collection_is_idempotent() {
        let (_dir, path) = temp_db_path("add_idempotent.sqlite");
        let conn = crate::open_at(&path).expect("open db");
        let collection = insert_test_collection(&conn, "Test Collection");
        insert_test_item(&conn, "item-1");
        insert_test_item(&conn, "item-2");

        add_item_to_collection(&conn, &collection.uid, "item-1").unwrap();
        add_item_to_collection(&conn, &collection.uid, "item-2").unwrap();
        // Re-add item-1: should be a no-op, not move it or create a duplicate.
        add_item_to_collection(&conn, &collection.uid, "item-1").unwrap();

        let items = list_collection_items(&conn, &collection.uid).unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].item_uid, "item-1");
        assert_eq!(items[0].position, 0);
        assert_eq!(items[1].item_uid, "item-2");
        assert_eq!(items[1].position, 1);
    }

    #[test]
    fn remove_item_from_collection_removes_row() {
        let (_dir, path) = temp_db_path("remove.sqlite");
        let conn = crate::open_at(&path).expect("open db");
        let collection = insert_test_collection(&conn, "Test Collection");
        insert_test_item(&conn, "item-1");
        insert_test_item(&conn, "item-2");
        add_item_to_collection(&conn, &collection.uid, "item-1").unwrap();
        add_item_to_collection(&conn, &collection.uid, "item-2").unwrap();

        remove_item_from_collection(&conn, &collection.uid, "item-1").unwrap();

        let items = list_collection_items(&conn, &collection.uid).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].item_uid, "item-2");
    }

    #[test]
    fn remove_item_from_collection_missing_membership_is_noop() {
        let (_dir, path) = temp_db_path("remove_missing.sqlite");
        let conn = crate::open_at(&path).expect("open db");
        let collection = insert_test_collection(&conn, "Test Collection");

        remove_item_from_collection(&conn, &collection.uid, "never-added")
            .expect("should not error");
    }

    // -- reorder --------------------------------------------------------

    #[test]
    fn reorder_collection_item_moves_item_to_new_position() {
        let (_dir, path) = temp_db_path("reorder.sqlite");
        let conn = crate::open_at(&path).expect("open db");
        let collection = insert_test_collection(&conn, "Test Collection");
        for uid in ["item-1", "item-2", "item-3"] {
            insert_test_item(&conn, uid);
            add_item_to_collection(&conn, &collection.uid, uid).unwrap();
        }
        // Starting order: item-1, item-2, item-3

        reorder_collection_item(&conn, &collection.uid, "item-3", 0).expect("reorder");

        let ordered: Vec<String> = list_collection_items(&conn, &collection.uid)
            .unwrap()
            .into_iter()
            .map(|i| i.item_uid)
            .collect();
        assert_eq!(ordered, vec!["item-3", "item-1", "item-2"]);

        // Positions should be a clean contiguous 0..n sequence.
        let positions: Vec<i64> = list_collection_items(&conn, &collection.uid)
            .unwrap()
            .into_iter()
            .map(|i| i.position)
            .collect();
        assert_eq!(positions, vec![0, 1, 2]);
    }

    #[test]
    fn reorder_collection_item_to_middle_position() {
        let (_dir, path) = temp_db_path("reorder_middle.sqlite");
        let conn = crate::open_at(&path).expect("open db");
        let collection = insert_test_collection(&conn, "Test Collection");
        for uid in ["item-1", "item-2", "item-3", "item-4"] {
            insert_test_item(&conn, uid);
            add_item_to_collection(&conn, &collection.uid, uid).unwrap();
        }

        // Move item-1 to index 2: item-2, item-3, item-1, item-4
        reorder_collection_item(&conn, &collection.uid, "item-1", 2).expect("reorder");

        let ordered: Vec<String> = list_collection_items(&conn, &collection.uid)
            .unwrap()
            .into_iter()
            .map(|i| i.item_uid)
            .collect();
        assert_eq!(ordered, vec!["item-2", "item-3", "item-1", "item-4"]);
    }

    #[test]
    fn reorder_collection_item_clamps_out_of_range_position() {
        let (_dir, path) = temp_db_path("reorder_clamp.sqlite");
        let conn = crate::open_at(&path).expect("open db");
        let collection = insert_test_collection(&conn, "Test Collection");
        for uid in ["item-1", "item-2", "item-3"] {
            insert_test_item(&conn, uid);
            add_item_to_collection(&conn, &collection.uid, uid).unwrap();
        }

        reorder_collection_item(&conn, &collection.uid, "item-1", 9999).expect("reorder");

        let ordered: Vec<String> = list_collection_items(&conn, &collection.uid)
            .unwrap()
            .into_iter()
            .map(|i| i.item_uid)
            .collect();
        assert_eq!(ordered, vec!["item-2", "item-3", "item-1"]);
    }

    #[test]
    fn reorder_collection_item_negative_position_clamps_to_front() {
        let (_dir, path) = temp_db_path("reorder_negative.sqlite");
        let conn = crate::open_at(&path).expect("open db");
        let collection = insert_test_collection(&conn, "Test Collection");
        for uid in ["item-1", "item-2", "item-3"] {
            insert_test_item(&conn, uid);
            add_item_to_collection(&conn, &collection.uid, uid).unwrap();
        }

        reorder_collection_item(&conn, &collection.uid, "item-3", -50).expect("reorder");

        let ordered: Vec<String> = list_collection_items(&conn, &collection.uid)
            .unwrap()
            .into_iter()
            .map(|i| i.item_uid)
            .collect();
        assert_eq!(ordered, vec!["item-3", "item-1", "item-2"]);
    }

    #[test]
    fn reorder_collection_item_not_a_member_is_noop() {
        let (_dir, path) = temp_db_path("reorder_not_member.sqlite");
        let conn = crate::open_at(&path).expect("open db");
        let collection = insert_test_collection(&conn, "Test Collection");
        insert_test_item(&conn, "item-1");
        add_item_to_collection(&conn, &collection.uid, "item-1").unwrap();

        reorder_collection_item(&conn, &collection.uid, "never-added", 0)
            .expect("should not error");

        let items = list_collection_items(&conn, &collection.uid).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].item_uid, "item-1");
    }

    // -- listing scoping --------------------------------------------------

    #[test]
    fn list_collection_items_is_scoped_per_collection() {
        let (_dir, path) = temp_db_path("scoped.sqlite");
        let conn = crate::open_at(&path).expect("open db");
        let collection_a = insert_test_collection(&conn, "Collection A");
        let collection_b = insert_test_collection(&conn, "Collection B");
        insert_test_item(&conn, "item-1");
        insert_test_item(&conn, "item-2");

        add_item_to_collection(&conn, &collection_a.uid, "item-1").unwrap();
        add_item_to_collection(&conn, &collection_b.uid, "item-2").unwrap();

        let items_a = list_collection_items(&conn, &collection_a.uid).unwrap();
        let items_b = list_collection_items(&conn, &collection_b.uid).unwrap();
        assert_eq!(items_a.len(), 1);
        assert_eq!(items_a[0].item_uid, "item-1");
        assert_eq!(items_a[0].position, 0);
        assert_eq!(items_b.len(), 1);
        assert_eq!(items_b[0].item_uid, "item-2");
        assert_eq!(items_b[0].position, 0);
    }

    #[test]
    fn list_collection_items_on_empty_collection_is_empty() {
        let (_dir, path) = temp_db_path("empty_items.sqlite");
        let conn = crate::open_at(&path).expect("open db");
        let collection = insert_test_collection(&conn, "Test Collection");

        assert_eq!(
            list_collection_items(&conn, &collection.uid).unwrap(),
            Vec::new()
        );
    }
}
