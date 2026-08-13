# ADR 0007: Separate Structure, Style, and Classification as Three Independent Axes

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Context

Kelpie aggregates media across a very wide range of formats and presentation styles — live-action video, GIFs, photography, illustration, manga, comics, prose fiction, audio, booru posts, imageboard threads, games, and more — sourced from many different provider websites, each with its own categorization conventions. kelpie.md §12 (Core Media Taxonomy) states the problem and resolution directly: "Separate three concepts: Structure — what the object technically is; Style — how it is visually presented; Classification — how the adult source categorizes it," and gives worked examples — Structure `manga` / Style `manga/anime` / Classification `adult, hentai`; Structure `animation` / Style `cartoon` / Classification `adult`. The stated rationale: "This prevents an explosion of redundant media types."

§13 (Structural Media Types) defines the `MediaKind` union — video/short_video/clip, gif/animated_image/animation, image/illustration/photograph/gallery, manga/comic/webcomic/comic_strip, story/serialized_text/article, audio/audio_story, booru_post, imageboard_thread/imageboard_post, live, game/visual_novel, external. §14 (Media Style) defines a separate `MediaStyle` union — photographic/realistic/illustrated/anime/manga/cartoon/3d/mixed/other — and explicitly notes "Style does not control which viewer is selected." §15 (Adult Classification) defines a third, independent `ContentClassification` interface (`adult: true`, normalized `labels[]`, raw `providerLabels[]`, optional `style?: MediaStyle[]`), with normalized labels including `adult`, `hentai`, `adult-manga`, `adult-comic`, `adult-cartoon`, `adult-animation`, `erotic-fiction`, `audio-erotica`.

## Decision

Model media along three independent axes rather than one flat media-type enum: a **Structural** type (`MediaKind`) determining what the object technically is and which viewer/player handles it, an orthogonal **Style** type (`MediaStyle`) describing visual presentation only (and never controlling viewer selection), and a separate **Classification** structure (`ContentClassification`) capturing both Kelpie's normalized adult-content labels and the source's own raw provider labels.

## Alternatives

kelpie.md presents the three-axis split as the direct fix for a named problem rather than choosing between two axis models; the rejected alternative is implicit in its own rationale:

- **A single flat media-type enum** covering structure, style, and classification together (e.g. separate types like `hentai_manga`, `cartoon_video`, `photo_gallery`, `hentai_video`, ...) — rejected because it multiplies combinatorially as new style/classification combinations appear, produces "an explosion of redundant media types" (kelpie.md §12), and conflates concerns that change independently: a provider's raw classification label can change without the underlying media structure changing, and a work's visual style is independent of both.

## Consequences

- Adding a new visual style (e.g. a new `MediaStyle` variant) or a new normalized classification label never requires adding a new `MediaKind`, and vice versa — the three unions can be extended independently as new provider conventions are encountered.
- Viewer/reader/player selection is driven solely by `MediaKind` (per the structure → viewer mapping implied by kelpie.md §3, e.g. Manga → sequential reader, Story → text reader, Audio → audio player, Imageboard → thread viewer), so style metadata can never accidentally change which component renders an item — this is stated explicitly for style (kelpie.md §14) and is the practical payoff of keeping the axes separate.
- `ContentClassification` retaining both normalized `labels` and raw `providerLabels` means Kelpie can apply consistent cross-provider adult-content handling (safe screen, neutral previews, filtering) via the normalized labels while still preserving each provider's original categorization for debugging, registry review, and provenance.
- This taxonomy is the shared vocabulary the rest of the domain model, feeds, and library features build on; see [`../architecture/DOMAIN-MODEL.md`](../architecture/DOMAIN-MODEL.md) and the per-media-type docs under `../media/`.
- Getting this axis split wrong or incomplete early would be costly to unwind later, since normalized items, search indexing (kelpie.md §105), and collections (kelpie.md §95, which explicitly mixes structural kinds) all key off `MediaKind`.

## Status

Proposed — kelpie.md v4.0 documents this as an intended decision; no code exists yet that depends on it. This will move to Accepted once an implementation lands that relies on it, or Superseded/Rejected if reconsidered.
