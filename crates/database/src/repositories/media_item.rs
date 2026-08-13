//! `MediaItem` repository (kelpie.md §136 Phase 2). Backs migration `0002_items.sql`.
//!
//! `items` holds one row per `MediaItem`; structured sub-fields that don't need
//! independent relational identity (`creators`, `thumbnails`, `tags`,
//! `classification`, `dimensions`, `series`, `sequence`) are serialized as JSON
//! TEXT columns rather than normalized into their own tables — see
//! `migrations/0002_items.sql` for the rationale. `media_sources` is a genuine
//! child table (an item may resolve to many sources), so it is loaded/replaced as
//! its own set of rows keyed by `item_uid` rather than serialized inline.
//!
//! `kind` and `availability` are plain-text enum columns: `MediaKind` and
//! `Availability` already carry the right `serde(rename_all = ...)` attributes
//! (kelpie.md §13, §18), so this module reuses `serde_json` to project each enum
//! to/from its serde string representation instead of hand-writing the mapping.

use kelpie_core::domain::media_item::{MediaItem, MediaSource, MediaTransport};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{de::DeserializeOwned, Serialize};

/// Errors from the MediaItem repository: SQLite failures, plus JSON
/// (de)serialization failures for the structured sub-fields stored as JSON TEXT
/// columns.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("failed to (de)serialize a MediaItem JSON field: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

const SELECT_COLUMNS: &str = "uid, provider_id, remote_id, canonical_url, kind, title, \
    description, creators, thumbnails, tags, classification, published_at, updated_at, \
    duration_seconds, dimensions, series, sequence, availability, provider_metadata";

/// Raw row shape for `items`, before its JSON TEXT columns are parsed back into
/// structured types. Kept separate from [`MediaItem`] so the `rusqlite` row-mapping
/// closure (which must return `rusqlite::Result`) never needs to also handle
/// `serde_json` errors.
struct ItemRow {
    uid: String,
    provider_id: String,
    remote_id: Option<String>,
    canonical_url: String,
    kind: String,
    title: String,
    description: Option<String>,
    creators: String,
    thumbnails: String,
    tags: String,
    classification: String,
    published_at: Option<String>,
    updated_at: Option<String>,
    duration_seconds: Option<f64>,
    dimensions: Option<String>,
    series: Option<String>,
    sequence: Option<String>,
    availability: String,
    provider_metadata: Option<String>,
}

fn row_to_item_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<ItemRow> {
    Ok(ItemRow {
        uid: row.get(0)?,
        provider_id: row.get(1)?,
        remote_id: row.get(2)?,
        canonical_url: row.get(3)?,
        kind: row.get(4)?,
        title: row.get(5)?,
        description: row.get(6)?,
        creators: row.get(7)?,
        thumbnails: row.get(8)?,
        tags: row.get(9)?,
        classification: row.get(10)?,
        published_at: row.get(11)?,
        updated_at: row.get(12)?,
        duration_seconds: row.get(13)?,
        dimensions: row.get(14)?,
        series: row.get(15)?,
        sequence: row.get(16)?,
        availability: row.get(17)?,
        provider_metadata: row.get(18)?,
    })
}

/// Project an enum with a `serde(rename_all = ...)` attribute to its serde string
/// representation (e.g. `MediaKind::ShortVideo` -> `"short_video"`), for storage in
/// a plain-text column.
fn enum_to_text<T: Serialize>(value: &T) -> Result<String> {
    match serde_json::to_value(value)? {
        serde_json::Value::String(s) => Ok(s),
        other => Ok(other.to_string()),
    }
}

/// Inverse of [`enum_to_text`].
fn enum_from_text<T: DeserializeOwned>(text: &str) -> Result<T> {
    Ok(serde_json::from_value(serde_json::Value::String(
        text.to_string(),
    ))?)
}

