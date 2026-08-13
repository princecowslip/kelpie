-- Migration 0008: history
--
-- kelpie.md §136 Phase 2 (Core Data Layer), §98 History: the storage primitive for
-- per-item access/completion tracking — first opened, last opened, open count, and
-- completed. One row per item, keyed by the item's stable UID.
--
-- "Progress" as named in §98 is only a concept pointer here: this table records
-- *that* an item has progress associated with it via the caller's own bookkeeping,
-- not the progress data itself (page index, scroll fraction, etc.), which is owned
-- by the separate `progress` table (migration 0009).
--
-- Retention policy (Forever/90 days/30 days/7 days/Session only/Never, §99 Private
-- Mode) is later-phase work; this migration only ships the delete/prune-before(cutoff)
-- primitive that policy will eventually be built on top of (see the `history`
-- repository's `prune_before`).
--
-- Every migration is numbered, immutable once merged, transactional, and tested
-- (§106) — once this ships, further schema changes land as a new, later-numbered
-- migration, not an edit to this file.

CREATE TABLE history (
    item_uid        TEXT PRIMARY KEY,
    first_opened_at TEXT NOT NULL,
    last_opened_at  TEXT NOT NULL,
    open_count      INTEGER NOT NULL DEFAULT 0,
    completed       INTEGER NOT NULL DEFAULT 0
);

-- Supports list_recent (ORDER BY last_opened_at DESC) and prune_before (WHERE
-- last_opened_at < cutoff).
CREATE INDEX idx_history_last_opened_at ON history (last_opened_at);
