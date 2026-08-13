# Database Migrations

SQLite schema migrations (kelpie.md §106 Migration Policy): numbered, immutable
once merged, transactional, and tested. Migration `0001_init.sql` (Stage B unit 2,
Phase 1 SQLite bootstrap) establishes only a `schema_migrations` bookkeeping table;
the full domain schema (§104 Database Domains) lands in Phase 2 (§136).