fn item_row_into_media_item(row: ItemRow, media_sources: Vec<MediaSource>) -> Result<MediaItem> {
    Ok(MediaItem {
        uid: row.uid,
        provider_id: row.provider_id,
        remote_id: row.remote_id,
        canonical_url: row.canonical_url,
        kind: enum_from_text(&row.kind)?,
        title: row.title,
        description: row.description,
        creators: serde_json::from_str(&row.creators)?,
        thumbnails: serde_json::from_str(&row.thumbnails)?,
        tags: serde_json::from_str(&row.tags)?,
        classification: serde_json::from_str(&row.classification)?,
        published_at: row.published_at,
        updated_at: row.updated_at,
        duration_seconds: row.duration_seconds,
        dimensions: row
            .dimensions
            .as_deref()
            .map(serde_json::from_str)
            .transpose()?,
        series: row
            .series
            .as_deref()
            .map(serde_json::from_str)
            .transpose()?,
        sequence: row
            .sequence
            .as_deref()
            .map(serde_json::from_str)
            .transpose()?,
        availability: enum_from_text(&row.availability)?,
        media_sources,
        provider_metadata: row.provider_metadata,
    })
}

fn load_media_sources(conn: &Connection, item_uid: &str) -> Result<Vec<MediaSource>> {
    let mut stmt = conn.prepare(
        "SELECT url, transport, mime_type, width, height, bitrate, label, expires_at \
         FROM media_sources WHERE item_uid = ?1 ORDER BY id",
    )?;

    #[allow(clippy::type_complexity)]
    let rows: Vec<(
        String,
        String,
        Option<String>,
        Option<u32>,
        Option<u32>,
        Option<u32>,
        Option<String>,
        Option<String>,
    )> = stmt
        .query_map(params![item_uid], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
                row.get(7)?,
            ))
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    rows.into_iter()
        .map(
            |(url, transport, mime_type, width, height, bitrate, label, expires_at)| {
                Ok(MediaSource {
                    url,
                    transport: enum_from_text::<MediaTransport>(&transport)?,
                    mime_type,
                    width,
                    height,
                    bitrate,
                    label,
                    expires_at,
                })
            },
        )
        .collect()
}

/// Delete and re-insert every `media_sources` row for `item_uid`, matching the
/// full `media_sources: Vec<MediaSource>` passed in. Used by both [`create`] and
/// [`update`] so a `MediaItem`'s sources always exactly reflect what was written.
fn replace_media_sources(conn: &Connection, item_uid: &str, sources: &[MediaSource]) -> Result<()> {
    conn.execute(
        "DELETE FROM media_sources WHERE item_uid = ?1",
        params![item_uid],
    )?;

    for source in sources {
        let transport = enum_to_text(&source.transport)?;
        conn.execute(
            "INSERT INTO media_sources (
                item_uid, url, transport, mime_type, width, height, bitrate, label, expires_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                item_uid,
                source.url,
                transport,
                source.mime_type,
                source.width,
                source.height,
                source.bitrate,
                source.label,
                source.expires_at,
            ],
        )?;
    }

    Ok(())
}

/// Insert a new `MediaItem`, including its `media_sources` rows.
pub fn create(conn: &Connection, item: &MediaItem) -> Result<()> {
    let kind = enum_to_text(&item.kind)?;
    let availability = enum_to_text(&item.availability)?;
    let creators = serde_json::to_string(&item.creators)?;
    let thumbnails = serde_json::to_string(&item.thumbnails)?;
    let tags = serde_json::to_string(&item.tags)?;
    let classification = serde_json::to_string(&item.classification)?;
    let dimensions = item
        .dimensions
        .as_ref()
        .map(serde_json::to_string)
        .transpose()?;
    let series = item
        .series
        .as_ref()
        .map(serde_json::to_string)
        .transpose()?;
    let sequence = item
        .sequence
        .as_ref()
        .map(serde_json::to_string)
        .transpose()?;

    conn.execute(
        "INSERT INTO items (
            uid, provider_id, remote_id, canonical_url, kind, title, description,
            creators, thumbnails, tags, classification, published_at, updated_at,
            duration_seconds, dimensions, series, sequence, availability, provider_metadata
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19)",
        params![
            item.uid,
            item.provider_id,
            item.remote_id,
            item.canonical_url,
            kind,
            item.title,
            item.description,
            creators,
            thumbnails,
            tags,
            classification,
            item.published_at,
            item.updated_at,
            item.duration_seconds,
            dimensions,
            series,
            sequence,
            availability,
            item.provider_metadata,
        ],
    )?;

    replace_media_sources(conn, &item.uid, &item.media_sources)?;

    Ok(())
}

