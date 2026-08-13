# Debian Packaging

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Packaging formats

The spec groups Debian and Ubuntu together under a single "Debian/Ubuntu" packaging bullet and names two formats for this grouping (kelpie.md §123):

- `.deb`
- AppImage

No further detail — no build tooling, dependency list, repository plan, or `.deb` control-file specifics — is given in §123 for this grouping.

## Relationship to Ubuntu

This document and [`UBUNTU.md`](./UBUNTU.md) are necessarily near-duplicates: kelpie.md §123 does not differentiate Debian from Ubuntu packaging. Both distributions are covered by the same "Debian/Ubuntu" bullet and the same two formats above. Treat packaging for the two as identical unless and until a maintainer deliberately differentiates them (e.g. distinct `.deb` builds per Debian/Ubuntu release, or a PPA for Ubuntu specifically) — nothing in the spec currently calls for that split.

## Later

The spec's "Later" note under §123 (not Debian/Ubuntu-specific, but applicable here) mentions eventual:

- Native repositories
- Community distribution packaging

as future-phase items, not part of the current plan.

## See also

- [`LINUX.md`](./LINUX.md) for general Linux desktop integration and filesystem layout.
- [`../development/BUILDING.md`](../development/BUILDING.md) for build instructions.
