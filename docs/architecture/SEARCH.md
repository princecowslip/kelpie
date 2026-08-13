# Search Architecture

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

This document covers how search works across Kelpie's local index and live providers: the two-stage search model, the query language, saved searches, deduplication, and how a duplicate item's best source is chosen. For the FTS5 schema backing local search, see [`DATABASE.md`](./DATABASE.md).

## Search Architecture

Search runs in two stages rather than waiting on a single unified result set:

**Immediate Local Search** — queries the local FTS index and returns near-instantly.

**Streaming Remote Search** — provider searches run concurrently in the background and stream in as they complete.

The intended UI reflects both stages at once, showing the local result count immediately and then updating per-provider status as remote searches resolve:

```
Local results: 42

Remote
✓ Provider A — 14
… Provider B
✓ Provider C — 9
! Provider D — sign in required
```

Users can interact with the local results immediately rather than waiting for every provider to respond.

## Search Query Language

Kelpie's search supports a structured query language layered on top of free text, using `field:value` tokens:

```
type:video
type:gif
type:manga
type:comic
type:story

source:provider
creator:"name"
series:"title"

tag:example
-tag:example

style:anime
style:cartoon

before:2026-01-01
after:2025-01-01

duration:>20m

saved:true
seen:false
following:true
offline:true
```

This gives users precise filtering (media kind, source, creator, series, tag inclusion/exclusion, style, date ranges, duration comparisons) alongside state-based filters like `seen`, `following`, and `offline` availability.

## Saved Searches

Any search — including one built from the query language above — can be:

- saved
- renamed
- pinned
- placed on Home
- placed in the sidebar
- restricted to the local index
- restricted to selected providers

For example, the query `type:manga following:true seen:false` could be saved with the display name **Unread Followed Manga**, turning an ad hoc filter into a persistent, named view.

## Deduplication

Because the same media can appear on multiple provider sources, search (and the feed pipeline — see [`FEEDS.md`](./FEEDS.md)) applies deduplication using progressively weaker signals, in order:

1. provider remote ID
2. canonical URL
3. declared cross-source canonical ID
4. creator + normalized title
5. duration similarity
6. thumbnail perceptual hash
7. optional media fingerprint

Each match is assigned a confidence level — `exact`, `high`, or `possible` — and only `exact`/`high` confidence matches should auto-collapse into a single deduplicated entry. `possible` matches are surfaced without being silently merged, since collapsing on weak signals risks hiding genuinely distinct items.

## Source Variant Selection

When a duplicate item is available from several providers, Kelpie needs to decide which source to present as primary. Selection follows this priority order:

1. user preferred source
2. authenticated source
3. higher quality
4. better metadata
5. recent provider reliability

The UI still exposes the existence of alternates rather than hiding them entirely, e.g.:

```
Available from 3 sources
```
