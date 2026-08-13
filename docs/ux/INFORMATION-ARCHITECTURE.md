# Information Architecture

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Overview

This document describes how Kelpie's screens are organized and addressed: the primary navigation tree, the URL/route model the frontend is expected to follow, the Library section's place in that tree, and — as a deliberate placement choice explained below — the top-level shape of Settings.

## Primary navigation

Kelpie's navigation is organized into five groups (kelpie.md §70):

```
HOME
  Home
  Discover
  Continue
  Search

MEDIA
  Video
  GIFs
  Images
  Manga
  Comics
  Cartoons
  Literature
  Audio

SOURCES
  All Sources
  Video
  Images & Booru
  Manga & Comics
  Literature
  Audio
  Creator Platforms
  Live
  + Add Source

LIBRARY
  Saved
  Series
  Collections
  Following
  History
  Downloads
  Local Files

SYSTEM
  Settings
```

A rule governs how this tree is expected to render: **empty groups disappear.** If a user has, for example, no audio sources configured and no audio items in their library, the Audio entries under MEDIA and SOURCES are not shown as empty/disabled placeholders — the group itself does not appear. This keeps the navigation proportional to what a given user's library and source configuration actually contains, rather than presenting a fixed menu sized for every possible medium and provider category.

The five groups serve distinct purposes:

- **HOME** is the discovery entry point — the mixed dashboard, the diversity-biased discovery feed, in-progress items, and search.
- **MEDIA** is medium-first browsing — entering by content type (video, GIFs, images, manga, and so on) rather than by source.
- **SOURCES** is source-first browsing — entering by provider or provider category, including the source-management action (`+ Add Source`).
- **LIBRARY** is the user's own provider-independent state — see below.
- **SYSTEM** currently holds only Settings.

## Route model

The frontend is expected to expose the following route shape (kelpie.md §71):

```
/
/discover
/continue

/search
/search/:savedSearch

/source/:provider
/source/:provider/search

/item/:uid

/series/:uid
/chapter/:uid

/library/saved
/library/series
/library/collections
/library/collection/:id
/library/following
/library/history
/library/downloads

/browser/:tab

/settings/*
```

A few structural points follow from this shape:

- `/item/:uid` is a single, medium-agnostic detail route — the UID resolves to whatever specialized viewer is appropriate for that item's structural media kind, rather than each medium having its own top-level item route.
- `/series/:uid` and `/chapter/:uid` are likewise shared across the sequential media families (manga, comics, and similar chaptered content), not duplicated per medium.
- `/source/:provider` and `/source/:provider/search` give every provider a consistent addressable browsing and search surface regardless of what medium it supplies.
- `/library/*` mirrors the LIBRARY navigation group one-to-one, with `/library/collection/:id` as the one parameterized member (an individual collection within `/library/collections`). Local Files is reachable under the Library section of the navigation tree but has no separately listed route above.
- `/browser/:tab` is the route for the isolated/embedded browser surface used for subscription-, DRM-, or otherwise browser-fallback sources.
- `/settings/*` is an open-ended subtree; see the Settings section below for what it is expected to contain.

## Library

The Library section is a fixed set of seven surfaces (kelpie.md §94):

- Saved
- Series
- Collections
- Following
- History
- Downloads
- Local Files

The defining property of the Library, stated explicitly in the spec, is that **it is provider independent.** Saved items, series tracking, collections, follow state, history, and download records belong to Kelpie's local database, not to any individual source — this is the concrete UX expression of the "local ownership" design principle described in `../PRODUCT.md`. A user's library composition and organization survives a source being removed, breaking, or disappearing entirely, because none of that state is keyed to the source's continued availability.

## Settings

> **Placement note:** The spec (kelpie.md §132) does not define a dedicated `docs/ux/SETTINGS.md` file, and no other UX document in this tree is a better structural fit for a settings *menu shape* than an information-architecture document. Settings is recorded here as the closest IA-shaped home for it. A future revision of the documentation tree may choose to split it into its own file if the settings surface grows detailed enough (per-setting behavior, validation rules, etc.) to outgrow this section.

Settings is organized as a flat top-level list with sub-groupings by concern (kelpie.md §130):

```
General
Appearance
Home
Feeds
Sources
Search

Video
GIFs
Images
Manga & Comics
Literature
Audio

Library
Downloads

Privacy
Network
Storage
Keyboard

Advanced
About
```

Three things to note about this structure:

1. **Cross-cutting settings first.** General, Appearance, Home, Feeds, Sources, and Search apply across the whole application rather than to a specific medium.
2. **Per-medium settings second.** Video, GIFs, Images, Manga & Comics, Literature, and Audio each get their own settings surface — this mirrors the medium-first grouping already used in the MEDIA navigation group above, so a user who thinks in terms of "how does my manga reader behave" has a matching settings page.
3. **Operational and system settings last.** Library, Downloads, Privacy, Network, Storage, Keyboard, Advanced, and About round out the list. Privacy settings here are the entry point for the privacy-related UX flows described in [`PRIVACY.md`](./PRIVACY.md) (Private Mode, Safe Screen, Neutral Preview Mode, Notification Privacy, Application Lock); Keyboard settings are the entry point for the bindings described in [`ACCESSIBILITY.md`](./ACCESSIBILITY.md).

The spec also requires that settings be searchable — a settings search surface is expected, not just the sectioned list above, so that a user does not have to know which section a given setting lives in to find it.

## Related documents

- [`CARDS.md`](./CARDS.md) — the card system and visual language used across MEDIA, SOURCES, and Library surfaces.
- [`FEEDS.md`](./FEEDS.md) — the Home screen's feed composition and filtering (the user-facing half; see `../architecture/FEEDS.md` for pipeline internals).
- [`VIEWERS.md`](./VIEWERS.md) — where `/item/:uid` and `/chapter/:uid` route to, by medium.
- [`PRIVACY.md`](./PRIVACY.md) — Private Mode, Safe Screen, and Application Lock, reachable from Settings → Privacy.
- [`ACCESSIBILITY.md`](./ACCESSIBILITY.md) — keyboard model, reachable from Settings → Keyboard.
