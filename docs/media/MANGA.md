# Manga/Comic Reader (Canonical Reader Engine)

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Overview

This document is the **canonical description of Kelpie's sequential/paginated reader engine** — reader modes, reading direction, preloading/performance behavior, progress tracking, and the series-level browsing page. It covers the `manga` structural media kind primarily, but the reader engine it describes is shared infrastructure: [`COMICS.md`](COMICS.md) (for `comic`/`webcomic`/`comic_strip`) is a thin delta on top of this document rather than a separate reader implementation. Any future reader-engine change should be made here first, then reflected as a cross-reference from `COMICS.md`.

For manga-source presets, see [`../providers/REGISTRY.md`](../providers/REGISTRY.md) (§39) and [`../providers/GENERIC-MANGA.md`](../providers/GENERIC-MANGA.md) for the generic manga-source integration pattern.

## Reader modes

The reader supports the following page-presentation modes (kelpie.md §81):

- **Single Page**
- **Double Page**
- **Continuous Vertical**
- **Webtoon**

A **Guided Panel** mode (panel-by-panel guided reading) is noted as a future addition, not part of the current baseline.

## Reading direction

Reading direction is a first-class, explicit setting rather than an inferred one (kelpie.md §82):

- **Source Default**
- **LTR**
- **RTL**
- **Vertical**

Reading direction is not cosmetic — it affects several concrete reader behaviors:

- Page pairing (which pages are grouped together in Double Page mode)
- Keyboard navigation (which key advances vs. goes back)
- Next/previous semantics
- Spread order (left-to-right vs. right-to-left ordering of a two-page spread)

## Sequential reader performance

To keep paging smooth, the reader maintains a baseline decode window around the current position (kelpie.md §83):

- Previous page
- Current page
- Next two pages

Beyond this fixed baseline, preloading is adaptive and is expected to take into account:

- RAM (available memory)
- Page size (image dimensions/file size)
- Network speed
- Reading speed (how quickly the user is actually advancing)
- Performance mode (the app-wide performance/battery posture)

## Reading progress

Reading progress is tracked at fine granularity so that a user can resume exactly where they left off (kelpie.md §84). The stored progress record includes:

- Series UID
- Chapter/issue UID
- Page index
- Scroll fraction
- Updated time
- Completed status

Continuous readers (Continuous Vertical, Webtoon) are expected to restore **precise scroll position**, not just the nearest page boundary — a stronger requirement than page-granularity resume.

## Series Page

The Series Page is the top-level browsing/entry surface for a serialized reading source, showing the cover, metadata, continue-reading affordance, and a chapter list (kelpie.md §85):

```
┌──────────────────────────────────────┐
│ Cover   Series Name                  │
│         Creator                      │
│         Ongoing                      │
│                                      │
│         Continue: Ch. 20 page 14     │
│                                      │
│ [Continue] [Follow] [Save]           │
├──────────────────────────────────────┤
│ Chapters                             │
│ ✓ 18                                 │
│ ✓ 19                                 │
│ ● 20                                 │
│   21                                 │
└──────────────────────────────────────┘
```

### Functions

- Follow
- Continue
- Mark read
- Mark unread
- Mark previous read
- Filter unread
- Sort
- Alternate source (switching the same logical series to a different provider/source)
