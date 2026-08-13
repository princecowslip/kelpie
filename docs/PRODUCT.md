# Product

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## What Kelpie is

Kelpie is a private Linux desktop application for discovering, browsing, searching, viewing, reading, listening to and organizing lawful adult media distributed across many websites and local files. It targets Debian, Ubuntu, Arch Linux and compatible distributions, built as a local-first desktop application with sandboxed provider integrations (Rust + Tauri 2 + React/TypeScript + SQLite/FTS5, with optional mpv/libmpv playback).

Kelpie combines the useful characteristics of a media center, feed reader, multi-site search engine, video browser, GIF and short-clip browser, image/gallery viewer, booru client, manga reader, comic reader, webcomic reader, adult cartoon/illustration browser, hentai/adult manga library, literature reader, audio player, imageboard/thread viewer, creator-platform browser, live-site launcher, personal media library, private web browser, and extensible provider/plugin platform — without being designed around any single website.

Its fundamental architecture flows in one direction:

```
Remote sources
      ↓
Provider adapters
      ↓
Normalized media model
      ↓
Local database/index
      ↓
Feed / Search / Library
      ↓
Specialized viewers
```

Because every source is normalized before it reaches the UI, the user owns a stable Kelpie library even when individual websites change, redesign, or disappear.

## Product thesis

The core product proposition:

> "One private Linux-native interface for adult media across the web, regardless of whether the source contains video, GIFs, images, manga, comics, stories, audio or mixed media."

The long-term value of the project is expected to come from ten durable assets, not from raw provider count:

1. The normalized media model.
2. The provider platform.
3. The global feed.
4. Unified search.
5. Specialized viewers.
6. Cross-site library organization.
7. Privacy controls.
8. Browser fallback.
9. Source-management tooling.
10. Linux-native integration.

Supporting a large number of sites is valuable, but provider count must never become the architecture itself.

## Product goals

### Unified discovery

Many sources should be able to participate in a common set of discovery surfaces: Home, Latest, Following, Continue, Search, Discover, and Saved searches.

### Media-aware consumption

Each medium is presented through the viewer appropriate to it, not through a one-size-fits-all page:

| Medium | Viewer |
|---|---|
| Video | Video player |
| GIF | Animation viewer |
| Image | Image viewer |
| Gallery | Gallery viewer |
| Manga | Sequential reader |
| Comic | Sequential reader |
| Story | Text reader |
| Audio | Audio player |
| Imageboard | Thread viewer |
| Unsupported | Isolated browser |

### Local ownership

Favorites, history, progress, collections, and follow state belong to Kelpie, not to individual websites. This state persists locally regardless of what happens to the source site.

### Resilience

Provider breakage must be isolated. One source failing must never crash the application, break the global feed, prevent local search, or destroy previously indexed content.

### Extensibility

Users should be able to add new sources without recompiling Kelpie.

### Privacy

Sensitive activity stays local unless the user explicitly enables some future synchronization feature.

## Hard product boundaries

These are safety-critical constraints on what Kelpie must never be designed to do. They apply to the product architecture itself, not just to a specific feature:

- Circumvent DRM.
- Circumvent authentication.
- Break subscription restrictions.
- Defeat paywalls.
- Break CAPTCHA systems.
- Evade technical access controls.
- Circumvent geographic restrictions.
- Publicly re-host third-party libraries.
- Facilitate non-consensual sexual imagery.
- Aggregate sexual material involving minors.
- Treat fictional/illustrated minor sexualization as acceptable simply because it is drawn.
- Execute untrusted provider code with native privileges.

For subscription- or DRM-protected services, browser access (the embedded/isolated browser, see §91 of the spec) is a valid and expected provider mode — Kelpie defers to the source's own access controls rather than working around them.

## Adult-only safety model

Kelpie is an adults-only application. Official presets undergo a registry review before release, and every built-in provider must receive one of five review outcomes:

- **APPROVED**
- **BROWSER_ONLY**
- **REVIEW_REQUIRED**
- **DISABLED**
- **BLOCKED**

A source does **not** become an official preset merely because it is popular, its API is convenient, someone has already written a scraper for it, or it describes itself as an adult site. Registry review instead considers:

- Source ownership.
- Age/content policy.
- Moderation model.
- Content-reporting process.
- Non-consensual-content policy.
- Reliability.
- Technical integration risk.

User-added custom sources remain separate from official endorsement — a user can add a source Kelpie has not reviewed, but that source carries no registry approval status.

## Design principles

Eight principles govern how Kelpie's architecture and UI are meant to behave:

1. **Local first** — core state resides locally.
2. **Source agnostic** — frontend components consume normalized objects, not provider-specific shapes.
3. **Media aware** — media type determines presentation, not website identity.
4. **Graceful degradation** — integration can fall back to the browser.
5. **Explicit permissions** — providers declare exactly which domains and capabilities they require.
6. **Recoverable** — database and provider updates have rollback paths.
7. **Explainable** — recommendations and filters can be understood and overridden by the user.
8. **Discreet** — the application should look like premium media software, not a stereotypical adult website.

## User modes

Kelpie is expected to support several overlapping workflows rather than a single fixed UX:

- **Browse mode** — for users who primarily want discovery and viewing.
- **Library mode** — for users maintaining favorites, series, and collections.
- **Reader mode** — for manga, comics, webcomics, and literature.
- **Power-user mode** — for keyboard navigation, saved searches, and filters.
- **Private mode** — for sessions that should leave minimal persistent local state.
- **Developer mode** — for creating and debugging provider adapters.

## Final product direction

Kelpie should not become "a giant scraper with a desktop UI." It should become "a private Linux media platform with a provider ecosystem." That distinction determines whether the project stays maintainable over time.

Individual sites will redesign, change APIs, introduce authentication, restrict regions, remove endpoints, become unavailable, or change ownership. Kelpie's user experience is meant to survive those events because its core concepts are independent of any one site:

Media · Series · Creator · Tag · Feed · Search · Collection · Follow · History · Progress · Provider

The project's strongest long-term assets, in order, are therefore:

1. The provider contract.
2. The normalized media and series models.
3. The local feed/search/library engine.
4. The specialized video, GIF, image, manga/comic, text, and audio experiences.
5. The isolated browser fallback.
6. The provider registry and governance system.
7. The privacy/security model.
8. The ability for users to add new sources without waiting for the Kelpie core application to be redesigned.

This architecture is what makes it possible for Kelpie to cover mainstream video, GIFs, image galleries, manga, comics, cartoons, hentai/adult illustration, literature, audio, creator platforms, live sites, boorus, imageboards, and future media categories — without turning each one into a separate application.
