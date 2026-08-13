# Kelpie

Kelpie is a planned local-first Linux desktop application for discovering, browsing, searching, viewing, reading, listening to, and organizing lawful adult media distributed across many websites and local files. It combines the useful characteristics of a media center, feed reader, multi-site search engine, and specialized viewers (video, GIF, image/gallery, booru, imageboard, manga/comic, literature, audio) into one local, normalized library — so the user's library, history, and organization survive even when individual source websites change.

Remote sources flow through sandboxed provider adapters into a normalized media model, a local SQLite/FTS5 database and index, a global feed and search layer, and finally into media-aware viewers — with an isolated embedded browser as the fallback for sources Kelpie doesn't (or shouldn't) scrape directly.

## Status

**Implementation is underway, early.** Phase 1 (§135, desktop foundation — Tauri/React shell, SQLite bootstrap, CI, Linux packaging) is complete; Phase 2 (§136, Core Data Layer) is in progress. See [`docs/ROADMAP.md`](./docs/ROADMAP.md) for the full phase sequencing and current status. Most of the application — providers, feed, search, viewers, browser fallback, and nearly everything else described below — remains unbuilt.

[`kelpie.md`](./kelpie.md) is the complete v4.0 Product/UX/Architecture/Provider-Registry/Implementation specification and is the source of truth for the project. [`docs/`](./docs) is a documentation set derived from that spec, organized into the structure the spec itself prescribes (see [`kelpie.md` §132](./kelpie.md)). Everything described in either location beyond what's actually landed — architecture, APIs, providers, later phases — is intended/aspirational, not built or verified behavior.

## Where to start

- [`docs/PRODUCT.md`](./docs/PRODUCT.md) — product vision, goals, hard boundaries, and safety model
- [`docs/ROADMAP.md`](./docs/ROADMAP.md) — phased implementation plan, MVP scope, and the 1.0 gate
- [`docs/architecture/OVERVIEW.md`](./docs/architecture/OVERVIEW.md) — trust architecture, tech stack, and major services
- [`docs/FEATURES.md`](./docs/FEATURES.md) — feature index pointing into the rest of `docs/`

## Contributing

See [`CLAUDE.md`](./CLAUDE.md) for the ground rules this repository (and any AI agent working in it) follows, including the content-safety boundaries that apply to everything produced here.
