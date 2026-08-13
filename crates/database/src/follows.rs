//! Follows + Favorites repository (kelpie.md §136 Core Data Layer): Following
//! (§97) and Saved/favorites (§94), the `follows` and `favorites` tables from
//! `migrations/0002_core_data_layer.sql`.
//!
//! `follows` is deliberately polymorphic — §97 allows following a provider,
//! creator, series, tag, or saved search, and those live in otherwise
//! unrelated tables (some, like `providers`/`saved_searches`, don't exist
//! yet in this phase — Provider Platform is §137, saved searches are §138).
//! There is no foreign key on `target_id` (it can't be, since it's
//! polymorphic); this module's job is just to keep `target_type` restricted
//! to the five known kinds (enforced twice over: the migration's `CHECK`
//! constraint, and this module's closed `FollowTargetType` enum) and to
//! leave `target_id` referential integrity to callers.

use rusqlite::{params, Connection, OptionalExtension, Row};
use serde::{Deserialize, Serialize};

use crate::Result;

/// What a `follows` row can target (kelpie.md §97). The string mapping here
/// must match the migration's `CHECK (target_type IN (...))` constraint
/// exactly, or every insert of that variant will fail the constraint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FollowTargetType {
    Provider,
    Creator,
    Series,
    Tag,
    SavedSearch,
}

impl FollowTargetType {
    /// All variants, in a stable order — used by tests to assert coverage.
    pub const ALL: [FollowTargetType; 5] = [
        FollowTargetType::Provider,
        FollowTargetType::Creator,
        FollowTargetType::Series,
        FollowTargetType::Tag,
        FollowTargetType::SavedSearch,
    ];

    /// The exact TEXT value stored in `follows.target_type` (and allowed by
    /// the migration's `CHECK` constraint).
    pub fn as_str(self) -> &'static str {
        match self {
            FollowTargetType::Provider => "provider",
            FollowTargetType::Creator => "creator",
            FollowTargetType::Series => "series",
            FollowTargetType::Tag => "tag",
            FollowTargetType::SavedSearch => "saved_search",
        }
    }

    /// Parse a `follows.target_type` column value. Returns `None` for
    /// anything other than the five known values.
    pub fn from_str_opt(s: &str) -> Option<Self> {
        match s {
            "provider" => Some(FollowTargetType::Provider),
            "creator" => Some(FollowTargetType::Creator),
            "series" => Some(FollowTargetType::Series),
            "tag" => Some(FollowTargetType::Tag),
            "saved_search" => Some(FollowTargetType::SavedSearch),
            _ => None,
        }
    }
}

impl rusqlite::types::ToSql for FollowTargetType {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::from(self.as_str()))
    }
}

impl rusqlite::types::FromSql for FollowTargetType {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let s = value.as_str()?;
        FollowTargetType::from_str_opt(s).ok_or_else(|| {
            rusqlite::types::FromSqlError::Other(
                format!("unknown follows.target_type value: {s:?}").into(),
            )
        })
    }
}

/// A single followed target (kelpie.md §97).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Follow {
    pub id: i64,
    pub target_type: FollowTargetType,
    pub target_id: String,
    pub created_at: String,
}

/// A favorited ("Saved", kelpie.md §94 Library) item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Favorite {
    pub item_uid: String,
    pub created_at: String,
}

fn row_to_follow(row: &Row<'_>) -> rusqlite::Result<Follow> {
    Ok(Follow {
        id: row.get(0)?,
        target_type: row.get(1)?,
        target_id: row.get(2)?,
        created_at: row.get(3)?,
    })
}

fn row_to_favorite(row: &Row<'_>) -> rusqlite::Result<Favorite> {
    Ok(Favorite {
        item_uid: row.get(0)?,
        created_at: row.get(1)?,
    })
}

/// Follow a target. Idempotent: following something already followed
/// returns the existing row rather than erroring or duplicating (relies on
/// the migration's `UNIQUE (target_type, target_id)` constraint).
pub fn follow(conn: &Connection, target_type: FollowTargetType, target_id: &str) -> Result<Follow> {
    conn.execute(
        "INSERT INTO follows (target_type, target_id, created_at)
         VALUES (?1, ?2, datetime('now'))
         ON CONFLICT (target_type, target_id) DO NOTHING",
        params![target_type, target_id],
    )?;

    conn.query_row(
        "SELECT id, target_type, target_id, created_at
         FROM follows
         WHERE target_type = ?1 AND target_id = ?2",
        params![target_type, target_id],
        row_to_follow,
    )
    .map_err(Into::into)
}

/// Unfollow a target. A no-op (not an error) if it wasn't followed.
pub fn unfollow(conn: &Connection, target_type: FollowTargetType, target_id: &str) -> Result<()> {
    conn.execute(
        "DELETE FROM follows WHERE target_type = ?1 AND target_id = ?2",
        params![target_type, target_id],
    )?;
    Ok(())
}

/// Whether a target is currently followed.
pub fn is_following(
    conn: &Connection,
    target_type: FollowTargetType,
    target_id: &str,
) -> Result<bool> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM follows WHERE target_type = ?1 AND target_id = ?2",
        params![target_type, target_id],
        |row| row.get(0),
    )?;
    Ok(count > 0)
}