/// Look up a `MediaItem` by its `uid`, including its `media_sources`. Returns
/// `Ok(None)` if no such item exists.
pub fn get_by_uid(conn: &Connection, uid: &str) -> Result<Option<MediaItem>> {
    let sql = format!("SELECT {SELECT_COLUMNS} FROM items WHERE uid = ?1");
    let row = conn
        .query_row(&sql, params![uid], row_to_item_row)
        .optional()?;

    let Some(row) = row else {
        return Ok(None);
    };

    let media_sources = load_media_sources(conn, uid)?;
    Ok(Some(item_row_into_media_item(row, media_sources)?))
}

/// Overwrite an existing `MediaItem` (matched by `uid`), including replacing its
/// `media_sources` rows. A no-op if no item with that `uid` exists.
pub fn update(conn: &Connection, item: &MediaItem) -> Result<()> {
    let kind = enum_to_text(&item.kind)?;
    let availability = enum_to_text(&item.availability)?;
    let creators = serde_json::to_string(&item.creators)?;
    let thumbnails = serde_json::to_string(&item.thumbnails)?;
    let tags = serde_json::to_string(&item.tags)?;
    let classification = serde_json::to_string(&item.classification)?;
    let dimensions = item
        .dimensions
        .as_ref()
        .map(serde_json::to_string)
        .transpose()?;
    let series = item
        .series
        .as_ref()
        .map(serde_json::to_string)
        .transpose()?;
    let sequence = item
        .sequence
        .as_ref()
        .map(serde_json::to_string)
        .transpose()?;

    let changed = conn.execute(
        "UPDATE items SET
            provider_id = ?2, remote_id = ?3, canonical_url = ?4, kind = ?5, title = ?6,
            description = ?7, creators = ?8, thumbnails = ?9, tags = ?10, classification = ?11,
            published_at = ?12, updated_at = ?13, duration_seconds = ?14, dimensions = ?15,
            series = ?16, sequence = ?17, availability = ?18, provider_metadata = ?19
         WHERE uid = ?1",
        params![
            item.uid,
            item.provider_id,
            item.remote_id,
            item.canonical_url,
            kind,
            item.title,
            item.description,
            creators,
            thumbnails,
            tags,
            classification,
            item.published_at,
            item.updated_at,
            item.duration_seconds,
            dimensions,
            series,
            sequence,
            availability,
            item.provider_metadata,
        ],
    )?;

    if changed > 0 {
        replace_media_sources(conn, &item.uid, &item.media_sources)?;
    }

    Ok(())
}

/// Delete a `MediaItem` and its `media_sources` rows by `uid`. A no-op if no item
/// with that `uid` exists.
pub fn delete(conn: &Connection, uid: &str) -> Result<()> {
    conn.execute(
        "DELETE FROM media_sources WHERE item_uid = ?1",
        params![uid],
    )?;
    conn.execute("DELETE FROM items WHERE uid = ?1", params![uid])?;
    Ok(())
}

