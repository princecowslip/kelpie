# Provider Capabilities

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Overview

Capabilities are the vocabulary a provider uses to declare what it can do. They appear as string entries in a provider's manifest `capabilities` array (see [`MANIFEST.md`](./MANIFEST.md)) and correspond to the optional methods on the `Provider` interface (kelpie.md §26, see [`SDK.md`](./SDK.md)). This document is the developer-facing reference for that string vocabulary: what capabilities exist and how the frontend is expected to use them. The same underlying spec section also informs [`../architecture/PROVIDERS.md`](../architecture/PROVIDERS.md), which treats capabilities from the conceptual angle of the provider platform as a whole rather than as a declaration format — this document is specifically about what a provider author writes into a manifest and what it unlocks in the UI.

## Capability vocabulary

kelpie.md §24 (Provider Capabilities) lists the following capability strings, grouped by the area of functionality they gate:

**Discovery**

- `feeds`
- `feed`
- `search`
- `suggest`

**Item-level**

- `item`
- `creator`
- `tags`
- `related`

**Series / sequential media**

- `series`
- `volumes`
- `chapters`
- `issues`
- `pages`

**Discussion**

- `thread`
- `posts`

**URL handling**

- `resolve-url`

**Media resolution**

- `media`
- `captions`

**Account and access**

- `authentication`
- `browser`
- `download`

## How capabilities gate the frontend

kelpie.md §24 states the rule plainly: **"The frontend checks capabilities before rendering actions."** A provider's declared `capabilities` array is what the UI consults before it offers an action tied to that provider — for example, a search box is only wired up for a provider that declares `search`, a "Read" action that walks a chapter list depends on `series`/`chapters`/`pages`, and a "Login" affordance depends on `authentication`. A provider that does not declare a capability should not have the corresponding UI surface shown for it, and should not be sent a request that only makes sense for that capability.

This makes the capability list a contract in both directions: it is what a provider author uses to describe what their implementation supports, and it is what the rest of Kelpie uses to decide what to render and what to call — without needing to know anything else about that specific provider.

## Declaring capabilities

Capabilities are declared in the `capabilities` array of a provider's manifest, alongside the other fields described in [`MANIFEST.md`](./MANIFEST.md):

```json
"capabilities": [
  "feed",
  "search",
  "series",
  "chapters",
  "pages",
  "browser"
]
```

Each declared capability is expected to correspond to an implemented method on the provider's `Provider` interface object (kelpie.md §26) — see [`SDK.md`](./SDK.md) for the method-by-method mapping.

## Related documents

- [`SDK.md`](./SDK.md) — the `Provider` interface methods these capability strings gate.
- [`MANIFEST.md`](./MANIFEST.md) — where capabilities are declared.
- [`REGISTRY.md`](./REGISTRY.md) — capability lists as they appear in specific official presets (e.g. Literotica's recommended capability set).
- [`../architecture/PROVIDERS.md`](../architecture/PROVIDERS.md) — the conceptual view of the provider platform that this same spec section (kelpie.md §24) also feeds.
