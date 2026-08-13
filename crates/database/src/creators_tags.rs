//! Creators + Tags repository (kelpie.md §136 Core Data Layer): the standalone
//! Creator lookup and TagReference (§89), the `creators`, `tags`, and
//! `item_tags` tables from `migrations/0002_core_data_layer.sql`.
//!
//! ## Interpretive notes
//!
//! - **Creator uid**: §89/§97 name creators as followable entities but the
//!   spec has no formally defined remote-id shape for a bare creator lookup
//!   row (unlike items/series, which get `provider-id:object-type:remote-id`
//!   via `kelpie_core::identity::stable_id`, §17). Pragmatic rule used here
//!   (see [`Creator::new`]): when a `provider_id` is known, `uid` is
//!   `"{provider_id}:creator:{name}"` — stable and collision-resistant enough
//!   for a lookup table keyed on provider-scoped display name. When there is
//!   no `provider_id` (a manually-entered/local creator), `uid` is a fresh
//!   `uuid::Uuid::new_v4()`, since there is nothing provider-stable to derive
//!   from. Callers that already have a stable uid (e.g. computed upstream)
//!   may of course construct a `Creator` directly instead of via `new`.
//! - **Tag uid**: per `migrations/0002_core_data_layer.sql`'s own schema
//!   comment, `uid` is the tag row's stable key — `normalized_id` when
//!   present, else `"{provider_id}:{source_value}"` (see
//!   [`TagReference::new`]).
//! - **Aliases gap**: §89's prose lists "aliases" as one of the things tag
//!   normalization should preserve, but the `TagReference` interface it then
//!   defines has no `aliases` field, and the `tags` table (Stage A,
//!   `migrations/0002_core_data_layer.sql`) has no aliases column either —
//!   a known spec inconsistency already flagged in that migration's own
//!   comments. This module does not invent an aliases field/column; it only
//!   implements the interface as actually specified.

use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};

use crate::Result;

/// A standalone creator lookup row (supports Follow-by-creator, §97, and
/// future creator-profile display). Distinct from the per-item/per-series
/// inline creator *credits* stored as `creators_json` on `items`/`series` —
/// see this module's doc comment and the `creators` table's own schema
/// comment for why those stay separate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Creator {
    pub uid: String,
    pub provider_id: Option<String>,
    pub name: String,
    pub profile_url: Option<String>,
    pub avatar_url: Option<String>,
}

impl Creator {
    /// Construct a `Creator`, generating `uid` per the interpretive rule
    /// documented on this module: `"{provider_id}:creator:{name}"` when a
    /// provider is known, otherwise a fresh random uuid.
    pub fn new(
        provider_id: Option<String>,
        name: impl Into<String>,
        profile_url: Option<String>,
        avatar_url: Option<String>,
    ) -> Self {
        let name = name.into();
        let uid = match &provider_id {
            Some(provider_id) => format!("{provider_id}:creator:{name}"),
            None => uuid::Uuid::new_v4().to_string(),
        };
        Self {
            uid,
            provider_id,
            name,
            profile_url,
            avatar_url,
        }
    }
}

/// TagReference (kelpie.md §89 Tag Normalization): normalized identity,
/// provider-local identity, display value, and category. See this module's
/// doc comment for the "aliases" gap between §89's prose and its actual
/// interface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TagReference {
    pub uid: String,
    pub normalized_id: Option<String>,
    pub provider_id: String,
    pub source_value: String,
    pub display_name: String,
    pub category: Option<String>,
}

impl TagReference {
    /// Construct a `TagReference`, generating `uid` per the `tags` table's
    /// own schema comment: `normalized_id` when present, else
    /// `"{provider_id}:{source_value}"`.
    pub fn new(
        normalized_id: Option<String>,
        provider_id: impl Into<String>,
        source_value: impl Into<String>,
        display_name: impl Into<String>,
        category: Option<String>,
    ) -> Self {
        let provider_id = provider_id.into();
        let source_value = source_value.into();
        let uid = normalized_id
            .clone()
            .unwrap_or_else(|| format!("{provider_id}:{source_value}"));
        Self {
            uid,
            normalized_id,
            provider_id,
            source_value,
            display_name: display_name.into(),
            category,
        }
    }
}

