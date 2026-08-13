# Generic HTML Sources

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Overview

Rather than requiring every source to ship a full provider package, kelpie.md §55 (Custom Source Builder) defines a four-level ladder that a user or provider author can climb depending on how much structure a source needs:

```
Level 1 — Feed               (RSS / Atom / JSON Feed)
Level 2 — Generic Adapter    (Gallery, Manga, Comic, Booru, Literature)
Level 3 — Visual Site Builder (CSS selectors and visual extraction)
Level 4 — Provider SDK       (sandboxed code package)
```

This document covers Levels 1 through 3 — the no-code and low-code path from "here is a feed URL" to "here is a site with no feed at all, mapped by pointing at it." Level 2's Booru and Manga adapters have their own dedicated documents ([`GENERIC-BOORU.md`](./GENERIC-BOORU.md), [`GENERIC-MANGA.md`](./GENERIC-MANGA.md)) because each has enough engine-specific mechanics to warrant separate treatment. Level 4, the full sandboxed Provider SDK, is covered in [`SDK.md`](./SDK.md).

## Level 1 — Feed

The simplest tier: a source that already exposes a standard syndication feed. Kelpie is specified to ship built-in support (kelpie.md §49, Generic Built-In Sources) for:

- RSS
- Atom
- JSON Feed

No mapping or extraction work is required from the user beyond supplying the feed URL — the format is already structured.

## Level 2 — Generic Adapter

Where a source doesn't expose a feed but follows a recognizable structural pattern, kelpie.md §55 specifies generic adapters for:

- Gallery
- Manga
- Comic
- Booru
- Literature

These adapters trade some of the flexibility of a full provider package for a much lower authoring cost — the adapter already knows the shape of a gallery, or a manga series/chapter/page structure, or a booru's tag/post model, and the user or author fills in the source-specific details rather than writing extraction logic from scratch. See [`GENERIC-BOORU.md`](./GENERIC-BOORU.md) for the Booru (and, since there is no separate generic-imageboard document, Imageboard) adapter mechanics, and [`GENERIC-MANGA.md`](./GENERIC-MANGA.md) for the Manga adapter's mapping fields.

## Level 3 — Visual Site Builder

For a source with no feed and no recognized structural pattern, kelpie.md §56 (Visual Site Builder) specifies a three-pane interface:

```
┌──────────────────┬────────────────────┬──────────────────┐
│ Mapping          │ Website Preview    │ Parsed Results   │
│                  │                    │                  │
│ Item selector    │ live page          │ Result 1         │
│ Title            │                    │ Result 2         │
│ Link             │                    │ Result 3         │
│ Thumbnail        │                    │                  │
│ Creator          │                    │                  │
│ Next page        │                    │                  │
└──────────────────┴────────────────────┴──────────────────┘
```

The user clicks elements directly on the live page preview to assign them semantic roles (item selector, title, link, thumbnail, creator, next-page control), and the parsed-results pane updates live so the mapping can be verified against real page content before the source is enabled. This is, functionally, a CSS-selector/visual-extraction builder — it produces a source definition without the user writing code.

## Level 4 — beyond the builder

When a source needs logic the visual builder can't express — pagination schemes, authentication flows, or media resolution beyond simple selector extraction — the next step up is a full Provider SDK package (kelpie.md §26, Provider Execution API). See [`SDK.md`](./SDK.md) for that interface, and [`PERMISSIONS.md`](./PERMISSIONS.md) for the sandbox such a package runs under.

## Security

Because Level 1–3 sources render content that ultimately comes from arbitrary third-party HTML the user has pointed Kelpie at, how that HTML is parsed and displayed safely is governed separately by kelpie.md §119 — see [`../security/CONTENT-SAFETY.md`](../security/CONTENT-SAFETY.md) for the content-safety model that applies regardless of which builder level produced the source definition.

## Related documents

- [`GENERIC-BOORU.md`](./GENERIC-BOORU.md) — Level 2 Booru and Imageboard adapter mechanics.
- [`GENERIC-MANGA.md`](./GENERIC-MANGA.md) — Level 2 Manga adapter mapping fields.
- [`SDK.md`](./SDK.md) — Level 4, the full Provider SDK.
- [`REGISTRY.md`](./REGISTRY.md) — the generic built-in source families (kelpie.md §49) these levels back.
- [`../security/CONTENT-SAFETY.md`](../security/CONTENT-SAFETY.md) — HTML rendering security (kelpie.md §119).
