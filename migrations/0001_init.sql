-- Migration 0001: init
--
-- Kelpie SQLite bootstrap (kelpie.md §106 Migration Policy). Every migration is
-- numbered, immutable once merged, transactional, and tested — this file must
-- never be edited after it ships; changes land as a new numbered migration.
--
-- Phase 1 (§135) scope only: the schema_migrations bookkeeping table itself, so
-- the migration runner has somewhere to record which migrations have applied.
-- The full domain schema (§104 Database Domains) lands in Phase 2 (§136).

CREATE TABLE schema_migrations (
    version     INTEGER PRIMARY KEY,
    applied_at  TEXT NOT NULL
);
