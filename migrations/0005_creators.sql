-- Migration 0005: creators
--
-- Creator Phase 2 (kelpie.md §136) work unit: the normalized creator attribution
-- registry (kelpie.md §104 Database Domains: Attribution & tagging) plus a
-- many-to-many item/creator link table.
--
-- `item_creators` is an inferred join-table name, not one literally spelled out in
-- kelpie.md §104 -- the spec groups `creators`/`tags`/`item_tags` under
-- "Attribution & tagging" and names `item_tags` explicitly for Tag, but does not
-- name a creators-to-item link table. `item_creators` mirrors that `item_tags`
-- naming pattern.
--
-- `creators` here is the standalone, normalized registry -- distinct from the
-- denormalized `creators` JSON column that other Phase 2 entities store directly
-- on their own `items`/`series` rows (that embedded column is expected and
-- unrelated). This table exists so a creator can be looked up, followed, and
-- deduplicated across items (kelpie.md §97 Following: "creator" is a followable
-- entity type).
--
-- No foreign key to `items`/`series` is declared on `item_creators.item_uid`:
-- those tables are owned by other Phase 2 work units landing in parallel, so
-- this migration does not assume their column shape.
--
-- Every migration is numbered, immutable once merged, transactional, and tested
-- (§106) -- this file must never be edited after it ships; changes land as a new
-- numbered migration.

CREATE TABLE creators (
    id           TEXT PRIMARY KEY,
    name         TEXT NOT NULL,
    aliases      TEXT NOT NULL DEFAULT '[]',
    provider_id  TEXT
);

CREATE INDEX idx_creators_provider_id ON creators (provider_id);

CREATE TABLE item_creators (
    item_uid    TEXT NOT NULL,
    creator_id  TEXT NOT NULL REFERENCES creators (id) ON DELETE CASCADE,
    role        TEXT,
    UNIQUE (item_uid, creator_id)
);

CREATE INDEX idx_item_creators_item_uid ON item_creators (item_uid);
CREATE INDEX idx_item_creators_creator_id ON item_creators (creator_id);
