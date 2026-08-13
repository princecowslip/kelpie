# Imageboard Support and Thread Viewer

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Overview

Imageboards are thread-based discussion boards where posts carry inline attachments. This document covers the `imageboard_thread` and `imageboard_post` structural media kinds (kelpie.md §13), how imageboard sources are onboarded, and the Thread Viewer used to read them.

## Imageboard support model

As with boorus, Kelpie does not attempt to enable arbitrary boards wholesale. Instead it ships **protocol adapters** for known imageboard software families (kelpie.md §47):

- 4chan JSON-compatible
- vichan-compatible
- Tinyboard-compatible
- LynxChan-compatible
- Generic JSON thread
- Generic HTML thread

### Setup flow

Adding an imageboard source follows this flow:

```
Add Source
 → Imageboard
 → Enter host
 → Detect engine
 → Select boards
 → Review safety
 → Enable
```

A critical constraint stated directly in the spec: **Kelpie must never index every board automatically.** Board selection is explicit and per-source; enabling a host does not implicitly enable everything hosted on it. This is a deliberate safety boundary, since imageboard hosts commonly host boards outside Kelpie's lawful-adult-media scope alongside boards within it.

## Imageboard Thread Viewer

The Thread Viewer renders a thread as an original post followed by a sequence of replies, each of which may carry its own attachment and may quote earlier posts (kelpie.md §90):

```
Original Post
 ├── attachment
 └── text

Reply
 ├── quote
 └── attachment

Reply
```

### Functions

- Inline attachments
- Expand
- Quote navigation (jumping to/from a quoted post)
- Hide
- Collapse
- Refresh
- Save attachment
- Open original

## Relationship to provider presets

Because board selection is explicit and safety-reviewed per §47 above, and because imageboards are adapted generically by protocol family rather than as individually curated presets, there is no dedicated imageboard preset table in the provider registry distinct from the engine families listed above.
