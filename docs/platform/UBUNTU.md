# Ubuntu Packaging

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Packaging formats

kelpie.md §123 does not treat Ubuntu as a distribution distinct from Debian: it groups them under a single "Debian/Ubuntu" packaging bullet and names two formats for that grouping:

- `.deb`
- AppImage

No further detail — no build tooling, dependency list, repository plan, or Ubuntu-specific packaging steps (e.g. a PPA) — is given in §123.

## Relationship to Debian

This document and [`DEBIAN.md`](./DEBIAN.md) are necessarily near-duplicates: the source spec groups Ubuntu with Debian rather than specifying it separately. Packaging is identical between the two unless and until a maintainer deliberately differentiates them. Do not read anything Ubuntu-specific into this document beyond what §123 states for the shared "Debian/Ubuntu" grouping.

## Later

The spec's "Later" note under §123 mentions eventual:

- Native repositories
- Community distribution packaging

as future-phase items, not part of the current plan.

## See also

- [`LINUX.md`](./LINUX.md) for general Linux desktop integration and filesystem layout.
- [`../development/BUILDING.md`](../development/BUILDING.md) for build instructions.
