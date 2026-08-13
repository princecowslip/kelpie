//! `Series` repository (kelpie.md §136 Phase 2, §20-21 Series Model / Series
//! Object). Backs migration `0003_series.sql`.
//!
//! `alternate_titles`, `creators`, `cover`, and `tags` are stored as JSON TEXT
//! columns (via `serde_json`) rather than normalized into their own tables — the
//! pragmatic Phase 2 approach for this entity. `status` and `reading_direction`
//! are stored as plain lowercase TEXT (e.g. `"ongoing"`) matching the wire values
//! from kelpie.md §21.
//!
//! Repositories take `&rusqlite::Connection` per call, matching the connection
//! lifecycle established by [`crate::bootstrap`] / [`crate::open_at`].

use kelpie_core::domain::series::{ReadingDirection, Series, SeriesStatus};
use rusqlite::{params, Connection, Row};

/// Errors that can occur while reading or writing `Series` rows.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("failed to (de)serialize series JSON column: {0}")]
    Json(#[from] serde_json::Error),

    #[error("invalid stored series status: {0}")]
    InvalidStatus(String),

    #[error("invalid stored series reading direction: {0}")]
    InvalidReadingDirection(String),
}

pub type Result<T> = std::result::Result<T, Error>;

const SELECT_COLUMNS: &str = "uid, provider_id, remote_id, canonical_url, title, \
    alternate_titles, description, creators, cover, tags, status, reading_direction";

/// Insert a new `Series` row.
pub fn create(conn: &Connection, series: &Series) -> Result<()> {
    let bound = params_from(series)?;
    conn.execute(
        "INSERT INTO series (\
            uid, provider_id, remote_id, canonical_url, title, alternate_titles, \
            description, creators, cover, tags, status, reading_direction\
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        bound.as_slice_params(),
    )?;
    Ok(())
}

/// Fetch a `Series` by its `uid`, or `None` if no such row exists.
pub fn get_by_uid(conn: &Connection, uid: &str) -> Result<Option<Series>> {
    let mut stmt = conn.prepare(&format!(
        "SELECT {SELECT_COLUMNS} FROM series WHERE uid = ?1"
    ))?;
    let mut rows = stmt.query(params![uid])?;
    match rows.next()? {
        Some(row) => Ok(Some(row_to_series(row)?)),
        None => Ok(None),
    }
}

/// Overwrite every column of an existing `Series` row, matched by `uid`.
pub fn update(conn: &Connection, series: &Series) -> Result<()> {
    let bound = params_from(series)?;
    conn.execute(
        "UPDATE series SET \
            provider_id = ?2, remote_id = ?3, canonical_url = ?4, title = ?5, \
            alternate_titles = ?6, description = ?7, creators = ?8, cover = ?9, \
            tags = ?10, status = ?11, reading_direction = ?12 \
         WHERE uid = ?1",
        bound.as_slice_params(),
    )?;
    Ok(())
}

/// Delete a `Series` row by `uid`. A no-op (not an error) if no such row exists.
pub fn delete(conn: &Connection, uid: &str) -> Result<()> {
    conn.execute("DELETE FROM series WHERE uid = ?1", params![uid])?;
    Ok(())
}

/// List every `Series` row for a given `providerId`, ordered by `uid`.
pub fn list_by_provider(conn: &Connection, provider_id: &str) -> Result<Vec<Series>> {
    let mut stmt = conn.prepare(&format!(
        "SELECT {SELECT_COLUMNS} FROM series WHERE provider_id = ?1 ORDER BY uid"
    ))?;
    let rows = stmt.query_map(params![provider_id], raw_row)?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row_to_series_from_raw(row?)?);
    }
    Ok(result)
}

/// The raw, unparsed column values for one `series` row — every column read out
/// as a plain SQLite-native type, before any JSON decoding or enum parsing.
struct RawRow {
    uid: String,
    provider_id: String,
    remote_id: Option<String>,
    canonical_url: String,
    title: String,
    alternate_titles: String,
    description: Option<String>,
    creators: String,
    cover: Option<String>,
    tags: String,
    status: Option<String>,
    reading_direction: Option<String>,
}

fn raw_row(row: &Row<'_>) -> rusqlite::Result<RawRow> {
    Ok(RawRow {
        uid: row.get(0)?,
        provider_id: row.get(1)?,
        remote_id: row.get(2)?,
        canonical_url: row.get(3)?,
        title: row.get(4)?,
        alternate_titles: row.get(5)?,
        description: row.get(6)?,
        creators: row.get(7)?,
        cover: row.get(8)?,
        tags: row.get(9)?,
        status: row.get(10)?,
        reading_direction: row.get(11)?,
    })
}