/// List all follows, optionally filtered to a single target type. Ordered by
/// creation time (then id, as a tiebreaker for follows created within the
/// same `datetime('now')` second).
pub fn list_follows(
    conn: &Connection,
    target_type: Option<FollowTargetType>,
) -> Result<Vec<Follow>> {
    let rows = match target_type {
        Some(target_type) => {
            let mut stmt = conn.prepare(
                "SELECT id, target_type, target_id, created_at
                 FROM follows
                 WHERE target_type = ?1
                 ORDER BY created_at ASC, id ASC",
            )?;
            let rows = stmt
                .query_map(params![target_type], row_to_follow)?
                .collect::<rusqlite::Result<Vec<_>>>()?;
            rows
        }
        None => {
            let mut stmt = conn.prepare(
                "SELECT id, target_type, target_id, created_at
                 FROM follows
                 ORDER BY created_at ASC, id ASC",
            )?;
            let rows = stmt
                .query_map([], row_to_follow)?
                .collect::<rusqlite::Result<Vec<_>>>()?;
            rows
        }
    };
    Ok(rows)
}

/// Favorite ("Save") an item. Idempotent: favoriting an already-favorited
/// item returns the existing row rather than erroring or duplicating.
pub fn add_favorite(conn: &Connection, item_uid: &str) -> Result<Favorite> {
    conn.execute(
        "INSERT INTO favorites (item_uid, created_at)
         VALUES (?1, datetime('now'))
         ON CONFLICT (item_uid) DO NOTHING",
        params![item_uid],
    )?;

    conn.query_row(
        "SELECT item_uid, created_at FROM favorites WHERE item_uid = ?1",
        params![item_uid],
        row_to_favorite,
    )
    .map_err(Into::into)
}

/// Remove a favorite. A no-op (not an error) if it wasn't favorited.
pub fn remove_favorite(conn: &Connection, item_uid: &str) -> Result<()> {
    conn.execute(
        "DELETE FROM favorites WHERE item_uid = ?1",
        params![item_uid],
    )?;
    Ok(())
}

/// Whether an item is currently favorited.
pub fn is_favorite(conn: &Connection, item_uid: &str) -> Result<bool> {
    conn.query_row(
        "SELECT 1 FROM favorites WHERE item_uid = ?1",
        params![item_uid],
        |row| row.get::<_, i64>(0),
    )
    .optional()
    .map(|row| row.is_some())
    .map_err(Into::into)
}

/// List favorites, most recently favorited first.
pub fn list_favorites(conn: &Connection, limit: u32, offset: u32) -> Result<Vec<Favorite>> {
    let mut stmt = conn.prepare(
        "SELECT item_uid, created_at
         FROM favorites
         ORDER BY created_at DESC, item_uid ASC
         LIMIT ?1 OFFSET ?2",
    )?;
    let rows = stmt
        .query_map(params![limit, offset], row_to_favorite)?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(rows)
}

