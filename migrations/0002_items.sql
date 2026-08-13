-- Migration 0002: items
--
-- MediaItem Phase 2 work unit (kelpie.md §136, §16 Unified MediaItem, §19 Media
-- Sources; kelpie.md §104 Database Domains: Media). Creates `items`, the unified
-- normalized media entity every provider adapter produces, and `media_sources`,
-- the one-to-many playable/renderable sources resolved for an item. Every
-- migration is numbered, immutable once merged, transactional, and tested (§106)
-- — now that this migration carries real schema, further changes to it land as a
-- new, later-numbered migration, never an edit to this file.
--
-- Structured sub-fields that don't need independent relational identity outside
-- this entity (creators, thumbnails, tags, classification, dimensions, series,
-- sequence) are stored as JSON TEXT columns on `items` rather than normalized
-- into their own tables -- full normalization of creators/tags into shared,
-- cross-entity tables is the Creator/Tag entities' own Phase 2 work units
-- (0005_creators.sql, 0006_tags.sql), not this one. `provider_metadata` is
-- likewise a JSON TEXT column holding arbitrary provider-specific JSON verbatim.
--
-- `media_sources` is a genuine child table, since a single item can resolve to
-- many sources (e.g. multiple quality renditions); it is linked back to its
-- parent by the `item_uid` foreign key.

CREATE TABLE items (
    uid                 TEXT PRIMARY KEY,

    provider_id         TEXT NOT NULL,
    remote_id           TEXT,

    canonical_url       TEXT NOT NULL,

    kind                TEXT NOT NULL,

    title               TEXT NOT NULL,
    description         TEXT,

    creators            TEXT NOT NULL DEFAULT '[]',
    thumbnails          TEXT NOT NULL DEFAULT '[]',
    tags                TEXT NOT NULL DEFAULT '[]',

    classification      TEXT NOT NULL,

    published_at        TEXT,
    updated_at          TEXT,

    duration_seconds    REAL,

    dimensions          TEXT,

    series              TEXT,

    sequence            TEXT,

    availability        TEXT NOT NULL,

    provider_metadata   TEXT
);

CREATE INDEX idx_items_provider_id ON items (provider_id);

CREATE TABLE media_sources (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    item_uid    TEXT NOT NULL REFERENCES items (uid) ON DELETE CASCADE,

    url         TEXT NOT NULL,
    transport   TEXT NOT NULL,
    mime_type   TEXT,
    width       INTEGER,
    height      INTEGER,
    bitrate     INTEGER,
    label       TEXT,
    expires_at  TEXT
);

CREATE INDEX idx_media_sources_item_uid ON media_sources (item_uid);
