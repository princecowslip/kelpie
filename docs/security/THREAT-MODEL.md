# Threat Model

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

Kelpie aggregates content from many external, largely untrusted provider websites and renders it inside a desktop application that also holds local secrets, a local media library, and (via the embedded browser) live authenticated sessions on third-party sites. That combination — untrusted remote content plus locally privileged capability — is the source of Kelpie's attack surface, and it is why security is treated as a first-class architectural concern rather than an afterthought.

The primary architectural reference for this threat model is the trust-domain split described in [`../architecture/OVERVIEW.md`](../architecture/OVERVIEW.md) §8 (Fundamental Trust Architecture), which divides the application into three execution domains:

- **Trusted Core** — the Rust application services, with access to SQLite, local configuration, approved filesystem locations, OS integration, secrets, and the media backend.
- **Restricted Provider Runtime** — sandboxed provider code, limited to explicitly permitted origins, provider-specific storage/session state, and parsing APIs, with no access to arbitrary shell commands, arbitrary filesystem, other providers' sessions, or native application commands.
- **Untrusted Web Content** — remote pages rendered in dedicated browser webviews, with no privileged native IPC.

Every specific concern documented elsewhere in this `security/` tree — provider sandboxing, webview isolation, content safety, secret storage — exists to enforce a boundary between two of these domains. This document is the umbrella threat model that ties them together and defines the regression surface used to verify those boundaries hold.

## Threat Actors and Assumptions

The threat model assumes the primary adversary is not a person attacking the user directly, but **untrusted or compromised content and code arriving through provider integrations**: a malicious or compromised provider website, a malicious or buggy provider adapter, a crafted media file, or a hostile page loaded in the embedded browser. Kelpie's defenses are built on the assumption that any of these can be adversarial at any time, even for providers that are officially registered — see the quarantine mechanism below.

## Required Regression Areas

Kelpie's security testing is defined around a fixed checklist of regression areas. Each item corresponds to a way the trust-domain boundaries above could fail in practice, and each is expected to be covered by ongoing security regression testing rather than validated once and forgotten:

- **Provider sandbox escape** — a provider adapter obtaining capabilities outside the Restricted Provider Runtime's allowed API surface (see [`PROVIDER-SANDBOX.md`](./PROVIDER-SANDBOX.md)).
- **Unexpected redirects** — a provider or browser navigation being redirected to an origin outside its declared/permitted set.
- **HTML injection** — provider-supplied HTML introducing markup that escapes its intended rendering context.
- **Script injection** — provider-supplied or page-supplied content executing script in a trusted context.
- **IPC exposure** — a webview or provider process gaining access to native IPC commands it should not have.
- **Path traversal** — provider-supplied paths or filenames escaping approved filesystem locations.
- **URL scheme abuse** — navigation to disallowed or dangerous URL schemes (see [`WEBVIEW-ISOLATION.md`](./WEBVIEW-ISOLATION.md)).
- **Cookie leakage** — cookies or session state crossing between providers, or leaving the domain they belong to.
- **Secret leakage** — credentials, tokens, or other secrets becoming visible outside the Trusted Core (see [`SECRET-STORAGE.md`](./SECRET-STORAGE.md)).
- **Oversized responses** — providers or remote pages returning responses large enough to exhaust memory or degrade the application.
- **Decompression bombs** — compressed payloads (images, archives, etc.) expanding to a disproportionate size on decode.
- **Malformed image** — crafted or corrupt image data intended to crash or exploit the image pipeline.
- **Malformed animation** — crafted or corrupt animated image/GIF data with the same intent.
- **Malformed video** — crafted or corrupt video data with the same intent.
- **Malformed archive** — crafted or corrupt archive data (e.g. manga/comic chapter packages) with the same intent.
- **SQL injection** — provider or user-supplied data reaching SQLite queries without safe parameterization.

These are not independent bug categories; they are the concrete failure modes of the trust boundaries described in `../architecture/OVERVIEW.md` §8. A successful sandbox escape, IPC exposure, or path traversal is a failure of the Restricted Provider Runtime boundary; script/HTML injection and URL scheme abuse are failures of the Untrusted Web Content boundary; secret leakage is a failure of the Trusted Core's custody of secrets; and the malformed-content and decompression-bomb items are failures of content handling that sits downstream of any of the three domains.

## How This Document Relates to the Rest of `security/`

This file defines *what must be regression-tested*. It intentionally does not re-derive the mechanics of each defense — those live in their own documents:

- [`PROVIDER-SANDBOX.md`](./PROVIDER-SANDBOX.md) — the host/denied API boundary that provider sandbox escape, IPC exposure, and unrestricted network access are tested against.
- [`WEBVIEW-ISOLATION.md`](./WEBVIEW-ISOLATION.md) — the webview capability model that unexpected redirects, script/HTML injection, and URL scheme abuse are tested against.
- [`CONTENT-SAFETY.md`](./CONTENT-SAFETY.md) — the sanitization and content-handling rules that malformed-media and oversized-response/decompression-bomb testing verify.
- [`SECRET-STORAGE.md`](./SECRET-STORAGE.md) — the storage and diagnostics rules that secret-leakage and cookie-leakage testing verify.

Architecturally, the trust domains these defenses sit on top of are described in [`../architecture/OVERVIEW.md`](../architecture/OVERVIEW.md) §8.
