# Features

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

This is a scannable index of Kelpie's major feature areas, not a full description of any of them. Each row points at the doc(s) intended to cover that area in depth, per the documentation architecture in kelpie.md §132. Some target files do not exist yet — they are named here so future docs land in the right place.

## Discovery: feeds and search

| Feature | Summary | See |
|---|---|---|
| Global feed pipeline | Provider → refresh → normalize → validate → deduplicate → cluster → SQLite → filter → rank → UI | `architecture/FEEDS.md`, `ux/FEEDS.md` |
| Feed modes | Home, Latest, Following, Continue, Unseen, Discover, Random | `architecture/FEEDS.md`, `ux/FEEDS.md` |
| Home screen | Configurable, reorderable/hideable/replaceable dashboard sections (Continue, Following Updates, New Manga & Comics, Global Feed, etc.) | `ux/INFORMATION-ARCHITECTURE.md`, `ux/FEEDS.md` |
| Feed filtering | Primary chips (Video, GIFs, Images, Manga, Comics, Cartoons, Stories, Audio) plus advanced filters (Animation, Illustration, Photography, Booru, Imageboard, Webcomic, Live) | `ux/FEEDS.md` |
| Feed ranking | Local scoring model (freshness, provider/media/tag/search preference, follow bonus, unseen/discovery bonus, repetition penalties) with selectable ranking modes and a "Why this item?" explanation | `architecture/FEEDS.md` |
| Feed clustering | Collapses bursty updates (series-update, gallery-batch, duplicate-set, provider-batch, thread-update) into single feed entries | `architecture/FEEDS.md` |
| Search architecture | Two-stage search: immediate local FTS results plus streaming, per-provider remote search | `architecture/SEARCH.md` |
| Search query language | Structured query syntax (`type:`, `source:`, `creator:`, `series:`, `tag:`, `style:`, date ranges, `duration:`, `saved:`, `seen:`, `following:`, `offline:`) | `architecture/SEARCH.md` |
| Saved searches | Searches can be saved, renamed, pinned, placed on Home/sidebar, and scoped to local index or selected providers | `architecture/SEARCH.md`, `ux/FEEDS.md` |
| Deduplication | Cross-source duplicate detection using progressively weaker signals (remote ID → canonical URL → cross-source ID → creator+title → duration → perceptual hash → fingerprint), with exact/high/possible confidence tiers | `architecture/FEEDS.md`, `architecture/DOMAIN-MODEL.md` |
| Source variant selection | When a duplicate exists across providers, priority order for which source to present (user preference, authenticated, quality, metadata, reliability) | `architecture/FEEDS.md` |

## Navigation and presentation

