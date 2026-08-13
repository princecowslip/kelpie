# Debian Packaging

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Packaging formats

The spec groups Debian and Ubuntu together under a single "Debian/Ubuntu" packaging bullet and names two formats for this grouping (kelpie.md §123):

- `.deb`
- AppImage

No further detail — no build tooling, dependency list, repository plan, or `.deb` control-file specifics — is given in §123 for this grouping.

## Testing / validation target

Packaging *format* is identical to Ubuntu's (see below), but kelpie.md does draw one genuine distinction between the two distributions: validation target. §148 Phase 14 — Beta Hardening lists testing against "Debian stable" as a distinct entry from "Ubuntu LTS" — i.e. Debian builds are expected to be validated against Debian's stable release channel specifically, not against Ubuntu's LTS cadence, even though both consume the same `.deb`/AppImage artifacts. §151 MVP Acceptance Criteria separately lists "Debian/Ubuntu/Arch validation" as a gate item for the 1.0 release (kelpie.md §152, Version 1.0 Gate, restates this as part of the 1.0 bar). Beyond naming "Debian stable" as the tracked channel, kelpie.md gives no further Debian-specific detail (no minimum version, no point-release policy).

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
