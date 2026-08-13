# Testing

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

This document covers Kelpie's app-wide testing strategy — unit and integration tests for core application logic, and end-to-end tests for the whole product. Provider-fixture testing (feed HTML, search JSON, item/chapter pages, and their expected normalized results) is covered separately in [`../providers/TESTING.md`](../providers/TESTING.md), since fixture testing is scoped to individual provider adapters rather than the application as a whole.

## Testing Strategy (kelpie.md §117)

### Unit tests

Unit-level coverage is expected for core application logic, including:

- query parser
- ranking
- deduplication
- chapter sorting
- tag normalization
- database models
- permissions

### Integration tests

Integration-level coverage is expected across the boundaries where subsystems meet:

- provider runtime
- scoped HTTP
- database migrations
- feed pipeline
- browser bridge
- media resolution

### End-to-end tests

End-to-end coverage is expected to exercise full user-facing flows:

- launch
- add source
- refresh
- search
- play
- read
- save
- resume
- private mode
- provider failure

Notably, "provider failure" and "private mode" are both named explicitly as end-to-end scenarios, not just unit-level edge cases — the spec treats degraded/adversarial conditions and privacy-sensitive modes as first-class flows to validate end-to-end, not just failure branches inside individual functions.

## Definition of Done — Application Feature (kelpie.md §153)

Application-level testing exists in service of a broader completion bar. Per kelpie.md §153, an application feature is complete only when it has:

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

"Tests" appears as one line item among several state/behavior requirements — the implication is that a feature's tests are expected to cover the other items in this list (happy path, loading/empty/failure states, offline behavior, private-mode behavior, keyboard behavior, accessibility behavior, and persistence behavior), not just a nominal test file. The unit/integration/end-to-end split above is the mechanism; this Definition of Done is the acceptance bar that testing is expected to satisfy for any given feature.

## Related documents

- [`../providers/TESTING.md`](../providers/TESTING.md) — provider fixture testing (feed HTML, search JSON, item/chapter pages, pagination, missing-field, malformed-response, rate-limit, login, browser fallback).
- [`CONTRIBUTING.md`](./CONTRIBUTING.md) — how the Definition of Done applies to contributed work.
- [`RELEASES.md`](./RELEASES.md) — how testing gates factor into the Version 1.0 Gate.