| Feature | Summary | See |
|---|---|---|
| Navigation structure | Home / Media / Sources / Library / System sidebar groups, with empty groups hidden | `ux/INFORMATION-ARCHITECTURE.md` |
| Route model | URL structure for home, search, sources, items, series/chapters, library subsections, browser tabs, and settings | `architecture/OVERVIEW.md`, `ux/INFORMATION-ARCHITECTURE.md` |
| Card system | Shared card anatomy (preview, media indicator, title, creator, source, metadata) plus per-medium badges (duration, GIF, chapter/page count, issue #, read time, track length, image count, LIVE) | `ux/CARDS.md` |
| Display modes | Comfortable, Compact, List, Mosaic — configurable globally, per provider, or per media type | `ux/CARDS.md` |
| Visual design & tokens | "Discreet premium media software" design target; example dark theme token palette plus a required complete light theme | `ux/CARDS.md`, `ux/ACCESSIBILITY.md` |

## Viewers

| Feature | Summary | See |
|---|---|---|
| Video viewer | Full playback controls, quality/subtitle/audio track selection, PiP, queueing, source variant switching; browser-native playback with optional mpv/libmpv fallback | `media/VIDEO.md`, `ux/VIEWERS.md` |
| GIF viewer | Presentation-based (not extension-based) animation handling across GIF/WebP/AVIF/WebM/MP4, with hover/visibility-based animation policy | `media/GIF.md`, `ux/VIEWERS.md` |
| GIF performance rules | Viewport pausing, focus pausing, reduced-motion and battery-mode compliance, concurrency limits | `media/GIF.md` |
| Image viewer | Fit/zoom/pan/rotate/fullscreen, metadata panel, save, open original | `media/IMAGES.md`, `ux/VIEWERS.md` |
| Gallery viewer | Single/Filmstrip/Grid/Continuous/Slideshow modes with lazy loading, prefetch, and memory-bounded decoding | `media/IMAGES.md`, `ux/VIEWERS.md` |
| Manga/comic reader | Single Page, Double Page, Continuous Vertical, Webtoon modes (Guided Panel planned as future work) | `media/MANGA.md`, `media/COMICS.md`, `ux/VIEWERS.md` |
| Reading direction | Source Default/LTR/RTL/Vertical, affecting pairing, navigation, and spread order | `media/MANGA.md`, `media/COMICS.md` |
| Sequential reader performance | Bounded page decode window with adaptive preloading based on RAM, page size, network, reading speed, and performance mode | `media/MANGA.md`, `media/COMICS.md` |
| Reading progress | Persisted series/chapter UID, page index, scroll fraction, timestamp, completion state | `architecture/DOMAIN-MODEL.md`, `media/MANGA.md` |
| Series page | Chapter list with per-chapter read state, follow/continue/save actions, alternate-source switching | `media/MANGA.md`, `media/COMICS.md`, `ux/VIEWERS.md` |
| Literature reader | Font/size/line-height/column controls, themes, find, bookmarks, chapter nav, reading-time estimate; provider HTML is sanitized | `media/LITERATURE.md`, `ux/VIEWERS.md` |
| Audio player | Persistent mini-player plus expanded view (chapters, description, transcript, sleep timer); MPRIS integration where practical | `media/AUDIO.md`, `ux/VIEWERS.md` |
| Booru browser | High-density tag-search grid UI with optional tag categories (creator, series, character, general, technical) | `media/BOORU.md` |
| Tag normalization | `TagReference` model preserving normalized identity, provider-local identity, display value, category, and aliases | `architecture/DOMAIN-MODEL.md`, `media/BOORU.md` |
| Imageboard thread viewer | Original post/reply structure with quote navigation, inline attachments, collapse/hide, refresh | `media/IMAGEBOARD.md`, `ux/VIEWERS.md` |
| Cartoons (adult illustration/animation) | Structural media family alongside manga/comics for adult cartoon and illustration content | `media/CARTOONS.md` |

## Browser fallback

| Feature | Summary | See |
|---|---|---|
| Embedded browser | Isolated webview for login, subscription content, unsupported providers, CAPTCHA, JS-heavy pages, DRM content, provider breakage, and manual browsing | `architecture/BROWSER.md`, `security/WEBVIEW-ISOLATION.md` |
| Add to Kelpie | Resolves the current browser URL through the provider resolver into a normalized item, or falls back to a generic external bookmark | `architecture/BROWSER.md` |

## Library, history, and privacy

| Feature | Summary | See |
|---|---|---|
| Library | Provider-independent Saved, Series, Collections, Following, History, Downloads, and Local Files | `ux/INFORMATION-ARCHITECTURE.md`, `architecture/DOMAIN-MODEL.md` |
| Collections | Mixed-media manual collections (video, GIF, gallery, manga, comic, story, audio, external page, local file) with multiple sort orders | `architecture/DOMAIN-MODEL.md` |
| Smart collections | Planned future query-backed collections (e.g. a saved search materialized as a live collection) | `architecture/DOMAIN-MODEL.md` |
| Following | Follow providers, creators, series, tags, or saved searches; feeds a dedicated feed and personalized ranking | `architecture/FEEDS.md`, `architecture/DOMAIN-MODEL.md` |
| History | Tracks first/last opened, open count, progress, completion, with configurable retention (Forever down to Session only, or Never) | `architecture/DOMAIN-MODEL.md`, `ux/PRIVACY.md` |
| Private mode | Suppresses persistence of searches, opened items, progress, browser history, and feed signals for the session; normal history stays untouched | `ux/PRIVACY.md`, `security/CONTENT-SAFETY.md` |
| Safe screen | Configurable global shortcut to pause/mute/hide active content and swap in a neutral UI, with optional lock | `ux/PRIVACY.md`, `security/CONTENT-SAFETY.md` |
| Neutral preview mode | Replaces explicit previews with neutral icons/placeholders under Never hide/Blur/Reveal-on-hover/Reveal-on-click/Neutral covers modes | `ux/PRIVACY.md`, `security/CONTENT-SAFETY.md` |
| Notification privacy | Generic notification text by default ("Download complete"), with detailed content opt-in | `ux/PRIVACY.md` |
| Application lock | PIN/passphrase/OS-backed secret, triggered manually, on inactivity, on system resume, or with the safe screen | `security/CONTENT-SAFETY.md`, `ux/PRIVACY.md` |

## Downloads and offline

| Feature | Summary | See |
|---|---|---|
| Downloads | Provider-gated download capability with queued/resolving/downloading/paused/complete/failed states, pause/resume, retry, concurrency and bandwidth limits, free-space checks; never bypasses DRM or access controls | `architecture/OVERVIEW.md` |
| Offline manga/comics | Keep Chapter/Issue/Volume Offline, distinct from transient page caching | `media/MANGA.md`, `media/COMICS.md` |

## Provider platform (see also: providers/ and security/ docs)

Provider manifest, capability, permission, sandboxing, and registry-review mechanics are covered under `providers/` (`SDK.md`, `MANIFEST.md`, `CAPABILITIES.md`, `PERMISSIONS.md`, `REGISTRY.md`, generic-provider docs, `TESTING.md`) and `security/` (`THREAT-MODEL.md`, `WEBVIEW-ISOLATION.md`, `PROVIDER-SANDBOX.md`, `CONTENT-SAFETY.md`, `SECRET-STORAGE.md`). See `docs/PRODUCT.md` for the registry review states (APPROVED/BROWSER_ONLY/REVIEW_REQUIRED/DISABLED/BLOCKED) and `docs/ROADMAP.md` for phasing.
