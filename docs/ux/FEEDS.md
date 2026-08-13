# Feeds

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Overview

This document covers the **user-facing** half of Kelpie's feed system: the set of feed modes a user can choose between, the suggested structure of the Home screen, and how a user filters what a feed shows. It does not cover how a feed is actually assembled behind the scenes.

For the pipeline that produces these feeds — ingestion/aggregation architecture, ranking algorithm, ranking modes' internal scoring, and clustering logic — see [`../architecture/FEEDS.md`](../architecture/FEEDS.md) (kelpie.md §58, §62–64: Global Feed Architecture, Feed Ranking, Ranking Modes, Feed Clustering). This document only concerns itself with what the user sees and chooses; that one concerns itself with how it is computed.

## Feed modes

Kelpie exposes seven feed modes (kelpie.md §59):

| Mode | Behavior |
|---|---|
| **Home** | Configurable mixed dashboard. |
| **Latest** | Chronological. |
| **Following** | Explicit follows only. |
| **Continue** | Partially consumed items. |
| **Unseen** | Unopened. |
| **Discover** | Diversity-biased recommendations. |
| **Random** | Intentional random browsing. |

These modes are distinct entry points, not filters layered on a single feed — each has its own selection logic. Following, Continue, and Unseen are all state-driven (they depend on the user's follow list, progress records, and open/seen history respectively — see `../architecture/DOMAIN-MODEL.md` for how that state is modeled), while Latest, Discover, and Random reflect three different, deliberately distinct strategies for surfacing content the user has not necessarily interacted with yet: pure recency, diversity-biased recommendation, and intentional randomness. Random is explicitly framed as *intentional* random browsing — a mode a user opts into for serendipity, not an accidental or low-effort fallback.

Home sits apart from the other six: it is not a single ranked stream but a **configurable mixed dashboard**, described in the next section.

## Home screen

The spec gives a suggested structure for the Home screen (kelpie.md §60):

```
Search everything

Continue
──────────────
[card][card][card]

Following Updates
─────────────────
[series][creator][series]

New Manga & Comics
──────────────────
[cards]

Global Feed
───────────
[adaptive infinite feed]
```

This is a worked example, not a fixed layout — Home is composed of independent sections (a Continue row, a Following Updates row, a per-category new-content row, a global adaptive feed, and so on), and every section is:

- **reorderable** — the user can change section order,
- **hideable** — the user can remove a section entirely, and
- **replaceable** — a section can be swapped for a different one.

The last section shown in the example, the Global Feed, is described as an *adaptive infinite feed* — this is the section that draws on the ranking/clustering pipeline documented in `../architecture/FEEDS.md`, surfaced at the bottom of an otherwise curated, state-driven Home screen.

## Feed filtering

Feeds are filterable through a two-tier chip system (kelpie.md §61).

**Primary chips** (medium-level, expected to be visible by default):

- All
- Video
- GIFs
- Images
- Manga
- Comics
- Cartoons
- Stories
- Audio

**Advanced chips** (finer-grained, for narrowing within or across those media):

- Animation
- Illustration
- Photography
- Booru
- Imageboard
- Webcomic
- Live

The two tiers serve different jobs. Primary chips map directly onto Kelpie's structural media families and the MEDIA navigation group described in [`INFORMATION-ARCHITECTURE.md`](./INFORMATION-ARCHITECTURE.md), letting a user collapse a mixed feed down to one medium in a single tap. Advanced chips cut across that grouping — several of them (Booru, Imageboard, Webcomic, Live) identify a *presentation/source style* rather than a structural medium, and others (Animation, Illustration, Photography) refine what kind of visual content is meant within the image/gallery-shaped media. Because advanced chips are not medium-exclusive, they are expected to be usable in combination with each other and with a primary chip, rather than being mutually exclusive single-select options.

## Related documents

- [`../architecture/FEEDS.md`](../architecture/FEEDS.md) — feed pipeline architecture, ranking, ranking modes, and clustering (kelpie.md §58, §62–64).
- [`INFORMATION-ARCHITECTURE.md`](./INFORMATION-ARCHITECTURE.md) — the navigation and route context Home and the other feed modes sit within.
- [`CARDS.md`](./CARDS.md) — the card system feeds render.
- [`PRIVACY.md`](./PRIVACY.md) — how feed interaction signals are handled (or not persisted) during Private Mode.
