# Database Migrations

SQLite schema migrations (kelpie.md §106 Migration Policy): numbered, immutable
once merged, transactional, and tested. Migration `0001_init.sql` (Phase 1,
§135 SQLite bootstrap) establishes only a `schema_migrations` bookkeeping
table. `0002_core_data_layer.sql` (Phase 2, §136) adds the domain schema for
MediaItem, Series, Chapter, Creator, Tag, Collection, History, Progress, and
Following/Saved — the subset of §104 Database Domains that phase covers.
Everything else in §104 (providers, feeds, pages, search history, downloads,
cache entries, provider errors) lands with the later phase that owns it.
