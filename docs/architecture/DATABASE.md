# Database & Storage Architecture

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

Kelpie's local persistence is built on SQLite. This document covers the database's domain layout, the full-text search index, migration policy, cache architecture, and the storage manager that accounts for space used across all of these.

## Database Domains

Kelpie's SQLite schema is organized into the following domains (grouped roughly by responsibility rather than as literal table names):

**Providers**
- providers
- provider_permissions
- provider_state

**Media**
- items
- media_sources

**Attribution & tagging**
- creators
- tags
- item_tags

**Series structure**
- series
- chapters
- pages

**Feed**
- feeds
- feed_entries
- feed_clusters

**Library / user state**
- favorites
- collections
- collection_items
- follows
- history
- progress
- saved_searches
- search_history
- downloads

**Operational**
- cache_entries
- provider_errors

This grouping mirrors the major application services described in [`OVERVIEW.md`](./OVERVIEW.md#major-application-services) — e.g. the series domain backs `SeriesService`, the feed domain backs `FeedService`, and so on.

## Search Database

Local text search is backed by a SQLite FTS5 virtual table:

```sql
CREATE VIRTUAL TABLE items_fts USING fts5(
    uid UNINDEXED,
    title,
    description,
    creators,
    tags,
    series_title,
    body
);
```

Full story text should only be indexed as `body` when local storage of that text is permitted and appropriate — this index is not assumed to always carry full text for every item. See [`SEARCH.md`](./SEARCH.md) for how this index is queried as part of the two-stage local/remote search architecture.

## Migration Policy

Every schema migration must be:

- numbered
- immutable
- transactional
- tested

A major upgrade follows this sequence:

```
database
→ backup
→ migration
→ integrity check
→ launch
```

If a migration fails, the recovery path is:

```
restore backup
launch recovery mode
show diagnostics
```

This keeps a failed migration from leaving the user with a corrupted or half-migrated database — the system falls back to the last good backup and surfaces diagnostics rather than launching into an inconsistent state.

## Cache Architecture

Caching is deliberately split into separate caches rather than one undifferentiated cache, so each can be sized, evicted, and cleared independently:

- memory cache
- metadata cache
- thumbnail cache
- page cache
- temporary media cache
- persistent downloads

Individual cache entries can carry their own:

- expiry
- sensitivity
- provider
- size

This per-entry metadata is what lets the cache be evicted intelligently (e.g. by age, by provider, or by sensitivity) rather than uniformly.

## Storage Manager

The Storage Manager gives the user visibility into, and control over, on-disk space consumption. It shows space used by:

- database
- thumbnails
- GIF previews
- image cache
- comic/manga cache
- video cache
- audio cache
- downloads
- provider data

Each of these categories can be cleared independently where safe — for example, clearing the image cache should not require also clearing persistent downloads or the database itself. This is placed alongside cache architecture in this document because storage accounting is fundamentally a view over the same cache and download domains described above.
