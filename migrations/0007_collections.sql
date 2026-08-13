-- Migration 0007: collections
--
-- Collection Phase 2 (kelpie.md §136, §94-95 Library / Collections) work unit:
-- `collections` and `collection_items` tables (kelpie.md §104 Database Domains:
-- Library / user state). Every migration is numbered, immutable once merged,
-- transactional, and tested (§106) — this file must never be edited after it ships;
-- changes land as a new numbered migration.
--
-- Collections are user-created and may mix any media kind (video, GIF, gallery,
-- manga, comic, story, audio, external page, local file — §95); no media-kind
-- restriction is enforced at the database level, so `collection_items.item_uid` is
-- an opaque foreign identifier rather than a foreign key into a specific entity
-- table. `position` holds the manual ordering index used when a collection's
-- `sort_mode` is `manual`.

CREATE TABLE collections (
    uid         TEXT PRIMARY KEY,
    name        TEXT NOT NULL,
    sort_mode   TEXT NOT NULL,
    created_at  TEXT NOT NULL,
    updated_at  TEXT NOT NULL
);

CREATE TABLE collection_items (
    collection_uid  TEXT NOT NULL REFERENCES collections (uid) ON DELETE CASCADE,
    item_uid        TEXT NOT NULL,
    position        INTEGER NOT NULL,
    added_at        TEXT NOT NULL,
    UNIQUE (collection_uid, item_uid)
);

CREATE INDEX idx_collection_items_collection_uid ON collection_items (collection_uid);
