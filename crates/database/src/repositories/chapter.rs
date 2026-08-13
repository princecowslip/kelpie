//! `Chapter` repository (kelpie.md §136 Phase 2). Backs migration `0004_chapters.sql`.
//!
//! `sequence` / `volume_sequence` are stored as JSON TEXT (serde_json-encoded
//! [`kelpie_core::domain::SequenceNumber`]); `sequence`'s `sort_key` is additionally
//! denormalized into the `sequence_sort_key` column so `list_chapters_by_series` can
//! order by it in SQL without decoding JSON per row. `image` on `pages` is stored as
//! JSON TEXT (serde_json-encoded [`kelpie_core::domain::ImageResource`]).
//!
//! `pages.chapter_uid REFERENCES chapters(uid)` has no `ON DELETE CASCADE`, so
//! [`delete_chapter`] explicitly deletes a chapter's pages in the same transaction
//! that deletes the chapter row, rather than relying on a cascade. `chapters.series_uid`
//! deliberately has no `REFERENCES` clause at all — see `migrations/0004_chapters.sql`
//! for why (this SQLite build enforces `PRAGMA foreign_keys` by default, and a chapter
//! must be insertable without requiring the `series` migration/entity to exist yet).

use kelpie_core::domain::chapter::{Chapter, Page};
use kelpie_core::domain::{ImageResource, SequenceNumber};
use rusqlite::{params, Connection, OptionalExtension};