/// List every `MediaItem` for a given `provider_id`, ordered by `uid`, each with
/// its `media_sources` populated.
pub fn list_by_provider(conn: &Connection, provider_id: &str) -> Result<Vec<MediaItem>> {
    let sql = format!("SELECT {SELECT_COLUMNS} FROM items WHERE provider_id = ?1 ORDER BY uid");
    let mut stmt = conn.prepare(&sql)?;
    let rows: Vec<ItemRow> = stmt
        .query_map(params![provider_id], row_to_item_row)?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    let mut items = Vec::with_capacity(rows.len());
    for row in rows {
        let media_sources = load_media_sources(conn, &row.uid)?;
        items.push(item_row_into_media_item(row, media_sources)?);
    }
    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::*;
    use kelpie_core::domain::media_item::{MediaDimensions, MediaSequence};
    use kelpie_core::domain::{
        Availability, ContentClassification, CreatorCredit, ImageResource, MediaKind,
        SeriesReference, TagReference,
    };
    use std::path::PathBuf;

    fn temp_conn(name: &str) -> (tempfile::TempDir, Connection) {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path: PathBuf = dir.path().join(name);
        let conn = crate::open_at(&path).expect("open_at should succeed");
        (dir, conn)
    }

    fn sample_item() -> MediaItem {
        MediaItem {
            uid: "fake-provider:video:1001".to_string(),
            provider_id: "fake-provider".to_string(),
            remote_id: Some("1001".to_string()),
            canonical_url: "https://example.invalid/videos/1001".to_string(),
            kind: MediaKind::Video,
            title: "Sample Title".to_string(),
            description: Some("A sample description.".to_string()),
            creators: vec![CreatorCredit {
                name: "Sample Creator".to_string(),
                role: Some("performer".to_string()),
                provider_id: Some("fake-provider".to_string()),
                url: None,
            }],
            thumbnails: vec![ImageResource {
                url: "https://example.invalid/thumbs/1001.jpg".to_string(),
                width: Some(320),
                height: Some(180),
            }],
            tags: vec![TagReference {
                normalized_id: Some("tag-sample".to_string()),
                provider_id: "fake-provider".to_string(),
                source_value: "Sample Tag".to_string(),
                display_name: "sample tag".to_string(),
                category: Some("theme".to_string()),
            }],
            classification: ContentClassification {
                labels: vec!["adult".to_string()],
                provider_labels: vec!["adult-content".to_string()],
                style: None,
            },
            published_at: Some("2026-01-01T00:00:00Z".to_string()),
            updated_at: Some("2026-01-02T00:00:00Z".to_string()),
            duration_seconds: Some(123.5),
            dimensions: Some(MediaDimensions {
                width: Some(1920),
                height: Some(1080),
            }),
            series: Some(SeriesReference {
                uid: "fake-provider:series:9".to_string(),
                title: "Sample Series".to_string(),
            }),
            sequence: Some(MediaSequence {
                volume: None,
                chapter: None,
                issue: None,
                page_count: None,
            }),
            availability: Availability::Direct,
            media_sources: vec![
                MediaSource {
                    url: "https://example.invalid/media/1001-hd.mp4".to_string(),
                    transport: MediaTransport::Http,
                    mime_type: Some("video/mp4".to_string()),
                    width: Some(1920),
                    height: Some(1080),
                    bitrate: Some(4_000_000),
                    label: Some("1080p".to_string()),
                    expires_at: None,
                },
                MediaSource {
                    url: "https://example.invalid/media/1001.m3u8".to_string(),
                    transport: MediaTransport::Hls,
                    mime_type: Some("application/vnd.apple.mpegurl".to_string()),
                    width: None,
                    height: None,
                    bitrate: None,
                    label: Some("adaptive".to_string()),
                    expires_at: Some("2026-01-03T00:00:00Z".to_string()),
                },
            ],
            provider_metadata: Some(r#"{"fake":"metadata"}"#.to_string()),
        }
    }

    #[test]
    fn create_get_update_delete_round_trip() {
        let (_dir, conn) = temp_conn("media_item.sqlite");
        let item = sample_item();

        create(&conn, &item).expect("create should succeed");

        let fetched = get_by_uid(&conn, &item.uid)
            .expect("get_by_uid should succeed")
            .expect("item should exist after create");

        assert_eq!(fetched.uid, item.uid);
        assert_eq!(fetched.provider_id, item.provider_id);
        assert_eq!(fetched.remote_id, item.remote_id);
        assert_eq!(fetched.canonical_url, item.canonical_url);
        assert_eq!(fetched.kind, MediaKind::Video);
        assert_eq!(fetched.title, item.title);
        assert_eq!(fetched.description, item.description);
        assert_eq!(fetched.published_at, item.published_at);
        assert_eq!(fetched.updated_at, item.updated_at);
        assert_eq!(fetched.duration_seconds, item.duration_seconds);
        assert_eq!(fetched.availability, Availability::Direct);
        assert_eq!(fetched.provider_metadata, item.provider_metadata);

        // Nested JSON fields.
        assert_eq!(fetched.tags.len(), 1);
        assert_eq!(fetched.tags[0].display_name, "sample tag");
        assert_eq!(fetched.tags[0].normalized_id.as_deref(), Some("tag-sample"));

        assert_eq!(fetched.thumbnails.len(), 1);
        assert_eq!(
            fetched.thumbnails[0].url,
            "https://example.invalid/thumbs/1001.jpg"
        );
        assert_eq!(fetched.thumbnails[0].width, Some(320));

        assert_eq!(fetched.creators.len(), 1);
        assert_eq!(fetched.creators[0].name, "Sample Creator");

        let dims = fetched
            .dimensions
            .clone()
            .expect("dimensions should round-trip");
        assert_eq!(dims.width, Some(1920));
        assert_eq!(dims.height, Some(1080));

        let series = fetched.series.clone().expect("series should round-trip");
        assert_eq!(series.uid, "fake-provider:series:9");
        assert_eq!(series.title, "Sample Series");

        // media_sources, loaded from the child table.
        assert_eq!(fetched.media_sources.len(), 2);
        assert_eq!(fetched.media_sources[0].transport, MediaTransport::Http);
        assert_eq!(fetched.media_sources[0].label.as_deref(), Some("1080p"));
        assert_eq!(fetched.media_sources[1].transport, MediaTransport::Hls);
        assert_eq!(
            fetched.media_sources[1].expires_at.as_deref(),
            Some("2026-01-03T00:00:00Z")
        );

        // Update: change title, drop a media source, add a tag.
        let mut updated = fetched.clone();
        updated.title = "Updated Title".to_string();
        updated.media_sources.truncate(1);
        updated.tags.push(TagReference {
            normalized_id: None,
            provider_id: "fake-provider".to_string(),
            source_value: "Second Tag".to_string(),
            display_name: "second tag".to_string(),
            category: None,
        });

        update(&conn, &updated).expect("update should succeed");

        let refetched = get_by_uid(&conn, &item.uid)
            .expect("get_by_uid should succeed")
            .expect("item should still exist after update");

        assert_eq!(refetched.title, "Updated Title");
        assert_eq!(refetched.media_sources.len(), 1);
        assert_eq!(refetched.tags.len(), 2);
        assert_eq!(refetched.tags[1].display_name, "second tag");

        // Delete.
        delete(&conn, &item.uid).expect("delete should succeed");
        assert!(get_by_uid(&conn, &item.uid)
            .expect("get_by_uid should succeed")
            .is_none());

        let sources_left: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM media_sources WHERE item_uid = ?1",
                params![item.uid],
                |row| row.get(0),
            )
            .expect("count media_sources");
        assert_eq!(
            sources_left, 0,
            "deleting an item should delete its media_sources"
        );
    }

    #[test]
    fn list_by_provider_returns_only_matching_items() {
        let (_dir, conn) = temp_conn("media_item_list.sqlite");

        let mut a = sample_item();
        a.uid = "fake-provider:video:1".to_string();
        a.provider_id = "fake-provider".to_string();

        let mut b = sample_item();
        b.uid = "fake-provider:video:2".to_string();
        b.provider_id = "fake-provider".to_string();

        let mut c = sample_item();
        c.uid = "other-provider:video:1".to_string();
        c.provider_id = "other-provider".to_string();

        create(&conn, &a).unwrap();
        create(&conn, &b).unwrap();
        create(&conn, &c).unwrap();

        let fake_items = list_by_provider(&conn, "fake-provider").expect("list should succeed");
        assert_eq!(fake_items.len(), 2);
        assert!(fake_items
            .iter()
            .all(|item| item.provider_id == "fake-provider"));

        let other_items = list_by_provider(&conn, "other-provider").expect("list should succeed");
        assert_eq!(other_items.len(), 1);
        assert_eq!(other_items[0].uid, "other-provider:video:1");

        let none_items = list_by_provider(&conn, "nonexistent").expect("list should succeed");
        assert!(none_items.is_empty());
    }

    #[test]
    fn get_by_uid_returns_none_when_missing() {
        let (_dir, conn) = temp_conn("media_item_missing.sqlite");
        assert!(get_by_uid(&conn, "does-not-exist").unwrap().is_none());
    }

    #[test]
    fn update_on_missing_uid_is_a_no_op() {
        let (_dir, conn) = temp_conn("media_item_update_missing.sqlite");
        let item = sample_item();
        // No create() call: item does not exist yet.
        update(&conn, &item).expect("update on a missing uid should not error");
        assert!(get_by_uid(&conn, &item.uid).unwrap().is_none());
    }
}
