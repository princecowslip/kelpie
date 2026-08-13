//! `Tag` repository (kelpie.md §136 Phase 2, §89-90 Tag Normalization). Backs
//! migration `0006_tags.sql`.
//!
//! Provides CRUD on the normalized tag registry (`tags`) plus link management
//! against the per-item join table (`item_tags`), which records which provider
//! supplied a given tagging and the provider's own raw tag string (per
//! [`kelpie_core::domain::TagReference::source_value`]).
//!
//! Functions take `&rusqlite::Connection` per call, matching the connection
//! lifecycle established by [`crate::bootstrap`] / [`crate::open_at`].

use kelpie_core::domain::tag::Tag;
use rusqlite::{params, Connection, OptionalExtension};

use crate::{Error, Result};

/// Serialize a tag's aliases to the JSON TEXT representation stored in `tags.aliases`.
fn encode_aliases(aliases: &[String]) -> Result<String> {
    serde_json::to_string(aliases)
        .map_err(|e| Error::Sqlite(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))
}

/// Deserialize a tag's aliases back out of `tags.aliases`.
fn decode_aliases(raw: &str) -> Result<Vec<String>> {
    serde_json::from_str(raw).map_err(|e| {
        Error::Sqlite(rusqlite::Error::FromSqlConversionFailure(
            raw.len(),
            rusqlite::types::Type::Text,
            Box::new(e),
        ))
    })
}

fn row_to_tag(
    row: &rusqlite::Row<'_>,
) -> rusqlite::Result<(String, String, Option<String>, String)> {
    Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
}

/// Insert a new normalized tag into the registry.
pub fn create(conn: &Connection, tag: &Tag) -> Result<()> {
    conn.execute(
        "INSERT INTO tags (normalized_id, display_name, category, aliases) VALUES (?1, ?2, ?3, ?4)",
        params![
            tag.normalized_id,
            tag.display_name,
            tag.category,
            encode_aliases(&tag.aliases)?,
        ],
    )?;
    Ok(())
}

/// Look up a normalized tag by its `normalized_id`. Returns `None` if no such tag
/// is registered.
pub fn get_by_normalized_id(conn: &Connection, normalized_id: &str) -> Result<Option<Tag>> {
    let result = conn
        .query_row(
            "SELECT normalized_id, display_name, category, aliases FROM tags WHERE normalized_id = ?1",
            params![normalized_id],
            row_to_tag,
        )
        .optional()?;

    result
        .map(|(normalized_id, display_name, category, aliases)| {
            Ok(Tag {
                normalized_id,
                display_name,
                category,
                aliases: decode_aliases(&aliases)?,
            })
        })
        .transpose()
}

/// Overwrite an existing tag's mutable fields (`display_name`, `category`,
/// `aliases`). `normalized_id` is the stable identity and is not itself mutated.
pub fn update(conn: &Connection, tag: &Tag) -> Result<()> {
    conn.execute(
        "UPDATE tags SET display_name = ?2, category = ?3, aliases = ?4 WHERE normalized_id = ?1",
        params![
            tag.normalized_id,
            tag.display_name,
            tag.category,
            encode_aliases(&tag.aliases)?,
        ],
    )?;
    Ok(())
}

/// Delete a normalized tag from the registry, along with any `item_tags` links
/// referencing it. Connections in this codebase do not run with `PRAGMA
/// foreign_keys = ON` (see `0006_tags.sql`), so the cascade is done explicitly
/// here rather than relied upon at the database level.
pub fn delete(conn: &Connection, normalized_id: &str) -> Result<()> {
    conn.execute(
        "DELETE FROM item_tags WHERE normalized_id = ?1",
        params![normalized_id],
    )?;
    conn.execute(
        "DELETE FROM tags WHERE normalized_id = ?1",
        params![normalized_id],
    )?;
    Ok(())
}

/// Link an item to a normalized tag, recording which provider supplied this
/// specific tagging and that provider's raw tag string. Replaces any existing
/// link between the same item and normalized tag (see the UNIQUE constraint on
/// `(item_uid, normalized_id)` in `0006_tags.sql`), so re-linking updates the
/// recorded provider/source rather than erroring.
pub fn link_to_item(
    conn: &Connection,
    item_uid: &str,
    normalized_id: &str,
    provider_id: &str,
    source_value: &str,
) -> Result<()> {
    conn.execute(
        "INSERT INTO item_tags (item_uid, normalized_id, provider_id, source_value)
         VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT (item_uid, normalized_id)
         DO UPDATE SET provider_id = excluded.provider_id, source_value = excluded.source_value",
        params![item_uid, normalized_id, provider_id, source_value],
    )?;
    Ok(())
}

/// Remove the link between an item and a normalized tag, if any.
pub fn unlink_from_item(conn: &Connection, item_uid: &str, normalized_id: &str) -> Result<()> {
    conn.execute(
        "DELETE FROM item_tags WHERE item_uid = ?1 AND normalized_id = ?2",
        params![item_uid, normalized_id],
    )?;
    Ok(())
}

/// All normalized tags currently linked to a given item.
pub fn list_tags_for_item(conn: &Connection, item_uid: &str) -> Result<Vec<Tag>> {
    let mut stmt = conn.prepare(
        "SELECT t.normalized_id, t.display_name, t.category, t.aliases
         FROM tags t
         JOIN item_tags it ON it.normalized_id = t.normalized_id
         WHERE it.item_uid = ?1
         ORDER BY t.normalized_id",
    )?;

    let rows = stmt
        .query_map(params![item_uid], row_to_tag)?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    rows.into_iter()
        .map(|(normalized_id, display_name, category, aliases)| {
            Ok(Tag {
                normalized_id,
                display_name,
                category,
                aliases: decode_aliases(&aliases)?,
            })
        })
        .collect()
}

