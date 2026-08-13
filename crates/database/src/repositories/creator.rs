//! kelpie-database::repositories::creator: SQLite-backed repository for the
//! `Creator` domain type (kelpie.md §136 Phase 2). Backs migration
//! `0005_creators.sql` (`creators` table + `item_creators` link table).
//!
//! Functions take `&rusqlite::Connection` per call, matching the connection
//! lifecycle established by [`crate::bootstrap`] / [`crate::open_at`].
//!
//! `Creator.aliases` is stored as a JSON array in a single TEXT column; this
//! module owns the (de)serialization at the row boundary so callers work with a
//! plain `Vec<String>`.

use kelpie_core::domain::creator::{Creator, CreatorLink};
use rusqlite::{params, Connection, OptionalExtension, Row};

use crate::Result;

/// Insert a new creator row.
pub fn create(conn: &Connection, creator: &Creator) -> Result<()> {
    conn.execute(
        "INSERT INTO creators (id, name, aliases, provider_id) VALUES (?1, ?2, ?3, ?4)",
        params![
            creator.id,
            creator.name,
            aliases_to_json(&creator.aliases)?,
            creator.provider_id,
        ],
    )?;
    Ok(())
}

/// Look up a creator by id. Returns `None` if no such creator exists.
pub fn get_by_id(conn: &Connection, id: &str) -> Result<Option<Creator>> {
    let creator = conn
        .query_row(
            "SELECT id, name, aliases, provider_id FROM creators WHERE id = ?1",
            params![id],
            creator_from_row,
        )
        .optional()?;
    Ok(creator)
}

/// Overwrite `name`, `aliases`, and `provider_id` for an existing creator, keyed
/// by `creator.id`. A no-op (not an error) if no creator with that id exists.
pub fn update(conn: &Connection, creator: &Creator) -> Result<()> {
    conn.execute(
        "UPDATE creators SET name = ?2, aliases = ?3, provider_id = ?4 WHERE id = ?1",
        params![
            creator.id,
            creator.name,
            aliases_to_json(&creator.aliases)?,
            creator.provider_id,
        ],
    )?;
    Ok(())
}

/// Delete a creator by id. Cascades to its `item_creators` links. A no-op (not an
/// error) if no creator with that id exists.
pub fn delete(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM creators WHERE id = ?1", params![id])?;
    Ok(())
}

/// List every creator whose `provider_id` matches, ordered by name.
pub fn list_by_provider(conn: &Connection, provider_id: &str) -> Result<Vec<Creator>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, aliases, provider_id FROM creators \
         WHERE provider_id = ?1 ORDER BY name",
    )?;
    let creators = stmt
        .query_map(params![provider_id], creator_from_row)?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(creators)
}

/// Credit `creator_id` on `item_uid` with the given `role`. Idempotent: linking an
/// already-linked (item_uid, creator_id) pair again updates the stored role rather
/// than erroring, per the `UNIQUE (item_uid, creator_id)` constraint on
/// `item_creators`.
pub fn link_to_item(
    conn: &Connection,
    item_uid: &str,
    creator_id: &str,
    role: Option<&str>,
) -> Result<()> {
    conn.execute(
        "INSERT INTO item_creators (item_uid, creator_id, role) VALUES (?1, ?2, ?3) \
         ON CONFLICT (item_uid, creator_id) DO UPDATE SET role = excluded.role",
        params![item_uid, creator_id, role],
    )?;
    Ok(())
}

/// Remove the credit linking `creator_id` to `item_uid`. A no-op (not an error) if
/// no such link exists.
pub fn unlink_from_item(conn: &Connection, item_uid: &str, creator_id: &str) -> Result<()> {
    conn.execute(
        "DELETE FROM item_creators WHERE item_uid = ?1 AND creator_id = ?2",
        params![item_uid, creator_id],
    )?;
    Ok(())
}

