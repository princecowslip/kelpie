-- Migration 0002: core data layer
--
-- Kelpie SQLite domain schema (kelpie.md §106 Migration Policy). Numbered,
-- immutable once merged, transactional, and tested — this file must never be
-- edited after it ships; changes land as a new numbered migration.
--
-- Phase 2 (§136 Core Data Layer) scope: the subset of §104's Database Domains
-- that map onto MediaItem, Series, Chapter, Creator, Tag, Collection, History,
-- Progress, plus Following/Saved (§94/§97). Out of scope here and left for
-- later phases per §104's own domain grouping: providers/provider_permissions/
-- provider_state (Provider Platform, §137), feeds/feed_entries/feed_clusters
-- and the items_fts search index (Feed and Search, §138), pages (deferred
-- until a reader implementation needs cached per-page rows), saved_searches/
-- search_history (§138), downloads (§145), cache_entries, provider_errors.
--
-- Schema design notes (kelpie.md leaves some MediaItem/Series sub-structures
-- without a formally named interface — see §16/§21/§89 gaps): complex nested
-- or array-valued fields that §104 does not name as their own domain table
-- (ContentClassification, CreatorCredit[], ImageResource[]/thumbnails,
-- SequenceNumber sub-fields, providerMetadata, Series.alternateTitles/cover/
-- tags) are stored as JSON TEXT columns rather than further normalized —
-- pragmatic for Phase 2, revisitable later without a data migration since
-- these are opaque blobs from SQLite's perspective. `item_tags` and
-- `media_sources` are real relational tables because §104 explicitly names
-- them as their own domains.

-- items: the Unified MediaItem (§16). Stable identity per §17 (preferred
-- "provider-id:object-type:remote-id", falling back to
-- SHA256(provider-id + canonical-url) — never title alone) is computed by
-- callers via kelpie_core::identity::stable_id and stored as `uid`.
CREATE TABLE items (
    uid                     TEXT PRIMARY KEY,
    provider_id             TEXT NOT NULL,
    remote_id               TEXT,
    canonical_url           TEXT NOT NULL,
    kind                    TEXT NOT NULL,
    title                   TEXT NOT NULL,
    description             TEXT,
    classification_json     TEXT NOT NULL,
    creators_json           TEXT NOT NULL DEFAULT '[]',
    thumbnails_json         TEXT NOT NULL DEFAULT '[]',
    published_at            TEXT,
    updated_at              TEXT,
    duration_seconds        INTEGER,
    width                   INTEGER,
    height                  INTEGER,
    series_uid              TEXT REFERENCES series(uid),
    sequence_json           TEXT,
    availability            TEXT NOT NULL,
    provider_metadata_json  TEXT,
    created_at              TEXT NOT NULL,
    updated_locally_at      TEXT NOT NULL
);

CREATE INDEX idx_items_series_uid ON items(series_uid);
CREATE INDEX idx_items_provider_id ON items(provider_id);
CREATE INDEX idx_items_kind ON items(kind);

-- media_sources: MediaSource (§19) — its own §104 domain, so a real table
-- with a foreign key rather than a JSON column on items. Signed/expiring
-- URLs (§19: "should be resolved on demand rather than persisted as
-- canonical identity") are still cached here as rows; callers are
-- responsible for treating `expires_at` as a freshness hint, not identity.
CREATE TABLE media_sources (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    item_uid        TEXT NOT NULL REFERENCES items(uid) ON DELETE CASCADE,
    url             TEXT NOT NULL,
    transport       TEXT NOT NULL,
    mime_type       TEXT,
    width           INTEGER,
    height          INTEGER,
    bitrate         INTEGER,
    label           TEXT,
    expires_at      TEXT
);

CREATE INDEX idx_media_sources_item_uid ON media_sources(item_uid);

-- creators: a standalone creator lookup (supports Follow-by-creator, §97).
-- Per-item credit detail (role etc.) stays in items.creators_json — §104
-- names `item_tags` as a junction domain but not an item-creator junction,
-- so credits are treated as item-owned data rather than a normalized M:N.
CREATE TABLE creators (
    uid             TEXT PRIMARY KEY,
    provider_id     TEXT,
    name            TEXT NOT NULL,
    profile_url     TEXT,
    avatar_url      TEXT
);

-- tags: TagReference (§89 — normalizedId/providerId/sourceValue/displayName/
-- category). `uid` is the row's own stable key (normalized_id when present,
-- else provider_id:source_value); normalized_id is kept as a separate
-- nullable column so lookups by normalized identity stay possible even when
-- it's absent for a given provider.
CREATE TABLE tags (
    uid             TEXT PRIMARY KEY,
    normalized_id   TEXT,
    provider_id     TEXT NOT NULL,
    source_value    TEXT NOT NULL,
    display_name    TEXT NOT NULL,
    category        TEXT
);

CREATE INDEX idx_tags_normalized_id ON tags(normalized_id);

