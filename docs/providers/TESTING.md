# Provider Testing

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Overview

kelpie.md §117 (Testing Strategy) describes a testing pyramid spanning Unit, Provider fixtures, Integration, and End-to-End layers for the application as a whole. This document covers the two layers of that pyramid that are specific to provider code — provider fixtures and provider-runtime integration testing — plus the provider-specific Definition of Done (kelpie.md §154). General application testing (query parser, ranking, deduplication, database models, feed pipeline, and the full End-to-End layer) is out of scope here; see [`../development/TESTING.md`](../development/TESTING.md) for that.

## Provider fixtures

kelpie.md §117 specifies a fixture-based testing layer sitting between plain unit tests and full integration tests, built around captured real inputs and their expected normalized outputs:

- Feed HTML
- Search JSON
- Item page
- Chapter page
- Expected normalized result

The pattern is to capture a real (or representative) response for each of a provider's supported request types, and pair it with the normalized object Kelpie's provider adapter is expected to produce from it. This gives a provider a regression-test suite that doesn't depend on the live source being reachable, responsive, or unchanged at test time — a fixture is a frozen snapshot of what the source once returned, checked against a frozen expectation of what the adapter should do with it.

Provider Developer Mode's fixture recorder (kelpie.md §116, see [`SDK.md`](./SDK.md)) is the tool specified for capturing these fixtures from a live provider run.

## Provider-runtime integration testing

Of the Integration-layer items kelpie.md §117 lists, the following are specific to exercising a provider inside its actual runtime rather than the application at large:

- Provider runtime
- Scoped HTTP
- Browser bridge
- Media resolution

These exercise the pieces described in [`PERMISSIONS.md`](./PERMISSIONS.md) and [`SDK.md`](./SDK.md) end-to-end within a single provider: that the sandbox actually constrains what the provider can do (provider runtime), that its declared origins are the only ones its HTTP calls can reach (scoped HTTP), that browser-fallback handoff works for providers that rely on it (browser bridge), and that `media` resolution produces usable `MediaSource` objects (media resolution). Database migrations and the feed pipeline are also part of kelpie.md §117's Integration layer, but they are application-wide concerns rather than provider-specific ones, and belong in [`../development/TESTING.md`](../development/TESTING.md).

## Definition of Done — Provider

kelpie.md §154 specifies that a provider is not considered complete until all of the following are satisfied:

- Manifest validation
- Permission review
- Fixture tests
- Pagination tests
- Missing-field tests
- Malformed response tests
- Rate-limit behavior
- Login handling
- Browser fallback
- Diagnostic redaction
- Last-tested date
- Registry review

Several of these map directly onto the documents elsewhere in this directory: manifest validation and permission review against [`MANIFEST.md`](./MANIFEST.md) and [`PERMISSIONS.md`](./PERMISSIONS.md); fixture, pagination, missing-field, and malformed-response tests against the fixture layer described above; and registry review against the governance process in [`REGISTRY.md`](./REGISTRY.md). Diagnostic redaction and rate-limit/login-handling behavior are provider-runtime concerns exercised as part of provider-runtime integration testing.

## Related documents

- [`SDK.md`](./SDK.md) — Provider Developer Mode's fixture recorder and the `Provider` interface being tested.
- [`MANIFEST.md`](./MANIFEST.md) — manifest validation.
- [`PERMISSIONS.md`](./PERMISSIONS.md) — permission review and the sandbox/network model exercised by provider-runtime integration tests.
- [`REGISTRY.md`](./REGISTRY.md) — registry review and governance cadence.
- [`../development/TESTING.md`](../development/TESTING.md) — general unit and end-to-end application testing, outside provider scope.
