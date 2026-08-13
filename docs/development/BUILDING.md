# Building Kelpie

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

Kelpie has no buildable code yet — no `apps/`, `crates/`, `packages/`, or `providers/` directories exist in this repository (kelpie.md §131, Repository Layout). This document describes what building Kelpie is intended to involve once **Phase 1 — Desktop Foundation** (kelpie.md §135) lands, so that the eventual build setup can be checked against the spec rather than improvised.

## Repository shape

kelpie.md §131 lays out the intended top-level layout:

```
kelpie/
├── apps/
│   └── desktop/
├── crates/
│   ├── core/
│   ├── database/
│   ├── providers/
│   ├── feed/
│   ├── search/
│   ├── media/
│   ├── browser/
│   ├── downloads/
│   ├── security/
│   └── networking/
├── packages/
│   ├── ui/
│   ├── normalized-types/
│   └── provider-sdk/
├── providers/
│   ├── official/
│   ├── generic/
│   └── examples/
├── registry/
├── migrations/
├── tests/
├── packaging/
└── docs/
```

This confirms the stack described elsewhere in the spec: a Rust workspace (`crates/`) underneath a Tauri desktop shell (`apps/desktop`), TypeScript/React packages (`packages/`), and a separate `providers/` tree for provider adapters. Building Kelpie will therefore mean building both a Rust workspace and a JS/TS frontend, packaged together by Tauri.

## What Phase 1 builds

kelpie.md §135 (Phase 1 — Desktop Foundation) scopes the first buildable slice of the application:

- Tauri shell
- React UI
- routing
- navigation
- theme
- settings
- SQLite bootstrap
- logging
- CI
- `.deb`
- AppImage

Phase 1 explicitly uses **fake providers only** — no real provider integrations are part of this phase. This means the first thing that can actually be built and run is the desktop shell itself (window, navigation, theming, settings, a bootstrapped SQLite database, and logging), wired up in CI and producing installable Linux packages, before any real content source is connected.

## Expected build outputs

Based on the Phase 1 scope, building Kelpie is expected to produce:

- A running Tauri desktop application (React frontend + Rust backend) for local development.
- A bootstrapped local SQLite database on first run.
- Two Linux package formats: a `.deb` and an AppImage.

kelpie.md does not specify build tool invocations, package manager choices, or CI provider configuration beyond stating that CI is part of the Phase 1 deliverable — those specifics are left to implementation and should not be assumed here.

## Platform validation

The Version 1.0 Gate (kelpie.md §152) requires Debian/Ubuntu/Arch validation before a 1.0 release. See [`../platform/DEBIAN.md`](../platform/DEBIAN.md), [`../platform/UBUNTU.md`](../platform/UBUNTU.md), [`../platform/ARCH.md`](../platform/ARCH.md), and [`../platform/LINUX.md`](../platform/LINUX.md) for platform-specific detail. Packaging formats beyond `.deb`/AppImage (e.g. an Arch package) are not specified as part of Phase 1 itself.

## Related documents

- [`../architecture/OVERVIEW.md`](../architecture/OVERVIEW.md) — overall system architecture.
- [`../ROADMAP.md`](../ROADMAP.md) — full phase sequencing (kelpie.md §134–148).
- [`RELEASES.md`](./RELEASES.md) — how the built artifacts relate to releases and versioning.
- [`TESTING.md`](./TESTING.md) — what CI is expected to run against these builds.