/// Errors that can occur while reading or writing chapters/pages.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("failed to (de)serialize JSON column: {0}")]
    Json(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Insert a new chapter row.
pub fn create_chapter(conn: &Connection, chapter: &Chapter) -> Result<()> {
    let sequence_json = serde_json::to_string(&chapter.sequence)?;
    let volume_sequence_json = chapter
        .volume_sequence
        .as_ref()
        .map(serde_json::to_string)
        .transpose()?;

    conn.execute(
        "INSERT INTO chapters (
            uid, series_uid, provider_id, remote_id, canonical_url, title,
            sequence, sequence_sort_key, volume_sequence, published_at, page_count
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![
            chapter.uid,
            chapter.series_uid,
            chapter.provider_id,
            chapter.remote_id,
            chapter.canonical_url,
            chapter.title,
            sequence_json,
            chapter.sequence.sort_key,
            volume_sequence_json,
            chapter.published_at,
            chapter.page_count,
        ],
    )?;
    Ok(())
}

/// Fetch a chapter by its `uid`. Returns `Ok(None)` if no such chapter exists.
pub fn get_chapter_by_uid(conn: &Connection, uid: &str) -> Result<Option<Chapter>> {
    conn.query_row(
        "SELECT uid, series_uid, provider_id, remote_id, canonical_url, title,
                sequence, volume_sequence, published_at, page_count
         FROM chapters WHERE uid = ?1",
        params![uid],
        row_to_chapter,
    )
    .optional()
    .map_err(Error::from)?
    .transpose()
}

/// Overwrite every mutable field of an existing chapter, keyed by `chapter.uid`.
pub fn update_chapter(conn: &Connection, chapter: &Chapter) -> Result<()> {
    let sequence_json = serde_json::to_string(&chapter.sequence)?;
    let volume_sequence_json = chapter
        .volume_sequence
        .as_ref()
        .map(serde_json::to_string)
        .transpose()?;

    conn.execute(
        "UPDATE chapters SET
            series_uid = ?2,
            provider_id = ?3,
            remote_id = ?4,
            canonical_url = ?5,
            title = ?6,
            sequence = ?7,
            sequence_sort_key = ?8,
            volume_sequence = ?9,
            published_at = ?10,
            page_count = ?11
         WHERE uid = ?1",
        params![
            chapter.uid,
            chapter.series_uid,
            chapter.provider_id,
            chapter.remote_id,
            chapter.canonical_url,
            chapter.title,
            sequence_json,
            chapter.sequence.sort_key,
            volume_sequence_json,
            chapter.published_at,
            chapter.page_count,
        ],
    )?;
    Ok(())
}

/// Delete a chapter by `uid`, along with all of its pages. Both deletes run inside
/// one transaction, since `pages.chapter_uid` has no `ON DELETE CASCADE` (see the
/// module doc comment) and so cannot be relied on to cascade the delete.
pub fn delete_chapter(conn: &mut Connection, uid: &str) -> Result<()> {
    let tx = conn.transaction()?;
    tx.execute("DELETE FROM pages WHERE chapter_uid = ?1", params![uid])?;
    tx.execute("DELETE FROM chapters WHERE uid = ?1", params![uid])?;
    tx.commit()?;
    Ok(())
}

/// List every chapter belonging to `series_uid`, ordered by the chapter's
/// `sequence.sort_key` (kelpie.md §22 Sequence Numbers).
pub fn list_chapters_by_series(conn: &Connection, series_uid: &str) -> Result<Vec<Chapter>> {
    let mut stmt = conn.prepare(
        "SELECT uid, series_uid, provider_id, remote_id, canonical_url, title,
                sequence, volume_sequence, published_at, page_count
         FROM chapters
         WHERE series_uid = ?1
         ORDER BY sequence_sort_key ASC",
    )?;
    let chapters = stmt
        .query_map(params![series_uid], row_to_chapter)?
        .collect::<rusqlite::Result<Vec<_>>>()?
        .into_iter()
        .collect::<Result<Vec<_>>>()?;
    Ok(chapters)
}

/// Replace the full ordered page list of a chapter with `pages`. Existing pages for
/// `chapter_uid` are deleted first, then `pages` is inserted as-is (its elements'
/// `chapter_uid` and `page_index` are used verbatim, so callers are responsible for
/// setting them consistently).
pub fn set_pages(conn: &mut Connection, chapter_uid: &str, pages: &[Page]) -> Result<()> {
    let tx = conn.transaction()?;
    tx.execute(
        "DELETE FROM pages WHERE chapter_uid = ?1",
        params![chapter_uid],
    )?;
    for page in pages {
        let image_json = serde_json::to_string(&page.image)?;
        tx.execute(
            "INSERT INTO pages (chapter_uid, page_index, image) VALUES (?1, ?2, ?3)",
            params![page.chapter_uid, page.page_index, image_json],
        )?;
    }
    tx.commit()?;
    Ok(())
}

/// Insert or replace a single page.
pub fn add_page(conn: &Connection, page: &Page) -> Result<()> {
    let image_json = serde_json::to_string(&page.image)?;
    conn.execute(
        "INSERT INTO pages (chapter_uid, page_index, image) VALUES (?1, ?2, ?3)
         ON CONFLICT (chapter_uid, page_index) DO UPDATE SET image = excluded.image",
        params![page.chapter_uid, page.page_index, image_json],
    )?;
    Ok(())
}

/// List every page of `chapter_uid`, ordered by `page_index` ascending.
pub fn list_pages_by_chapter(conn: &Connection, chapter_uid: &str) -> Result<Vec<Page>> {
    let mut stmt = conn.prepare(
        "SELECT chapter_uid, page_index, image FROM pages
         WHERE chapter_uid = ?1
         ORDER BY page_index ASC",
    )?;
    let pages = stmt
        .query_map(params![chapter_uid], row_to_page)?
        .collect::<rusqlite::Result<Vec<_>>>()?
        .into_iter()
        .collect::<Result<Vec<_>>>()?;
    Ok(pages)
}

/// Decode a `chapters` row into a [`Chapter`], deserializing its JSON columns.
fn row_to_chapter(row: &rusqlite::Row<'_>) -> rusqlite::Result<Result<Chapter>> {
    let sequence_json: String = row.get(6)?;
    let volume_sequence_json: Option<String> = row.get(7)?;

    Ok((|| -> Result<Chapter> {
        let sequence: SequenceNumber = serde_json::from_str(&sequence_json)?;
        let volume_sequence = volume_sequence_json
            .map(|json| serde_json::from_str(&json))
            .transpose()?;

        Ok(Chapter {
            uid: row.get(0)?,
            series_uid: row.get(1)?,
            provider_id: row.get(2)?,
            remote_id: row.get(3)?,
            canonical_url: row.get(4)?,
            title: row.get(5)?,
            sequence,
            volume_sequence,
            published_at: row.get(8)?,
            page_count: row.get(9)?,
        })
    })())
}

/// Decode a `pages` row into a [`Page`], deserializing its JSON `image` column.
fn row_to_page(row: &rusqlite::Row<'_>) -> rusqlite::Result<Result<Page>> {
    let image_json: String = row.get(2)?;

    Ok((|| -> Result<Page> {
        let image: ImageResource = serde_json::from_str(&image_json)?;
        Ok(Page {
            chapter_uid: row.get(0)?,
            page_index: row.get(1)?,
            image,
        })
    })())
}

#[cfg(test)]
mod tests {
    use super::*;
    use kelpie_core::domain::ImageResource;

    fn sample_chapter(uid: &str, series_uid: &str, raw: &str, sort_key: &str) -> Chapter {
        Chapter {
            uid: uid.to_string(),
            series_uid: series_uid.to_string(),
            provider_id: "fake-provider".to_string(),
            remote_id: Some("remote-42".to_string()),
            canonical_url: format!("https://example.invalid/series/{series_uid}/{uid}"),
            title: Some("Chapter Title".to_string()),
            sequence: SequenceNumber {
                raw: raw.to_string(),
                numeric: raw.parse::<f64>().ok(),
                sort_key: sort_key.to_string(),
            },
            volume_sequence: Some(SequenceNumber {
                raw: "1".to_string(),
                numeric: Some(1.0),
                sort_key: "00001".to_string(),
            }),
            published_at: Some("2026-01-01T00:00:00Z".to_string()),
            page_count: Some(3),
        }
    }

    fn sample_page(chapter_uid: &str, page_index: u32) -> Page {
        Page {
            chapter_uid: chapter_uid.to_string(),
            page_index,
            image: ImageResource {
                url: format!("https://example.invalid/pages/{chapter_uid}/{page_index}.jpg"),
                width: Some(800),
                height: Some(1200),
            },
        }
    }

    #[test]
    fn create_get_update_delete_chapter_round_trips() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let mut conn =
            crate::open_at(dir.path().join("chapter.sqlite")).expect("bootstrap database");

        let series_uid = "series-made-up-1";
        let chapter = sample_chapter("chapter-1", series_uid, "10", "00010");
        create_chapter(&conn, &chapter).expect("create chapter");

        let fetched = get_chapter_by_uid(&conn, "chapter-1")
            .expect("get chapter")
            .expect("chapter should exist");
        assert_eq!(fetched.uid, chapter.uid);
        assert_eq!(fetched.series_uid, series_uid);
        assert_eq!(fetched.title.as_deref(), Some("Chapter Title"));
        assert_eq!(fetched.sequence.raw, "10");
        assert_eq!(fetched.sequence.numeric, Some(10.0));
        assert_eq!(fetched.sequence.sort_key, "00010");
        assert_eq!(fetched.volume_sequence.unwrap().raw, "1");
        assert_eq!(fetched.page_count, Some(3));

        let mut updated = chapter.clone();
        updated.title = Some("Updated Title".to_string());
        updated.page_count = Some(5);
        update_chapter(&conn, &updated).expect("update chapter");

        let refetched = get_chapter_by_uid(&conn, "chapter-1")
            .expect("get chapter")
            .expect("chapter should still exist");
        assert_eq!(refetched.title.as_deref(), Some("Updated Title"));
        assert_eq!(refetched.page_count, Some(5));

        delete_chapter(&mut conn, "chapter-1").expect("delete chapter");
        let gone = get_chapter_by_uid(&conn, "chapter-1").expect("get chapter");
        assert!(gone.is_none());
    }

    #[test]
    fn list_chapters_by_series_orders_by_sequence_sort_key() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let conn = crate::open_at(dir.path().join("chapter.sqlite")).expect("bootstrap database");

        let series_uid = "series-made-up-2";
        // Insert out of order to prove ORDER BY does the sorting, not insertion order.
        create_chapter(
            &conn,
            &sample_chapter("chapter-10", series_uid, "10", "00010"),
        )
        .unwrap();
        create_chapter(
            &conn,
            &sample_chapter("chapter-2", series_uid, "2", "00002"),
        )
        .unwrap();
        create_chapter(
            &conn,
            &sample_chapter("chapter-special", series_uid, "Special", "zzzzz"),
        )
        .unwrap();
        // Different series entirely, must not appear in the listing.
        create_chapter(
            &conn,
            &sample_chapter("chapter-other-series", "series-made-up-other", "1", "00001"),
        )
        .unwrap();

        let listed = list_chapters_by_series(&conn, series_uid).expect("list chapters");
        let uids: Vec<&str> = listed.iter().map(|c| c.uid.as_str()).collect();
        assert_eq!(uids, vec!["chapter-2", "chapter-10", "chapter-special"]);
    }

    #[test]
    fn set_pages_replaces_full_ordered_list_and_pages_are_deleted_with_chapter() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let mut conn =
            crate::open_at(dir.path().join("chapter.sqlite")).expect("bootstrap database");

        let series_uid = "series-made-up-3";
        let chapter = sample_chapter("chapter-with-pages", series_uid, "1", "00001");
        create_chapter(&conn, &chapter).expect("create chapter");

        let pages = vec![
            sample_page("chapter-with-pages", 0),
            sample_page("chapter-with-pages", 1),
            sample_page("chapter-with-pages", 2),
        ];
        set_pages(&mut conn, "chapter-with-pages", &pages).expect("set pages");

        let listed = list_pages_by_chapter(&conn, "chapter-with-pages").expect("list pages");
        assert_eq!(listed.len(), 3);
        assert_eq!(
            listed.iter().map(|p| p.page_index).collect::<Vec<_>>(),
            vec![0, 1, 2]
        );
        assert_eq!(listed[1].image.width, Some(800));

        // Replacing with a shorter list drops the extra old rows.
        let fewer_pages = vec![sample_page("chapter-with-pages", 0)];
        set_pages(&mut conn, "chapter-with-pages", &fewer_pages).expect("set fewer pages");
        let listed_after = list_pages_by_chapter(&conn, "chapter-with-pages").expect("list pages");
        assert_eq!(listed_after.len(), 1);

        add_page(&conn, &sample_page("chapter-with-pages", 1)).expect("add page");
        let listed_after_add =
            list_pages_by_chapter(&conn, "chapter-with-pages").expect("list pages");
        assert_eq!(listed_after_add.len(), 2);

        delete_chapter(&mut conn, "chapter-with-pages").expect("delete chapter");
        let pages_after_delete =
            list_pages_by_chapter(&conn, "chapter-with-pages").expect("list pages after delete");
        assert!(
            pages_after_delete.is_empty(),
            "pages should be deleted along with their chapter"
        );
    }
}
