# Generic Booru and Imageboard Adapters

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Overview

This document covers the engine-adapter mechanics for two related generic source families: booru-style tag/post boards (kelpie.md §45–§46) and imageboards (kelpie.md §47). Both are covered together here deliberately: the docs tree prescribed by kelpie.md §132 has no dedicated `GENERIC-IMAGEBOARD.md` file, so imageboard protocol-adapter mechanics live in this document alongside booru mechanics, rather than being split out or omitted. [`../media/BOORU.md`](../media/BOORU.md) covers the same underlying spec sections from the reader/browsing-UI angle; this document is specifically about how the engine-detection and adapter mechanics work.

## Booru support

Rather than individually endorsing a large number of specific adult booru sites, kelpie.md §45 specifies that Kelpie should ship booru **engine/provider families** — adapters for the underlying software a booru runs on, not the board itself. The specified built-ins are:

- Danbooru-compatible
- Gelbooru-compatible
- Shimmie/Shimmie2-compatible
- Booru-on-Rails-compatible
- Generic JSON Booru
- Generic XML Booru

The user enters a domain, and Kelpie attempts engine detection against that list. This is what lets Kelpie provide solid booru functionality broadly, per the spec, "without turning every compatible public board into an official preset" — the registry endorses the engine adapter, not each individual board that happens to run it.

### Booru auto-detection

kelpie.md §46 specifies the following detection flow:

```
Enter domain
     ↓
Probe known API signatures
     ↓
Detect engine
     ↓
Map endpoints
     ↓
Fetch sample posts
     ↓
Validate tags/media
     ↓
Show permission review
     ↓
Enable
```

Once detection succeeds, a validation summary is shown before the source is enabled:

```
API                    ✓
Search                 ✓
Tag metadata           ✓
Preview images         ✓
Original media         ✓
Pagination             ✓
Authentication         none
```

This validation-before-enable step lines up with the permission-review pattern used elsewhere in the provider platform (see [`PERMISSIONS.md`](./PERMISSIONS.md)) — a source isn't enabled silently; its capabilities are demonstrated against real sample data first.

## Imageboard support

kelpie.md §47 specifies that Kelpie should ship **protocol adapters** for imageboards, rather than enabling arbitrary boards directly. The specified built-ins are:

- 4chan JSON-compatible
- vichan-compatible
- Tinyboard-compatible
- LynxChan-compatible
- Generic JSON thread
- Generic HTML thread

Setup follows a similar shape to booru auto-detection, but with an explicit per-board selection step:

```
Add Source
→ Imageboard
→ Enter host
→ Detect engine
→ Select boards
→ Review safety
→ Enable
```

The spec is explicit: **Kelpie must never index every board automatically.** Board selection is a deliberate, per-board user action, not a default that enables an entire imageboard's board list at once.

## Related documents

- [`GENERIC-HTML.md`](./GENERIC-HTML.md) — the broader Custom Source Builder ladder (Levels 1–3) these adapters belong to.
- [`PERMISSIONS.md`](./PERMISSIONS.md) — the permission-review step surfaced during engine detection.
- [`REGISTRY.md`](./REGISTRY.md) — Generic Booru and Generic Imageboard as built-in source families (kelpie.md §49), and global-feed eligibility defaults for imageboards and user boorus (kelpie.md §51).
- [`../media/BOORU.md`](../media/BOORU.md) — the same spec sections (kelpie.md §45–§47) from the reader/browsing-UI angle.
