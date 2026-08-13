# Provider SDK

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Overview

A Kelpie provider is the piece of code that converts a remote source into normalized Kelpie objects (kelpie.md §23). Providers are expected to come in several flavors — a specific website, a site family/protocol adapter, a generic HTML adapter, a generic feed adapter, a browser-only integration, or a local-filesystem source — but every flavor is expected to be reachable through the same execution contract described below.

This document is the developer-facing reference for that contract: the `Provider` interface itself and the in-app tooling (Provider Developer Mode) used to build and debug an implementation of it. For the declarative side of a provider (what it claims about itself before any code runs), see [`MANIFEST.md`](./MANIFEST.md) and [`CAPABILITIES.md`](./CAPABILITIES.md). For what a provider is and isn't allowed to do at runtime, see [`PERMISSIONS.md`](./PERMISSIONS.md).

## The Provider interface

kelpie.md §26 (Provider Execution API) describes the interface conceptually as follows:

```ts
interface Provider {
  initialize(context: ProviderContext): Promise<void>;

  feeds?(): Promise<FeedDescriptor[]>;

  feed?(
    request: FeedRequest
  ): Promise<Page<MediaItem>>;

  search?(
    request: SearchRequest
  ): Promise<Page<SearchResult>>;

  item?(
    request: ItemRequest
  ): Promise<MediaItem>;

  series?(
    request: SeriesRequest
  ): Promise<Series>;

  chapters?(
    request: ChapterRequest
  ): Promise<Page<Chapter>>;

  pages?(
    request: PageRequest
  ): Promise<PageManifest>;

  resolveUrl?(
    request: ResolveUrlRequest
  ): Promise<ResolvedObject | null>;

  media?(
    request: MediaRequest
  ): Promise<MediaSource[]>;

  related?(
    request: RelatedRequest
  ): Promise<Page<MediaItem>>;
}
```

`initialize` is the only method every provider must implement. Every other method is optional (`?`), and which ones a given provider implements is expected to line up with the `capabilities` array it declares in its manifest (see [`CAPABILITIES.md`](./CAPABILITIES.md)) — the frontend is not meant to call a method a provider hasn't declared support for, and a provider is not meant to declare a capability it doesn't implement.

The methods map roughly onto Kelpie's structural concepts:

- `feeds` / `feed` — discovery surfaces the provider exposes (kelpie.md §58–§59, Global Feed Architecture / Feed Modes).
- `search` — the provider's contribution to unified search (kelpie.md §65–§67).
- `item` — a single normalized `MediaItem` (kelpie.md §16, Unified MediaItem).
- `series`, `chapters`, `pages` — the Series → Chapter → Page model used by manga, comics, and similar sequential media (kelpie.md §20–§22).
- `resolveUrl` — turning an arbitrary URL a user pasted or navigated to into a resolved Kelpie object.
- `media` — resolving playable/viewable `MediaSource` variants for an item.
- `related` — related-item recommendations sourced from the provider itself.

kelpie.md §26 notes one cross-cutting requirement: **requests should support cancellation.** Every one of the request types above is expected to be abortable mid-flight (for example, when a user navigates away or the frontend supersedes a stale request with a newer one).

## Provider Developer Mode

kelpie.md §116 describes a Provider Developer Mode intended to give provider authors direct visibility into how their code is being executed and interpreted by Kelpie. The specified toolset is:

- Provider console
- Network inspector
- HTML inspector
- Normalized output viewer
- Manifest viewer
- Permission viewer
- Fixture recorder
- Parser tester

The spec gives a representative console transcript to illustrate the kind of feedback Developer Mode is meant to surface after invoking a provider method:

```
GET feed
200 OK
324 ms

Parsed
  24 items

Warnings
  2 missing dates
  1 missing thumbnail
```

This kind of parsed-item-count-plus-warnings output is meant to make it obvious, during development, when a provider is technically returning `200 OK` responses but silently degrading the quality of its normalized output (missing dates, missing thumbnails, and similar partial-field issues) rather than only surfacing hard failures.

## Related documents

- [`MANIFEST.md`](./MANIFEST.md) — the manifest a provider ships alongside its code.
- [`CAPABILITIES.md`](./CAPABILITIES.md) — the capability strings that gate which `Provider` methods the frontend will call.
- [`PERMISSIONS.md`](./PERMISSIONS.md) — the sandbox and network-permission model a provider executes under.
- [`TESTING.md`](./TESTING.md) — provider fixture and integration testing.
- [`../architecture/PROVIDERS.md`](../architecture/PROVIDERS.md) — the conceptual/architectural view of the provider platform, including lifecycle, health, updates, and rollback.