fn row_to_series(row: &Row<'_>) -> Result<Series> {
    row_to_series_from_raw(raw_row(row)?)
}

fn row_to_series_from_raw(raw: RawRow) -> Result<Series> {
    Ok(Series {
        uid: raw.uid,
        provider_id: raw.provider_id,
        remote_id: raw.remote_id,
        canonical_url: raw.canonical_url,
        title: raw.title,
        alternate_titles: serde_json::from_str(&raw.alternate_titles)?,
        description: raw.description,
        creators: serde_json::from_str(&raw.creators)?,
        cover: raw.cover.map(|c| serde_json::from_str(&c)).transpose()?,
        tags: serde_json::from_str(&raw.tags)?,
        status: raw.status.map(|s| parse_status(&s)).transpose()?,
        reading_direction: raw
            .reading_direction
            .map(|s| parse_reading_direction(&s))
            .transpose()?,
    })
}

/// Owned, already-serialized bind values for an INSERT/UPDATE, so `create` and
/// `update` share exactly the same JSON/enum encoding.
struct BoundParams {
    uid: String,
    provider_id: String,
    remote_id: Option<String>,
    canonical_url: String,
    title: String,
    alternate_titles: String,
    description: Option<String>,
    creators: String,
    cover: Option<String>,
    tags: String,
    status: Option<&'static str>,
    reading_direction: Option<&'static str>,
}

impl BoundParams {
    fn as_slice_params(&self) -> [&dyn rusqlite::ToSql; 12] {
        [
            &self.uid,
            &self.provider_id,
            &self.remote_id,
            &self.canonical_url,
            &self.title,
            &self.alternate_titles,
            &self.description,
            &self.creators,
            &self.cover,
            &self.tags,
            &self.status,
            &self.reading_direction,
        ]
    }
}

fn params_from(series: &Series) -> Result<BoundParams> {
    Ok(BoundParams {
        uid: series.uid.clone(),
        provider_id: series.provider_id.clone(),
        remote_id: series.remote_id.clone(),
        canonical_url: series.canonical_url.clone(),
        title: series.title.clone(),
        alternate_titles: serde_json::to_string(&series.alternate_titles)?,
        description: series.description.clone(),
        creators: serde_json::to_string(&series.creators)?,
        cover: series
            .cover
            .as_ref()
            .map(serde_json::to_string)
            .transpose()?,
        tags: serde_json::to_string(&series.tags)?,
        status: series.status.map(status_to_str),
        reading_direction: series.reading_direction.map(reading_direction_to_str),
    })
}

fn status_to_str(status: SeriesStatus) -> &'static str {
    match status {
        SeriesStatus::Ongoing => "ongoing",
        SeriesStatus::Completed => "completed",
        SeriesStatus::Hiatus => "hiatus",
        SeriesStatus::Cancelled => "cancelled",
        SeriesStatus::Unknown => "unknown",
    }
}

fn parse_status(raw: &str) -> Result<SeriesStatus> {
    match raw {
        "ongoing" => Ok(SeriesStatus::Ongoing),
        "completed" => Ok(SeriesStatus::Completed),
        "hiatus" => Ok(SeriesStatus::Hiatus),
        "cancelled" => Ok(SeriesStatus::Cancelled),
        "unknown" => Ok(SeriesStatus::Unknown),
        other => Err(Error::InvalidStatus(other.to_string())),
    }
}

fn reading_direction_to_str(direction: ReadingDirection) -> &'static str {
    match direction {
        ReadingDirection::Ltr => "ltr",
        ReadingDirection::Rtl => "rtl",
        ReadingDirection::Vertical => "vertical",
    }
}

