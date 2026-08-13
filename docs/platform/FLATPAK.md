# Flatpak Packaging

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Packaging format

kelpie.md §123 lists Flatpak as Kelpie's sole "Cross-distribution" packaging format, distinct from the per-distribution formats covered elsewhere ([`DEBIAN.md`](./DEBIAN.md), [`UBUNTU.md`](./UBUNTU.md), [`ARCH.md`](./ARCH.md)). No further detail — no manifest, runtime/SDK choice, or publishing target (e.g. Flathub) — is given in §123.

## Relationship to Flatpak portals

§121 (Linux Integration) separately lists "Flatpak portals" among the desktop standards Kelpie is expected to follow — see [`LINUX.md`](./LINUX.md). That entry concerns runtime sandboxing/portal integration (e.g. file access, notifications going through portals when running under Flatpak confinement), while this document concerns the packaging format itself. The spec does not connect the two beyond both mentioning Flatpak; no specific portal names or usage details are given in either section.

## Later

The spec's "Later" note under §123 mentions eventual:

- Native repositories
- Community distribution packaging

as future-phase items, not part of the current plan.

## See also

- [`LINUX.md`](./LINUX.md) for general Linux desktop integration and filesystem layout.
- [`../development/BUILDING.md`](../development/BUILDING.md) for build instructions.
