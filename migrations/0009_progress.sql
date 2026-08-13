-- Migration 0009: progress
--
-- Progress Phase 2 (kelpie.md §136, §84 Reading Progress) work unit: the
-- `progress` table (kelpie.md §104 Database Domains: Library / user state).
-- Every migration is numbered, immutable once merged, transactional, and
-- tested (§106) — once shipped, further schema changes land as a new,
-- later-numbered migration, not an edit to this file.
--
-- Stores resumable playback/reading position for an item. `item_uid` is the
-- primary key: either a plain item's uid (video/audio/etc, with
-- `series_uid` NULL) or a manga/comic chapter's uid within a series (with
-- `series_uid` set). `page_index` / `scroll_fraction` serve paginated /
-- continuous readers; `position_seconds` serves time-based playback media.

CREATE TABLE progress (
    item_uid          TEXT PRIMARY KEY,
    series_uid        TEXT,
    page_index        INTEGER,
    scroll_fraction   REAL,
    position_seconds  REAL,
    updated_at        TEXT NOT NULL,
    completed         INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX idx_progress_updated_at ON progress (updated_at);
CREATE INDEX idx_progress_completed ON progress (completed);
