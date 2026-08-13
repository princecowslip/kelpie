# Cartoons

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Scope of this document

Kelpie's spec does not define a dedicated cartoon viewer, reader, or structural media kind. "Cartoon" appears only as a **style** value and as **classification labels** — it is a descriptive axis applied to other structural kinds (`video`, `animation`, `comic`, `webcomic`, etc.), not a viewer of its own. This document is intentionally short: it states what the spec actually says about cartoons and cross-references where cartoon-relevant content actually lives.

## Media Style

`cartoon` is one value of the `MediaStyle` enum (kelpie.md §14):

```ts
type MediaStyle =
  | "photographic"
  | "realistic"
  | "illustrated"
  | "anime"
  | "manga"
  | "cartoon"
  | "3d"
  | "mixed"
  | "other";
```

Per kelpie.md §12/§14, style does not control which viewer is selected — it is metadata describing how a piece of content looks, layered on top of its structural `MediaKind` (video, comic, webcomic, animation, etc.). A cartoon-styled work is viewed using whichever viewer matches its actual structure (e.g. the Video Viewer for cartoon-styled animation, the manga/comic reader engine documented in [`MANGA.md`](MANGA.md) for cartoon-styled comics/webcomics).

## Adult classification labels

The `ContentClassification` interface's normalized `labels` set (kelpie.md §15) includes two cartoon-relevant values:

- `adult-cartoon`
- `adult-animation`

These sit alongside the other normalized labels (`adult`, `hentai`, `adult-manga`, `adult-comic`, `erotic-fiction`, `audio-erotica`) as part of the same classification vocabulary, distinguishing cartoon/animation-style adult content from, e.g., manga- or comic-style adult content at the metadata level.

## Navigation and filtering presence

Despite having no dedicated viewer or structural kind, "Cartoons" is not merely a background metadata value — kelpie.md gives it real, first-class prominence in the application's information architecture:

- **§70 Navigation**: "Cartoons" is one of the top-level MEDIA navigation items, listed as a peer to Video, GIFs, Images, Manga, Comics, Literature, and Audio (kelpie.md §70) — the same navigation tier documented for those media types across [`VIDEO.md`](VIDEO.md), [`IMAGES.md`](IMAGES.md), [`MANGA.md`](MANGA.md), [`LITERATURE.md`](LITERATURE.md), and [`AUDIO.md`](AUDIO.md).
- **§61 Feed Filtering**: "Cartoons" appears among the **primary** filter chips (`All, Video, GIFs, Images, Manga, Comics, Cartoons, Stories, Audio`), not the secondary "Advanced" row. Notably, "Animation" is a separate, Advanced-tier chip — kelpie.md treats "Cartoons" (a style/classification-driven grouping) and "Animation" (closer to a structural grouping) as two distinct filters rather than synonyms.
- **§1 Product Summary**: "Adult cartoon/illustration browser" is listed among the constituent identities the whole application combines (alongside "Manga reader," "Comic reader," "Booru client," etc.) — cartoons are treated as one of the product's named use cases, not an afterthought of the classification schema.

This means a user-facing "Cartoons" surface is expected to exist in the nav and in feed filtering even though kelpie.md never defines a dedicated `CartoonViewer` or cartoon-specific route (§71 Route Model has no cartoon-specific entry). The practical implication is that a "Cartoons" nav/filter selection is expected to route through the same generic card and viewer machinery already used for cartoon-*styled* content of whatever underlying structural kind it actually is — the Video Viewer for cartoon-styled animation, or the shared reader engine documented in [`MANGA.md`](MANGA.md) for cartoon-styled comics/webcomics — consistent with the "style does not control which viewer is selected" rule in the [Media Style](#media-style) section above.

## Provider presets

There is no cartoon-specific preset table in the spec. The candidate provider set that is cartoon-relevant is the same one covering comics and webcomics (kelpie.md §40 — Comic, Cartoon and Webcomic Presets): Filthy Figments, Oglaf, and Slipshine. See [`COMICS.md`](COMICS.md) for the recommended-integration detail on those three, and [`../providers/REGISTRY.md`](../providers/REGISTRY.md) for the canonical registry entry. This document does not duplicate that table.
