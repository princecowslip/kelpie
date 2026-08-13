-- Migration 0003: series
--
-- Series Phase 2 (kelpie.md §136, §20-21 Series Model / Series Object) work unit:
-- `series` table (kelpie.md §104 Database Domains: Series structure). Every
-- migration is numbered, immutable once merged, transactional, and tested (§106) —
-- once shipped, further schema changes for this entity land as a new, later-
-- numbered migration, not an edit to this file.
--
-- `alternate_titles`, `creators`, `cover`, and `tags` are stored as JSON TEXT
-- columns rather than normalized into their own tables — the pragmatic Phase 2
-- approach for this entity (kelpie.md §104, §21 Series Object).

CREATE TABLE series (
    uid                 TEXT PRIMARY KEY,

    provider_id         TEXT NOT NULL,
    remote_id           TEXT,

    canonical_url       TEXT NOT NULL,

    title               TEXT NOT NULL,
    alternate_titles    TEXT NOT NULL DEFAULT '[]',

    description         TEXT,

    creators            TEXT NOT NULL DEFAULT '[]',

    cover               TEXT,

    tags                TEXT NOT NULL DEFAULT '[]',

    status              TEXT,
    reading_direction   TEXT
);

CREATE INDEX idx_series_provider_id ON series (provider_id);
