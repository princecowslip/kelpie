# Architecture Overview

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

This is the entry point into the `architecture/` documentation tree. It describes Kelpie's overall system shape: the trust domains the application is divided into, the recommended technology stack, the major application services, the process model those services run under, the non-functional (performance) targets the system is designed against, and a final architectural summary tying discovery, the normalized media model, and the provider platform together.

For the normalized data types referenced throughout (media items, series, tags, availability), see [`DOMAIN-MODEL.md`](./DOMAIN-MODEL.md). For how providers are structured and operated, see [`PROVIDERS.md`](./PROVIDERS.md).

## Fundamental Trust Architecture

Kelpie is organized around three execution domains with different levels of trust and different capabilities. This separation is the foundation the rest of the architecture is built on — provider sandboxing, webview isolation, and the process model all exist to enforce these boundaries.

### Trusted Core

The Trusted Core consists of Rust application services. It can access:

- SQLite
- Local configuration
- Approved filesystem locations
- OS integration
- Secrets
- The media backend

### Restricted Provider Runtime

Provider code executes in a restricted runtime. It can access:

- Explicitly permitted origins
- Provider-specific storage
- Provider-specific session state
- Parsing APIs
- Provider logs

It cannot access:

- Arbitrary shell commands
- Arbitrary filesystem
- Other providers' sessions
- Native application commands

### Untrusted Web Content

Remote pages (embedded browser sessions, subscription content, CAPTCHAs, and similar) render in dedicated browser webviews and are treated as fully untrusted. They receive no privileged native IPC.

Tauri 2's current capability model allows permissions to be constrained to specific windows/webviews; a webview without a matching capability receives no IPC access. This is intended to be used as the hard boundary between Kelpie's trusted UI and remote sites. See [`../security/WEBVIEW-ISOLATION.md`](../security/WEBVIEW-ISOLATION.md) for the isolation mechanics.

## Recommended Technology Stack

**Native/application core**

- Rust
- Tokio
- SQLite

**Desktop shell**

- Tauri 2

**Frontend**

- React
- TypeScript
- Vite
- TanStack Query
- Zustand or Redux Toolkit

**Media**

- HTML5/WebView playback, with an optional mpv/libmpv fallback. mpv remains suitable as a secondary playback backend because it supports a broad set of video/audio formats and codecs.

**Search**

- SQLite FTS5. SQLite documents FTS5 as its full-text-search virtual-table module, which makes it appropriate for local title/tag/creator/story indexing. See [`DATABASE.md`](./DATABASE.md) for the FTS schema.

## Major Application Services

The Trusted Core is organized as a tree of services under a top-level `AppCore`:

```
AppCore
├── ProviderService
├── FeedService
├── SearchService
├── LibraryService
├── SeriesService
├── HistoryService
├── PlaybackService
├── ReaderService
├── BrowserService
├── DownloadService
├── CacheService
├── SchedulerService
├── PrivacyService
├── SecurityService
├── RegistryService
└── SettingsService
```

Services communicate through typed commands/events rather than frontend components reaching directly into persistence. This keeps the frontend from bypassing service-level validation, permissioning, and caching logic, and it is the pattern that the inferred event model in [`EVENTS.md`](./EVENTS.md) is built around.

## Application Process Model

The recommended eventual process model separates trusted, restricted, and untrusted code into distinct processes:

```
kelpie
   ├── trusted Rust core
   └── trusted frontend webview

kelpie-provider-host
   └── restricted provider execution

remote webviews
   └── untrusted websites

mpv
   └── optional media subprocess
```

The MVP may initially keep some provider infrastructure in-process, but the interfaces should be designed as though provider execution can move out-of-process later. In other words, the trust boundaries above are meant to hold even before the process boundaries fully match them.

## Non-Functional Requirements

There is no dedicated performance document elsewhere in the `architecture/` tree, so Kelpie's performance targets, virtualization rules, image pipeline, and performance profiles are recorded here.

### Performance Targets

Targets on a typical modern Linux desktop:

| Metric | Target |
|---|---|
| Cold launch | < 2 s |
| Cached Home | < 300 ms after DB open |
| Local search | < 100 ms typical |
| Feed scrolling | 60 fps target |
| Indexed items | 100,000+ |
| Providers | 100 without redesign |

### UI Virtualization

The following surfaces must be virtualized:

- global feed
- search
- history
- collections
- booru grid
- downloads
- large chapter lists

Thousands of cards should never be mounted simultaneously.

### Image Pipeline

Image loading follows a staged pipeline:

```
viewport request
     ↓
memory cache
     ↓
disk cache
     ↓
network
     ↓
decode
     ↓
resize
     ↓
render
```

Work is prioritized as: visible, near viewport, off-screen. Low-value work should be cancelled during fast scrolling.

### Performance Profiles

Three user-selectable performance profiles are intended:

- Battery Saver
- Balanced
- High Performance

Battery Saver specifically is expected to:

- disable animated previews
- reduce prefetch
- reduce thumbnail resolution
- reduce provider refresh rate
- lower concurrency

## Final Architecture

The following diagram summarizes how discovery, the normalized media model, the media-type families, and the provider platform relate to each other end to end:

```
                           kelpie

                     ┌──── Discovery ────┐
                     │                   │
                     │ Home              │
                     │ Search            │
                     │ Following         │
                     │ Continue          │
                     └─────────┬─────────┘
                               │
                     Normalized Media Model
                               │
      ┌───────────┬────────────┼───────────┬────────────┐
      │           │            │           │            │
    Video        GIF        Images      Sequential      Text
                                           │
                                      Manga/Comic
                                           │
                                         Pages
      │           │            │           │            │
      └───────────┴────────────┼───────────┴────────────┘
                               │
                              Audio
                               │
                     Local Library / FTS
                               │
                      Provider Platform
                               │
      ┌──────────┬─────────┬─────────┬────────┬─────────┐
      │          │         │         │        │         │
    APIs        RSS       HTML     Booru    Manga     Browser
      │          │         │         │        │         │
      └──────────┴─────────┴─────────┴────────┴─────────┘
                               │
                         External Sources
```

Discovery surfaces (Home, Search, Following, Continue) sit above a single Normalized Media Model that spans every media family Kelpie supports (video, GIF, images, sequential/manga/comic content, text, and audio). That normalized model is backed by the Local Library / FTS index, which in turn is populated by the Provider Platform — a set of adapters over external sources reached via APIs, RSS, generic HTML, booru-style endpoints, manga-specific structures, or the embedded browser.
