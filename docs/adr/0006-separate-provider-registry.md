# ADR 0006: Decouple the Provider Registry/Updates from Full Desktop Application Releases

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Context

kelpie.md §32 (Provider Updates) states plainly: "Application and provider releases must be decoupled," identifying three separable artifacts — Kelpie App, Provider Registry, Provider Package. The stated reason is that "site integration assumptions can change suddenly," citing a concrete example: "RedGIFs, for example, currently states that it no longer offers third-party API access as of June 17, 2026." A provider needs to be able to move down the integration-level ladder — Native → Integrated → Metadata → Browser+ — "through a registry update without requiring a complete desktop release."

§34 (Official Preset Registry) reinforces the scale of this: Kelpie should ship roughly 40–50 provider definitions/source families, with far fewer enabled automatically by default, each carrying provider ID, display name, site type, integration level, capabilities, permissions, global-feed eligibility, last verification date, registry safety status, and known limitations. §35 (Preset Registry Philosophy) draws a sharp distinction: a preset means "Kelpie knows how to represent or safely open this source," not necessarily "Kelpie extracts its media directly" — which is what allows strong support for subscription sources without defeating their business/access models. §52 (Registry Metadata) formalizes this as a `RegistryEntry` interface (id, name, siteType, integration level, capabilities, defaultEnabled, globalFeedEligible, lastVerifiedAt, safetyStatus, knownLimitations), and §53 (Registry Governance) requires ongoing maintenance per official provider — last technical verification, last safety review, provider version, host/domain ownership snapshot, integration health, permission set — on a review cadence ranging from monthly (high-volume/safety-sensitive) to per-release (generic protocols).

## Decision

Treat the provider registry (and individual provider packages within it) as artifacts that are versioned, updated, and shipped independently of the Kelpie desktop application release train, so that a provider's integration level, capabilities, permissions, or safety status can change — including a same-day demotion in response to a site changing its access policy — without waiting on or requiring a full application release.

## Alternatives

kelpie.md does not name an alternative directly; it presents decoupling as the necessary response to a problem it illustrates concretely (RedGIFs' access-policy change). The reasoned rejected alternative is:

- **Bundling provider definitions/packages into the application release itself** — the obvious default for a desktop app, but rejected because provider integration assumptions can break on a provider's own timeline (as with RedGIFs' third-party API access ending), and tying a safety-relevant demotion (e.g. Native → Browser+) to the cadence of full desktop releases would leave Kelpie technically broken or, worse, still attempting an access pattern a site no longer permits, for as long as a release takes to ship.

## Consequences

- Provider rollback (kelpie.md §33) — maintaining a current and previous known-good provider version, and automatically disabling/reverting on failed post-update health tests — is only meaningful because provider packages already version and ship independently of the app; this ADR is the architectural prerequisite for that rollback mechanism.
- The registry needs its own metadata schema, safety-review cadence, and governance process (kelpie.md §52–53) entirely separate from the app's own release notes/versioning.
- Because a preset means "Kelpie knows how to represent or safely open this source" rather than "extracts its media directly" (kelpie.md §35), a provider can be legitimately downgraded to Metadata or Browser+ integration — see [`0005-browser-fallback.md`](./0005-browser-fallback.md) — as a registry-only change, without that being treated as an application regression.
- The sandboxed provider-execution model (see [`0003-provider-sandbox.md`](./0003-provider-sandbox.md)) is what makes it safe to update/ship provider packages on a faster, independent cadence than the trusted application core — a provider package update is still constrained by the same denied-API boundary regardless of how recently it shipped.
- This decoupling introduces its own operational surface (registry distribution, per-provider versioning, safety-review scheduling) that the application itself does not need; see [`../architecture/PROVIDERS.md`](../architecture/PROVIDERS.md).

## Status

Proposed — kelpie.md v4.0 documents this as an intended decision; no code exists yet that depends on it. This will move to Accepted once an implementation lands that relies on it, or Superseded/Rejected if reconsidered.
