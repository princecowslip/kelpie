# ADR 0008: Favorites, History, Progress, Collections, and Follow State Are Owned Locally by Kelpie

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Context

kelpie.md's design principles open with "Local first: Core state resides locally" (kelpie.md §6), and §3.3 (Local ownership) states the concrete scope of that principle directly: "Favorites, history, progress, collections and follow state belong to Kelpie rather than individual websites." §3.4 (Resilience) adds a related consequence: "Provider breakage must be isolated" — the app's own record of what the user has favorited, watched, read, or followed must not depend on any single provider staying reachable.

§94 (Library) confirms this architecturally: the library — Saved, Series, Collections, Following, History, Downloads, Local Files — "is provider independent." §95 (Collections) describes collections as able to mix structurally different media kinds (video, GIF, gallery, manga, comic, story, audio, external page, local file) with several sort orders, which only makes sense if collection membership is Kelpie-native state rather than something read back from a provider. §97 (Following) allows following providers, creators, series, tags, or saved searches, feeding a dedicated feed and personalized ranking. §98 (History) tracks first/last opened, open count, progress, and completion, with configurable retention (Forever/90 days/30 days/7 days/Session only/Never). §99 (Private Mode) specifies that during private mode, searches, opened items, media/reader progress, browser history, feed interaction signals, and temporary private session state must not be persisted — while "Existing normal history remains untouched," implying normal history is itself a durable, locally-owned record that private mode deliberately does not touch or delete.

## Decision

Store favorites, history, progress, collections, and follow state as first-class local data owned by Kelpie's own database, independent of any provider — not derived from, cached from, or dependent on a provider website's own account/history features — so that this state survives provider breakage, works uniformly across structurally different media kinds, and is subject to Kelpie's own retention and privacy controls (including private mode) rather than a provider's.

## Alternatives

kelpie.md frames local ownership as the resolution to a resilience problem (§3.4) rather than presenting a named rejected design, but the alternative it argues against is clear from context:

- **Relying on each provider's own account features for favorites/history/follow state** (i.e. reading a user's watch history, favorites, or follows back from the provider's website/API per session) — rejected because it would make this state only as available as the weakest currently-reachable provider, defeats the "provider independent" library requirement (kelpie.md §94), cannot represent cross-provider collections that mix structurally different media kinds (kelpie.md §95), and would make private mode's per-session non-persistence guarantee (kelpie.md §99) impossible to enforce uniformly, since some providers would keep their own server-side history regardless of Kelpie's local mode.

## Consequences

- The Database Domains list (kelpie.md §104) accordingly includes `favorites`, `collections`/`collection_items`, `follows`, `history`, and `progress` as first-class local tables, independent of any `providers`/`provider_state` tables — this ADR is the rationale for that schema shape; see [`0002-sqlite.md`](./0002-sqlite.md) and [`../architecture/DOMAIN-MODEL.md`](../architecture/DOMAIN-MODEL.md).
- Because collections can mix video/GIF/gallery/manga/comic/story/audio/external-page/local-file items (kelpie.md §95), collection membership must be modeled against Kelpie's own normalized item identity, not against any single provider's native grouping feature.
- History gets its own configurable retention policy (Forever down to Session-only or Never — kelpie.md §98) and a private-mode carve-out that suspends new persistence without touching existing history (kelpie.md §99) — both are only possible because this state is locally owned and not mirrored from external accounts.
- If a provider is quarantined, demoted, or goes offline entirely (kelpie.md §30, §33), the user's favorites/history/progress/follows for content from that provider remain intact in Kelpie's own database, consistent with the resilience principle that "provider breakage must be isolated" (kelpie.md §3.4).
- This local-ownership guarantee is a privacy commitment as much as a resilience one: it is the basis for safe-screen, neutral-preview, and notification-privacy features (kelpie.md §100–102) operating on data Kelpie fully controls rather than data partially held by third-party sites.

## Status

Proposed — kelpie.md v4.0 documents this as an intended decision; no code exists yet that depends on it. This will move to Accepted once an implementation lands that relies on it, or Superseded/Rejected if reconsidered.
