//! Series + Chapters repository (kelpie.md §136 Core Data Layer): Series
//! (§21) and the Chapter level of the Series hierarchy (§20), the `series`
//! and `chapters` tables from `migrations/0002_core_data_layer.sql`.
//!
//! ## Underspecified sub-types (documented interpretive decisions)
//!
//! `kelpie.md` references several sub-structures on `Series` (§21) without
//! ever formally defining their shape (a known spec gap, also called out in
//! the migration's own comment block). This module picks the smallest
//! reasonable shape for each and documents the choice next to the type:
//!
//! - [`LocalizedTitle`] — referenced by `Series.alternateTitles` (§21).
//! - [`CreatorCredit`] — referenced by `Series.creators` (§21) and by
//!   `MediaItem.creators` (§16).
//! - [`ImageResource`] — referenced by `Series.cover` (§21) and
//!   `MediaItem.thumbnails` (§16).
//!
//! [`TagReference`] (§89) and [`SequenceNumber`] (§22) *are* formally defined
//! by the spec and are reproduced here field-for-field.
//!
//! ## Storage shape
//!
//! Per the migration's own schema notes, `alternate_titles`, `creators`,
//! `cover`, and `tags` on `series` are stored as JSON TEXT columns rather
//! than further normalized — `item_tags` is the only tag junction table
//! §104 names, and it covers items only, not series, so series tags stay
//! inline JSON rather than a normalized `series_tags` junction.
//!
//! `chapters.sequence_json` holds a single [`SequenceNumber`] describing the
//! chapter's own position. The table has no separate volume/issue columns,
//! so a chapter that needs to express both (e.g. "Volume 3, Chapter 12")
//! is expected to fold that into `SequenceNumber.raw` /
//! `SequenceNumber.sort_key` rather than needing a richer nested shape —
//! the simplest reasonable choice given the single-column schema.

use rusqlite::{params, Connection, OptionalExtension, Row};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::{Error, Result};

// ---------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------

/// A title in a specific locale (kelpie.md §21 `Series.alternateTitles`).
///
/// Not formally defined by the spec; this is the minimal reasonable shape —
/// a locale tag paired with the title text in that locale.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalizedTitle {
    pub locale: String,
    pub title: String,
}

/// A creator credit (kelpie.md §21 `Series.creators`, also used by
/// `MediaItem.creators` per §16).
///
/// Not formally defined by the spec; this is the minimal reasonable shape —
/// an optional link to a standalone `creators` row (by uid) plus the
/// credited name and role, since a series can credit someone who has no
/// standalone creator record yet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreatorCredit {
    pub uid: Option<String>,
    pub name: String,
    pub role: Option<String>,
}

/// An image resource (kelpie.md §21 `Series.cover`, also used by
/// `MediaItem.thumbnails` per §16).
///
/// Not formally defined by the spec; this is the minimal reasonable shape —
/// a URL plus optional pixel dimensions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageResource {
    pub url: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

/// A tag reference (kelpie.md §89 Tag Normalization), reproduced field for
/// field. Series store these inline as JSON (see module docs) rather than
/// through the `item_tags`-style junction table, which §104 scopes to items
/// only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TagReference {
    pub normalized_id: Option<String>,
    pub provider_id: String,
    pub source_value: String,
    pub display_name: String,
    pub category: Option<String>,
}

/// The closed status union from kelpie.md §21.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SeriesStatus {
    Ongoing,
    Completed,
    Hiatus,
    Cancelled,
    Unknown,
}

impl SeriesStatus {
    fn as_db_str(self) -> &'static str {
        match self {
            SeriesStatus::Ongoing => "ongoing",
            SeriesStatus::Completed => "completed",
            SeriesStatus::Hiatus => "hiatus",
            SeriesStatus::Cancelled => "cancelled",
            SeriesStatus::Unknown => "unknown",
        }
    }

    fn from_db_str(s: &str) -> Result<Self> {
        match s {
            "ongoing" => Ok(SeriesStatus::Ongoing),
            "completed" => Ok(SeriesStatus::Completed),
            "hiatus" => Ok(SeriesStatus::Hiatus),
            "cancelled" => Ok(SeriesStatus::Cancelled),
            "unknown" => Ok(SeriesStatus::Unknown),
            other => Err(conversion_error(format!(
                "invalid series status in database: {other:?}"
            ))),
        }
    }
}

