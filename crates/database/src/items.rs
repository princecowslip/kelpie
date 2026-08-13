//! Items repository (kelpie.md §136 Core Data Layer): MediaItem (§16) and
//! MediaSource (§19), the `items` and `media_sources` tables from
//! `migrations/0002_core_data_layer.sql`.
//!
//! Owned by a Stage B unit — repository code and comprehensive unit tests
//! land here.
//!
//! ## Interface gaps (kelpie.md §16/§21) filled in pragmatically here
//!
//! kelpie.md references `CreatorCredit[]` and `ImageResource[]` (§16, §21)
//! but never formally defines their fields. This module makes a reasonable,
//! minimal call for both (documented on each struct below) rather than
//! inventing a larger shape the spec doesn't ask for.
//!
//! ## Storage strategy
//!
//! `items.kind` is stored as a plain `String`, not a Rust enum, even though
//! §13 enumerates a closed-looking `MediaKind` list — that list already spans
//! video/image/manga/comic/text/audio/imageboard/game categories and is the
//! kind of thing new provider categories are likely to extend over time.
//! Modeling it as an open string avoids a compile-time enum that has to be
//! kept in lockstep with §13 (and with every provider adapter) as it grows.
//!
//! `items.availability` and `media_sources.transport`, by contrast, are
//! small, genuinely closed sets (§18, §19) with real branching behavior
//! attached to each variant elsewhere in the spec, so those are modeled as
//! proper enums ([`Availability`], [`Transport`]) with `rusqlite`
//! `FromSql`/`ToSql` impls so they round-trip through their TEXT columns
//! directly.
//!
//! The nested/array fields kelpie.md leaves as JSON columns on `items`
//! (`classification_json`, `creators_json`, `thumbnails_json`,
//! `sequence_json`, `provider_metadata_json` — see the migration file's own
//! notes) are modeled as real nested structs here and serialized with
//! `serde_json` at the repository boundary, so callers work with typed Rust
//! values everywhere except the SQL itself.

use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSql, ToSqlOutput, ValueRef};
use rusqlite::{named_params, params, Connection, OptionalExtension, Row};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

// ---------------------------------------------------------------------------
// Domain structs
// ---------------------------------------------------------------------------

/// ContentClassification (kelpie.md §15). `adult` is modeled as a plain
/// `bool` rather than a literal-`true` type (Rust has no such thing without a
/// unit struct wrapper) — callers are expected to always set it `true`, per
/// Kelpie's adult-only product scope (CLAUDE.md "Content boundaries").
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentClassification {
    pub adult: bool,
    pub labels: Vec<String>,
    pub provider_labels: Vec<String>,
    pub style: Option<Vec<String>>,
}

/// CreatorCredit — referenced by §16 (`MediaItem.creators`) and §21
/// (`Series.creators`) but never formally defined by kelpie.md. Minimal,
/// reasonable shape: an optional stable identity for the creator (so a
/// credit can later be linked to a row in the `creators` table this crate
/// also owns) plus the display name and the credited role (e.g. "author",
/// "artist", "voice actor").
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatorCredit {
    pub creator_uid: Option<String>,
    pub name: String,
    pub role: Option<String>,
}

/// ImageResource — referenced by §16 (`MediaItem.thumbnails`) and §21
/// (`Series.cover`) but never formally defined. Minimal, reasonable shape: a
/// URL plus optional pixel dimensions (useful for picking a thumbnail size
/// without fetching the image first).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageResource {
    pub url: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

/// SequenceNumber (kelpie.md §22): chapter/volume/issue numbering is not
/// assumed to be an integer ("10", "10.5", "10a", "Special", "Bonus",
/// "Prologue" are all real-world values), so the raw provider string is
/// always kept, `numeric` is a best-effort parse for sorting/arithmetic, and
/// `sort_key` is a separately computable, always-present ordering key.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SequenceNumber {
    pub raw: String,
    pub numeric: Option<f64>,
    pub sort_key: String,
}

/// The `sequence` sub-object of `MediaItem` (kelpie.md §16).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sequence {
    pub volume: Option<SequenceNumber>,
    pub chapter: Option<SequenceNumber>,
    pub issue: Option<SequenceNumber>,
    pub page_count: Option<u32>,
}

