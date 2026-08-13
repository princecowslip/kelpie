# Database Migrations

SQLite schema migrations (kelpie.md §106 Migration Policy): numbered, immutable
once merged, transactional, and tested. Migration `0001_init.sql` (Stage B unit 2,
Phase 1 SQLite bootstrap) establishes only a `schema_migrations` bookkeeping table;
the full domain schema (§104 Database Domains) lands in Phase 2 (§136).

## Phase 2 numbering convention

Phase 2 (§136 Core Data Layer) reserves one migration number per entity, fixed
up front so the entities can be implemented in parallel without colliding on
migration numbering:

| Version | File | Entity | Tables |
|---|---|---|---|
| 0002 | `0002_items.sql` | MediaItem | `items`, `media_sources` |
| 0003 | `0003_series.sql` | Series | `series` |
| 0004 | `0004_chapters.sql` | Chapter | `chapters`, `pages` |
| 0005 | `0005_creators.sql` | Creator | `creators` (+ item/series link table) |
| 0006 | `0006_tags.sql` | Tag | `tags`, `item_tags` |
| 0007 | `0007_collections.sql` | Collection | `collections`, `collection_items` |
| 0008 | `0008_history.sql` | History | `history` |
| 0009 | `0009_progress.sql` | Progress | `progress` |

Each file starts as a harmless placeholder (registered in `MIGRATIONS` in
`crates/database/src/lib.rs`) and is filled in by its own entity's Phase 2 work
unit. Once a migration has actually shipped with real schema, it becomes subject
to the immutability rule above like any other migration — further changes to
that entity's schema land as a new, later-numbered migration, not an edit to
these files.
