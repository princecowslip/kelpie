# Viewers

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Overview

Kelpie renders every item through the specialized viewer appropriate to its structural medium, reached via the medium-agnostic `/item/:uid` and `/chapter/:uid` routes described in [`INFORMATION-ARCHITECTURE.md`](./INFORMATION-ARCHITECTURE.md). This document is intentionally short: it is an **index** into the per-medium viewer documents, plus the small set of rules that apply across all of them. Medium-specific detail — controls, modes, reader features, and player functions — lives entirely in the linked documents below and is not duplicated here.

## Per-medium viewers

| Medium | Document |
|---|---|
| Video | [`../media/VIDEO.md`](../media/VIDEO.md) |
| GIF | [`../media/GIF.md`](../media/GIF.md) |
| Images / Gallery | [`../media/IMAGES.md`](../media/IMAGES.md) |
| Booru | [`../media/BOORU.md`](../media/BOORU.md) |
| Imageboard | [`../media/IMAGEBOARD.md`](../media/IMAGEBOARD.md) |
| Manga | [`../media/MANGA.md`](../media/MANGA.md) |
| Comics | [`../media/COMICS.md`](../media/COMICS.md) |
| Cartoons | [`../media/CARTOONS.md`](../media/CARTOONS.md) |
| Literature | [`../media/LITERATURE.md`](../media/LITERATURE.md) |
| Audio | [`../media/AUDIO.md`](../media/AUDIO.md) |

## Cross-viewer rules

A small number of performance and safety rules are stated in the spec at a level that applies across viewers rather than to one medium in particular:

- **Lazy loading and memory-bounded decoding.** The Gallery Viewer's performance requirements (kelpie.md §80) call for lazy loading, near-image prefetch, and memory-bounded decoding — decoded image/frame data must not be allowed to grow unbounded as a user browses. The same discipline underlies the staged image pipeline (viewport request → memory cache → disk cache → network → decode → resize → render) described in `../architecture/OVERVIEW.md`, which every image-bearing viewer sits on top of.
- **Adaptive preloading.** The sequential reader's preload strategy (kelpie.md §83) — baseline decoding of the previous page, current page, and next two pages — is expected to adapt based on available RAM, page size, network speed, reading speed, and the active performance profile (see `../architecture/OVERVIEW.md` for the Battery Saver / Balanced / High Performance profiles). This same adapt-to-conditions principle is the general shape any viewer's prefetch behavior should follow, not just the sequential reader's.
- **Provider-supplied HTML must be sanitized.** Stated explicitly for the Literature Reader (kelpie.md §86), this rule is not literature-specific: any viewer that renders provider-supplied markup (chapter text, descriptions, embedded HTML) must sanitize it before rendering. See [`../security/CONTENT-SAFETY.md`](../security/CONTENT-SAFETY.md) for the sanitization mechanics and threat model this defends against.

## Related documents

- [`INFORMATION-ARCHITECTURE.md`](./INFORMATION-ARCHITECTURE.md) — the routes that lead into these viewers.
- [`CARDS.md`](./CARDS.md) — the card representation an item has before it is opened into a viewer.
- [`../architecture/OVERVIEW.md`](../architecture/OVERVIEW.md) — the image pipeline and performance profiles referenced above.
- [`../security/CONTENT-SAFETY.md`](../security/CONTENT-SAFETY.md) — provider-HTML sanitization and related content-safety mechanics.