/// Availability (kelpie.md §18) — a closed set with real UI-facing
/// branching behavior, so it's a proper enum rather than a plain string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Availability {
    Direct,
    Embedded,
    Browser,
    LoginRequired,
    SubscriptionRequired,
    TemporarilyUnavailable,
    Removed,
    Unknown,
}

impl Availability {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::Embedded => "embedded",
            Self::Browser => "browser",
            Self::LoginRequired => "login-required",
            Self::SubscriptionRequired => "subscription-required",
            Self::TemporarilyUnavailable => "temporarily-unavailable",
            Self::Removed => "removed",
            Self::Unknown => "unknown",
        }
    }

    /// Not named `from_str` to avoid colliding with (and being mistaken for
    /// an incomplete implementation of) `std::str::FromStr`.
    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "direct" => Self::Direct,
            "embedded" => Self::Embedded,
            "browser" => Self::Browser,
            "login-required" => Self::LoginRequired,
            "subscription-required" => Self::SubscriptionRequired,
            "temporarily-unavailable" => Self::TemporarilyUnavailable,
            "removed" => Self::Removed,
            "unknown" => Self::Unknown,
            _ => return None,
        })
    }
}

impl FromSql for Availability {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value
            .as_str()
            .and_then(|s| Self::parse(s).ok_or(FromSqlError::InvalidType))
    }
}

impl ToSql for Availability {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.as_str()))
    }
}

/// MediaSource transport (kelpie.md §19) — a closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Transport {
    Http,
    File,
    Hls,
    Dash,
    Browser,
}

impl Transport {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Http => "http",
            Self::File => "file",
            Self::Hls => "hls",
            Self::Dash => "dash",
            Self::Browser => "browser",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "http" => Self::Http,
            "file" => Self::File,
            "hls" => Self::Hls,
            "dash" => Self::Dash,
            "browser" => Self::Browser,
            _ => return None,
        })
    }
}

impl FromSql for Transport {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        value
            .as_str()
            .and_then(|s| Self::parse(s).ok_or(FromSqlError::InvalidType))
    }
}

impl ToSql for Transport {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.as_str()))
    }
}

/// MediaSource (kelpie.md §19). One row per playable/renderable source URL
/// for an item; a real table (`media_sources`) rather than a JSON column,
/// per the migration's own domain-table rationale.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MediaSource {
    pub url: String,
    pub transport: Transport,
    pub mime_type: Option<String>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub bitrate: Option<i64>,
    pub label: Option<String>,
    pub expires_at: Option<String>,
}

/// The Unified MediaItem (kelpie.md §16), shaped to match the `items` table
/// columns. `tags` (owned by the creators/tags Stage B unit, via `item_tags`)
/// and `mediaSources` (this module's own `media_sources` table, but a
/// separate repository call rather than an embedded field) are intentionally
/// not fields here — they live in, and are fetched from, their own tables.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    /// Stable identity per §17, computed by callers via
    /// `kelpie_core::identity::stable_id(provider_id, "item", remote_id, canonical_url)`
    /// (or supplied directly, e.g. when round-tripping a value already
    /// loaded from the database). This module does not compute it for
    /// callers implicitly, so a caller who already has a uid never pays for
    /// a redundant hash.
    pub uid: String,
    pub provider_id: String,
    pub remote_id: Option<String>,
    pub canonical_url: String,
    pub kind: String,
    pub title: String,
    pub description: Option<String>,
    pub classification: ContentClassification,
    pub creators: Vec<CreatorCredit>,
    pub thumbnails: Vec<ImageResource>,
    pub published_at: Option<String>,
    pub updated_at: Option<String>,
    pub duration_seconds: Option<i64>,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub series_uid: Option<String>,
    pub sequence: Option<Sequence>,
    pub availability: Availability,
    pub provider_metadata: Option<serde_json::Value>,
    pub created_at: String,
    pub updated_locally_at: String,
}

// ---------------------------------------------------------------------------
// JSON column (de)serialization helpers
// ---------------------------------------------------------------------------