/// All item uids currently linked to a given normalized tag.
pub fn list_items_for_tag(conn: &Connection, normalized_id: &str) -> Result<Vec<String>> {
    let mut stmt =
        conn.prepare("SELECT item_uid FROM item_tags WHERE normalized_id = ?1 ORDER BY item_uid")?;
    let rows = stmt
        .query_map(params![normalized_id], |row| row.get::<_, String>(0))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_conn() -> (tempfile::TempDir, Connection) {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = dir.path().join("tag_repo.sqlite");
        let conn = crate::open_at(&path).expect("open_at should apply migrations");
        (dir, conn)
    }

    fn sample_tag() -> Tag {
        Tag {
            normalized_id: "tag:solo".to_string(),
            display_name: "Solo".to_string(),
            category: Some("composition".to_string()),
            aliases: vec!["solo_female".to_string(), "single_subject".to_string()],
        }
    }

    #[test]
    fn create_and_get_round_trip() {
        let (_dir, conn) = temp_conn();
        let tag = sample_tag();

        create(&conn, &tag).expect("create tag");

        let fetched = get_by_normalized_id(&conn, &tag.normalized_id)
            .expect("get_by_normalized_id should succeed")
            .expect("tag should exist");

        assert_eq!(fetched.normalized_id, tag.normalized_id);
        assert_eq!(fetched.display_name, tag.display_name);
        assert_eq!(fetched.category, tag.category);
        assert_eq!(fetched.aliases, tag.aliases);
    }

    #[test]
    fn get_missing_tag_returns_none() {
        let (_dir, conn) = temp_conn();
        let fetched =
            get_by_normalized_id(&conn, "tag:does-not-exist").expect("query should succeed");
        assert!(fetched.is_none());
    }

    #[test]
    fn full_lifecycle_create_link_read_unlink_update_delete() {
        let (_dir, conn) = temp_conn();
        let tag = sample_tag();
        let item_uid = "item:made-up-0001";

        // Insert a real Tag via the repository.
        create(&conn, &tag).expect("create tag");

        // Link it to a made-up item_uid.
        link_to_item(
            &conn,
            item_uid,
            &tag.normalized_id,
            "provider:example",
            "solo (raw)",
        )
        .expect("link_to_item should succeed");

        // Read back tags-for-item and items-for-tag, verify both.
        let tags_for_item = list_tags_for_item(&conn, item_uid).expect("list_tags_for_item");
        assert_eq!(tags_for_item.len(), 1);
        assert_eq!(tags_for_item[0].normalized_id, tag.normalized_id);
        assert_eq!(tags_for_item[0].aliases, tag.aliases);

        let items_for_tag =
            list_items_for_tag(&conn, &tag.normalized_id).expect("list_items_for_tag");
        assert_eq!(items_for_tag, vec![item_uid.to_string()]);

        // Unlink, verify removal.
        unlink_from_item(&conn, item_uid, &tag.normalized_id).expect("unlink_from_item");
        assert!(list_tags_for_item(&conn, item_uid)
            .expect("list_tags_for_item after unlink")
            .is_empty());
        assert!(list_items_for_tag(&conn, &tag.normalized_id)
            .expect("list_items_for_tag after unlink")
            .is_empty());

        // Update the tag.
        let mut updated = tag.clone();
        updated.display_name = "Solo (Updated)".to_string();
        updated.category = None;
        updated.aliases = vec!["solo_only".to_string()];
        update(&conn, &updated).expect("update tag");

        let refetched = get_by_normalized_id(&conn, &tag.normalized_id)
            .expect("get after update")
            .expect("tag should still exist");
        assert_eq!(refetched.display_name, "Solo (Updated)");
        assert_eq!(refetched.category, None);
        assert_eq!(refetched.aliases, vec!["solo_only".to_string()]);

        // Delete it, confirm it's gone.
        delete(&conn, &tag.normalized_id).expect("delete tag");
        assert!(get_by_normalized_id(&conn, &tag.normalized_id)
            .expect("get after delete")
            .is_none());
    }

    #[test]
    fn relinking_same_item_and_tag_updates_provider_and_source() {
        let (_dir, conn) = temp_conn();
        let tag = sample_tag();
        let item_uid = "item:made-up-0002";

        create(&conn, &tag).expect("create tag");
        link_to_item(&conn, item_uid, &tag.normalized_id, "provider:a", "solo")
            .expect("first link");
        link_to_item(
            &conn,
            item_uid,
            &tag.normalized_id,
            "provider:b",
            "Solo (raw)",
        )
        .expect("relink");

        let items_for_tag =
            list_items_for_tag(&conn, &tag.normalized_id).expect("list_items_for_tag");
        assert_eq!(
            items_for_tag,
            vec![item_uid.to_string()],
            "relinking must not create a duplicate item_tags row"
        );
    }

    #[test]
    fn deleting_tag_removes_its_item_tags_links() {
        let (_dir, conn) = temp_conn();
        let tag = sample_tag();
        let item_uid = "item:made-up-0003";

        create(&conn, &tag).expect("create tag");
        link_to_item(
            &conn,
            item_uid,
            &tag.normalized_id,
            "provider:example",
            "solo",
        )
        .expect("link");

        delete(&conn, &tag.normalized_id).expect("delete tag");

        assert!(list_tags_for_item(&conn, item_uid)
            .expect("list_tags_for_item after cascade delete")
            .is_empty());
    }
}
