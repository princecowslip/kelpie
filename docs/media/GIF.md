# GIF Viewer

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Overview

The GIF Viewer covers Kelpie's `gif`, `animated_image`, and `animation` structural media kinds (kelpie.md §13). Its defining architectural decision is that GIF handling is **presentation-based rather than file-extension-based** (kelpie.md §77): "GIF" in Kelpie's UX is a behavior class (looping, typically-muted, feed-friendly animation), not a literal `.gif` file check.

## Supported formats

Under the GIF behavior model, the viewer may play any of:

- GIF
- Animated WebP
- Animated AVIF
- WebM
- MP4

All of these are treated uniformly as "GIF-like" content for feed and viewer purposes, regardless of container format.

## Feed animation modes

Because GIF-like content commonly appears embedded in scrolling feeds, Kelpie defines explicit feed playback policy options (kelpie.md §77):

- **Never animate**
- **Animate on hover**
- **Animate when visible**
- **Always animate**

The recommended default is **Animate on hover**.

## Performance rules

Because animated feed cards are a well-known performance and battery hazard, animated cards are governed by explicit performance rules (kelpie.md §78). Animated cards should:

- Pause outside the viewport
- Pause when the window/app is unfocused
- Respect the operating system's reduced-motion setting
- Limit concurrent playback (i.e. cap how many animated cards may animate at once)
- Prefer preview resources (lower-cost preview assets over full-resolution originals for in-feed animation)
- Respect battery mode (reduce or suspend animation under a battery-saving state)

These rules apply to the feed/card presentation of GIF-like content; the dedicated single-item GIF Viewer is expected to play the selected item without the feed-density constraints above.

## Relationship to provider presets

Which providers are recommended as GIF sources, and at what integration level, is tracked as the canonical registry in [`../providers/REGISTRY.md`](../providers/REGISTRY.md) (kelpie.md §37 — GIF Presets). This document does not duplicate that preset list.
