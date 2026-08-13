# Roadmap

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

This roadmap reflects the phase sequencing and scope gates defined in kelpie.md §134–155. Phase 1 (Desktop Foundation) is complete and Phase 2 (Core Data Layer) is in progress; phases 3 onward are not started, and nothing described here beyond what's landed is built or verified behavior. Phase order matters — it is deliberately structured so that architectural risk (data model, provider platform, feed/search) is resolved before the provider count grows.

## Implementation phases

| Phase | Name | Focus |
|---|---|---|
| 0 | Architecture Definition | Finalize domain vocabulary, `MediaItem`, Series model, provider API, capabilities, security model, UI information architecture, and registry schema. Exit condition: "Core types and security boundaries are stable enough to implement without redesigning them every sprint." |
| 1 | Desktop Foundation | Tauri shell, React UI, routing, navigation, theme, settings, SQLite bootstrap, logging, CI, `.deb`, AppImage. Uses fake providers only. |
| 2 | Core Data Layer | `MediaItem`, Series, Chapter, Creator, Tag, Collection, History, Progress, database migrations, repositories, with comprehensive unit tests. |
| 3 | Provider Platform | Manifest parser, capability system, provider lifecycle, provider permissions, scoped HTTP, provider storage, fixture runtime, diagnostics. No large real-source effort yet. |
| 4 | Feed and Search | Refresh scheduler, normalization pipeline, validation, deduplication, clustering, FTS, local search, remote search, ranking, filters. Stress-tested with synthetic datasets. |
| 5 | Core Viewers | Video, GIF, image, gallery, literature, audio viewers, plus history and progress tracking. |
| 6 | Manga and Comics | Series and chapter list UI, Single/Double Page, LTR/RTL, Continuous, Webtoon reading modes, prefetch, progress, following. |
| 7 | Isolated Browser | Remote webview, tabs, address bar, cookie isolation, provider sessions, login, URL resolver, Add to Kelpie, external browser handoff. Includes a dedicated security review before provider count scales. |
| 8 | Generic Providers | RSS, Atom, JSON Feed, Generic HTML, Generic Gallery, Generic Manga, Generic Comic, Generic Booru, Generic Imageboard, Browser-only provider type. |
| 9 | Reference Providers | A deliberately diverse provider set chosen to exercise different architecture paths (text/tags/creators/audio; animation/clips; sequential comics; galleries; adult manga + auth/browser; audio-first; mainstream video/browser; mixed galleries and clips) rather than many similar video sources. |
| 10 | Privacy and Library | Private mode, app lock, safe screen, neutral previews, history retention, following, series library, collections, storage management. |
| 11 | Downloads and Offline | Download manager, resume, storage quota, offline chapters, offline audio/video where permitted, local file imports. |
| 12 | Provider Developer Experience | CLI (`kelpie-provider new/test/lint/pack`) and developer UI (fixture recorder, parser inspector, network inspector, permission inspector, normalized output view). |
| 13 | Preset Catalogue Expansion | Grow the bundled registry toward 12–18 integrated/metadata providers, 15–20 Browser+ presets, and 8–12 generic source families (≈40–50 total definitions). Not all are enabled by default. |
| 14 | Beta Hardening | Validate across Debian stable, Ubuntu LTS, Arch current; GNOME and KDE; Wayland and X11; HiDPI and multi-monitor; offline, low-memory, provider outage, registry outage, and database migration failure conditions. |

### Phase 9 reference providers in detail

Phase 9 is explicitly chosen for architectural coverage, not popularity:

- **Literotica** — text + tags + creators + audio.
- **RedGIFs** — animation/clip integration.
- **Filthy Figments** — sequential comics.
- **PornPics** — galleries.
- **FAKKU** — adult manga + authentication/browser.
- **Dipsea or Quinn** — audio-first.
- **One mainstream video provider** — video/browser.
- **EroMe** — mixed galleries and clips.

This spread is intended to provide more architectural coverage than implementing ten similar video sources first.

## MVP scope

The MVP is required to include:

- Desktop shell, SQLite, provider system.
- Global feed, provider feed.
- Local search, remote search.
- Favorites, collections, history, progress.
- Video, GIF, image/gallery, manga/comic, literature, audio viewers.
- Browser fallback.
- RSS/Atom, generic HTML, generic gallery, generic manga providers.
- 3–5 real providers.
- Private mode, safe screen.
- `.deb` and AppImage packaging.

## Explicit MVP deferrals

The following are explicitly **not** blockers for MVP:

- Cloud sync.
- Mobile.
- Browser extension.
- Community marketplace.
- Guided comic panels.
- OCR.
- Advanced fingerprints.
- Complex AI recommendations.
- Huge provider count.
- Full game library support.

## MVP acceptance criteria

MVP is considered to pass when all of the following hold:

- Kelpie installs on target Linux systems.
- Cached Home works offline.
- One provider failure cannot break Home.
- Local search responds while remote searches continue.
- Video resumes correctly.
- Audio resumes correctly.
- Literature resumes correctly.
- Manga/comic page progress restores correctly.
- RTL/LTR reading works.
- GIFs stop when outside viewport.
- Mixed collections work.
- Private mode persists no browsing history.
- Remote webviews cannot access privileged IPC.
- RSS source can be added without restart.
- Generic manga source can be added without restart.
- A provider can be disabled without losing library references.

## Version 1.0 gate

Reaching 1.0 requires all of the following, beyond MVP:

- Stable provider API.
- Signed official providers.
- Provider rollback.
- Registry signatures.
- Migration recovery.
- Security review.
- Accessibility pass.
- Provider development docs.
- Complete source review workflow.
- Debian/Ubuntu/Arch validation.

## Key risks

| Risk | Mitigation |
|---|---|
| Provider churn | Independent provider packages; Browser+ fallback. |
| Anti-bot systems | Do not design around bypassing them; downgrade provider functionality instead. |
| Regulation/region changes | Runtime provider availability state and registry updates. |
| Unsafe custom provider | Sandbox plus origin permissions. |
| Database corruption | Transactions, backups, migration recovery. |
| Huge image/GIF feeds | Virtualization, memory budgets, viewport-aware loading. |
| Provider monopoly over feed | Diversity penalties and feed clusters. |
| Sensitive desktop exposure | Neutral previews, safe screen, lock, discreet notifications. |
