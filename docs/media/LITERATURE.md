# Literature Reader

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Overview

The Literature Reader covers Kelpie's text-based structural media kinds — `story`, `serialized_text`, and `article` (kelpie.md §13). Unlike the paginated-image reader engine used for manga/comics (see [`MANGA.md`](MANGA.md)), this is a text-reflow reader.

## Reader features

The Literature Reader is specified to provide (kelpie.md §86):

- Reader mode (a distraction-reduced reading layout)
- Font (selection)
- Size (font size)
- Line height
- Paragraph spacing
- Column width
- Theme
- Find (in-text search)
- Bookmark
- Chapter navigation
- Progress (reading progress tracking, analogous in spirit to the manga/comic reading-progress model in `MANGA.md` §84, though the spec does not specify an identical stored-field schema for text)
- Reading-time estimate
- Open original

### Content safety requirement

Because literature providers commonly supply **HTML** content (formatted story text, potentially including provider-authored markup), the spec places an explicit, non-optional safety requirement on this viewer: **provider-supplied HTML must be sanitized** before rendering (kelpie.md §86). This is a content-safety boundary, not a formatting nicety — unsanitized third-party HTML rendered in-app would be a script-injection / tracking risk.

## Literature Presets

The primary literature sources named in the spec are (kelpie.md §41):

- Literotica
- Lush Stories

### Literotica as an architectural case study

Literotica is called out specifically — not primarily for its content, but because its breadth of features makes it a strong test case for Kelpie's mixed-media normalization model. The spec notes that Literotica's current service exposes stories, categories, tags, authors, audio, and interactive story games all within one ecosystem, which stresses Kelpie's provider abstraction more than a single-content-type source would.

The recommended Literotica capability set to model (kelpie.md §41):

- feed
- search
- story
- creator
- tags
- series
- audio
- interactive-fiction
- related
- browser

Because a single provider here spans text (`story`), audio (`audio`/`audio_story`), and effectively game-like content (`interactive-fiction`), it exercises Kelpie's structure/style/classification taxonomy (kelpie.md §12) and its cross-media provider model more thoroughly than most other candidate sources — the spec calls this out as making it "one of the best early providers for testing mixed-media normalization."

For audio-erotica specifically (including where Literotica's own audio content fits relative to dedicated audio-erotica providers), see [`AUDIO.md`](AUDIO.md).

## Relationship to provider presets

The full canonical registry entries for Literotica and Lush Stories (integration levels, endpoints, etc.) live in [`../providers/REGISTRY.md`](../providers/REGISTRY.md) (kelpie.md §41 — Literature Presets). The capability list above is reproduced here because it is directly useful as an architecture reference for how the reader/provider model handles a multi-capability source; treat the registry as authoritative for anything beyond that summary.
