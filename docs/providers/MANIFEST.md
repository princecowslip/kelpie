# Provider Manifest

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Overview

Every Kelpie provider ships a manifest — a declarative description of what the provider is, what it claims to do, and what it needs access to. The manifest is read before any provider code runs, and it is what the permission layer, the capability-gated frontend, and the registry all reason about. kelpie.md §25 (Provider Manifest) gives the following example:

```json
{
  "schema": 1,

  "id": "example",
  "name": "Example",

  "version": "1.2.0",
  "providerApi": "1",

  "siteType": "manga",

  "contentKinds": [
    "manga",
    "illustration"
  ],

  "capabilities": [
    "feed",
    "search",
    "series",
    "chapters",
    "pages",
    "browser"
  ],

  "origins": [
    "https://example.com",
    "https://cdn.example.com"
  ],

  "authentication": "browser-cookie"
}
```

## Field reference

| Field | Purpose |
|---|---|
| `schema` | Manifest schema version — lets Kelpie evolve the manifest format itself over time without breaking older providers. |
| `id` | Stable provider identifier. |
| `name` | Human-readable display name. |
| `version` | The provider package's own version (see [Versioning](#versioning-and-provider-updates) below). |
| `providerApi` | The version of the `Provider` execution API (kelpie.md §26, see [`SDK.md`](./SDK.md)) this provider was built against — how Kelpie determines compatibility between an installed provider package and the host application. |
| `siteType` | The kind of source this is (e.g. `manga`, and by extension the other site types referenced across the registry — video, image, literature, audio, imageboard, booru, creator-platform, live, and so on). |
| `contentKinds` | The structural media kinds this provider can produce (kelpie.md §13, Structural Media Types) — e.g. `manga`, `illustration`. |
| `capabilities` | The declared capability strings this provider supports. See [`CAPABILITIES.md`](./CAPABILITIES.md) for the full vocabulary and how the frontend uses it. |
| `origins` | The network origins this provider is allowed to reach. See [`PERMISSIONS.md`](./PERMISSIONS.md) for how these are validated at runtime. |
| `authentication` | The provider's authentication mode (e.g. `browser-cookie`). |

The `capabilities` array and the `Provider` interface methods a package actually implements (kelpie.md §26) are expected to correspond directly — a provider should not declare a capability whose method it doesn't implement, and the frontend is not meant to invoke a method whose capability wasn't declared.

## Versioning and provider updates

The manifest's `version` and `providerApi` fields exist because kelpie.md §32 (Provider Updates) treats application releases and provider releases as deliberately decoupled:

```
Kelpie App
Provider Registry
Provider Package
```

The spec's stated reasoning is that site-integration assumptions can change suddenly and independently of the Kelpie application's own release cadence — it cites RedGIFs discontinuing third-party API access as of June 17, 2026 as a concrete example of the kind of change a provider needs to absorb without waiting on a desktop release.

Consequently, a provider is expected to be able to move down the integration-level ladder —

```
Native → Integrated → Metadata → Browser+
```

— through a registry update alone, without a new Kelpie application version. `providerApi` is what lets the host application and an independently-updated provider package agree on whether they're still speaking a compatible execution contract.

kelpie.md §33 (Provider Rollback) further specifies that the registry maintains both the current provider version and the previous known-good version for each provider, so that a failed post-update health check can disable the new version, restore the previous one, and record the rollback — see [`../architecture/PROVIDERS.md`](../architecture/PROVIDERS.md) for the full versioning, rollback, and registry-update model this manifest field feeds into.

## Related documents

- [`SDK.md`](./SDK.md) — the `Provider` interface a manifest's `capabilities` and `providerApi` fields correspond to.
- [`CAPABILITIES.md`](./CAPABILITIES.md) — the full capability-string vocabulary usable in `capabilities`.
- [`PERMISSIONS.md`](./PERMISSIONS.md) — how `origins` and `authentication` are enforced at runtime.
- [`REGISTRY.md`](./REGISTRY.md) — the catalogue of official presets, each of which is backed by a manifest of this shape.
- [`../architecture/PROVIDERS.md`](../architecture/PROVIDERS.md) — provider updates, versioning, and rollback (kelpie.md §32–§33).