/// Serialize a value for a JSON TEXT column. `Error` has no JSON variant of
/// its own (this crate's `Error` is deliberately kept to `Io`/`Sqlite`/
/// `IntegrityCheck` — see `lib.rs`), so a serialization failure is folded
/// into `Error::Sqlite` via `rusqlite::Error::ToSqlConversionFailure`, the
/// variant `rusqlite` itself uses for this exact situation (a value that
/// can't be turned into something SQLite can store).
fn to_json<T: Serialize>(value: &T) -> Result<String> {
    serde_json::to_string(value)
        .map_err(|e| Error::Sqlite(rusqlite::Error::ToSqlConversionFailure(Box::new(e))))
}

/// Deserialize a JSON TEXT column value, for use inside row-mapping closures
/// that must return `rusqlite::Result`. The column index is only used for
/// the error's diagnostic payload, so a fixed placeholder is fine here.
fn from_json<T: DeserializeOwned>(s: &str) -> rusqlite::Result<T> {
    serde_json::from_str(s).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
    })
}

// ---------------------------------------------------------------------------
// items repository
// ---------------------------------------------------------------------------

const ITEM_COLUMNS: &str = "uid, provider_id, remote_id, canonical_url, kind, title, description, \
    classification_json, creators_json, thumbnails_json, published_at, updated_at, \
    duration_seconds, width, height, series_uid, sequence_json, availability, \
    provider_metadata_json, created_at, updated_locally_at";

fn item_from_row(row: &Row<'_>) -> rusqlite::Result<Item> {
    let classification_json: String = row.get("classification_json")?;
    let creators_json: String = row.get("creators_json")?;
    let thumbnails_json: String = row.get("thumbnails_json")?;
    let sequence_json: Option<String> = row.get("sequence_json")?;
    let provider_metadata_json: Option<String> = row.get("provider_metadata_json")?;

    Ok(Item {
        uid: row.get("uid")?,
        provider_id: row.get("provider_id")?,
        remote_id: row.get("remote_id")?,
        canonical_url: row.get("canonical_url")?,
        kind: row.get("kind")?,
        title: row.get("title")?,
        description: row.get("description")?,
        classification: from_json(&classification_json)?,
        creators: from_json(&creators_json)?,
        thumbnails: from_json(&thumbnails_json)?,
        published_at: row.get("published_at")?,
        updated_at: row.get("updated_at")?,
        duration_seconds: row.get("duration_seconds")?,
        width: row.get("width")?,
        height: row.get("height")?,
        series_uid: row.get("series_uid")?,
        sequence: sequence_json.as_deref().map(from_json).transpose()?,
        availability: row.get("availability")?,
        provider_metadata: provider_metadata_json
            .as_deref()
            .map(from_json)
            .transpose()?,
        created_at: row.get("created_at")?,
        updated_locally_at: row.get("updated_locally_at")?,
    })
}

