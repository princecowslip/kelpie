# CLAUDE.md

Guidance for Claude Code (or any AI agent) working in this repository.

## Repository status

**Implementation is underway, early.** Phase 1 (§135, Desktop Foundation) is complete and Phase 2 (§136, Core Data Layer) is in progress; see `docs/ROADMAP.md` for phase-by-phase status. This repository currently contains:

- `kelpie.md` — the canonical v4.0 Product/UX/Architecture/Provider-Registry/Implementation specification. This is the **source of truth**.
- `docs/` — a documentation set derived from `kelpie.md`, organized per the repository/documentation layout the spec itself prescribes (see `kelpie.md` §131–133). It is an extraction and reorganization of the spec, not an independent authority. If `docs/` and `kelpie.md` ever disagree, treat `kelpie.md` as correct unless a maintainer has deliberately updated both together.
- `README.md` — a short pointer into the above.
- `crates/`, `apps/desktop/`, `packages/` — landed implementation (Phase 1 complete, Phase 2 in progress), matching the `kelpie.md` §131 Repository Layout.
- `providers/`, `registry/`, `tests/`, `packaging/` — placeholder directories per §131, not yet populated with real content; populated starting Phase 3 (§137) onward per the phase sequencing.

Everything described anywhere in this repository beyond what has actually landed in `crates/`, `apps/`, and `packages/` — architecture, APIs, providers, later phases — remains **planned/aspirational**, not built or verified behavior.

## Working in this repo

- Don't scaffold directories or phases ahead of the implementation plan. If asked to start or continue implementation, confirm the request against `kelpie.md` §131 (Repository Layout) and the phase sequencing in §134–148 (Implementation Plan) — check `docs/ROADMAP.md` for what's already landed — rather than inventing structure or skipping ahead of the current phase.
- When adding or editing documentation, keep it consistent with `kelpie.md`. Cite or link back to the relevant section when practical.
- Keep every `docs/**/*.md` and `docs/adr/*.md` file's guardrail banner intact — the one-line note stating the doc is derived from `kelpie.md` v4.0 and describes intended, not built, behavior. Don't let generated docs drift into asserting things as shipped fact.
- ADRs under `docs/adr/` use `Status: Proposed` until an actual implementation exists that depends on the decision. Don't bump an ADR to `Accepted` without a maintainer decision tied to real, landed code.

## Content boundaries

Kelpie's product scope is lawful adult media aggregation. The boundaries in `kelpie.md` §4 (Hard Product Boundaries) and §5 (Adult-Only Safety Model) apply to everything produced in this repository, not only to runtime application behavior:

- Never produce, describe, or endorse child sexual abuse material, including fictional/illustrated sexualization of minors, in documentation, examples, provider fixtures, or test data.
- Never produce non-consensual sexual imagery or content, or guidance for defeating consent/reporting systems.
- Never write or request code that circumvents DRM, authentication, paywalls, CAPTCHAs, or geographic restrictions.
- Real third-party sites named in the spec (e.g. Pornhub, XVideos, FAKKU, Literotica) are registry-planning references only — used to reason about provider categories and integration levels. Do not write live scraper/integration code against them outside the sandboxed provider model in `kelpie.md` §26–28, and do not fabricate additional named providers beyond what the spec or a maintainer specifies.

## Navigation

`kelpie.md` is long (~4,000 lines, 158 numbered sections). Prefer targeted reads:

- Use the numbered section headings (`grep -n '^[0-9]\+\. '`) to jump to the relevant part instead of reading the whole file for small tasks.
- `docs/` mirrors the spec's own prescribed documentation architecture (`kelpie.md` §132) — check there first for a focused summary before going back to the full spec.
