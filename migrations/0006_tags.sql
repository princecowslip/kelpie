-- Migration 0006: tags
--
-- Tag Phase 2 (kelpie.md §136, §89-90 Tag Normalization) work unit: `tags` and
-- `item_tags` tables (kelpie.md §104 Database Domains: Attribution & tagging).
-- Every migration is numbered, immutable once merged, transactional, and tested
-- (§106) — once this file ships with real schema it must never be edited again;
-- further changes to this entity's schema land as a new, later-numbered migration.
--
-- `tags` is the normalized tag registry: one row per stable normalized identity
-- (§89 "normalized identity"), so a tag can be looked up, deduplicated, and
-- filtered across the whole library (booru-style browsing, §88). It is distinct
-- from the per-item `TagReference` shape (§90) that other entities embed as
-- denormalized JSON directly on their own rows (e.g. `MediaItem.tags`).
--
-- `item_tags` links an item to a normalized tag, preserving per-reference
-- traceability back to the source (§90 `TagReference.providerId` /
-- `sourceValue`): the same normalized tag can be supplied by more than one
-- provider (or more than once by the same provider under a different raw
-- value), so the identifying key is (item_uid, normalized_id, provider_id),
-- with a UNIQUE constraint on (item_uid, normalized_id) for the common
-- "does this item already carry this normalized tag" case. `item_tags.normalized_id
-- REFERENCES tags(normalized_id)` is a same-migration reference (both tables are
-- created here, and a `tags` row always exists before an `item_tags` row can link
-- to it), so it is safe to declare and does get enforced — this SQLite build runs
-- with `PRAGMA foreign_keys` on by default. It has no `ON DELETE CASCADE`, though:
-- removing a tag's `item_tags` rows when the tag itself is deleted is the owning
-- repository's responsibility, done explicitly in the same transaction as the
-- tag delete, rather than relying on a cascade.

CREATE TABLE tags (
    normalized_id   TEXT PRIMARY KEY,
    display_name    TEXT NOT NULL,
    category        TEXT,
    aliases         TEXT NOT NULL DEFAULT '[]'
);

CREATE TABLE item_tags (
    item_uid        TEXT NOT NULL,
    normalized_id   TEXT NOT NULL REFERENCES tags (normalized_id),
    provider_id     TEXT NOT NULL,
    source_value    TEXT NOT NULL,
    UNIQUE (item_uid, normalized_id)
);

CREATE INDEX idx_item_tags_item_uid ON item_tags (item_uid);
CREATE INDEX idx_item_tags_normalized_id ON item_tags (normalized_id);