fn parse_reading_direction(raw: &str) -> Result<ReadingDirection> {
    match raw {
        "ltr" => Ok(ReadingDirection::Ltr),
        "rtl" => Ok(ReadingDirection::Rtl),
        "vertical" => Ok(ReadingDirection::Vertical),
        other => Err(Error::InvalidReadingDirection(other.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kelpie_core::domain::{CreatorCredit, ImageResource, LocalizedTitle, TagReference};

    fn sample_series(uid: &str) -> Series {
        Series {
            uid: uid.to_string(),
            provider_id: "fake-provider".to_string(),
            remote_id: Some("remote-123".to_string()),
            canonical_url: "https://example.invalid/series/123".to_string(),
            title: "Sample Series".to_string(),
            alternate_titles: vec![LocalizedTitle {
                locale: "ja".to_string(),
                value: "サンプルシリーズ".to_string(),
            }],
            description: Some("A sample series for tests.".to_string()),
            creators: vec![CreatorCredit {
                name: "Jane Author".to_string(),
                role: Some("writer".to_string()),
                provider_id: Some("fake-provider".to_string()),
                url: None,
            }],
            cover: Some(ImageResource {
                url: "https://example.invalid/cover.jpg".to_string(),
                width: Some(600),
                height: Some(900),
            }),
            tags: vec![TagReference {
                normalized_id: Some("tag:romance".to_string()),
                provider_id: "fake-provider".to_string(),
                source_value: "Romance".to_string(),
                display_name: "Romance".to_string(),
                category: Some("genre".to_string()),
            }],
            status: Some(SeriesStatus::Ongoing),
            reading_direction: Some(ReadingDirection::Rtl),
        }
    }

    #[test]
    fn create_get_update_delete_round_trip() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = dir.path().join("series.sqlite");
        let conn = crate::open_at(&path).expect("open database");

        let series = sample_series("series-1");
        create(&conn, &series).expect("create series");

        let fetched = get_by_uid(&conn, "series-1")
            .expect("get_by_uid should succeed")
            .expect("series should exist");
        assert_eq!(fetched.uid, series.uid);
        assert_eq!(fetched.provider_id, series.provider_id);
        assert_eq!(fetched.remote_id, series.remote_id);
        assert_eq!(fetched.canonical_url, series.canonical_url);
        assert_eq!(fetched.title, series.title);
        assert_eq!(fetched.description, series.description);
        assert_eq!(fetched.status, series.status);
        assert_eq!(fetched.reading_direction, series.reading_direction);
        assert_eq!(fetched.alternate_titles.len(), 1);
        assert_eq!(fetched.alternate_titles[0].locale, "ja");
        assert_eq!(fetched.tags.len(), 1);
        assert_eq!(fetched.tags[0].display_name, "Romance");
        assert_eq!(fetched.creators.len(), 1);
        assert_eq!(fetched.cover.as_ref().unwrap().width, Some(600));

        let mut updated = fetched;
        updated.title = "Updated Title".to_string();
        updated.status = Some(SeriesStatus::Completed);
        updated.tags.clear();
        update(&conn, &updated).expect("update series");

        let refetched = get_by_uid(&conn, "series-1")
            .expect("get_by_uid should succeed")
            .expect("series should still exist after update");
        assert_eq!(refetched.title, "Updated Title");
        assert_eq!(refetched.status, Some(SeriesStatus::Completed));
        assert!(refetched.tags.is_empty());

        delete(&conn, "series-1").expect("delete series");
        let gone = get_by_uid(&conn, "series-1").expect("get_by_uid should succeed");
        assert!(gone.is_none());
    }

    #[test]
    fn get_by_uid_returns_none_for_missing_row() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = dir.path().join("series.sqlite");
        let conn = crate::open_at(&path).expect("open database");

        let result = get_by_uid(&conn, "does-not-exist").expect("get_by_uid should succeed");
        assert!(result.is_none());
    }

    #[test]
    fn list_by_provider_filters_and_orders_by_uid() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = dir.path().join("series.sqlite");
        let conn = crate::open_at(&path).expect("open database");

        let mut a = sample_series("series-b");
        a.provider_id = "provider-a".to_string();
        let mut b = sample_series("series-a");
        b.provider_id = "provider-a".to_string();
        let mut other = sample_series("series-c");
        other.provider_id = "provider-b".to_string();

        create(&conn, &a).expect("create a");
        create(&conn, &b).expect("create b");
        create(&conn, &other).expect("create other");

        let listed = list_by_provider(&conn, "provider-a").expect("list_by_provider");
        let uids: Vec<&str> = listed.iter().map(|s| s.uid.as_str()).collect();
        assert_eq!(uids, vec!["series-a", "series-b"]);
    }

    #[test]
    fn minimal_series_with_no_optional_fields_round_trips() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = dir.path().join("series.sqlite");
        let conn = crate::open_at(&path).expect("open database");

        let series = Series {
            uid: "minimal".to_string(),
            provider_id: "fake-provider".to_string(),
            remote_id: None,
            canonical_url: "https://example.invalid/series/minimal".to_string(),
            title: "Minimal Series".to_string(),
            alternate_titles: Vec::new(),
            description: None,
            creators: Vec::new(),
            cover: None,
            tags: Vec::new(),
            status: None,
            reading_direction: None,
        };
        create(&conn, &series).expect("create minimal series");

        let fetched = get_by_uid(&conn, "minimal")
            .expect("get_by_uid should succeed")
            .expect("series should exist");
        assert_eq!(fetched.remote_id, None);
        assert_eq!(fetched.description, None);
        assert!(fetched.alternate_titles.is_empty());
        assert!(fetched.creators.is_empty());
        assert!(fetched.tags.is_empty());
        assert!(fetched.cover.is_none());
        assert_eq!(fetched.status, None);
        assert_eq!(fetched.reading_direction, None);
    }
}
