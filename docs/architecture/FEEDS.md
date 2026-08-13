# Feed Pipeline & Ranking

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

This document covers the backend/pipeline half of Kelpie's global feed: how provider data flows into a locally-ranked, clustered feed. The user-facing half — feed modes, the Home screen, and feed filtering (spec §59–61) — lives in [`../ux/FEEDS.md`](../ux/FEEDS.md); see that document for how these mechanics surface to the user.

## Global Feed Architecture

The global feed is designed to primarily read from local normalized data rather than hitting providers live on every interaction. Data flows through a fixed pipeline from provider to UI:

```
Provider
  ↓
Refresh
  ↓
Normalize
  ↓
Validate
  ↓
Deduplicate
  ↓
Cluster
  ↓
SQLite
  ↓
Filter
  ↓
Rank
  ↓
UI
```

A guiding constraint on this pipeline: scrolling the feed must not generate dozens of remote requests. Refresh, normalization, deduplication, and clustering happen ahead of time (on a schedule — see [`EVENTS.md`](./EVENTS.md) for the refresh-trigger model) and are persisted to SQLite; filtering and ranking then operate purely against that local data at scroll time.

## Feed Ranking

Each candidate feed item is given a conceptual local score, computed entirely on-device:

```
score =
    freshness
  + provider preference
  + media-family preference
  + follow bonus
  + preferred-tag match
  + saved-search match
  + unseen bonus
  + discovery bonus

  - seen penalty
  - provider repetition
  - creator repetition
  - series repetition
  - media-type repetition
```

The positive terms reward relevance and novelty (fresh, followed, tag-matching, previously-unseen items); the negative terms discourage the feed from being dominated by repetition of the same provider, creator, series, or media type in a row. No cloud recommendation engine should be necessary — ranking is meant to be fully explainable and computable locally.

## Ranking Modes

Kelpie is intended to support four selectable ranking modes:

- Chronological
- Balanced
- Personalized
- Manual

For finer control, advanced settings expose numeric weights (default values shown):

| Signal | Default weight |
|---|---|
| Freshness | 80 |
| Followed series | 90 |
| Creators | 70 |
| Tags | 60 |
| Provider priority | 50 |
| History influence | 30 |
| Discovery | 20 |

Alongside the score itself, the feed should be able to explain *why* an item was ranked where it is — a "Why this item?" affordance — with example explanations such as:

- New chapter from a followed series
- Matches two preferred tags
- Published recently

## Feed Clustering

Related bursts of activity are collapsed into a single cluster rather than flooding the feed with near-duplicate entries. For example, instead of showing:

```
Series A Ch 30
Series A Ch 31
Series A Ch 32
Series A Ch 33
```

the feed shows:

```
Series A

4 new chapters
Latest: Chapter 33
```

Defined cluster types are:

- `series-update`
- `gallery-batch`
- `duplicate-set`
- `provider-batch`
- `thread-update`

Clustering happens as its own pipeline stage (see the diagram above), after deduplication and before persistence to SQLite, so clusters are a stored, queryable concept rather than a purely presentational grouping computed at render time.