/// The closed reading-direction union from kelpie.md §21.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReadingDirection {
    Ltr,
    Rtl,
    Vertical,
}

impl ReadingDirection {
    fn as_db_str(self) -> &'static str {
        match self {
            ReadingDirection::Ltr => "ltr",
            ReadingDirection::Rtl => "rtl",
            ReadingDirection::Vertical => "vertical",
        }
    }

    fn from_db_str(s: &str) -> Result<Self> {
        match s {
            "ltr" => Ok(ReadingDirection::Ltr),
            "rtl" => Ok(ReadingDirection::Rtl),
            "vertical" => Ok(ReadingDirection::Vertical),
            other => Err(conversion_error(format!(
                "invalid reading direction in database: {other:?}"
            ))),
        }
    }
}

/// The Series object (kelpie.md §21), reproduced field for field where the
/// schema allows (see module docs for the JSON sub-fields' shapes).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Series {
    pub uid: String,
    pub provider_id: String,
    pub remote_id: Option<String>,
    pub canonical_url: String,
    pub title: String,
    pub alternate_titles: Vec<LocalizedTitle>,
    pub description: Option<String>,
    pub creators: Vec<CreatorCredit>,
    pub cover: Option<ImageResource>,
    pub tags: Vec<TagReference>,
    pub status: Option<SeriesStatus>,
    pub reading_direction: Option<ReadingDirection>,
}

/// A sequence number (kelpie.md §22), reproduced field for field. Never
/// assume `numeric` is present or that it round-trips to an integer —
/// `raw` is the label that must stay visible to users.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SequenceNumber {
    pub raw: String,
    pub numeric: Option<f64>,
    pub sort_key: String,
}

/// The Chapter level of the Series hierarchy (kelpie.md §20). `sequence` is
/// the chapter's own position within its series (see module docs on why
/// this is a single `SequenceNumber` rather than separate volume/chapter/
/// issue sub-fields).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Chapter {
    pub uid: String,
    pub series_uid: String,
    pub provider_id: String,
    pub remote_id: Option<String>,
    pub canonical_url: String,
    pub title: Option<String>,
    pub sequence: Option<SequenceNumber>,
    pub page_count: Option<i64>,
    pub published_at: Option<String>,
}

// ---------------------------------------------------------------------
// JSON <-> Error plumbing
// ---------------------------------------------------------------------

/// The crate's `Error` enum (deliberately not touched by this module, per
/// scope) has no JSON-specific variant, so serde errors are wrapped as a
/// `rusqlite::Error`, which `crate::Error`'s `#[from] rusqlite::Error`
/// already covers.
fn conversion_error(message: String) -> Error {
    rusqlite::Error::InvalidParameterName(message).into()
}

fn to_json(value: &impl Serialize) -> Result<String> {
    serde_json::to_string(value)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)).into())
}

fn from_json<T: DeserializeOwned>(raw: &str) -> Result<T> {
    serde_json::from_str(raw)
        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)).into())
}

// ---------------------------------------------------------------------
// Series repository
// ---------------------------------------------------------------------

/// Raw column tuple pulled out of a `series` row inside the `rusqlite`
/// mapping closure; converted to a [`Series`] (JSON decoding, enum parsing)
/// after the closure returns since those conversions produce `crate::Result`,
/// not `rusqlite::Result`.
type SeriesRawRow = (
    String,
    String,
    Option<String>,
    String,
    String,
    String,
    Option<String>,
    String,
    Option<String>,
    String,
    Option<String>,
    Option<String>,
);

