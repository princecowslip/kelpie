# Cards and Visual Language

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Overview

This document covers two related things: the card system that is Kelpie's dominant unit of visual browsing (used across the global feed, per-medium browsing, source browsing, and Library), and the app-wide visual design language — target aesthetic, things to avoid, and the design-token palette.

> **Placement note:** kelpie.md's prescribed documentation tree (§132) has no dedicated design-system file. Visual Design and Visual Tokens (§74–75 of the spec) describe app-wide look-and-feel, not something specific to cards — but because the card is the single most repeated visual unit in the application, this document is the closest fit in the current tree and is where those sections land. A future revision of the documentation tree may want to split visual tokens and the general design language out into their own `docs/ux/DESIGN-SYSTEM.md` (or similar) once the token set grows beyond what fits comfortably alongside the card spec.

## Card system

Every card in Kelpie, regardless of medium or source, is expected to share a common set of elements (kelpie.md §72):

- Visual preview
- Media indicator
- Title
- Creator
- Source
- Secondary metadata

This shared shape is what lets a single feed, a single search result list, or a single collection mix video, GIFs, images, manga, comics, and audio without each medium needing its own distinct card layout — the frontend can render one generic card component driven by the normalized media model (see `../architecture/DOMAIN-MODEL.md`), varying only the badge.

### Media-specific badges

The media indicator is medium-specific. The spec gives the following worked examples of what that badge shows per medium:

| Medium | Badge example |
|---|---|
| Video | `21:43` (duration) |
| GIF | `GIF` |
| Manga | `Ch. 18 · 32p` (chapter and page count) |
| Comic | `Issue #7` |
| Story | `18 min read` (estimated reading time) |
| Audio | `26 min` (duration) |
| Gallery | `42 images` (image count) |
| Live | `LIVE` |

The pattern is consistent: the badge surfaces whichever single piece of metadata is most useful for deciding, at a glance, whether to open that item — duration for time-based media, a count for paginated/collected media, and a status word (`LIVE`) where the item is a live stream rather than fixed content.

## Display modes

Cards can be laid out in four display modes (kelpie.md §73):

- **Comfortable**
- **Compact**
- **List**
- **Mosaic**

Display-mode preference is not necessarily a single global setting — the spec allows it to be stored at three different scopes:

- globally
- per provider
- per media type

This lets a user, for example, browse a booru source in a dense mosaic while keeping their manga library in a comfortable card view, without those two preferences conflicting.

## Visual design

Kelpie's stated design target (kelpie.md §74):

> "Discreet premium media software."

This target is defined as much by what to avoid as by what to aim for.

**Avoid:**

- The red/black adult-site stereotype
- Banner-advertising aesthetics
- Flashing UI
- Gratuitous sexualized chrome
- Excessive gradients

**Prefer:**

- Neutral surfaces
- Media-forward cards
- Subtle borders
- Strong typography
- Short motion

This is the concrete UX expression of the "discreet" design principle from `../PRODUCT.md` — the application's chrome should read as general-purpose premium media software (closer to a well-designed media library or streaming client) rather than announcing its content category through its interface style.

## Visual tokens

The spec provides a worked example dark-theme token palette (kelpie.md §75). It is reproduced here as a concrete reference point, not as a final locked palette:

| Token | Value |
|---|---|
| Canvas | `#090B0F` |
| Surface | `#101419` |
| Raised | `#181D24` |
| Border | `#29313A` |
| Primary Text | `#F2F4F3` |
| Secondary | `#A7B0AE` |
| Muted | `#687370` |
| Accent | `#9B7BFF` |
| Accent | `#7ED6C4` |
| Accent | `#C47AFF` |
| Accent | `#FF6F91` |
| Accent | `#769CFF` |
| Accent | `#B7E05A` |
| Success | `#55C995` |
| Warning | `#E7B45D` |
| Danger | `#E46772` |

Surface tokens (Canvas → Surface → Raised → Border) form an elevation ramp from the base background up through progressively raised/bordered UI layers. Text tokens (Primary → Secondary → Muted) form a similar de-emphasis ramp. The six Accent values are not a strict single-purpose set in the spec — they read as a small accent palette available for categorization, highlighting, and similar uses — while Success, Warning, and Danger are reserved for status semantics.

The spec is explicit that this dark palette is an example, not the complete requirement: **a complete light theme must also ship.** No light-theme token values are given in the spec; producing them is left as a design task that should follow the same elevation/text/accent/status structure shown above.

## Related documents

- [`INFORMATION-ARCHITECTURE.md`](./INFORMATION-ARCHITECTURE.md) — where cards appear across the navigation and route tree.
- [`FEEDS.md`](./FEEDS.md) — the feed surfaces that render cards in bulk.
- [`VIEWERS.md`](./VIEWERS.md) — what opening a card leads to, by medium.
- [`ACCESSIBILITY.md`](./ACCESSIBILITY.md) — non-color-only status cues and high-contrast requirements that constrain how these tokens may be used.
