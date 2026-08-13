# Accessibility

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Overview

This document covers Kelpie's accessibility requirements and its keyboard model — the global, browser, media, and reader key bindings the application is expected to support, and the rule that all of them remain user-configurable.

## Accessibility requirements

The spec lists nine required accessibility capabilities (kelpie.md §128), without ranking them:

- Full keyboard navigation
- Visible focus
- Semantic controls
- Screen-reader labels
- Reduced motion
- UI scaling
- High contrast
- Reader text resizing
- Non-color-only status cues

These requirements are stated as a flat, required list rather than a phased or optional one, which means they constrain the rest of the UX surface rather than sitting apart from it:

- **Full keyboard navigation** and **visible focus** are the foundation the Keyboard Model below exists to satisfy — every interactive surface needs to be reachable and operable without a pointer, with a clearly visible focus indicator at every step.
- **Semantic controls** and **screen-reader labels** mean the card system (see [`CARDS.md`](./CARDS.md)) and the specialized viewers (see [`VIEWERS.md`](./VIEWERS.md)) must expose their real structure and state to assistive technology, not just render visually.
- **Reduced motion** constrains the "short motion" visual-design preference described in `CARDS.md` — short is not the same as required, and a reduced-motion mode must be respected where the OS or the user requests it.
- **UI scaling**, **high contrast**, and **reader text resizing** are user-controllable presentation adjustments; reader text resizing in particular ties directly into the font/size/line-height/column-width controls of the Literature Reader (see `../media/LITERATURE.md`).
- **Non-color-only status cues** means any status communicated by color alone elsewhere in the app — for example the Success/Warning/Danger tokens in `CARDS.md`, or provider validation checklists such as the one shown in `../media/BOORU.md` — must also be communicated through a non-color channel (icon, label, or pattern).

## Keyboard model

The spec organizes key bindings into four scopes (kelpie.md §129).

**Global:**

| Binding | Action |
|---|---|
| `Ctrl+K` | Search / Command Palette |
| `Ctrl+,` | Settings |
| `Ctrl+R` | Refresh |
| `Esc` | Back / Close |

**Browser** (active within the embedded/isolated browser surface, `/browser/:tab`):

- `Ctrl+L`
- `Ctrl+T`
- `Ctrl+W`
- `Alt+Left`
- `Alt+Right`

These mirror conventional browser-tab bindings (address bar focus, new tab, close tab, back, forward), which is expected behavior given that the embedded browser is meant to feel like a browser to a user who ends up in it via a browser-fallback provider.

**Media** (active in the video/GIF/audio viewers):

- `Space`
- `Left`/`Right`
- `M`
- `F`

These correspond to the transport conventions used across Kelpie's playback surfaces — play/pause, seek, mute, and fullscreen — consistent with the Video Viewer's function list in `../media/VIDEO.md`.

**Reader** (active in the manga/comic/literature readers):

- `Left`/`Right`
- `Page Up`/`Page Down`
- `Home`/`End`
- `F`

These map onto page-by-page and jump-to-start/end navigation, plus fullscreen, across the sequential and text readers documented in `../media/MANGA.md` and `../media/LITERATURE.md`. Because reading direction (source default, LTR, RTL, or vertical) affects next/previous semantics for sequential media, what `Left`/`Right` actually navigate to in the reader scope is direction-dependent rather than fixed.

The spec is explicit on one closing point: **all bindings are configurable.** None of the tables above are meant to be read as fixed, unchangeable defaults — they are the out-of-box mapping, adjustable per user through Settings → Keyboard (see [`INFORMATION-ARCHITECTURE.md`](./INFORMATION-ARCHITECTURE.md)).

## Related documents

- [`INFORMATION-ARCHITECTURE.md`](./INFORMATION-ARCHITECTURE.md) — Settings → Keyboard is the configuration surface for the bindings above.
- [`CARDS.md`](./CARDS.md) — visual tokens and status cues constrained by the non-color-only requirement.
- [`VIEWERS.md`](./VIEWERS.md) — the media and reader viewers the Media and Reader keyboard scopes apply to.
