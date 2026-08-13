# Domain Model

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

Kelpie normalizes media from many different provider websites into a single, provider-agnostic set of types. This document is the canonical reference for that normalized data model: the taxonomy that separates *what a piece of media technically is* from *how it looks* from *how the source classifies it*, the unified `MediaItem` interface, identity and availability, series/hierarchical structures, and tag normalization.

Everything here describes intended TypeScript-level shapes for the normalized model, not implemented code. For provider-facing concerns (capabilities, manifests, sandboxing), see [`PROVIDERS.md`](./PROVIDERS.md). For how tags surface in the booru-style UI specifically, see [`../media/BOORU.md`](../media/BOORU.md).

## Core Media Taxonomy

The domain model deliberately separates three distinct concepts so that the media-type system doesn't explode combinatorially:

- **Structure** — what the object technically is (e.g. manga, animation).
- **Style** — how it is visually presented (e.g. manga/anime, cartoon).
- **Classification** — how the adult source categorizes it (e.g. adult, hentai).

For example, one item might have Structure `manga`, Style `manga/anime`, Classification `adult, hentai`; another might have Structure `animation`, Style `cartoon`, Classification `adult`. Keeping these three axes independent prevents an explosion of redundant, overlapping media types (e.g. a separate "adult anime manga" type distinct from "adult manga").

## Structural Media Types

The `MediaKind` union enumerates every structural media type Kelpie's normalized model supports:

```ts
type MediaKind =
  | "video"
  | "short_video"
  | "clip"

  | "gif"
  | "animated_image"
  | "animation"

  | "image"
  | "illustration"
  | "photograph"
  | "gallery"

  | "manga"
  | "comic"
  | "webcomic"
  | "comic_strip"

  | "story"
  | "serialized_text"
  | "article"

  | "audio"
  | "audio_story"

  | "booru_post"

  | "imageboard_thread"
  | "imageboard_post"

  | "live"

  | "game"
  | "visual_novel"

  | "external";
```

## Media Style

`MediaStyle` captures how an item is visually presented, independent of its structural kind:

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

Style does not control which viewer is selected — that decision is driven by `MediaKind`, not `MediaStyle`.

## Adult Classification

Every item in Kelpie is adult content, but classification metadata still records how the source categorizes it, using normalized labels layered over the provider's own labels:

```ts
interface ContentClassification {
  adult: true;

  labels: string[];

  providerLabels: string[];

  style?: MediaStyle[];
}
```

Possible normalized labels include: `adult`, `hentai`, `adult-manga`, `adult-comic`, `adult-cartoon`, `adult-animation`, `erotic-fiction`, `audio-erotica`.

## Unified MediaItem

`MediaItem` is the single normalized representation that every provider adapter produces, regardless of source format:

```ts
interface MediaItem {
  uid: string;

  providerId: string;
  remoteId?: string;

  canonicalUrl: string;

  kind: MediaKind;

  title: string;
  description?: string;

  creators: CreatorCredit[];

  thumbnails: ImageResource[];

  tags: TagReference[];

  classification: ContentClassification;

  publishedAt?: string;
  updatedAt?: string;

  durationSeconds?: number;

  dimensions?: {
    width?: number;
    height?: number;
  };

  series?: SeriesReference;

  sequence?: {
    volume?: SequenceNumber;
    chapter?: SequenceNumber;
    issue?: SequenceNumber;
    pageCount?: number;
  };

  availability: Availability;

  mediaSources?: MediaSource[];

  providerMetadata?: Record<string, unknown>;
}
```

## Stable Identity

The preferred stable identity for a `MediaItem` is a composite key:

```
provider-id : object-type : remote-id
```

For example: `provider-x:manga-chapter:38172`.

When a provider doesn't expose a stable remote ID, identity falls back to:

```
SHA256(provider-id + canonical-url)
```

Title alone must never be treated as identity — titles are not guaranteed unique or stable across a provider's catalog.

## Availability Model

`Availability` describes whether and how an item can currently be accessed, so the UI can inform the user before they attempt playback:

```ts
type Availability =
  | "direct"
  | "embedded"
  | "browser"
  | "login-required"
  | "subscription-required"
  | "temporarily-unavailable"
  | "removed"
  | "unknown";
```

## Media Sources

A `MediaItem` may resolve to one or more playable/renderable sources:

```ts
interface MediaSource {
  url: string;

  transport:
    | "http"
    | "file"
    | "hls"
    | "dash"
    | "browser";

  mimeType?: string;

  width?: number;
  height?: number;

  bitrate?: number;

  label?: string;

  expiresAt?: string;
}
```

Ephemeral signed URLs should be resolved on demand rather than persisted as canonical identity — `MediaSource.url` may expire and needs to be re-resolved through the provider rather than treated as permanent.

## Series Model

Manga, comics, webcomics, and episodic audio all need a hierarchical structure above the individual `MediaItem`:

```
Series
 ├── Volume
 │    └── Chapter / Issue
 │          └── Page
```

or, for audio:

```
Audio Series
 └── Episode / Chapter
```

## Series Object

The `Series` object represents the top of that hierarchy:

```ts
interface Series {
  uid: string;

  providerId: string;
  remoteId?: string;

  canonicalUrl: string;

  title: string;
  alternateTitles: LocalizedTitle[];

  description?: string;

  creators: CreatorCredit[];

  cover?: ImageResource;

  tags: TagReference[];

  status?:
    | "ongoing"
    | "completed"
    | "hiatus"
    | "cancelled"
    | "unknown";

  readingDirection?:
    | "ltr"
    | "rtl"
    | "vertical";
}
```

## Sequence Numbers

Chapter/volume/issue numbering cannot be assumed to be plain integers. Sources may expose values like `10`, `10.5`, `10a`, `Special`, `Bonus`, `Prologue`, or `Epilogue`. To accommodate this, sequence positions are represented as:

```ts
interface SequenceNumber {
  raw: string;
  numeric?: number;
  sortKey: string;
}
```

The raw source label remains visible to the user even when a numeric or sort-key value has also been derived for ordering purposes.

## Tag Normalization

Tags are normalized while preserving enough information to trace back to the original source. Kelpie preserves, per tag:

- normalized identity
- provider-local identity
- display value
- category
- aliases

This is represented as a `TagReference`, which is what populates `MediaItem.tags` and `Series.tags` above:

```ts
interface TagReference {
  normalizedId?: string;
  providerId: string;
  sourceValue: string;
  displayName: string;
  category?: string;
}
```

`normalizedId` is optional because not every provider-local tag can necessarily be mapped to a normalized identity; `sourceValue` and `displayName` preserve what the provider actually said even when normalization succeeds. For how normalized tags are presented and filtered in the booru-style browsing UI, see [`../media/BOORU.md`](../media/BOORU.md).
