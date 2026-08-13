# Provider Architecture

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

Providers are the adapters that convert remote sources into Kelpie's normalized objects (see [`DOMAIN-MODEL.md`](./DOMAIN-MODEL.md)). This document covers the conceptual and operational shape of the provider system — what a provider is, what it declares it can do, and how it moves through its lifecycle, health states, and updates.

The manifest schema, execution API, and sandbox/permission mechanics that implement this shape live in `../providers/*.md` — see [`../providers/SDK.md`](../providers/SDK.md), [`../providers/MANIFEST.md`](../providers/MANIFEST.md), [`../providers/CAPABILITIES.md`](../providers/CAPABILITIES.md), [`../providers/PERMISSIONS.md`](../providers/PERMISSIONS.md), and [`../providers/REGISTRY.md`](../providers/REGISTRY.md). Sandboxing detail also intersects with [`../security/PROVIDER-SANDBOX.md`](../security/PROVIDER-SANDBOX.md). This file covers the *why and how* the provider system is shaped, not the concrete schemas.

## Provider Model

A provider converts a remote source into normalized Kelpie objects. Provider categories include:

- specific website
- site family/protocol
- generic HTML
- generic feed
- browser-only
- local filesystem

This range — from a bespoke adapter for one specific site down to a browser-only or local-filesystem "provider" — is intentional: it lets Kelpie represent sources with very different levels of structured access under one common model, rather than requiring every source to expose the same rich API-like surface.

## Provider Capabilities

Each provider declares which capabilities it supports, and the frontend checks capabilities before rendering related actions (e.g. not showing a "Search" affordance for a provider that can't search). The capability surface includes:

- **feeds** — feed, search, suggest
- **item** — creator, tags, related
- **series** — volumes, chapters, issues, pages
- **thread** — posts
- **resolve-url**
- **media** — captions
- **authentication, browser, download**

Because capability-gating happens in the frontend, a provider that only implements a subset of these (say, item + media, with no search) should simply omit the rest rather than needing to stub them out.

## Provider Lifecycle

A provider moves through the following lifecycle states:

`available → installed → enabled → disabled → updating → degraded → incompatible → broken → quarantined → removed`

A provider becoming `broken` must not delete:

- History
- Favorites
- Collections
- Indexed metadata
- Source URLs

In other words, breakage in the adapter itself is treated as separable from the value of data Kelpie has already indexed locally — a broken provider degrades future refreshes, not past user data.

## Provider Integration Levels

Providers are further classified by how deep their integration with Kelpie is, independent of lifecycle state:

- **Native** — reliable structured interface.
- **Integrated** — normal provider adapter with feed/search support.
- **Metadata** — metadata/indexing integration with browser consumption (i.e. Kelpie indexes metadata but playback/reading happens via the embedded browser).
- **Browser+** — a dedicated isolated browser preset with useful Kelpie integration, for sources that can't be adapted more deeply but still benefit from some Kelpie-side tooling.
- **Generic** — a protocol/source-family adapter rather than a bespoke one.
- **Experimental** — a community or locally-installed provider.
- **Quarantined** — a provider package prevented from executing.

## Provider Health

Kelpie records, per provider:

- `last_success`
- `last_attempt`
- `last_search_success`
- `failure_count`
- `average_latency`
- `version`
- `last_error`

From these, the UI surfaces one of the following health states: Healthy, Refreshing, Degraded, Authentication Required, Rate Limited, Offline, Outdated, Broken, Disabled. See [`EVENTS.md`](./EVENTS.md) for how these health states relate to the broader event/state-transition model.

## Provider Updates

Application releases and provider releases are intentionally decoupled, across three independently-versioned layers:

- Kelpie App
- Provider Registry
- Provider Package

This decoupling matters because site integration assumptions can change suddenly. RedGIFs, for example, currently states that it no longer offers third-party API access as of June 17, 2026 — the kind of change that a provider needs to absorb without waiting on a full desktop app release.

Because of this, a provider should be able to move down the integration-level ladder — Native → Integrated → Metadata → Browser+ — through a registry update alone, without requiring a complete desktop release.

## Provider Rollback

Kelpie maintains both a current provider version and the previous known-good provider version. If post-update health tests fail after a provider update:

1. disable the new version
2. restore the previous version
3. record the rollback

This gives provider updates an automatic safety net: a bad update degrades gracefully back to the last version known to work rather than leaving the provider broken.
