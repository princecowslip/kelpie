# Video Viewer

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Overview

The Video Viewer is Kelpie's playback surface for the `video`, `short_video`, and `clip` structural media kinds (kelpie.md §13). It is a standard transport-control player intended to work across both native site integrations and generic/browser-fallback providers.

## Functions

The viewer is specified to support the following functions (kelpie.md §76):

- Play
- Pause
- Seek
- Volume
- Mute
- Speed
- Quality (source variant selection)
- Subtitle track
- Audio track
- Fullscreen
- Picture-in-Picture (PiP)
- Loop
- Queue
- Previous / Next
- Source variants
- Open original (escape hatch to the source page)

## Playback engine strategy

Two playback strategies are specified, chosen per source:

1. **Browser-native playback** — used where appropriate, i.e. wherever a provider's media can be played directly in a standard web `<video>` context.
2. **mpv/libmpv** — an optional fallback for eligible direct media sources, used when native playback is insufficient or unavailable.

The spec does not enumerate a decision matrix for which sources use which engine; this is left as an implementation detail to be resolved against each provider's actual delivery mechanism (see `../providers/REGISTRY.md`).

## Relationship to provider presets

The Video Viewer itself is medium-agnostic — it renders whatever video-structured media a provider supplies. Which providers are recommended as video sources, and at what integration level, is tracked as the canonical registry in [`../providers/REGISTRY.md`](../providers/REGISTRY.md) (kelpie.md §36 — Video Presets). This document does not duplicate that preset list.
