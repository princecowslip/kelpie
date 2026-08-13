# Contributing

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

kelpie.md has no dedicated section on contribution process — no CLA, review SLA, branch strategy, or code-of-conduct is specified anywhere in the spec, and none is asserted here. This document is a conservative synthesis of the pieces of kelpie.md that bear on contributing: the repository layout a contribution would land in, the tooling and workflow described for provider development, and the Definition-of-Done checklists that the spec uses to define when work is complete. Until a maintainer establishes an explicit contribution process, treat this document as a description of the codebase shape and completion bar, not a procedural guide.

## Repository layout

kelpie.md §131 (Repository Layout) defines where contributed code is expected to live:

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

Application/platform code (Rust services, React UI) belongs under `apps/`, `crates/`, and `packages/`; individual provider adapters belong under `providers/` (split into `official`, `generic`, and `examples`); schema changes belong under `migrations/`; and documentation belongs under `docs/`, organized per [`../../kelpie.md`](../../kelpie.md) §132. None of these directories currently exist in this repository — per `CLAUDE.md`, they should not be scaffolded speculatively.

## Provider developer workflow

Provider contribution is the one area of kelpie.md with the most concrete tooling description. kelpie.md §116 (Provider Developer Mode) describes an in-app developer toolset:

- provider console
- network inspector
- HTML inspector
- normalized output viewer
- manifest viewer
- permission viewer
- fixture recorder
- parser tester

The spec gives a worked example of the kind of feedback this tooling surfaces — a feed request logged with its status and timing, the parsed item count, and specific warnings:

```
GET feed
200 OK
324 ms

Parsed
  24 items

Warnings
  2 missing dates
  1 missing thumbnail
```

kelpie.md §146 (Phase 12 — Provider Developer Experience) plans a companion CLI, `kelpie-provider`, with subcommands:

- `kelpie-provider new`
- `kelpie-provider test`
- `kelpie-provider lint`
- `kelpie-provider pack`

alongside a developer UI covering the same ground as §116: fixture recorder, parser inspector, network inspector, permission inspector, normalized output. Together, these describe an intended workflow of scaffolding a new provider (`new`), iterating against recorded fixtures and a parser tester/inspector, validating it (`test`, `lint`), and packaging it (`pack`) — with the in-app Provider Developer Mode tools available for interactive debugging throughout. This tooling is planned as part of Phase 12, not Phase 1; see [`../ROADMAP.md`](../ROADMAP.md) for phase sequencing.

## Definition of Done

kelpie.md defines two separate completion checklists, depending on what is being contributed. Anything landed against this repository should be checked against the relevant one.

### Application features (kelpie.md §153)

A feature is complete only when it has:

- happy path
- loading state
- empty state
- failure state
- offline behavior
- private-mode behavior
- keyboard behavior
- accessibility behavior
- persistence behavior
- tests
- documentation

### Providers (kelpie.md §154)

Provider completion requires:

- manifest validation
- permission review
- fixture tests
- pagination tests
- missing-field tests
- malformed response tests
- rate-limit behavior
- login handling
- browser fallback
- diagnostic redaction
- last-tested date
- registry review

These checklists are the acceptance bar this spec defines for contributed work — an application-feature contribution should satisfy §153 in full, and a provider contribution should satisfy §154 in full, including the registry review step for providers.

## Architecture decisions

kelpie.md §133 (Architecture Decision Records) is the one piece of genuine "how a nontrivial decision gets made" process the spec defines, and it belongs in a contributing guide more than anywhere else in this tree. It calls for ADRs on irreversible decisions, in a fixed format:

```
Context
Decision
Alternatives
Consequences
Status
```

and names eight ADR stubs, all of which now exist under [`../adr/`](../adr/): `0001-tauri.md`, `0002-sqlite.md`, `0003-provider-sandbox.md`, `0004-no-remote-native-ipc.md`, `0005-browser-fallback.md`, `0006-separate-provider-registry.md`, `0007-structural-media-taxonomy.md`, `0008-local-first-history.md`. Per `CLAUDE.md`, every ADR in this repository stays at `Status: Proposed` until an actual implementation lands that depends on the decision — a contribution that touches one of these areas should update the relevant ADR's Consequences/Status rather than silently diverging from a decision it documents, and a genuinely new irreversible decision should get a new ADR following the same format.

## Related documents

- [`../providers/SDK.md`](../providers/SDK.md), [`../providers/MANIFEST.md`](../providers/MANIFEST.md) — provider SDK and manifest schema referenced by the developer tooling above.
- [`TESTING.md`](./TESTING.md) — the app-wide testing strategy underlying the "tests" line of the Definition of Done.
- [`../providers/TESTING.md`](../providers/TESTING.md) — provider fixture testing underlying the Provider Definition of Done.
- [`BUILDING.md`](./BUILDING.md) — what building the repository involves.