/// Insert a new item, or update an existing one with the same `uid`.
///
/// Uses `INSERT ... ON CONFLICT(uid) DO UPDATE` rather than `INSERT OR
/// REPLACE`: `OR REPLACE` deletes-then-reinserts the conflicting row, which
/// would fire `media_sources`' `ON DELETE CASCADE` and silently wipe out an
/// item's media sources on every metadata refresh. `ON CONFLICT DO UPDATE`
/// updates in place instead. It also deliberately does not overwrite
/// `created_at` on conflict, so `created_at` keeps meaning "first time this
/// item was seen locally" across repeated upserts (e.g. from feed refreshes)
/// while `updated_locally_at` moves forward each time.
pub fn upsert_item(conn: &Connection, item: &Item) -> Result<()> {
    let classification_json = to_json(&item.classification)?;
    let creators_json = to_json(&item.creators)?;
    let thumbnails_json = to_json(&item.thumbnails)?;
    let sequence_json = item.sequence.as_ref().map(to_json).transpose()?;
    let provider_metadata_json = item.provider_metadata.as_ref().map(to_json).transpose()?;

    conn.execute(
        "INSERT INTO items (
            uid, provider_id, remote_id, canonical_url, kind, title, description,
            classification_json, creators_json, thumbnails_json, published_at, updated_at,
            duration_seconds, width, height, series_uid, sequence_json, availability,
            provider_metadata_json, created_at, updated_locally_at
        ) VALUES (
            :uid, :provider_id, :remote_id, :canonical_url, :kind, :title, :description,
            :classification_json, :creators_json, :thumbnails_json, :published_at, :updated_at,
            :duration_seconds, :width, :height, :series_uid, :sequence_json, :availability,
            :provider_metadata_json, :created_at, :updated_locally_at
        )
        ON CONFLICT(uid) DO UPDATE SET
            provider_id = excluded.provider_id,
            remote_id = excluded.remote_id,
            canonical_url = excluded.canonical_url,
            kind = excluded.kind,
            title = excluded.title,
            description = excluded.description,
            classification_json = excluded.classification_json,
            creators_json = excluded.creators_json,
            thumbnails_json = excluded.thumbnails_json,
            published_at = excluded.published_at,
            updated_at = excluded.updated_at,
            duration_seconds = excluded.duration_seconds,
            width = excluded.width,
            height = excluded.height,
            series_uid = excluded.series_uid,
            sequence_json = excluded.sequence_json,
            availability = excluded.availability,
            provider_metadata_json = excluded.provider_metadata_json,
            updated_locally_at = excluded.updated_locally_at",
        named_params! {
            ":uid": item.uid,
            ":provider_id": item.provider_id,
            ":remote_id": item.remote_id,
            ":canonical_url": item.canonical_url,
            ":kind": item.kind,
            ":title": item.title,
            ":description": item.description,
            ":classification_json": classification_json,
            ":creators_json": creators_json,
            ":thumbnails_json": thumbnails_json,
            ":published_at": item.published_at,
            ":updated_at": item.updated_at,
            ":duration_seconds": item.duration_seconds,
            ":width": item.width,
            ":height": item.height,
            ":series_uid": item.series_uid,
            ":sequence_json": sequence_json,
            ":availability": item.availability,
            ":provider_metadata_json": provider_metadata_json,
            ":created_at": item.created_at,
            ":updated_locally_at": item.updated_locally_at,
        },
    )?;
    Ok(())
}

/// Fetch a single item by uid, or `None` if it doesn't exist.
pub fn get_item(conn: &Connection, uid: &str) -> Result<Option<Item>> {
    let sql = format!("SELECT {ITEM_COLUMNS} FROM items WHERE uid = ?1");
    conn.query_row(&sql, params![uid], item_from_row)
        .optional()
        .map_err(Error::from)
}

/// List items, most-recently-locally-updated first, with `uid` as a stable
/// tiebreaker (so pagination stays deterministic even when several items
/// share the same `updated_locally_at` timestamp).
pub fn list_items(conn: &Connection, limit: u32, offset: u32) -> Result<Vec<Item>> {
    let sql =
        format!("SELECT {ITEM_COLUMNS} FROM items ORDER BY updated_locally_at DESC, uid ASC LIMIT ?1 OFFSET ?2");
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params![limit, offset], item_from_row)?;
    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Error::from)
}

/// Delete an item by uid. `media_sources`, `item_tags`, `collection_items`,
/// `favorites`, `progress`, and `history` all declare `ON DELETE CASCADE`
/// back to `items(uid)` (see `migrations/0002_core_data_layer.sql`), but
/// SQLite only fires those cascades when foreign key enforcement is turned
/// on for the connection performing the delete — it is off by default, and
/// nothing in the frozen bootstrap path (`crates/database/src/lib.rs`) turns
/// it on. So it's enabled here, defensively, immediately before the delete.
pub fn delete_item(conn: &Connection, uid: &str) -> Result<()> {
    conn.pragma_update(None, "foreign_keys", true)?;
    conn.execute("DELETE FROM items WHERE uid = ?1", params![uid])?;
    Ok(())
}

// ---------------------------------------------------------------------------
// media_sources repository
// ---------------------------------------------------------------------------

