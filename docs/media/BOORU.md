# Booru Support and Browser

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Overview

Boorus are tag-driven image/post boards. Kelpie's approach to booru support is deliberately generic: rather than shipping a long list of officially endorsed adult boorus, Kelpie ships **booru engine/provider families** — protocol-level adapters for known booru software, plus generic fallbacks (kelpie.md §45). This document covers the `booru_post` structural media kind (kelpie.md §13), its viewer, and how new boorus are onboarded. For the canonical list of specific booru-related presets, see [`../providers/REGISTRY.md`](../providers/REGISTRY.md).

## Booru engine families

Built-in engine adapters (kelpie.md §45):

- Danbooru-compatible
- Gelbooru-compatible
- Shimmie/Shimmie2-compatible
- Booru-on-Rails-compatible
- Generic JSON Booru
- Generic XML Booru

The user enters a domain, and Kelpie attempts engine detection rather than requiring a dedicated integration to exist for each specific board. This gives broad booru coverage without turning every compatible public board into an official, individually maintained preset.

## Booru auto-detection flow

Adding a booru source follows this flow (kelpie.md §46):

```
Enter domain
     ↓
Probe known API signatures
     ↓
Detect engine
     ↓
Map endpoints
     ↓
Fetch sample posts
     ↓
Validate tags/media
     ↓
Show permission review
     ↓
Enable
```

During validation, Kelpie surfaces a checklist of what it was able to confirm before the source is enabled, of the form:

```
API                    ✓
Search                 ✓
Tag metadata           ✓
Preview images         ✓
Original media         ✓
Pagination             ✓
Authentication         none
```

Each row reports whether that capability was successfully detected/validated (or, for Authentication, what auth mode — if any — the board requires) before the user is asked to review permissions and enable the source.

## Booru Browser

The booru browsing UI is specified to be **highly information-dense** — a deliberate departure from the card-oriented feed UI used elsewhere in Kelpie, reflecting how booru users actually work: fast tag-driven search over a dense image grid (kelpie.md §88).

Key elements:

- A tag search box supporting inclusion/exclusion syntax, e.g. `+tag1 +tag2 -tag3`
- A dense image grid layout:

```
┌────┬────┬────┬────┬────┐
│img │img │img │img │img │
├────┼────┼────┼────┼────┤
│img │img │img │img │img │
└────┴────┴────┴────┴────┘
```

- Optional tag categorization, where the source engine supports it:
  - creator
  - series
  - character
  - general
  - technical

## Tag model

Booru tags are not treated as opaque per-provider strings — they participate in Kelpie's cross-provider tag-normalization model, keyed off a `TagReference`-style construct. That model (including how a given board's raw tags map onto normalized/categorized tags) is architecturally shared across booru and imageboard sources and is documented canonically in [`../architecture/DOMAIN-MODEL.md`](../architecture/DOMAIN-MODEL.md) (kelpie.md §89). This document does not restate that schema.
