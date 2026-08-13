# Provider Sandbox

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

Providers are Kelpie's adapters onto external, third-party sources — code that is, by nature, less trusted than the application's own Trusted Core. A provider may be well-behaved, buggy, compromised after the fact, or occasionally outright malicious. The provider sandbox exists to make sure that regardless of which of those is true at any given moment, a provider cannot do more damage than "misbehave within its own restricted runtime." This document explains the security rationale for that boundary — what it defends against and why it is shaped the way it is. For the developer-facing view of the same underlying mechanism (how a provider declares its permissions and what the manifest format looks like), see [`../providers/PERMISSIONS.md`](../providers/PERMISSIONS.md).

This is the Restricted Provider Runtime domain described in [`../architecture/OVERVIEW.md`](../architecture/OVERVIEW.md) §8, and the boundary this document covers is the one exercised by "provider sandbox escape" and "IPC exposure" in the regression checklist in [`THREAT-MODEL.md`](./THREAT-MODEL.md).

## Why Provider Code Is Sandboxed

Kelpie's product boundaries are explicit that it must not execute untrusted provider code with native privileges. Provider adapters are written to interoperate with arbitrary third-party websites, which means they are inherently processing untrusted input (HTML, JSON, XML, redirects, cookies) from sources outside Kelpie's control. Sandboxing provider execution means a hostile or compromised website cannot use a provider adapter as a stepping stone into the user's filesystem, shell, other providers' sessions, or the OS — even if the provider's parsing logic itself is exploited.

## Execution Model

The preferred future implementation runs provider code as a WASM/WASI package, with a QuickJS-based provider host as an alternative. Both are chosen because they allow provider code to run with a narrow, explicitly-granted capability surface rather than ambient access to the host system — the sandbox is enforced by the execution environment itself, not only by convention in the provider code.

## Host APIs (Allowed)

Providers are given access only to a deliberately narrow set of host-provided capabilities, sufficient to implement a content adapter but nothing more:

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

Each of these is scoped to the calling provider: `cookies.readOwn`/`cookies.writeOwn` operate only on that provider's own cookies, not any other provider's or the application's; `http.request` is further constrained by the network permission model described below rather than being unrestricted outbound access.

## Denied APIs

The following are explicitly denied to provider code, regardless of implementation host:

- Arbitrary filesystem
- Shell execution
- Process spawning
- Unrestricted network
- Environment secrets
- Other-provider sessions
- Native UI IPC

This list is the direct enforcement of the Restricted Provider Runtime's constraints in the trust architecture: a provider cannot read or write outside its own storage, cannot spawn processes or shell out, cannot see secrets or environment configuration belonging to the Trusted Core, cannot reach into another provider's session state, and cannot call native UI IPC commands — the same IPC isolation principle documented for webviews in [`WEBVIEW-ISOLATION.md`](./WEBVIEW-ISOLATION.md) applies here to provider code.

## Provider Network Permissions

Because `http.request` is a permitted host API, network access is the primary channel through which a provider could otherwise reach unintended destinations — so it is independently constrained on top of the sandbox itself. Every provider declares the origins it is allowed to talk to, for example:

- `example.com`
- `api.example.com`
- `cdn.example.com`

The permission layer validates network activity against that declared origin set at multiple points, not just on the first request:

- Initial request
- Redirect destination
- WebSocket destination, if supported
- Media host
- API host

Unexpected origins are rejected. Validating redirect destinations and media/API hosts separately from the initial request matters because a provider's declared origin being safe does not guarantee every redirect or embedded resource it might encounter stays within that set — an adversarial or compromised upstream site could otherwise use a redirect to route a provider's traffic somewhere unintended.

## Automatic Provider Quarantine

Sandboxing bounds what a provider can do at any instant, but providers can also change behavior over time — a package can be tampered with, a domain can change hands, or a provider can start behaving suspiciously after having been trustworthy. Kelpie addresses this with automatic quarantine, which stops provider execution while preserving user library state whenever one of the following is detected:

- Provider signature invalid
- Package hash mismatch
- Unexpected domain change
- Unexpected permission expansion
- Persistent parser failure
- Security advisory
- Malicious redirect pattern

Quarantine is a runtime response layered on top of the sandbox and network-permission checks above: the sandbox limits what a provider can do while it runs, and quarantine limits how long a provider that starts behaving suspiciously is allowed to keep running at all. The full registry lifecycle this quarantine mechanism is embedded in — including how providers move between registry states and how a quarantined provider is surfaced to the user — is documented primarily in [`../providers/REGISTRY.md`](../providers/REGISTRY.md).

## Summary

| Layer | Defends against |
|---|---|
| WASM/WASI or QuickJS sandboxed execution | Native privilege escalation, filesystem/shell/process access from provider code |
| Narrow host API allowlist | Provider code doing anything beyond parsing and making declared network calls |
| Denied API list | Cross-provider session access, secret access, native UI IPC from provider code |
| Declared-origin network permissions, validated per-request/redirect/media/API | A provider (or a site it talks to) redirecting or reaching outside its declared network footprint |
| Automatic quarantine | A provider that was safe becoming unsafe after release — tampering, domain hijack, permission creep, security advisories |
