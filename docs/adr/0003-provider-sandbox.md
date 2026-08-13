# ADR 0003: Sandbox Provider Execution Instead of Running Provider Code Natively

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Context

Kelpie aggregates content from many third-party provider websites via provider adapters/packages, some of which are described as community-authored or locally installed (kelpie.md §30, Experimental integration level). Kelpie's own hard product boundaries explicitly rule out executing "untrusted provider code with native privileges" (kelpie.md §4). The trust-domain model places provider code in a distinct "Restricted Provider Runtime" domain, separate from the trusted Rust core and from untrusted remote web content: it may access explicitly permitted origins, provider-specific storage, provider-specific session state, parsing APIs, and provider logs, but it must not be able to reach arbitrary shell commands, arbitrary filesystem, other providers' sessions, or native application commands (kelpie.md §8).

kelpie.md §27 (Provider Sandbox) states the preferred future implementation is a "WASM/WASI provider package," with a "QuickJS provider host" named as the alternative. It also specifies the host API surface providers are allowed to call — `http.request`, `html.parse`, `xml.parse`, `json.parse`, `cookies.readOwn`, `cookies.writeOwn`, `cache.get`, `cache.put`, `log`, `clock` — and the APIs explicitly denied: arbitrary filesystem, shell execution, process spawning, unrestricted network, environment secrets, other-provider sessions, and native UI IPC.

## Decision

Execute provider code inside a restricted sandbox rather than as native, fully-trusted code — preferably as WASM/WASI provider packages, with a QuickJS-based provider host as the fallback/alternative implementation — exposing only an explicit host API (HTTP requests, HTML/XML/JSON parsing, provider-scoped cookies, provider-scoped cache, logging, clock) and denying arbitrary filesystem access, shell/process execution, unrestricted network access, environment secrets, other providers' sessions, and native UI IPC.

## Alternatives

kelpie.md §27 names this alternatives pair explicitly:

- **Preferred: WASM/WASI provider package** — a WebAssembly/WASI-based sandbox, giving strong memory-safety and capability-based isolation with a well-defined, restrictable host API surface.
- **Alternative: QuickJS provider host** — an embedded JavaScript engine (QuickJS) hosting provider code, offering a scripting environment that may be simpler to author providers against, at the cost of a different (and generally weaker) isolation model than WASM/WASI.

A third option the spec rules out rather than merely deprioritizes is running provider code with native privileges in the trusted core — explicitly forbidden by the hard product boundaries (kelpie.md §4) because provider code is untrusted and, for community/experimental providers (kelpie.md §30), not authored or reviewed by Kelpie's maintainers at all.

## Consequences

- Every provider integration, including official first-party ones, runs under the same restricted capability set rather than being implicitly trusted, which is what makes the "Quarantined" integration level (a provider package prevented from executing — kelpie.md §30) meaningful as a real enforcement mechanism rather than a policy statement.
- Provider capabilities must be modeled and granted explicitly (permitted origins, provider-scoped storage/session/cache) rather than assumed, which ties directly into the registry's per-provider `capabilities`/`permissions` metadata (kelpie.md §52) — see [`0006-separate-provider-registry.md`](./0006-separate-provider-registry.md).
- Choosing between the WASM/WASI and QuickJS paths is a real implementation fork that affects how provider packages are authored, distributed, and versioned; either choice must preserve the same denied-API boundary.
- The sandbox is one of the three trust-domain boundaries the application's security regression testing is built around; see [`../security/THREAT-MODEL.md`](../security/THREAT-MODEL.md) (provider sandbox escape) and [`../architecture/PROVIDERS.md`](../architecture/PROVIDERS.md).
- Because the sandbox is a process/runtime boundary rather than a policy convention, moving provider execution out-of-process later (as the process model in kelpie.md §11 anticipates via `kelpie-provider-host`) is a deployment change, not an architectural one; see [`0001-tauri.md`](./0001-tauri.md).

## Status

Proposed — kelpie.md v4.0 documents this as an intended decision; no code exists yet that depends on it. This will move to Accepted once an implementation lands that relies on it, or Superseded/Rejected if reconsidered.