/// Add a media source for an item, returning the new row's id.
pub fn add_media_source(conn: &Connection, item_uid: &str, source: &MediaSource) -> Result<i64> {
    conn.execute(
        "INSERT INTO media_sources (
            item_uid, url, transport, mime_type, width, height, bitrate, label, expires_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            item_uid,
            source.url,
            source.transport,
            source.mime_type,
            source.width,
            source.height,
            source.bitrate,
            source.label,
            source.expires_at,
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

/// List an item's media sources, in insertion order.
pub fn list_media_sources(conn: &Connection, item_uid: &str) -> Result<Vec<MediaSource>> {
    let mut stmt = conn.prepare(
        "SELECT url, transport, mime_type, width, height, bitrate, label, expires_at
         FROM media_sources WHERE item_uid = ?1 ORDER BY id ASC",
    )?;
    let rows = stmt.query_map(params![item_uid], |row| {
        Ok(MediaSource {
            url: row.get("url")?,
            transport: row.get("transport")?,
            mime_type: row.get("mime_type")?,
            width: row.get("width")?,
            height: row.get("height")?,
            bitrate: row.get("bitrate")?,
            label: row.get("label")?,
            expires_at: row.get("expires_at")?,
        })
    })?;
    rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Error::from)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn temp_db_path(name: &str) -> (tempfile::TempDir, PathBuf) {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = dir.path().join(name);
        (dir, path)
    }

    /// A minimal, obviously-synthetic item for tests (CLAUDE.md content
    /// boundaries: no real third-party site names, no CSAM-adjacent or
    /// non-consensual test fixtures).
    fn sample_item(uid_suffix: &str) -> Item {
        let remote_id = format!("remote-{uid_suffix}");
        let canonical_url = format!("https://example.test/items/{uid_suffix}");
        let uid = kelpie_core::identity::stable_id(
            "provider-x",
            "item",
            Some(remote_id.as_str()),
            canonical_url.as_str(),
        );

        Item {
            uid,
            provider_id: "provider-x".to_string(),
            remote_id: Some(remote_id),
            canonical_url,
            kind: "video".to_string(),
            title: format!("Sample Item {uid_suffix}"),
            description: Some("A synthetic test fixture.".to_string()),
            classification: ContentClassification {
                adult: true,
                labels: vec!["adult".to_string()],
                provider_labels: vec!["provider-adult-tag".to_string()],
                style: Some(vec!["photographic".to_string()]),
            },
            creators: vec![CreatorCredit {
                creator_uid: Some("provider-x:creator:1".to_string()),
                name: "Test Creator".to_string(),
                role: Some("performer".to_string()),
            }],
            thumbnails: vec![ImageResource {
                url: "https://example.test/thumbs/1.jpg".to_string(),
                width: Some(320),
                height: Some(180),
            }],
            published_at: Some("2026-01-01T00:00:00Z".to_string()),
            updated_at: Some("2026-01-02T00:00:00Z".to_string()),
            duration_seconds: Some(600),
            width: Some(1920),
            height: Some(1080),
            series_uid: None,
            sequence: None,
            availability: Availability::Direct,
            provider_metadata: Some(serde_json::json!({ "example_field": "example_value" })),
            created_at: "2026-01-01T00:00:00Z".to_string(),
            updated_locally_at: "2026-01-01T00:00:00Z".to_string(),
        }
    }

    fn sample_media_source() -> MediaSource {
        MediaSource {
            url: "https://example.test/stream/1.m3u8".to_string(),
            transport: Transport::Hls,
            mime_type: Some("application/vnd.apple.mpegurl".to_string()),
            width: Some(1920),
            height: Some(1080),
            bitrate: Some(4_000_000),
            label: Some("1080p".to_string()),
            expires_at: Some("2026-01-01T01:00:00Z".to_string()),
        }
    }

    #[test]
    fn upsert_then_get_round_trips() {
        let (_dir, path) = temp_db_path("upsert_get.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        let item = sample_item("a");
        upsert_item(&conn, &item).expect("upsert item");

        let fetched = get_item(&conn, &item.uid)
            .expect("get_item should succeed")
            .expect("item should exist");
        assert_eq!(fetched, item);
    }

    #[test]
    fn upsert_uses_stable_id_from_identity_module() {
        let (_dir, path) = temp_db_path("stable_id.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        let item = sample_item("b");
        let expected_uid = kelpie_core::identity::stable_id(
            "provider-x",
            "item",
            item.remote_id.as_deref(),
            &item.canonical_url,
        );
        assert_eq!(item.uid, expected_uid);

        upsert_item(&conn, &item).expect("upsert item");
        let fetched = get_item(&conn, &expected_uid)
            .expect("get_item should succeed")
            .expect("item should exist under its stable_id-derived uid");
        assert_eq!(fetched.uid, expected_uid);
    }

    #[test]
    fn upsert_twice_updates_not_duplicates() {
        let (_dir, path) = temp_db_path("upsert_twice.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        let mut item = sample_item("c");
        upsert_item(&conn, &item).expect("first upsert");

        item.title = "Updated Title".to_string();
        item.availability = Availability::TemporarilyUnavailable;
        item.updated_locally_at = "2026-01-03T00:00:00Z".to_string();
        upsert_item(&conn, &item).expect("second upsert");

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM items", [], |row| row.get(0))
            .expect("count items");
        assert_eq!(count, 1, "upsert must not create a duplicate row");

        let fetched = get_item(&conn, &item.uid)
            .expect("get_item should succeed")
            .expect("item should exist");
        assert_eq!(fetched.title, "Updated Title");
        assert_eq!(fetched.availability, Availability::TemporarilyUnavailable);
        assert_eq!(
            fetched.created_at, "2026-01-01T00:00:00Z",
            "created_at should not move on update"
        );
        assert_eq!(fetched.updated_locally_at, "2026-01-03T00:00:00Z");
    }

    #[test]
    fn get_missing_returns_none() {
        let (_dir, path) = temp_db_path("get_missing.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        let result = get_item(&conn, "does-not-exist").expect("get_item should succeed");
        assert!(result.is_none());
    }

    #[test]
    fn list_items_paginates_in_stable_order() {
        let (_dir, path) = temp_db_path("list_items.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        // Distinct updated_locally_at timestamps, inserted out of order, so
        // ordering is actually exercised rather than coincidentally correct.
        for (suffix, ts) in [
            ("1", "2026-01-01T00:00:00Z"),
            ("2", "2026-01-03T00:00:00Z"),
            ("3", "2026-01-02T00:00:00Z"),
        ] {
            let mut item = sample_item(suffix);
            item.updated_locally_at = ts.to_string();
            upsert_item(&conn, &item).expect("upsert item");
        }

        let page1 = list_items(&conn, 2, 0).expect("list_items page 1");
        assert_eq!(page1.len(), 2);
        assert_eq!(page1[0].title, "Sample Item 2"); // most recently updated
        assert_eq!(page1[1].title, "Sample Item 3");

        let page2 = list_items(&conn, 2, 2).expect("list_items page 2");
        assert_eq!(page2.len(), 1);
        assert_eq!(page2[0].title, "Sample Item 1");

        let page3 = list_items(&conn, 2, 4).expect("list_items page 3 (empty)");
        assert!(page3.is_empty());
    }

    #[test]
    fn delete_item_removes_row_and_cascades_to_media_sources() {
        let (_dir, path) = temp_db_path("delete_item.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        let item = sample_item("d");
        upsert_item(&conn, &item).expect("upsert item");
        add_media_source(&conn, &item.uid, &sample_media_source()).expect("add media source");

        assert_eq!(list_media_sources(&conn, &item.uid).unwrap().len(), 1);

        delete_item(&conn, &item.uid).expect("delete item");

        assert!(get_item(&conn, &item.uid).unwrap().is_none());
        assert!(
            list_media_sources(&conn, &item.uid).unwrap().is_empty(),
            "media_sources should be cascade-deleted with their item"
        );
    }

    #[test]
    fn delete_missing_item_is_a_no_op() {
        let (_dir, path) = temp_db_path("delete_missing.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        // Should not error even though nothing matches.
        delete_item(&conn, "does-not-exist").expect("delete_item should succeed");
    }

    #[test]
    fn add_and_list_media_sources() {
        let (_dir, path) = temp_db_path("media_sources.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        let item = sample_item("e");
        upsert_item(&conn, &item).expect("upsert item");

        let hls_source = sample_media_source();
        let mp4_source = MediaSource {
            url: "https://example.test/download/1.mp4".to_string(),
            transport: Transport::Http,
            mime_type: Some("video/mp4".to_string()),
            width: Some(1280),
            height: Some(720),
            bitrate: Some(2_000_000),
            label: Some("720p".to_string()),
            expires_at: None,
        };

        let hls_id = add_media_source(&conn, &item.uid, &hls_source).expect("add hls source");
        let mp4_id = add_media_source(&conn, &item.uid, &mp4_source).expect("add mp4 source");
        assert_ne!(hls_id, mp4_id);

        let sources = list_media_sources(&conn, &item.uid).expect("list media sources");
        assert_eq!(sources.len(), 2);
        assert_eq!(sources[0], hls_source, "should be in insertion order");
        assert_eq!(sources[1], mp4_source);
    }

    #[test]
    fn list_media_sources_for_unknown_item_is_empty() {
        let (_dir, path) = temp_db_path("media_sources_missing.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        let sources =
            list_media_sources(&conn, "does-not-exist").expect("list_media_sources should succeed");
        assert!(sources.is_empty());
    }

    #[test]
    fn item_without_optional_fields_round_trips() {
        let (_dir, path) = temp_db_path("minimal_item.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        // Exercise the SHA256 fallback branch of stable_id (no remote_id).
        let canonical_url = "https://example.test/items/minimal".to_string();
        let uid = kelpie_core::identity::stable_id("provider-x", "item", None, &canonical_url);

        let item = Item {
            uid: uid.clone(),
            provider_id: "provider-x".to_string(),
            remote_id: None,
            canonical_url,
            kind: "image".to_string(),
            title: "Minimal Item".to_string(),
            description: None,
            classification: ContentClassification {
                adult: true,
                labels: vec![],
                provider_labels: vec![],
                style: None,
            },
            creators: vec![],
            thumbnails: vec![],
            published_at: None,
            updated_at: None,
            duration_seconds: None,
            width: None,
            height: None,
            series_uid: None,
            sequence: None,
            availability: Availability::Unknown,
            provider_metadata: None,
            created_at: "2026-01-01T00:00:00Z".to_string(),
            updated_locally_at: "2026-01-01T00:00:00Z".to_string(),
        };

        upsert_item(&conn, &item).expect("upsert minimal item");
        let fetched = get_item(&conn, &uid)
            .expect("get_item should succeed")
            .expect("item should exist");
        assert_eq!(fetched, item);
    }

    #[test]
    fn availability_round_trips_every_variant() {
        let (_dir, path) = temp_db_path("availability_variants.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        let variants = [
            Availability::Direct,
            Availability::Embedded,
            Availability::Browser,
            Availability::LoginRequired,
            Availability::SubscriptionRequired,
            Availability::TemporarilyUnavailable,
            Availability::Removed,
            Availability::Unknown,
        ];

        for (i, availability) in variants.into_iter().enumerate() {
            let mut item = sample_item(&format!("avail-{i}"));
            item.availability = availability;
            upsert_item(&conn, &item).expect("upsert item");

            let fetched = get_item(&conn, &item.uid).unwrap().unwrap();
            assert_eq!(fetched.availability, availability);
        }
    }

    #[test]
    fn sequence_and_series_uid_round_trip() {
        let (_dir, path) = temp_db_path("sequence.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        // series_uid has a foreign key to series(uid); insert the parent row
        // first (minimal columns only — this module doesn't own `series`).
        conn.execute(
            "INSERT INTO series (uid, provider_id, canonical_url, title) VALUES (?1, ?2, ?3, ?4)",
            params![
                "provider-x:series:1",
                "provider-x",
                "https://example.test/series/1",
                "Sample Series",
            ],
        )
        .expect("insert parent series row");

        let mut item = sample_item("f");
        item.series_uid = Some("provider-x:series:1".to_string());
        item.sequence = Some(Sequence {
            volume: Some(SequenceNumber {
                raw: "2".to_string(),
                numeric: Some(2.0),
                sort_key: "0002".to_string(),
            }),
            chapter: Some(SequenceNumber {
                raw: "10.5".to_string(),
                numeric: Some(10.5),
                sort_key: "0010.5".to_string(),
            }),
            issue: None,
            page_count: Some(24),
        });

        upsert_item(&conn, &item).expect("upsert item with sequence");
        let fetched = get_item(&conn, &item.uid).unwrap().unwrap();
        assert_eq!(fetched, item);
    }
}
