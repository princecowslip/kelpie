# ADR 0005: Isolated Embedded-Browser Fallback for Sources Kelpie Can't or Shouldn't Scrape Directly

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Context

Kelpie's hard product boundaries forbid circumventing DRM, authentication, subscription restrictions, paywalls, CAPTCHA systems, and geographic restrictions (kelpie.md §4). The same section immediately follows that list with: "For subscription or DRM-protected services, browser access is a valid and expected provider mode" — i.e. the boundary is not "Kelpie can't support these sources," it's "Kelpie must not defeat their access controls, and should instead fall back to a real browser session."

kelpie.md §91 (Embedded Browser) lists the concrete cases this fallback exists for: login, subscription content, unsupported provider, CAPTCHA, JavaScript-heavy page, DRM content, provider breakage, and manual browsing — with browser controls for back/forward/reload/address/tabs/open-externally/"add to Kelpie." §92 (Add to Kelpie) describes how a page opened in the browser can be promoted into Kelpie's normalized item model: the current URL runs through a provider resolver, becoming a normalized item if recognized, or a generic external bookmark if not. §30 (Provider Integration Levels) formalizes "Browser+" as a distinct integration level — "Dedicated isolated browser preset with useful Kelpie integration" — sitting between "Metadata" (metadata/indexing integration with browser consumption) and "Generic" (protocol/source-family adapter) in the level ladder.

## Decision

Provide an isolated embedded browser as a first-class fallback provider mode for sources Kelpie cannot or should not scrape/extract directly — DRM-protected content, subscription-gated content, sites with no native/integrated adapter, CAPTCHA-protected flows, and general manual browsing — with a dedicated "Browser+" integration level for presets that pair this isolated browser with useful (non-extraction) Kelpie integration such as URL recognition and "Add to Kelpie" promotion into the normalized item model.

## Alternatives

kelpie.md frames the browser fallback itself as the alternative to direct extraction for these source classes, rather than presenting a second fallback option. The reasoned alternatives, both rejected, are:

- **Attempting to scrape/extract DRM or subscription-gated content directly** — rejected outright by the hard product boundaries (kelpie.md §4): Kelpie must not circumvent DRM, authentication, subscriptions, paywalls, CAPTCHAs, or geographic restrictions, regardless of technical feasibility.
- **Simply not supporting unsupported/DRM/subscription sources at all** — rejected in favor of still giving the user a useful, isolated browsing surface plus optional metadata/bookmark integration (kelpie.md §30, §91–92), so these sources degrade gracefully to "Browser+"/"Metadata"/"Generic" integration levels rather than being excluded from Kelpie entirely.

## Consequences

- The embedded browser must be built on the same isolation guarantees as any other untrusted web content — no privileged native IPC (kelpie.md §8) — so falling back to a real browser session never reopens the sandbox/IPC boundary the rest of the architecture depends on; see [`0004-no-remote-native-ipc.md`](./0004-no-remote-native-ipc.md).
- "Add to Kelpie" requires a provider-resolver step capable of recognizing a URL and normalizing it into an item, with an explicit unrecognized-URL fallback to a generic external bookmark (kelpie.md §92) — this resolver is shared surface with the provider registry described in [`0006-separate-provider-registry.md`](./0006-separate-provider-registry.md).
- Because "Browser+" is a distinct, named integration level (kelpie.md §30), the provider registry must be able to represent and independently upgrade/downgrade a source's browser-fallback status (e.g. Native → ... → Browser+, per the rollback path in kelpie.md §32) without shipping a new desktop release.
- Kelpie's DRM/subscription/paywall boundary is a hard product constraint, not a technical limitation to be worked around later; any future change to this decision would require revisiting the hard product boundaries themselves (kelpie.md §4), not just the browser implementation.

## Status

Proposed — kelpie.md v4.0 documents this as an intended decision; no code exists yet that depends on it. This will move to Accepted once an implementation lands that relies on it, or Superseded/Rejected if reconsidered.
