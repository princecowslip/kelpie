# Image and Gallery Viewers

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Overview

This document covers the two viewers used for Kelpie's static-image structural kinds — `image`, `illustration`, `photograph`, and `gallery` (kelpie.md §13): the single-item **Image Viewer** and the multi-item **Gallery Viewer**.

## Image Viewer

The Image Viewer is the single-item view for a standalone image, illustration, or photograph. Its specified functions are (kelpie.md §79):

- Fit
- Actual size
- Zoom
- Pan
- Rotate
- Fullscreen
- Metadata
- Previous
- Next
- Save
- Open original

## Gallery Viewer

The Gallery Viewer is used for the `gallery` structural kind — a set of related images browsed together. It defines four presentation modes plus a slideshow mode (kelpie.md §80):

- **Single** — one image at a time
- **Filmstrip** — a primary image with a scrollable strip of thumbnails
- **Grid** — a grid of images at once
- **Continuous** — a scrollable, continuous sequence of images
- **Slideshow** — automatic advancing presentation

### Performance requirements

Regardless of mode, the Gallery Viewer is required to use:

- Lazy loading
- Near-image prefetch (loading images adjacent to the current position ahead of need)
- Memory-bounded decoding (capping how much decoded image data is held in memory at once)

## Relationship to provider presets

Which providers are recommended as image/gallery sources, and at what integration level, is tracked as the canonical registry in [`../providers/REGISTRY.md`](../providers/REGISTRY.md) (kelpie.md §38 — Image/Gallery Presets). This document does not duplicate that preset list.
