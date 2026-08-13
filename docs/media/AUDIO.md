# Audio Player

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Overview

The Audio Player covers Kelpie's `audio` and `audio_story` structural media kinds (kelpie.md §13), including audio-erotica and episodic audio fiction. It is specified as a persistent mini-player with an optional expanded view, plus OS-level media-control integration.

## Persistent mini-player

The mini-player is expected to remain available/persistent while audio plays, exposing (kelpie.md §87):

- Artwork
- Title
- Creator
- Play/pause
- Seek
- Speed
- Queue
- Volume

## Expanded view

From the mini-player, an expanded view surfaces additional detail and controls:

- Chapters
- Description
- Transcript
- Sleep timer
- Series navigation

The series-navigation function ties audio content into the same Series → Chapter model used elsewhere for episodic content (see the Dipsea note below).

## Linux media-control integration

The spec calls for integrating Linux media controls through **MPRIS** "where practical" (kelpie.md §87) — i.e. exposing play/pause/seek/track metadata to the desktop environment's standard media-control surfaces (lock screen widgets, keyboard media keys, shell applets) rather than confining playback control to Kelpie's own window.

## Audio Presets

Recommended audio-erotica/audio-fiction sources (kelpie.md §42):

- Quinn
- Dipsea
- femtasy
- Literotica Audio
- Bloom Stories

Notable framing from the spec:

- **Quinn** currently describes itself as an audio-erotica application.
- **Dipsea** currently offers audio stories/audiobooks organized into multi-chapter series, which the spec calls out as making it suitable for Kelpie's **Series → Chapter → Audio** hierarchical model (the same structural pattern used for manga/comics, applied to audio).

Recommended integration levels:

| Provider | Recommended integration |
|---|---|
| Quinn | Metadata / Browser+ |
| Dipsea | Metadata / Browser+ |
| femtasy | Browser+ |
| Literotica | Integrated through main provider |
| Bloom Stories | Browser+ |

Literotica's audio capability is integrated through its main provider entry rather than as a separate source — see [`LITERATURE.md`](LITERATURE.md) for the full Literotica capability list, of which `audio` is one facet.

## Relationship to provider presets

The canonical registry entries for these five providers live in [`../providers/REGISTRY.md`](../providers/REGISTRY.md) (kelpie.md §42 — Audio Presets). This document reproduces the recommended-integration summary for context; treat the registry as authoritative if the two ever diverge.
