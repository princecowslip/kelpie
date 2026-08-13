# Provider Permissions

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Overview

Provider code is untrusted by default: it comes from a registry entry or a user-added custom source, not from the Kelpie core application itself. Two spec sections govern what a provider is allowed to do — kelpie.md §27 (Provider Sandbox), which bounds what a provider's code can call, and kelpie.md §28 (Provider Network Permissions), which bounds what network origins it can reach. This document is the developer-facing angle on both: what you declare in a provider's manifest, and what the validator checks against that declaration before and during execution.

The same two spec sections also seed [`../security/PROVIDER-SANDBOX.md`](../security/PROVIDER-SANDBOX.md), which covers the same mechanisms from a threat-modeling angle — what attack a given restriction defends against. This document instead answers the practical question a provider author faces: what am I allowed to call, what do I need to declare, and why will the validator reject my package if I don't?

## Execution sandbox

kelpie.md §27 states a preferred and an alternative implementation strategy for running provider code:

- **Preferred future implementation:** a WASM/WASI provider package.
- **Alternative:** a QuickJS provider host.

Regardless of which host is used, a provider only ever runs against a fixed set of host APIs — it cannot reach outside that surface to the underlying system.

### Host APIs

The APIs a provider is allowed to call:

- `http.request`
- `html.parse`
- `xml.parse`
- `json.parse`
- `cookies.readOwn`
- `cookies.writeOwn`
- `cache.get`
- `cache.put`
- `log`
- `clock`

Note that `cookies.readOwn` and `cookies.writeOwn` are scoped to the provider's own cookies — not the cookies of other providers or of the host application.

### Denied APIs

The following are explicitly denied to provider code:

- Arbitrary filesystem access
- Shell execution
- Process spawning
- Unrestricted network access
- Environment secrets
- Other-provider sessions
- Native UI IPC

A provider author should treat this list as fixed: there is no manifest field or permission request that grants access to a denied API. If a provider needs a capability that isn't on the host-API list, that is a signal the capability doesn't belong in provider code.

## Network origin allowlisting

kelpie.md §28 states the rule for network access simply: **"Every provider declares allowed origins."** A manifest's `origins` array (see [`MANIFEST.md`](./MANIFEST.md)) is that declaration, for example:

```
example.com
api.example.com
cdn.example.com
```

### What the permission layer validates

The permission layer checks the declared origin list against several distinct points in a provider's actual network activity, not just the first request:

- Initial request.
- Redirect destination.
- WebSocket destination, if the provider supports one.
- Media host.
- API host.

**Unexpected origins are rejected.** A provider that declares `example.com` and `cdn.example.com` cannot silently redirect through, or fetch media from, some third origin that wasn't part of its declared set — every hop is checked, not just the request the provider code initiated.

## Declaring permissions

In practice, declaring a provider's permission footprint means populating two manifest fields, described fully in [`MANIFEST.md`](./MANIFEST.md):

- `origins` — the allowlisted network origins checked as described above.
- `authentication` — the provider's authentication mode (e.g. `browser-cookie`).

Because the manifest is read before any provider code executes, this is also what Provider Developer Mode's permission viewer (kelpie.md §116, see [`SDK.md`](./SDK.md)) and the registry's permission-review process (kelpie.md §34, see [`REGISTRY.md`](./REGISTRY.md)) are expected to inspect — a provider's declared permissions are meant to be reviewable independently of running its code.

## Related documents

- [`MANIFEST.md`](./MANIFEST.md) — where `origins` and `authentication` are declared.
- [`SDK.md`](./SDK.md) — the `Provider` interface these host APIs and origins are exercised from.
- [`REGISTRY.md`](./REGISTRY.md) — registry governance's permission-set review (kelpie.md §53) and automatic quarantine on unexpected permission expansion (kelpie.md §54).
- [`../security/PROVIDER-SANDBOX.md`](../security/PROVIDER-SANDBOX.md) — the same sandbox and network-permission mechanisms described from a threat-model perspective.