fn series_raw_from_row(row: &Row<'_>) -> rusqlite::Result<SeriesRawRow> {
    Ok((
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
        row.get(4)?,
        row.get(5)?,
        row.get(6)?,
        row.get(7)?,
        row.get(8)?,
        row.get(9)?,
        row.get(10)?,
        row.get(11)?,
    ))
}

fn series_from_raw(raw: SeriesRawRow) -> Result<Series> {
    let (
        uid,
        provider_id,
        remote_id,
        canonical_url,
        title,
        alternate_titles_json,
        description,
        creators_json,
        cover_json,
        tags_json,
        status,
        reading_direction,
    ) = raw;

    Ok(Series {
        uid,
        provider_id,
        remote_id,
        canonical_url,
        title,
        alternate_titles: from_json(&alternate_titles_json)?,
        description,
        creators: from_json(&creators_json)?,
        cover: cover_json.as_deref().map(from_json).transpose()?,
        tags: from_json(&tags_json)?,
        status: status
            .as_deref()
            .map(SeriesStatus::from_db_str)
            .transpose()?,
        reading_direction: reading_direction
            .as_deref()
            .map(ReadingDirection::from_db_str)
            .transpose()?,
    })
}

const SERIES_COLUMNS: &str = "uid, provider_id, remote_id, canonical_url, title, \
     alternate_titles_json, description, creators_json, cover_json, tags_json, \
     status, reading_direction";

/// Insert a series, or update it in place (by `uid`) if one already exists.
pub fn upsert_series(conn: &Connection, series: &Series) -> Result<()> {
    let alternate_titles_json = to_json(&series.alternate_titles)?;
    let creators_json = to_json(&series.creators)?;
    let cover_json = series.cover.as_ref().map(to_json).transpose()?;
    let tags_json = to_json(&series.tags)?;
    let status = series.status.map(SeriesStatus::as_db_str);
    let reading_direction = series.reading_direction.map(ReadingDirection::as_db_str);

    conn.execute(
        &format!(
            "INSERT INTO series ({SERIES_COLUMNS}) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12) \
             ON CONFLICT(uid) DO UPDATE SET \
               provider_id = excluded.provider_id, \
               remote_id = excluded.remote_id, \
               canonical_url = excluded.canonical_url, \
               title = excluded.title, \
               alternate_titles_json = excluded.alternate_titles_json, \
               description = excluded.description, \
               creators_json = excluded.creators_json, \
               cover_json = excluded.cover_json, \
               tags_json = excluded.tags_json, \
               status = excluded.status, \
               reading_direction = excluded.reading_direction"
        ),
        params![
            series.uid,
            series.provider_id,
            series.remote_id,
            series.canonical_url,
            series.title,
            alternate_titles_json,
            series.description,
            creators_json,
            cover_json,
            tags_json,
            status,
            reading_direction,
        ],
    )?;
    Ok(())
}

/// Fetch a series by `uid`, or `None` if it doesn't exist.
pub fn get_series(conn: &Connection, uid: &str) -> Result<Option<Series>> {
    let raw = conn
        .query_row(
            &format!("SELECT {SERIES_COLUMNS} FROM series WHERE uid = ?1"),
            params![uid],
            series_raw_from_row,
        )
        .optional()?;
    raw.map(series_from_raw).transpose()
}

/// List series, ordered by title for deterministic pagination, `limit` rows
/// starting at `offset`.
pub fn list_series(conn: &Connection, limit: i64, offset: i64) -> Result<Vec<Series>> {
    let mut stmt = conn.prepare(&format!(
        "SELECT {SERIES_COLUMNS} FROM series ORDER BY title ASC, uid ASC LIMIT ?1 OFFSET ?2"
    ))?;
    let rows = stmt
        .query_map(params![limit, offset], series_raw_from_row)?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    rows.into_iter().map(series_from_raw).collect()
}

/// Delete a series by `uid`. Cascades to its chapters (`ON DELETE CASCADE`
/// on `chapters.series_uid`, per the migration — the caller's connection
/// must have `PRAGMA foreign_keys = ON` for SQLite to actually enforce
/// that). A no-op (not an error) if the uid doesn't exist.
pub fn delete_series(conn: &Connection, uid: &str) -> Result<()> {
    conn.execute("DELETE FROM series WHERE uid = ?1", params![uid])?;
    Ok(())
}