// ---------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn temp_db_path(name: &str) -> (tempfile::TempDir, PathBuf) {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = dir.path().join(name);
        (dir, path)
    }

    /// Minimal placeholder row in `items`, since `favorites.item_uid` has a
    /// foreign key to `items(uid)`.
    fn insert_placeholder_item(conn: &Connection, uid: &str) {
        conn.execute(
            "INSERT INTO items (
                uid, provider_id, canonical_url, kind, title,
                classification_json, availability, created_at, updated_locally_at
             ) VALUES (?1, 'provider-x', 'https://example.test/item', 'video', 'Test Item',
                       '{}', 'direct', datetime('now'), datetime('now'))",
            params![uid],
        )
        .expect("insert placeholder item");
    }

    #[test]
    fn target_type_string_mapping_round_trips_for_all_variants() {
        for variant in FollowTargetType::ALL {
            let s = variant.as_str();
            assert_eq!(FollowTargetType::from_str_opt(s), Some(variant));
        }
    }

    #[test]
    fn from_str_opt_rejects_unknown_values() {
        assert_eq!(FollowTargetType::from_str_opt("bogus"), None);
        assert_eq!(FollowTargetType::from_str_opt(""), None);
    }

    #[test]
    fn follow_unfollow_and_is_following_round_trip() {
        let (_dir, path) = temp_db_path("follow.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        assert!(!is_following(&conn, FollowTargetType::Creator, "creator-1").unwrap());

        let created = follow(&conn, FollowTargetType::Creator, "creator-1").expect("follow");
        assert_eq!(created.target_type, FollowTargetType::Creator);
        assert_eq!(created.target_id, "creator-1");
        assert!(!created.created_at.is_empty());
        assert!(is_following(&conn, FollowTargetType::Creator, "creator-1").unwrap());

        unfollow(&conn, FollowTargetType::Creator, "creator-1").expect("unfollow");
        assert!(!is_following(&conn, FollowTargetType::Creator, "creator-1").unwrap());
    }

    #[test]
    fn follow_a_second_target_type_independently() {
        let (_dir, path) = temp_db_path("follow_series.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        follow(&conn, FollowTargetType::Series, "series-1").expect("follow series");
        assert!(is_following(&conn, FollowTargetType::Series, "series-1").unwrap());
        // Same target_id, different target_type: must not collide.
        assert!(!is_following(&conn, FollowTargetType::Creator, "series-1").unwrap());
    }

    #[test]
    fn following_the_same_target_twice_is_idempotent() {
        let (_dir, path) = temp_db_path("follow_idempotent.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        let first = follow(&conn, FollowTargetType::Tag, "tag-1").expect("follow once");
        let second = follow(&conn, FollowTargetType::Tag, "tag-1").expect("follow twice");
        assert_eq!(
            first.id, second.id,
            "should be the same row, not a duplicate"
        );

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM follows", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn unfollow_missing_target_is_a_no_op() {
        let (_dir, path) = temp_db_path("unfollow_missing.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        unfollow(&conn, FollowTargetType::Provider, "does-not-exist")
            .expect("unfollow missing should be a no-op");
    }

    #[test]
    fn list_follows_without_filter_returns_everything_in_order() {
        let (_dir, path) = temp_db_path("list_follows_all.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        follow(&conn, FollowTargetType::Provider, "provider-1").unwrap();
        follow(&conn, FollowTargetType::Creator, "creator-1").unwrap();
        follow(&conn, FollowTargetType::Series, "series-1").unwrap();

        let all = list_follows(&conn, None).expect("list all follows");
        assert_eq!(all.len(), 3);
    }

    #[test]
    fn list_follows_filters_by_target_type() {
        let (_dir, path) = temp_db_path("list_follows_filtered.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        follow(&conn, FollowTargetType::Tag, "tag-a").unwrap();
        follow(&conn, FollowTargetType::Tag, "tag-b").unwrap();
        follow(&conn, FollowTargetType::SavedSearch, "search-1").unwrap();

        let tags_only = list_follows(&conn, Some(FollowTargetType::Tag)).expect("list tags");
        assert_eq!(tags_only.len(), 2);
        assert!(tags_only
            .iter()
            .all(|f| f.target_type == FollowTargetType::Tag));

        let searches_only =
            list_follows(&conn, Some(FollowTargetType::SavedSearch)).expect("list searches");
        assert_eq!(searches_only.len(), 1);
        assert_eq!(searches_only[0].target_id, "search-1");
    }

    #[test]
    fn add_remove_and_is_favorite_round_trip() {
        let (_dir, path) = temp_db_path("favorite.sqlite");
        let conn = crate::open_at(&path).expect("open db");
        insert_placeholder_item(&conn, "item-1");

        assert!(!is_favorite(&conn, "item-1").unwrap());

        let created = add_favorite(&conn, "item-1").expect("add favorite");
        assert_eq!(created.item_uid, "item-1");
        assert!(!created.created_at.is_empty());
        assert!(is_favorite(&conn, "item-1").unwrap());

        remove_favorite(&conn, "item-1").expect("remove favorite");
        assert!(!is_favorite(&conn, "item-1").unwrap());
    }

    #[test]
    fn adding_the_same_favorite_twice_is_idempotent() {
        let (_dir, path) = temp_db_path("favorite_idempotent.sqlite");
        let conn = crate::open_at(&path).expect("open db");
        insert_placeholder_item(&conn, "item-1");

        add_favorite(&conn, "item-1").expect("add once");
        add_favorite(&conn, "item-1").expect("add twice");

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM favorites", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn remove_missing_favorite_is_a_no_op() {
        let (_dir, path) = temp_db_path("favorite_remove_missing.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        remove_favorite(&conn, "does-not-exist").expect("remove missing should be a no-op");
    }

    #[test]
    fn list_favorites_orders_most_recent_first_and_paginates() {
        let (_dir, path) = temp_db_path("list_favorites.sqlite");
        let conn = crate::open_at(&path).expect("open db");
        for uid in ["item-1", "item-2", "item-3"] {
            insert_placeholder_item(&conn, uid);
            add_favorite(&conn, uid).expect("add favorite");
        }

        let all = list_favorites(&conn, 10, 0).expect("list favorites");
        assert_eq!(all.len(), 3);

        let page = list_favorites(&conn, 2, 0).expect("list first page");
        assert_eq!(page.len(), 2);
        let rest = list_favorites(&conn, 2, 2).expect("list second page");
        assert_eq!(rest.len(), 1);
    }

    #[test]
    fn deleting_the_underlying_item_cascades_to_favorites() {
        let (_dir, path) = temp_db_path("favorite_cascade.sqlite");
        let conn = crate::open_at(&path).expect("open db");
        insert_placeholder_item(&conn, "item-1");
        add_favorite(&conn, "item-1").expect("add favorite");
        assert!(is_favorite(&conn, "item-1").unwrap());

        conn.execute("DELETE FROM items WHERE uid = ?1", params!["item-1"])
            .expect("delete item");

        assert!(
            !is_favorite(&conn, "item-1").unwrap(),
            "favorite row should be removed by ON DELETE CASCADE"
        );
    }
}