fn creator_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Creator> {
    Ok(Creator {
        uid: row.get(0)?,
        provider_id: row.get(1)?,
        name: row.get(2)?,
        profile_url: row.get(3)?,
        avatar_url: row.get(4)?,
    })
}

fn tag_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<TagReference> {
    Ok(TagReference {
        uid: row.get(0)?,
        normalized_id: row.get(1)?,
        provider_id: row.get(2)?,
        source_value: row.get(3)?,
        display_name: row.get(4)?,
        category: row.get(5)?,
    })
}

/// Insert a creator, or update it in place if `creator.uid` already exists.
pub fn upsert_creator(conn: &Connection, creator: &Creator) -> Result<()> {
    conn.execute(
        "INSERT INTO creators (uid, provider_id, name, profile_url, avatar_url)
         VALUES (?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(uid) DO UPDATE SET
             provider_id = excluded.provider_id,
             name = excluded.name,
             profile_url = excluded.profile_url,
             avatar_url = excluded.avatar_url",
        params![
            creator.uid,
            creator.provider_id,
            creator.name,
            creator.profile_url,
            creator.avatar_url,
        ],
    )?;
    Ok(())
}

/// Fetch a creator by uid. Returns `Ok(None)` if no such creator exists.
pub fn get_creator(conn: &Connection, uid: &str) -> Result<Option<Creator>> {
    let creator = conn
        .query_row(
            "SELECT uid, provider_id, name, profile_url, avatar_url
             FROM creators WHERE uid = ?1",
            params![uid],
            creator_from_row,
        )
        .optional()?;
    Ok(creator)
}

/// List creators ordered by name, paginated.
pub fn list_creators(conn: &Connection, limit: u32, offset: u32) -> Result<Vec<Creator>> {
    let mut stmt = conn.prepare(
        "SELECT uid, provider_id, name, profile_url, avatar_url
         FROM creators ORDER BY name ASC LIMIT ?1 OFFSET ?2",
    )?;
    let creators = stmt
        .query_map(params![limit, offset], creator_from_row)?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(creators)
}

/// Delete a creator by uid. A no-op (not an error) if it doesn't exist.
pub fn delete_creator(conn: &Connection, uid: &str) -> Result<()> {
    conn.execute("DELETE FROM creators WHERE uid = ?1", params![uid])?;
    Ok(())
}

/// Insert a tag, or update it in place if `tag.uid` already exists.
pub fn upsert_tag(conn: &Connection, tag: &TagReference) -> Result<()> {
    conn.execute(
        "INSERT INTO tags (uid, normalized_id, provider_id, source_value, display_name, category)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)
         ON CONFLICT(uid) DO UPDATE SET
             normalized_id = excluded.normalized_id,
             provider_id = excluded.provider_id,
             source_value = excluded.source_value,
             display_name = excluded.display_name,
             category = excluded.category",
        params![
            tag.uid,
            tag.normalized_id,
            tag.provider_id,
            tag.source_value,
            tag.display_name,
            tag.category,
        ],
    )?;
    Ok(())
}

/// Fetch a tag by uid. Returns `Ok(None)` if no such tag exists.
pub fn get_tag(conn: &Connection, uid: &str) -> Result<Option<TagReference>> {
    let tag = conn
        .query_row(
            "SELECT uid, normalized_id, provider_id, source_value, display_name, category
             FROM tags WHERE uid = ?1",
            params![uid],
            tag_from_row,
        )
        .optional()?;
    Ok(tag)
}

/// List tags ordered by display name, paginated.
pub fn list_tags(conn: &Connection, limit: u32, offset: u32) -> Result<Vec<TagReference>> {
    let mut stmt = conn.prepare(
        "SELECT uid, normalized_id, provider_id, source_value, display_name, category
         FROM tags ORDER BY display_name ASC LIMIT ?1 OFFSET ?2",
    )?;
    let tags = stmt
        .query_map(params![limit, offset], tag_from_row)?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(tags)
}

/// Look up a tag by its normalized identity, using `idx_tags_normalized_id`.
/// Returns `Ok(None)` if no tag has that `normalized_id`.
pub fn find_tag_by_normalized_id(
    conn: &Connection,
    normalized_id: &str,
) -> Result<Option<TagReference>> {
    let tag = conn
        .query_row(
            "SELECT uid, normalized_id, provider_id, source_value, display_name, category
             FROM tags WHERE normalized_id = ?1",
            params![normalized_id],
            tag_from_row,
        )
        .optional()?;
    Ok(tag)
}

/// Associate a tag with an item. Idempotent — tagging the same item+tag pair
/// twice is a no-op, not an error or a duplicate row.
pub fn tag_item(conn: &Connection, item_uid: &str, tag_uid: &str) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO item_tags (item_uid, tag_uid) VALUES (?1, ?2)",
        params![item_uid, tag_uid],
    )?;
    Ok(())
}

