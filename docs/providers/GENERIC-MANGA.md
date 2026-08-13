# Generic Manga Source Builder

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Overview

The Manga Source Builder (kelpie.md §57) is the Level 2 Generic Adapter for manga-shaped sources described in [`GENERIC-HTML.md`](./GENERIC-HTML.md) — a mapping-driven builder that lets a user describe a site that follows the Series → Chapter → Page structure (kelpie.md §20–§22, Series Model) without writing a full provider package. It sits between the fully generic Visual Site Builder (Level 3, arbitrary CSS-selector mapping) and a hand-written provider (Level 4, the full Provider SDK) — it already understands manga's structural shape, so the user only needs to point it at the right elements.

## Mapping fields

kelpie.md §57 specifies the following fields a user maps when defining a manga source:

**Series-level**

- Series URL
- Series title
- Alternate title
- Cover
- Description

**Chapter-level**

- Chapter list
- Chapter label
- Chapter URL

**Page-level**

- Page list
- Page image

**Navigation**

- Next chapter
- Previous chapter

**Reading**

- Reading direction

This mirrors the general Series → Chapter → Page object model used throughout Kelpie (kelpie.md §20–§22) closely enough that a completed mapping produces normalized `Series`, `Chapter`, and `PageManifest` objects compatible with the same reader surfaces a full manga provider would feed (see [`SDK.md`](./SDK.md) for the `series`, `chapters`, and `pages` methods on the `Provider` interface).

## Preview and validation

Before a mapped source is enabled, kelpie.md §57 specifies a preview step that reports what the mapping actually found on the live site:

```
Series detected           1
Chapters detected        42
Pages in sample chapter  31
Reading direction        RTL
```

This gives the user (or provider author) a concrete sanity check — a mapping that detects zero chapters, or a reading direction that doesn't match what the source actually uses, is visible before the source goes live rather than surfacing as broken reading behavior later.

## Related documents

- [`GENERIC-HTML.md`](./GENERIC-HTML.md) — the Custom Source Builder ladder this adapter belongs to (Level 2), and the Level 3 Visual Site Builder it sits above.
- [`REGISTRY.md`](./REGISTRY.md) — Generic Manga as a built-in source family (kelpie.md §49), and the Manga/Doujinshi/Hentai preset candidates (kelpie.md §39) this builder complements for sources not covered by an official preset.
- [`SDK.md`](./SDK.md) — the `series`, `chapters`, and `pages` methods a completed mapping ultimately feeds.