// ---------------------------------------------------------------------
// Chapter repository
// ---------------------------------------------------------------------

type ChapterRawRow = (
    String,
    String,
    String,
    Option<String>,
    String,
    Option<String>,
    Option<String>,
    Option<i64>,
    Option<String>,
);

fn chapter_raw_from_row(row: &Row<'_>) -> rusqlite::Result<ChapterRawRow> {
    Ok((
        row.get(0)?,
        row.get(1)?,
        row.get(2)?,
        row.get(3)?,
        row.get(4)?,
        row.get(5)?,
        row.get(6)?,
        row.get(7)?,
        row.get(8)?,
    ))
}

fn chapter_from_raw(raw: ChapterRawRow) -> Result<Chapter> {
    let (
        uid,
        series_uid,
        provider_id,
        remote_id,
        canonical_url,
        title,
        sequence_json,
        page_count,
        published_at,
    ) = raw;

    Ok(Chapter {
        uid,
        series_uid,
        provider_id,
        remote_id,
        canonical_url,
        title,
        sequence: sequence_json.as_deref().map(from_json).transpose()?,
        page_count,
        published_at,
    })
}

const CHAPTER_COLUMNS: &str = "uid, series_uid, provider_id, remote_id, canonical_url, \
     title, sequence_json, page_count, published_at";

/// Insert a chapter, or update it in place (by `uid`) if one already exists.
pub fn upsert_chapter(conn: &Connection, chapter: &Chapter) -> Result<()> {
    let sequence_json = chapter.sequence.as_ref().map(to_json).transpose()?;

    conn.execute(
        &format!(
            "INSERT INTO chapters ({CHAPTER_COLUMNS}) \
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9) \
             ON CONFLICT(uid) DO UPDATE SET \
               series_uid = excluded.series_uid, \
               provider_id = excluded.provider_id, \
               remote_id = excluded.remote_id, \
               canonical_url = excluded.canonical_url, \
               title = excluded.title, \
               sequence_json = excluded.sequence_json, \
               page_count = excluded.page_count, \
               published_at = excluded.published_at"
        ),
        params![
            chapter.uid,
            chapter.series_uid,
            chapter.provider_id,
            chapter.remote_id,
            chapter.canonical_url,
            chapter.title,
            sequence_json,
            chapter.page_count,
            chapter.published_at,
        ],
    )?;
    Ok(())
}

/// Fetch a chapter by `uid`, or `None` if it doesn't exist.
pub fn get_chapter(conn: &Connection, uid: &str) -> Result<Option<Chapter>> {
    let raw = conn
        .query_row(
            &format!("SELECT {CHAPTER_COLUMNS} FROM chapters WHERE uid = ?1"),
            params![uid],
            chapter_raw_from_row,
        )
        .optional()?;
    raw.map(chapter_from_raw).transpose()
}

/// List every chapter belonging to `series_uid`, ordered by
/// [`SequenceNumber::sort_key`] (chapters with a sequence sort before
/// chapters without one; ties/missing sequences fall back to `uid` for a
/// stable, deterministic order — sorted in Rust after fetching rather than
/// in SQL, since `sort_key` lives inside a JSON blob column).
pub fn list_chapters_for_series(conn: &Connection, series_uid: &str) -> Result<Vec<Chapter>> {
    let mut stmt = conn.prepare(&format!(
        "SELECT {CHAPTER_COLUMNS} FROM chapters WHERE series_uid = ?1"
    ))?;
    let rows = stmt
        .query_map(params![series_uid], chapter_raw_from_row)?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    let mut chapters = rows
        .into_iter()
        .map(chapter_from_raw)
        .collect::<Result<Vec<_>>>()?;

    chapters.sort_by(|a, b| {
        let a_key = a.sequence.as_ref().map(|s| s.sort_key.as_str());
        let b_key = b.sequence.as_ref().map(|s| s.sort_key.as_str());
        match (a_key, b_key) {
            (Some(ak), Some(bk)) => ak.cmp(bk).then_with(|| a.uid.cmp(&b.uid)),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.uid.cmp(&b.uid),
        }
    });

    Ok(chapters)
}