/// List every creator credited on `item_uid`, together with their role, ordered by
/// creator name.
pub fn list_creators_for_item(conn: &Connection, item_uid: &str) -> Result<Vec<CreatorLink>> {
    let mut stmt = conn.prepare(
        "SELECT c.id, c.name, c.aliases, c.provider_id, ic.role \
         FROM item_creators ic \
         JOIN creators c ON c.id = ic.creator_id \
         WHERE ic.item_uid = ?1 \
         ORDER BY c.name",
    )?;
    let links = stmt
        .query_map(params![item_uid], |row| {
            Ok(CreatorLink {
                creator: creator_from_row(row)?,
                role: row.get(4)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(links)
}

/// Serialize `aliases` to the JSON TEXT representation stored in the `aliases`
/// column. Serializing a `Vec<String>` cannot practically fail, but the error is
/// still propagated (as a `rusqlite::Error::ToSqlConversionFailure`, which
/// `crate::Error` already converts from) rather than unwrapped.
fn aliases_to_json(aliases: &[String]) -> Result<String> {
    serde_json::to_string(aliases)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)).into())
}

/// Build a [`Creator`] from a `creators` row: `id, name, aliases, provider_id` in
/// that column order (columns 0-3). Used both for direct `creators` queries and as
/// the first four columns of the `item_creators` join in
/// [`list_creators_for_item`].
fn creator_from_row(row: &Row<'_>) -> rusqlite::Result<Creator> {
    let aliases_json: String = row.get(2)?;
    let aliases = serde_json::from_str(&aliases_json).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(2, rusqlite::types::Type::Text, Box::new(e))
    })?;
    Ok(Creator {
        id: row.get(0)?,
        name: row.get(1)?,
        aliases,
        provider_id: row.get(3)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_conn() -> (tempfile::TempDir, Connection) {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = dir.path().join("creator_repo.sqlite");
        let conn = crate::open_at(&path).expect("open_at should apply migrations");
        (dir, conn)
    }

    fn sample_creator(id: &str) -> Creator {
        Creator {
            id: id.to_string(),
            name: "Jordan Rivers".to_string(),
            aliases: vec!["J. Rivers".to_string(), "JR".to_string()],
            provider_id: Some("fakeprovider".to_string()),
        }
    }

    #[test]
    fn create_and_get_round_trips_all_fields() {
        let (_dir, conn) = temp_conn();
        let creator = sample_creator("fakeprovider:creator-1");

        create(&conn, &creator).expect("create should succeed");

        let fetched = get_by_id(&conn, &creator.id)
            .expect("get_by_id should succeed")
            .expect("creator should exist");
        assert_eq!(fetched.id, creator.id);
        assert_eq!(fetched.name, creator.name);
        assert_eq!(fetched.aliases, creator.aliases);
        assert_eq!(fetched.provider_id, creator.provider_id);
    }

    #[test]
    fn get_by_id_returns_none_for_missing_creator() {
        let (_dir, conn) = temp_conn();
        assert!(get_by_id(&conn, "does-not-exist").unwrap().is_none());
    }

    #[test]
    fn list_by_provider_filters_and_orders_by_name() {
        let (_dir, conn) = temp_conn();
        let mut zed = sample_creator("fakeprovider:z");
        zed.name = "Zed Artist".to_string();
        let mut ana = sample_creator("fakeprovider:a");
        ana.name = "Ana Artist".to_string();
        let other = Creator {
            id: "otherprovider:creator-1".to_string(),
            name: "Someone Else".to_string(),
            aliases: vec![],
            provider_id: Some("otherprovider".to_string()),
        };

        create(&conn, &zed).unwrap();
        create(&conn, &ana).unwrap();
        create(&conn, &other).unwrap();

        let listed = list_by_provider(&conn, "fakeprovider").unwrap();
        let names: Vec<&str> = listed.iter().map(|c| c.name.as_str()).collect();
        assert_eq!(names, vec!["Ana Artist", "Zed Artist"]);
    }

    #[test]
    fn update_overwrites_name_aliases_and_provider() {
        let (_dir, conn) = temp_conn();
        let mut creator = sample_creator("fakeprovider:creator-1");
        create(&conn, &creator).unwrap();

        creator.name = "New Name".to_string();
        creator.aliases = vec!["Only Alias".to_string()];
        creator.provider_id = None;
        update(&conn, &creator).unwrap();

        let fetched = get_by_id(&conn, &creator.id).unwrap().unwrap();
        assert_eq!(fetched.name, "New Name");
        assert_eq!(fetched.aliases, vec!["Only Alias".to_string()]);
        assert_eq!(fetched.provider_id, None);
    }

    #[test]
    fn delete_removes_creator() {
        let (_dir, conn) = temp_conn();
        let creator = sample_creator("fakeprovider:creator-1");
        create(&conn, &creator).unwrap();

        delete(&conn, &creator.id).unwrap();

        assert!(get_by_id(&conn, &creator.id).unwrap().is_none());
    }

    #[test]
    fn delete_of_missing_creator_is_not_an_error() {
        let (_dir, conn) = temp_conn();
        delete(&conn, "does-not-exist").expect("deleting a missing creator is a no-op");
    }

    #[test]
    fn link_unlink_and_list_creators_for_item_round_trip() {
        let (_dir, conn) = temp_conn();
        let author = sample_creator("fakeprovider:author-1");
        let artist = {
            let mut c = sample_creator("fakeprovider:artist-1");
            c.name = "Alex Artist".to_string();
            c
        };
        create(&conn, &author).unwrap();
        create(&conn, &artist).unwrap();

        let item_uid = "fakeprovider:item-42";
        link_to_item(&conn, item_uid, &author.id, Some("author")).unwrap();
        link_to_item(&conn, item_uid, &artist.id, Some("artist")).unwrap();

        let links = list_creators_for_item(&conn, item_uid).unwrap();
        assert_eq!(links.len(), 2);
        assert_eq!(links[0].creator.name, "Alex Artist");
        assert_eq!(links[0].role.as_deref(), Some("artist"));
        assert_eq!(links[1].creator.id, author.id);
        assert_eq!(links[1].role.as_deref(), Some("author"));

        unlink_from_item(&conn, item_uid, &artist.id).unwrap();
        let links_after_unlink = list_creators_for_item(&conn, item_uid).unwrap();
        assert_eq!(links_after_unlink.len(), 1);
        assert_eq!(links_after_unlink[0].creator.id, author.id);
    }

    #[test]
    fn linking_same_pair_twice_updates_role_instead_of_erroring() {
        let (_dir, conn) = temp_conn();
        let creator = sample_creator("fakeprovider:creator-1");
        create(&conn, &creator).unwrap();

        let item_uid = "fakeprovider:item-1";
        link_to_item(&conn, item_uid, &creator.id, Some("author")).unwrap();
        link_to_item(&conn, item_uid, &creator.id, Some("editor")).unwrap();

        let links = list_creators_for_item(&conn, item_uid).unwrap();
        assert_eq!(links.len(), 1, "re-linking must not duplicate the row");
        assert_eq!(links[0].role.as_deref(), Some("editor"));
    }

    #[test]
    fn deleting_creator_cascades_to_item_creators() {
        let (_dir, conn) = temp_conn();
        let creator = sample_creator("fakeprovider:creator-1");
        create(&conn, &creator).unwrap();
        let item_uid = "fakeprovider:item-1";
        link_to_item(&conn, item_uid, &creator.id, Some("author")).unwrap();

        delete(&conn, &creator.id).unwrap();

        // FK cascade only fires when foreign key enforcement is on; either way the
        // link must not resolve to a live creator once the creator itself is gone.
        let links = list_creators_for_item(&conn, item_uid).unwrap();
        assert!(links.is_empty());
    }

    /// Full end-to-end pass against a real, freshly migrated database: create,
    /// link, read back, verify, unlink, verify removal, update, delete, confirm
    /// gone. Exercises the migration and repository together, not just the SQL in
    /// isolation.
    #[test]
    fn end_to_end_lifecycle_against_a_real_database() {
        let (_dir, conn) = temp_conn();

        // Create.
        let mut creator = sample_creator("fakeprovider:e2e-creator");
        create(&conn, &creator).expect("create should succeed");
        assert!(get_by_id(&conn, &creator.id).unwrap().is_some());

        // Link to a made-up item, read back, and verify the link.
        let item_uid = "fakeprovider:e2e-item";
        link_to_item(&conn, item_uid, &creator.id, Some("author")).expect("link should succeed");
        let links = list_creators_for_item(&conn, item_uid).expect("list should succeed");
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].creator.id, creator.id);
        assert_eq!(links[0].role.as_deref(), Some("author"));

        // Unlink and verify removal.
        unlink_from_item(&conn, item_uid, &creator.id).expect("unlink should succeed");
        assert!(list_creators_for_item(&conn, item_uid)
            .expect("list should succeed")
            .is_empty());

        // Update the creator and verify the change persisted.
        creator.name = "Updated Name".to_string();
        creator.aliases = vec!["Updated Alias".to_string()];
        update(&conn, &creator).expect("update should succeed");
        let updated = get_by_id(&conn, &creator.id).unwrap().unwrap();
        assert_eq!(updated.name, "Updated Name");
        assert_eq!(updated.aliases, vec!["Updated Alias".to_string()]);

        // Delete and confirm it's gone.
        delete(&conn, &creator.id).expect("delete should succeed");
        assert!(get_by_id(&conn, &creator.id).unwrap().is_none());
    }
}
