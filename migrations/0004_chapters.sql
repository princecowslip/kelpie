-- Migration 0004: chapters
--
-- Chapter Phase 2 (kelpie.md §136, §20 Series Model, §22 Sequence Numbers) work
-- unit: `chapters` and `pages` tables (kelpie.md §104 Database Domains: Series
-- structure). Every migration is numbered, immutable once merged, transactional,
-- and tested (§106) — once this ships it must never be renumbered, reused for
-- another entity, or edited in place; further changes land as a new migration.
--
-- `chapters.series_uid` is a logical reference to `series.uid` (migration 0003, a
-- sibling Phase 2 work unit), deliberately declared WITHOUT a `REFERENCES` clause:
-- this SQLite build defaults `PRAGMA foreign_keys` to ON, so a declared FK to
-- `series` would be enforced immediately, and Phase 2's entities are designed to be
-- implemented, migrated, and tested independently of one another (§136) — a chapter
-- must be insertable without requiring the `series` migration to have landed, let
-- alone a matching row to already exist. Referential integrity across the two
-- entities is an application-layer concern, not a DB-engine one, here.
--
-- `sequence` and `volume_sequence` store a JSON-encoded `SequenceNumber` (kelpie.md
-- §22: `{ raw, numeric, sortKey }`) as TEXT rather than being split into columns,
-- since sort order is served by the separate `sequence_sort_key` column below.
--
-- `pages.chapter_uid REFERENCES chapters(uid)` (below), by contrast, is a same-file,
-- same-migration reference — `chapters` always exists by the time `pages` is
-- created, and a chapter row always exists before its pages are inserted (see the
-- Chapter repository), so this FK is safe to declare and does get enforced. It has
-- no `ON DELETE CASCADE`: [`crate::repositories::chapter::delete_chapter`] deletes a
-- chapter's pages explicitly, in the same transaction that deletes the chapter row,
-- rather than relying on a cascade.

CREATE TABLE chapters (
    uid                 TEXT PRIMARY KEY,

    -- Logical reference to series.uid (migration 0003) — intentionally not an
    -- enforced FK; see header comment above.
    series_uid          TEXT NOT NULL,

    provider_id         TEXT NOT NULL,
    remote_id           TEXT,

    canonical_url       TEXT NOT NULL,

    title               TEXT,

    -- JSON-encoded SequenceNumber (kelpie.md §22).
    sequence            TEXT NOT NULL,
    -- Denormalized copy of sequence.sort_key, so chapters can be ordered by
    -- sequence without decoding JSON in the query.
    sequence_sort_key   TEXT NOT NULL,

    -- JSON-encoded SequenceNumber (kelpie.md §22), or NULL when the source has no
    -- volume level above this chapter.
    volume_sequence     TEXT,

    published_at        TEXT,
    page_count          INTEGER
);

CREATE INDEX idx_chapters_series_uid ON chapters (series_uid);
CREATE INDEX idx_chapters_series_sort_key ON chapters (series_uid, sequence_sort_key);

CREATE TABLE pages (
    chapter_uid   TEXT NOT NULL REFERENCES chapters(uid),
    page_index    INTEGER NOT NULL,

    -- JSON-encoded ImageResource (kelpie.md §16: `{ url, width, height }`).
    image         TEXT NOT NULL,

    PRIMARY KEY (chapter_uid, page_index)
);