-- item_tags: the item<->tag junction §104 explicitly names as its own domain.
CREATE TABLE item_tags (
    item_uid    TEXT NOT NULL REFERENCES items(uid) ON DELETE CASCADE,
    tag_uid     TEXT NOT NULL REFERENCES tags(uid) ON DELETE CASCADE,
    PRIMARY KEY (item_uid, tag_uid)
);

CREATE INDEX idx_item_tags_tag_uid ON item_tags(tag_uid);

-- series: the Series object (§21). alternateTitles/cover/tags are JSON
-- columns (no separate domain table named for them in §104); status and
-- readingDirection are the closed enums from §21.
CREATE TABLE series (
    uid                     TEXT PRIMARY KEY,
    provider_id             TEXT NOT NULL,
    remote_id               TEXT,
    canonical_url           TEXT NOT NULL,
    title                   TEXT NOT NULL,
    alternate_titles_json   TEXT NOT NULL DEFAULT '[]',
    description             TEXT,
    creators_json           TEXT NOT NULL DEFAULT '[]',
    cover_json              TEXT,
    tags_json               TEXT NOT NULL DEFAULT '[]',
    status                  TEXT,
    reading_direction       TEXT
);

CREATE INDEX idx_series_provider_id ON series(provider_id);

-- chapters: the Chapter level of the Series hierarchy (§20), distinct from
-- items per §104's separate `chapters` domain. sequence_json holds the
-- SequenceNumber sub-fields (§22: raw/numeric/sortKey — never assume an
-- integer).
CREATE TABLE chapters (
    uid             TEXT PRIMARY KEY,
    series_uid      TEXT NOT NULL REFERENCES series(uid) ON DELETE CASCADE,
    provider_id     TEXT NOT NULL,
    remote_id       TEXT,
    canonical_url   TEXT NOT NULL,
    title           TEXT,
    sequence_json   TEXT,
    page_count      INTEGER,
    published_at    TEXT
);

CREATE INDEX idx_chapters_series_uid ON chapters(series_uid);

-- collections: manual, locally-owned collections (§95). Smart/query-backed
-- collections (§96) are explicitly "Future" and out of scope here. `uid` is
-- a locally generated UUID since collections have no provider-issued id.
CREATE TABLE collections (
    uid             TEXT PRIMARY KEY,
    name            TEXT NOT NULL,
    sort_mode       TEXT NOT NULL DEFAULT 'manual',
    created_at      TEXT NOT NULL,
    updated_at      TEXT NOT NULL
);

CREATE TABLE collection_items (
    collection_uid  TEXT NOT NULL REFERENCES collections(uid) ON DELETE CASCADE,
    item_uid        TEXT NOT NULL REFERENCES items(uid) ON DELETE CASCADE,
    position        INTEGER NOT NULL,
    added_at        TEXT NOT NULL,
    PRIMARY KEY (collection_uid, item_uid)
);

CREATE INDEX idx_collection_items_item_uid ON collection_items(item_uid);

-- history: per-item activity tracking (§98: first opened, last opened, open
-- count, completed). Retention policy (Forever/90/30/7/Session only/Never)
-- is a user setting enforced by a future sweep job, not a per-row field.
CREATE TABLE history (
    item_uid            TEXT PRIMARY KEY REFERENCES items(uid) ON DELETE CASCADE,
    first_opened_at     TEXT NOT NULL,
    last_opened_at      TEXT NOT NULL,
    open_count          INTEGER NOT NULL DEFAULT 0,
    completed           INTEGER NOT NULL DEFAULT 0
);

-- progress: Reading Progress (§84 — series UID, chapter/issue UID, page
-- index, scroll fraction, updated time, completed status). One row per item;
-- series_uid/chapter_uid are nullable since not every item is part of a
-- series (e.g. standalone video/audio progress still keys off item_uid).
CREATE TABLE progress (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    item_uid        TEXT NOT NULL UNIQUE REFERENCES items(uid) ON DELETE CASCADE,
    series_uid      TEXT REFERENCES series(uid),
    chapter_uid     TEXT REFERENCES chapters(uid),
    page_index      INTEGER,
    scroll_fraction REAL,
    updated_at      TEXT NOT NULL,
    completed       INTEGER NOT NULL DEFAULT 0
);

-- follows: Following (§97 — provider, creator, series, tag, or saved search).
-- Polymorphic by design (target_type + target_id) since the followable set
-- spans several otherwise-unrelated tables.
CREATE TABLE follows (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    target_type     TEXT NOT NULL CHECK (target_type IN ('provider', 'creator', 'series', 'tag', 'saved_search')),
    target_id       TEXT NOT NULL,
    created_at      TEXT NOT NULL,
    UNIQUE (target_type, target_id)
);

-- favorites: "Saved" in the Library (§94), its own §104 domain.
CREATE TABLE favorites (
    item_uid        TEXT PRIMARY KEY REFERENCES items(uid) ON DELETE CASCADE,
    created_at      TEXT NOT NULL
);
