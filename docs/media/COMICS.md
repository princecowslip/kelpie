# Comics and Webcomics

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Scope of this document

This is a **thin delta document**. Comics, webcomics, and comic strips are read using the same shared reader engine as manga — reader modes (Single Page / Double Page / Continuous Vertical / Webtoon), reading direction, sequential preloading/performance behavior, and reading-progress tracking are all specified once, canonically, in [`MANGA.md`](MANGA.md) (kelpie.md §81–85) and are not restated here. This file only covers what is distinct about comics/webcomics as a structural and provider category.

## Structural types

Kelpie's taxonomy separates comics from manga at the structural level (kelpie.md §13, `MediaKind`):

- `comic`
- `webcomic`
- `comic_strip`

These are distinct `MediaKind` values from `manga`, reflecting that comics, webcomics, and manga are different sequential-art structures even though they may currently share the same reader implementation. Per Kelpie's taxonomy split of structure/style/classification (kelpie.md §12), a `comic`-structured work can independently carry any applicable style (e.g. `illustrated`, `cartoon`) and classification (e.g. `adult-comic`) — structure alone does not imply style.

## Why not "just manga"

The spec is explicit that comic/webcomic/strip sources "exercise different sequential-art structures and should not be forced through the manga assumptions" (kelpie.md §40). In practice this means: the reader UI and performance model are shared (see `MANGA.md`), but the *data modeling* of a comic source — its structural kind, and how its series/issue hierarchy is represented — is kept distinct from manga rather than aliased to it.

## Candidate comic/webcomic/cartoon sources

The spec's candidate provider set for this category is (kelpie.md §40):

- Filthy Figments
- Oglaf
- Slipshine

Recommended integration levels:

| Provider | Recommended integration |
|---|---|
| Filthy Figments | Integrated / Browser+ |
| Oglaf | Integrated / Browser+ |
| Slipshine | Browser+ initially |

Filthy Figments is called out specifically as remaining reachable as an adult-comics source and as a useful **reference provider for series/page-oriented integration** — i.e. a good template case when building out the series → chapter/issue → page integration pattern for comics generally.

Note the overlap with [`CARTOONS.md`](CARTOONS.md): this same candidate set (§40) is the source list for both comics and cartoons in the spec — see that document for the cartoon-specific angle (style/classification only; no separate cartoon viewer or reader exists).

## Relationship to provider presets

The full, canonical preset table for this category (all three providers, with any additional detail beyond the recommended-integration summary above) lives in [`../providers/REGISTRY.md`](../providers/REGISTRY.md) (kelpie.md §40 — Comic, Cartoon and Webcomic Presets). This document reproduces only the comic-relevant portion of that table for context; treat the registry as authoritative if the two ever diverge.
