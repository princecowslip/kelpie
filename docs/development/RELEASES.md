# Releases

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

This document connects the scattered release- and versioning-relevant statements in kelpie.md into a single narrative: how the desktop application, the provider registry, and the database schema are each versioned and updated, and what has to be true before a 1.0 release can happen. It does not define a release cadence, channel structure, or branching strategy — kelpie.md does not specify these, so they are left out rather than invented.

## Application releases are decoupled from provider releases

kelpie.md §32 (Provider Updates) states plainly that **application and provider releases must be decoupled**, and names three separately-versioned things:

- Kelpie App
- Provider Registry
- Provider Package

The stated reason is that site-integration assumptions can change suddenly — the spec gives the concrete example of RedGIFs stating it no longer offers third-party API access as of June 17, 2026. Because of this, a provider must be able to move down its integration-level ladder (Native → Integrated → Metadata → Browser+) through a registry update alone, without requiring a full desktop application release. The primary reference for provider integration levels and the registry mechanics that make this possible is [`../architecture/PROVIDERS.md`](../architecture/PROVIDERS.md); this document only covers the release-process implication — that shipping a provider fix or downgrade is not gated on shipping a new version of the desktop app.

## Provider rollback

kelpie.md §33 (Provider Rollback) requires the registry to maintain both:

- the current provider (version)
- the previous known-good provider (version)

If post-update health tests fail for a newly updated provider, the required response is to:

1. disable the new version
2. restore the previous version
3. record the rollback

This is a per-provider rollback mechanism, independent of application versioning — it lets a bad provider update be reverted without touching the Kelpie App release at all.

## Database migrations and application upgrades

Provider versioning is decoupled from the app, but the local SQLite database is upgraded in lockstep with application releases. kelpie.md §106 (Migration Policy) requires every migration to be:

- numbered
- immutable
- transactional
- tested

A major upgrade follows this sequence:

```
database → backup → migration → integrity check → launch
```

If that sequence fails, the required recovery path is:

1. restore backup
2. launch recovery mode
3. show diagnostics

So an application release that ships a schema change carries an implicit contract: the upgrade must be backed up and integrity-checked before the new version is allowed to launch normally, and a failed migration must not leave the user without a working (recovered) application. See [`../architecture/DATABASE.md`](../architecture/DATABASE.md) for the primary treatment of schema and migration mechanics; this document covers only the release-process shape around it.

## The Version 1.0 Gate

kelpie.md §152 (Version 1.0 Gate) lists what must be true before a 1.0 release can happen:

- stable provider API
- signed official providers
- provider rollback
- registry signatures
- migration recovery
- security review
- accessibility pass
- provider development docs
- complete source review workflow
- Debian/Ubuntu/Arch validation

Read together with §32/§33 and §106, this gate is largely a checklist that the decoupled-release and migration-recovery machinery described above must actually exist and work before 1.0: provider rollback and registry signatures must be real (not just planned), migration recovery must be real, and the provider API must be stable enough that Native/Integrated/Metadata/Browser+ registry moves (§32) don't require breaking changes. Pre-1.0 releases are therefore implicitly understood to predate these guarantees; the spec does not describe a pre-1.0 versioning or channel scheme beyond this gate.

## Related documents

- [`../architecture/PROVIDERS.md`](../architecture/PROVIDERS.md) — provider lifecycle, integration levels, and registry mechanics (primary source for §32/§33 detail).
- [`../architecture/DATABASE.md`](../architecture/DATABASE.md) — schema and migration mechanics (primary source for §106 detail).
- [`BUILDING.md`](./BUILDING.md) — what building Kelpie produces (the artifacts a release would ship).
- [`../ROADMAP.md`](../ROADMAP.md) — phase sequencing leading up to 1.0.