/// Remove a tag association from an item. A no-op (not an error) if the
/// association doesn't exist.
pub fn untag_item(conn: &Connection, item_uid: &str, tag_uid: &str) -> Result<()> {
    conn.execute(
        "DELETE FROM item_tags WHERE item_uid = ?1 AND tag_uid = ?2",
        params![item_uid, tag_uid],
    )?;
    Ok(())
}

/// List every tag associated with an item, ordered by display name.
pub fn list_tags_for_item(conn: &Connection, item_uid: &str) -> Result<Vec<TagReference>> {
    let mut stmt = conn.prepare(
        "SELECT t.uid, t.normalized_id, t.provider_id, t.source_value, t.display_name, t.category
         FROM item_tags it
         JOIN tags t ON t.uid = it.tag_uid
         WHERE it.item_uid = ?1
         ORDER BY t.display_name ASC",
    )?;
    let tags = stmt
        .query_map(params![item_uid], tag_from_row)?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(tags)
}

/// List the uids of every item associated with a tag. Returns bare item uids
/// (not full `Item` rows) — this module doesn't own the `items` table's row
/// shape.
pub fn list_items_for_tag(conn: &Connection, tag_uid: &str) -> Result<Vec<String>> {
    let mut stmt =
        conn.prepare("SELECT item_uid FROM item_tags WHERE tag_uid = ?1 ORDER BY item_uid ASC")?;
    let item_uids = stmt
        .query_map(params![tag_uid], |row| row.get::<_, String>(0))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(item_uids)
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

    /// Minimal, placeholder-valued row satisfying `items`' NOT NULL columns
    /// and foreign key target — this module doesn't own the `items` table's
    /// full row shape (that's the items-repository unit), so raw SQL is used
    /// here purely to give `item_tags` something to reference.
    fn insert_item(conn: &Connection, uid: &str) {
        conn.execute(
            "INSERT INTO items (
                uid, provider_id, canonical_url, kind, title,
                classification_json, availability, created_at, updated_locally_at
             ) VALUES (?1, 'provider-x', 'https://example.test/item', 'video', 'Untitled',
                '{}', 'available', '2026-01-01T00:00:00Z', '2026-01-01T00:00:00Z')",
            params![uid],
        )
        .expect("insert placeholder item");
    }

    fn sample_creator(uid: &str) -> Creator {
        Creator {
            uid: uid.to_string(),
            provider_id: Some("provider-x".to_string()),
            name: "Creator One".to_string(),
            profile_url: Some("https://example.test/creators/one".to_string()),
            avatar_url: None,
        }
    }

    fn sample_tag(uid: &str) -> TagReference {
        TagReference {
            uid: uid.to_string(),
            normalized_id: Some("general".to_string()),
            provider_id: "provider-x".to_string(),
            source_value: "General".to_string(),
            display_name: "General".to_string(),
            category: Some("technical".to_string()),
        }
    }

    #[test]
    fn creator_new_uses_provider_scoped_uid_when_provider_known() {
        let creator = Creator::new(Some("provider-x".to_string()), "Creator One", None, None);
        assert_eq!(creator.uid, "provider-x:creator:Creator One");
    }

    #[test]
    fn creator_new_uses_random_uuid_when_provider_unknown() {
        let a = Creator::new(None, "Local Creator", None, None);
        let b = Creator::new(None, "Local Creator", None, None);
        assert_ne!(a.uid, b.uid, "provider-less creators get independent uids");
        assert!(uuid::Uuid::parse_str(&a.uid).is_ok());
    }

    #[test]
    fn tag_reference_new_prefers_normalized_id_for_uid() {
        let tag = TagReference::new(
            Some("general".to_string()),
            "provider-x",
            "General",
            "General",
            None,
        );
        assert_eq!(tag.uid, "general");
    }

    #[test]
    fn tag_reference_new_falls_back_to_provider_and_source_value() {
        let tag = TagReference::new(None, "provider-x", "General", "General", None);
        assert_eq!(tag.uid, "provider-x:General");
    }

    #[test]
    fn creator_upsert_and_get_round_trip() {
        let (_dir, path) = temp_db_path("creator_round_trip.sqlite");
        let conn = crate::open_at(&path).expect("bootstrap");

        let creator = sample_creator("provider-x:creator:Creator One");
        upsert_creator(&conn, &creator).expect("upsert creator");

        let fetched = get_creator(&conn, &creator.uid)
            .expect("get creator")
            .expect("creator should exist");
        assert_eq!(fetched, creator);
    }

    #[test]
    fn creator_upsert_twice_updates_not_duplicates() {
        let (_dir, path) = temp_db_path("creator_upsert_twice.sqlite");
        let conn = crate::open_at(&path).expect("bootstrap");

        let mut creator = sample_creator("creator-1");
        upsert_creator(&conn, &creator).expect("first upsert");

        creator.name = "Creator One Renamed".to_string();
        creator.avatar_url = Some("https://example.test/avatar.png".to_string());
        upsert_creator(&conn, &creator).expect("second upsert");

        let all = list_creators(&conn, 100, 0).expect("list creators");
        assert_eq!(all.len(), 1, "upsert must not duplicate rows");
        assert_eq!(all[0].name, "Creator One Renamed");
        assert_eq!(
            all[0].avatar_url,
            Some("https://example.test/avatar.png".to_string())
        );
    }

    #[test]
    fn get_creator_missing_returns_none() {
        let (_dir, path) = temp_db_path("creator_missing.sqlite");
        let conn = crate::open_at(&path).expect("bootstrap");

        assert_eq!(get_creator(&conn, "does-not-exist").expect("get"), None);
    }

    #[test]
    fn list_creators_paginates_and_orders_by_name() {
        let (_dir, path) = temp_db_path("creator_list.sqlite");
        let conn = crate::open_at(&path).expect("bootstrap");

        for (uid, name) in [("c1", "Zeta"), ("c2", "Alpha"), ("c3", "Mid")] {
            let creator = Creator {
                uid: uid.to_string(),
                provider_id: None,
                name: name.to_string(),
                profile_url: None,
                avatar_url: None,
            };
            upsert_creator(&conn, &creator).expect("upsert creator");
        }

        let page1 = list_creators(&conn, 2, 0).expect("list page 1");
        assert_eq!(
            page1.iter().map(|c| c.name.as_str()).collect::<Vec<_>>(),
            vec!["Alpha", "Mid"]
        );

        let page2 = list_creators(&conn, 2, 2).expect("list page 2");
        assert_eq!(
            page2.iter().map(|c| c.name.as_str()).collect::<Vec<_>>(),
            vec!["Zeta"]
        );
    }

    #[test]
    fn delete_creator_removes_row_and_is_a_noop_when_missing() {
        let (_dir, path) = temp_db_path("creator_delete.sqlite");
        let conn = crate::open_at(&path).expect("bootstrap");

        let creator = sample_creator("creator-1");
        upsert_creator(&conn, &creator).expect("upsert creator");
        delete_creator(&conn, &creator.uid).expect("delete creator");
        assert_eq!(get_creator(&conn, &creator.uid).expect("get"), None);

        // Deleting again (already gone) must not error.
        delete_creator(&conn, &creator.uid).expect("delete missing creator");
    }

    #[test]
    fn tag_upsert_and_get_round_trip() {
        let (_dir, path) = temp_db_path("tag_round_trip.sqlite");
        let conn = crate::open_at(&path).expect("bootstrap");

        let tag = sample_tag("general");
        upsert_tag(&conn, &tag).expect("upsert tag");

        let fetched = get_tag(&conn, &tag.uid)
            .expect("get tag")
            .expect("tag should exist");
        assert_eq!(fetched, tag);
    }

    #[test]
    fn tag_upsert_twice_updates_not_duplicates() {
        let (_dir, path) = temp_db_path("tag_upsert_twice.sqlite");
        let conn = crate::open_at(&path).expect("bootstrap");

        let mut tag = sample_tag("general");
        upsert_tag(&conn, &tag).expect("first upsert");

        tag.display_name = "General (renamed)".to_string();
        tag.category = Some("technical-renamed".to_string());
        upsert_tag(&conn, &tag).expect("second upsert");

        let all = list_tags(&conn, 100, 0).expect("list tags");
        assert_eq!(all.len(), 1, "upsert must not duplicate rows");
        assert_eq!(all[0].display_name, "General (renamed)");
        assert_eq!(all[0].category, Some("technical-renamed".to_string()));
    }

    #[test]
    fn get_tag_missing_returns_none() {
        let (_dir, path) = temp_db_path("tag_missing.sqlite");
        let conn = crate::open_at(&path).expect("bootstrap");

        assert_eq!(get_tag(&conn, "does-not-exist").expect("get"), None);
    }

    #[test]
    fn list_tags_paginates_and_orders_by_display_name() {
        let (_dir, path) = temp_db_path("tag_list.sqlite");
        let conn = crate::open_at(&path).expect("bootstrap");

        for (uid, display_name) in [("t1", "Zeta"), ("t2", "Alpha"), ("t3", "Mid")] {
            let tag = TagReference {
                uid: uid.to_string(),
                normalized_id: None,
                provider_id: "provider-x".to_string(),
                source_value: display_name.to_string(),
                display_name: display_name.to_string(),
                category: None,
            };
            upsert_tag(&conn, &tag).expect("upsert tag");
        }

        let page1 = list_tags(&conn, 2, 0).expect("list page 1");
        assert_eq!(
            page1
                .iter()
                .map(|t| t.display_name.as_str())
                .collect::<Vec<_>>(),
            vec!["Alpha", "Mid"]
        );

        let page2 = list_tags(&conn, 2, 2).expect("list page 2");
        assert_eq!(
            page2
                .iter()
                .map(|t| t.display_name.as_str())
                .collect::<Vec<_>>(),
            vec!["Zeta"]
        );
    }

    #[test]
    fn find_tag_by_normalized_id_hit() {
        let (_dir, path) = temp_db_path("tag_find_hit.sqlite");
        let conn = crate::open_at(&path).expect("bootstrap");

        let tag = sample_tag("general");
        upsert_tag(&conn, &tag).expect("upsert tag");

        let found = find_tag_by_normalized_id(&conn, "general")
            .expect("find tag")
            .expect("tag should be found");
        assert_eq!(found, tag);
    }

    #[test]
    fn find_tag_by_normalized_id_miss() {
        let (_dir, path) = temp_db_path("tag_find_miss.sqlite");
        let conn = crate::open_at(&path).expect("bootstrap");

        let tag = sample_tag("general");
        upsert_tag(&conn, &tag).expect("upsert tag");

        assert_eq!(
            find_tag_by_normalized_id(&conn, "does-not-exist").expect("find tag"),
            None
        );
    }

    #[test]
    fn find_tag_by_normalized_id_handles_tags_without_one() {
        let (_dir, path) = temp_db_path("tag_find_no_normalized.sqlite");
        let conn = crate::open_at(&path).expect("bootstrap");

        let tag = TagReference {
            uid: "provider-x:Unnormalized".to_string(),
            normalized_id: None,
            provider_id: "provider-x".to_string(),
            source_value: "Unnormalized".to_string(),
            display_name: "Unnormalized".to_string(),
            category: None,
        };
        upsert_tag(&conn, &tag).expect("upsert tag");

        // A tag with no normalized_id must never be findable by an empty or
        // arbitrary normalized_id lookup.
        assert_eq!(
            find_tag_by_normalized_id(&conn, "").expect("find tag"),
            None
        );
    }

    #[test]
    fn tag_item_and_list_tags_for_item_round_trip() {
        let (_dir, path) = temp_db_path("tag_item_round_trip.sqlite");
        let conn = crate::open_at(&path).expect("bootstrap");

        insert_item(&conn, "item-1");
        let tag = sample_tag("general");
        upsert_tag(&conn, &tag).expect("upsert tag");

        tag_item(&conn, "item-1", &tag.uid).expect("tag item");

        let tags = list_tags_for_item(&conn, "item-1").expect("list tags for item");
        assert_eq!(tags, vec![tag]);
    }

    #[test]
    fn list_items_for_tag_round_trip() {
        let (_dir, path) = temp_db_path("list_items_for_tag.sqlite");
        let conn = crate::open_at(&path).expect("bootstrap");

        insert_item(&conn, "item-1");
        insert_item(&conn, "item-2");
        let tag = sample_tag("general");
        upsert_tag(&conn, &tag).expect("upsert tag");

        tag_item(&conn, "item-1", &tag.uid).expect("tag item 1");
        tag_item(&conn, "item-2", &tag.uid).expect("tag item 2");

        let mut item_uids = list_items_for_tag(&conn, &tag.uid).expect("list items for tag");
        item_uids.sort();
        assert_eq!(item_uids, vec!["item-1".to_string(), "item-2".to_string()]);
    }

    #[test]
    fn tag_item_is_idempotent() {
        let (_dir, path) = temp_db_path("tag_item_idempotent.sqlite");
        let conn = crate::open_at(&path).expect("bootstrap");

        insert_item(&conn, "item-1");
        let tag = sample_tag("general");
        upsert_tag(&conn, &tag).expect("upsert tag");

        tag_item(&conn, "item-1", &tag.uid).expect("tag item first time");
        tag_item(&conn, "item-1", &tag.uid).expect("tag item second time");

        let tags = list_tags_for_item(&conn, "item-1").expect("list tags for item");
        assert_eq!(
            tags.len(),
            1,
            "tagging twice must not duplicate the association"
        );
    }

    #[test]
    fn untag_item_removes_association_and_is_a_noop_when_missing() {
        let (_dir, path) = temp_db_path("untag_item.sqlite");
        let conn = crate::open_at(&path).expect("bootstrap");

        insert_item(&conn, "item-1");
        let tag = sample_tag("general");
        upsert_tag(&conn, &tag).expect("upsert tag");
        tag_item(&conn, "item-1", &tag.uid).expect("tag item");

        untag_item(&conn, "item-1", &tag.uid).expect("untag item");
        assert!(list_tags_for_item(&conn, "item-1")
            .expect("list tags for item")
            .is_empty());

        // Untagging again (already gone) must not error.
        untag_item(&conn, "item-1", &tag.uid).expect("untag missing association");
    }

    #[test]
    fn deleting_tag_cascades_to_item_tags() {
        let (_dir, path) = temp_db_path("tag_delete_cascade.sqlite");
        let conn = crate::open_at(&path).expect("bootstrap");

        // Foreign keys are off by default in SQLite unless enabled per
        // connection; enable them explicitly so ON DELETE CASCADE fires.
        conn.execute("PRAGMA foreign_keys = ON", []).unwrap();

        insert_item(&conn, "item-1");
        let tag = sample_tag("general");
        upsert_tag(&conn, &tag).expect("upsert tag");
        tag_item(&conn, "item-1", &tag.uid).expect("tag item");

        conn.execute("DELETE FROM tags WHERE uid = ?1", params![tag.uid])
            .expect("delete tag");

        assert!(
            list_tags_for_item(&conn, "item-1")
                .expect("list tags for item")
                .is_empty(),
            "deleting a tag must cascade-delete its item_tags rows"
        );
    }

    #[test]
    fn deleting_item_cascades_to_item_tags() {
        let (_dir, path) = temp_db_path("item_delete_cascade.sqlite");
        let conn = crate::open_at(&path).expect("bootstrap");

        conn.execute("PRAGMA foreign_keys = ON", []).unwrap();

        insert_item(&conn, "item-1");
        let tag = sample_tag("general");
        upsert_tag(&conn, &tag).expect("upsert tag");
        tag_item(&conn, "item-1", &tag.uid).expect("tag item");

        conn.execute("DELETE FROM items WHERE uid = ?1", params!["item-1"])
            .expect("delete item");

        assert!(
            list_items_for_tag(&conn, &tag.uid)
                .expect("list items for tag")
                .is_empty(),
            "deleting an item must cascade-delete its item_tags rows"
        );
    }
}