/// Delete a chapter by `uid`. A no-op (not an error) if the uid doesn't
/// exist.
pub fn delete_chapter(conn: &Connection, uid: &str) -> Result<()> {
    conn.execute("DELETE FROM chapters WHERE uid = ?1", params![uid])?;
    Ok(())
}

// ---------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use kelpie_core::identity::stable_id;
    use std::path::PathBuf;

    fn temp_db_path(name: &str) -> (tempfile::TempDir, PathBuf) {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = dir.path().join(name);
        (dir, path)
    }

    fn sample_series(provider_id: &str, remote_id: &str, title: &str) -> Series {
        let canonical_url = format!("https://example.test/series/{remote_id}");
        let uid = stable_id(provider_id, "series", Some(remote_id), &canonical_url);
        Series {
            uid,
            provider_id: provider_id.to_string(),
            remote_id: Some(remote_id.to_string()),
            canonical_url,
            title: title.to_string(),
            alternate_titles: vec![LocalizedTitle {
                locale: "ja".to_string(),
                title: format!("{title} (JA)"),
            }],
            description: Some("A synthetic test series.".to_string()),
            creators: vec![CreatorCredit {
                uid: Some("creator-1".to_string()),
                name: "Test Creator".to_string(),
                role: Some("author".to_string()),
            }],
            cover: Some(ImageResource {
                url: "https://example.test/cover.jpg".to_string(),
                width: Some(600),
                height: Some(900),
            }),
            tags: vec![TagReference {
                normalized_id: Some("tag:fantasy".to_string()),
                provider_id: provider_id.to_string(),
                source_value: "Fantasy".to_string(),
                display_name: "Fantasy".to_string(),
                category: Some("genre".to_string()),
            }],
            status: Some(SeriesStatus::Ongoing),
            reading_direction: Some(ReadingDirection::Rtl),
        }
    }

    fn sample_chapter(series_uid: &str, remote_id: &str, sort_key: &str) -> Chapter {
        let provider_id = "provider-x";
        let canonical_url = format!("https://example.test/chapter/{remote_id}");
        let uid = stable_id(provider_id, "chapter", Some(remote_id), &canonical_url);
        Chapter {
            uid,
            series_uid: series_uid.to_string(),
            provider_id: provider_id.to_string(),
            remote_id: Some(remote_id.to_string()),
            canonical_url,
            title: Some(format!("Chapter {remote_id}")),
            sequence: Some(SequenceNumber {
                raw: remote_id.to_string(),
                numeric: remote_id.parse::<f64>().ok(),
                sort_key: sort_key.to_string(),
            }),
            page_count: Some(20),
            published_at: Some("2026-01-01T00:00:00Z".to_string()),
        }
    }

    #[test]
    fn identity_helper_produces_expected_uid_shape() {
        let series = sample_series("provider-x", "42", "Test Series");
        assert_eq!(series.uid, "provider-x:series:42");

        let chapter = sample_chapter(&series.uid, "5", "005");
        assert_eq!(chapter.uid, "provider-x:chapter:5");
    }

    #[test]
    fn series_upsert_then_get_round_trips() {
        let (_dir, path) = temp_db_path("series_round_trip.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        let series = sample_series("provider-x", "1", "Round Trip Series");
        upsert_series(&conn, &series).expect("upsert series");

        let fetched = get_series(&conn, &series.uid)
            .expect("get series")
            .expect("series should exist");
        assert_eq!(fetched, series);
    }

    #[test]
    fn get_series_missing_returns_none() {
        let (_dir, path) = temp_db_path("series_missing.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        assert_eq!(
            get_series(&conn, "does-not-exist").expect("get series"),
            None
        );
    }

    #[test]
    fn series_upsert_twice_updates_not_duplicates() {
        let (_dir, path) = temp_db_path("series_upsert_twice.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        let mut series = sample_series("provider-x", "1", "Original Title");
        upsert_series(&conn, &series).expect("upsert series");

        series.title = "Updated Title".to_string();
        series.status = Some(SeriesStatus::Completed);
        upsert_series(&conn, &series).expect("upsert series again");

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM series", [], |row| row.get(0))
            .expect("count series");
        assert_eq!(count, 1, "upsert must not duplicate rows");

        let fetched = get_series(&conn, &series.uid)
            .expect("get series")
            .expect("series should exist");
        assert_eq!(fetched.title, "Updated Title");
        assert_eq!(fetched.status, Some(SeriesStatus::Completed));
    }

    #[test]
    fn series_with_no_optional_fields_round_trips() {
        let (_dir, path) = temp_db_path("series_minimal.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        let canonical_url = "https://example.test/series/no-remote-id".to_string();
        let uid = stable_id("provider-x", "series", None, &canonical_url);
        let series = Series {
            uid: uid.clone(),
            provider_id: "provider-x".to_string(),
            remote_id: None,
            canonical_url,
            title: "Minimal Series".to_string(),
            alternate_titles: Vec::new(),
            description: None,
            creators: Vec::new(),
            cover: None,
            tags: Vec::new(),
            status: None,
            reading_direction: None,
        };
        upsert_series(&conn, &series).expect("upsert series");

        let fetched = get_series(&conn, &uid)
            .expect("get series")
            .expect("series should exist");
        assert_eq!(fetched, series);
    }

    #[test]
    fn list_series_orders_by_title_and_paginates() {
        let (_dir, path) = temp_db_path("series_list.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        upsert_series(&conn, &sample_series("provider-x", "3", "Charlie")).unwrap();
        upsert_series(&conn, &sample_series("provider-x", "1", "Alpha")).unwrap();
        upsert_series(&conn, &sample_series("provider-x", "2", "Bravo")).unwrap();

        let all = list_series(&conn, 10, 0).expect("list series");
        let titles: Vec<&str> = all.iter().map(|s| s.title.as_str()).collect();
        assert_eq!(titles, vec!["Alpha", "Bravo", "Charlie"]);

        let page = list_series(&conn, 1, 1).expect("list series page");
        assert_eq!(page.len(), 1);
        assert_eq!(page[0].title, "Bravo");
    }

    #[test]
    fn delete_series_removes_row() {
        let (_dir, path) = temp_db_path("series_delete.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        let series = sample_series("provider-x", "1", "Deletable Series");
        upsert_series(&conn, &series).expect("upsert series");
        delete_series(&conn, &series.uid).expect("delete series");

        assert_eq!(get_series(&conn, &series.uid).expect("get series"), None);
    }

    #[test]
    fn delete_series_missing_is_not_an_error() {
        let (_dir, path) = temp_db_path("series_delete_missing.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        delete_series(&conn, "does-not-exist").expect("delete missing series should be a no-op");
    }

    #[test]
    fn chapter_upsert_then_get_round_trips() {
        let (_dir, path) = temp_db_path("chapter_round_trip.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        let series = sample_series("provider-x", "1", "Parent Series");
        upsert_series(&conn, &series).expect("upsert series");

        let chapter = sample_chapter(&series.uid, "1", "001");
        upsert_chapter(&conn, &chapter).expect("upsert chapter");

        let fetched = get_chapter(&conn, &chapter.uid)
            .expect("get chapter")
            .expect("chapter should exist");
        assert_eq!(fetched, chapter);
    }

    #[test]
    fn get_chapter_missing_returns_none() {
        let (_dir, path) = temp_db_path("chapter_missing.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        assert_eq!(
            get_chapter(&conn, "does-not-exist").expect("get chapter"),
            None
        );
    }

    #[test]
    fn chapter_upsert_twice_updates_not_duplicates() {
        let (_dir, path) = temp_db_path("chapter_upsert_twice.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        let series = sample_series("provider-x", "1", "Parent Series");
        upsert_series(&conn, &series).expect("upsert series");

        let mut chapter = sample_chapter(&series.uid, "1", "001");
        upsert_chapter(&conn, &chapter).expect("upsert chapter");

        chapter.title = Some("Updated Chapter Title".to_string());
        chapter.page_count = Some(30);
        upsert_chapter(&conn, &chapter).expect("upsert chapter again");

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM chapters", [], |row| row.get(0))
            .expect("count chapters");
        assert_eq!(count, 1, "upsert must not duplicate rows");

        let fetched = get_chapter(&conn, &chapter.uid)
            .expect("get chapter")
            .expect("chapter should exist");
        assert_eq!(fetched.title, Some("Updated Chapter Title".to_string()));
        assert_eq!(fetched.page_count, Some(30));
    }

    #[test]
    fn chapter_with_no_sequence_round_trips() {
        let (_dir, path) = temp_db_path("chapter_no_sequence.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        let series = sample_series("provider-x", "1", "Parent Series");
        upsert_series(&conn, &series).expect("upsert series");

        let canonical_url = "https://example.test/chapter/no-sequence".to_string();
        let uid = stable_id("provider-x", "chapter", None, &canonical_url);
        let chapter = Chapter {
            uid: uid.clone(),
            series_uid: series.uid.clone(),
            provider_id: "provider-x".to_string(),
            remote_id: None,
            canonical_url,
            title: None,
            sequence: None,
            page_count: None,
            published_at: None,
        };
        upsert_chapter(&conn, &chapter).expect("upsert chapter");

        let fetched = get_chapter(&conn, &uid)
            .expect("get chapter")
            .expect("chapter should exist");
        assert_eq!(fetched, chapter);
    }

    #[test]
    fn list_chapters_for_series_orders_by_sort_key_and_filters_by_series() {
        let (_dir, path) = temp_db_path("chapters_list.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        let series_a = sample_series("provider-x", "1", "Series A");
        let series_b = sample_series("provider-x", "2", "Series B");
        upsert_series(&conn, &series_a).expect("upsert series a");
        upsert_series(&conn, &series_b).expect("upsert series b");

        // Out of insertion order on purpose, to prove sorting happens by
        // sort_key rather than insertion or uid order.
        upsert_chapter(&conn, &sample_chapter(&series_a.uid, "10", "010")).unwrap();
        upsert_chapter(&conn, &sample_chapter(&series_a.uid, "2", "002")).unwrap();
        upsert_chapter(&conn, &sample_chapter(&series_a.uid, "1", "001")).unwrap();
        // A chapter with no sequence should sort after all sequenced ones.
        let no_sequence_uid = stable_id(
            "provider-x",
            "chapter",
            Some("special"),
            "https://example.test/chapter/special",
        );
        upsert_chapter(
            &conn,
            &Chapter {
                uid: no_sequence_uid.clone(),
                series_uid: series_a.uid.clone(),
                provider_id: "provider-x".to_string(),
                remote_id: Some("special".to_string()),
                canonical_url: "https://example.test/chapter/special".to_string(),
                title: Some("Special".to_string()),
                sequence: None,
                page_count: None,
                published_at: None,
            },
        )
        .unwrap();
        // Belongs to series_b, must not show up when listing series_a. Uses a
        // remote_id distinct from series_a's chapters: per §17, remote_id is
        // scoped to the provider (stable_id doesn't mix in series identity
        // when a remote_id is present), so two real chapters can only share
        // a remote_id if the provider itself reused one -- not something to
        // simulate here without changing what this test is exercising.
        upsert_chapter(&conn, &sample_chapter(&series_b.uid, "999", "999")).unwrap();

        let chapters = list_chapters_for_series(&conn, &series_a.uid).expect("list chapters");
        let remote_ids: Vec<Option<String>> =
            chapters.iter().map(|c| c.remote_id.clone()).collect();
        assert_eq!(
            remote_ids,
            vec![
                Some("1".to_string()),
                Some("2".to_string()),
                Some("10".to_string()),
                Some("special".to_string()),
            ]
        );
        assert!(chapters.iter().all(|c| c.series_uid == series_a.uid));
    }

    #[test]
    fn delete_chapter_removes_row() {
        let (_dir, path) = temp_db_path("chapter_delete.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        let series = sample_series("provider-x", "1", "Parent Series");
        upsert_series(&conn, &series).expect("upsert series");
        let chapter = sample_chapter(&series.uid, "1", "001");
        upsert_chapter(&conn, &chapter).expect("upsert chapter");

        delete_chapter(&conn, &chapter.uid).expect("delete chapter");
        assert_eq!(get_chapter(&conn, &chapter.uid).expect("get chapter"), None);
    }

    #[test]
    fn delete_chapter_missing_is_not_an_error() {
        let (_dir, path) = temp_db_path("chapter_delete_missing.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        delete_chapter(&conn, "does-not-exist").expect("delete missing chapter should be a no-op");
    }

    #[test]
    fn deleting_series_cascades_to_its_chapters() {
        let (_dir, path) = temp_db_path("series_cascade.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        let series = sample_series("provider-x", "1", "Cascade Series");
        upsert_series(&conn, &series).expect("upsert series");
        let chapter = sample_chapter(&series.uid, "1", "001");
        upsert_chapter(&conn, &chapter).expect("upsert chapter");

        // ON DELETE CASCADE requires the foreign_keys pragma to be on;
        // SQLite defaults it off per-connection unless enabled, and
        // `open_at` doesn't turn it on explicitly (a bootstrap concern out
        // of this module's scope), so enable it directly for this test to
        // exercise the cascade the schema declares.
        conn.execute("PRAGMA foreign_keys = ON", []).unwrap();

        delete_series(&conn, &series.uid).expect("delete series");

        assert_eq!(get_series(&conn, &series.uid).expect("get series"), None);
        assert_eq!(
            get_chapter(&conn, &chapter.uid).expect("get chapter"),
            None,
            "deleting a series should cascade-delete its chapters"
        );
    }

    #[test]
    fn series_status_and_reading_direction_round_trip_all_variants() {
        let (_dir, path) = temp_db_path("series_enums.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        let statuses = [
            SeriesStatus::Ongoing,
            SeriesStatus::Completed,
            SeriesStatus::Hiatus,
            SeriesStatus::Cancelled,
            SeriesStatus::Unknown,
        ];
        let directions = [
            ReadingDirection::Ltr,
            ReadingDirection::Rtl,
            ReadingDirection::Vertical,
        ];

        for (i, (status, direction)) in statuses.iter().zip(directions.iter().cycle()).enumerate() {
            let mut series = sample_series("provider-x", &i.to_string(), &format!("Series {i}"));
            series.status = Some(*status);
            series.reading_direction = Some(*direction);
            upsert_series(&conn, &series).expect("upsert series");

            let fetched = get_series(&conn, &series.uid)
                .expect("get series")
                .expect("series should exist");
            assert_eq!(fetched.status, Some(*status));
            assert_eq!(fetched.reading_direction, Some(*direction));
        }
    }

    #[test]
    fn temp_check_cascade_without_pragma() {
        let (_dir, path) = temp_db_path("series_cascade_no_pragma.sqlite");
        let conn = crate::open_at(&path).expect("open db");

        let fk: i64 = conn
            .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
            .unwrap();
        eprintln!("foreign_keys pragma value = {fk}");

        let series = sample_series("provider-x", "1", "Cascade Series");
        upsert_series(&conn, &series).expect("upsert series");
        let chapter = sample_chapter(&series.uid, "1", "001");
        upsert_chapter(&conn, &chapter).expect("upsert chapter");

        delete_series(&conn, &series.uid).expect("delete series");

        assert_eq!(get_series(&conn, &series.uid).expect("get series"), None);
        assert_eq!(
            get_chapter(&conn, &chapter.uid).expect("get chapter"),
            None,
            "EXPECTED TO FAIL: chapter should be orphaned since foreign_keys pragma is off by default"
        );
    }
}
