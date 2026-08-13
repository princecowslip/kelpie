# ADR 0002: Use SQLite (with FTS5) as the Local Database and Search Engine

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Context

Kelpie is a local-first desktop application (kelpie.md §6, Design Principles — "Local first: core state resides locally") that needs to store a fairly wide relational domain model locally: providers and their permissions/state, normalized items and media sources, creators/tags, series/chapters/pages, feeds and feed entries/clusters, favorites, collections, follows, history/progress, saved searches and search history, downloads, cache entries, and provider errors (kelpie.md §104, Database Domains). It also needs local full-text search over titles, descriptions, creators, tags, series titles, and (conditionally) body text, without depending on any external service (kelpie.md §105, Search Database).

kelpie.md's recommended technology stack lists SQLite as part of the native/application core alongside Rust and Tokio, and specifies SQLite FTS5 as the search layer, noting that "SQLite documents FTS5 as its full-text-search virtual-table module, making it appropriate for local title/tag/creator/story indexing" (kelpie.md §9). The Database Domains section (§104) enumerates the relational tables the app is expected to need, and the Search Database section (§105) gives a concrete `items_fts` FTS5 virtual table definition indexing `title`, `description`, `creators`, `tags`, `series_title`, and `body`, with the caveat that full story text should only be indexed when local storage of that text is permitted and appropriate. The Migration Policy section (§106) further specifies that every schema migration must be numbered, immutable, transactional, and tested, and that major upgrades follow a backup → migration → integrity-check → launch sequence, falling back to backup restore, recovery mode, and diagnostics on failure.

## Decision

Use SQLite as Kelpie's local database, including its FTS5 extension as the local full-text search engine (via an `items_fts` virtual table indexing item title/description/creators/tags/series title, and body text only when permitted), governed by a numbered/immutable/transactional/tested migration policy with backup-and-integrity-check on major upgrades.

## Alternatives

kelpie.md does not name a rejected database alternative explicitly, but the choice follows directly from the "local first" design principle (kelpie.md §6, §3.3). The reasoned alternatives are:

- **A client-server database** (e.g. Postgres/MySQL) — would require running or depending on a separate database service, which conflicts with a single-user local-first desktop application that must work fully offline and store core state locally.
- **No structured local index / relying on providers' own search** — would leave search and library state at the mercy of individual provider websites, directly contradicting the local-ownership goal that favorites, history, progress, collections, and follow state belong to Kelpie rather than to individual sites (kelpie.md §3.3) — see [`0008-local-first-history.md`](./0008-local-first-history.md).
- **A separate dedicated search engine** (e.g. an embedded inverted-index library distinct from the relational store) — would add an additional storage engine and a synchronization problem between it and the relational data, where SQLite FTS5 gives an in-process full-text index that stays transactionally consistent with the rest of the schema.

## Consequences

- All of Kelpie's core relational domains (providers, items, series, feeds, favorites, collections, follows, history/progress, downloads, cache, provider errors — kelpie.md §104) live in a single embedded SQLite file, which is what makes the local-first and local-ownership principles concretely enforceable rather than aspirational.
- Full-text search over the library is available offline and does not depend on any provider or external index; see [`../architecture/SEARCH.md`](../architecture/SEARCH.md).
- Whether story body text is indexed in `items_fts` is a policy decision gated on local-storage permissibility, not purely a technical one, so search completeness for text-heavy sources (manga/story providers) may be intentionally limited.
- Schema evolution carries real operational weight: because migrations must be numbered, immutable, and tested with a backup/integrity-check/recovery path (kelpie.md §106), every schema change becomes a permanent, ordered artifact rather than something that can be silently rewritten.
- SQLite's single-writer-process model is a natural fit for Kelpie's single-user desktop core, but it does mean the database is not designed to be shared concurrently across multiple independent processes beyond the trusted core described in [`../architecture/OVERVIEW.md`](../architecture/OVERVIEW.md).

## Status

Proposed — kelpie.md v4.0 documents this as an intended decision; no code exists yet that depends on it. This will move to Accepted once an implementation lands that relies on it, or Superseded/Rejected if reconsidered.
