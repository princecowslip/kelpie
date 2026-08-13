# ADR 0001: Use Tauri 2 as the Desktop Shell

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Context

Kelpie needs a desktop application shell for a Rust-backed core with a React/TypeScript frontend, running on Linux (with the door left open to other desktop platforms). The application also has an unusual security requirement for a typical desktop app: it must render a mix of trusted application UI, sandboxed provider-driven content, and fully untrusted remote websites (via an embedded browser) within the same process family, without letting the untrusted content reach privileged native capabilities.

kelpie.md's recommended technology stack names Tauri 2 as the desktop shell, paired with Rust/Tokio/SQLite on the native side and React/TypeScript/Vite on the frontend (kelpie.md §9). Tauri 2's capability model is called out specifically: permissions can be constrained to specific windows/webviews, so a webview without a matching capability receives no IPC access at all (kelpie.md §8, Untrusted Web Content). The recommended process model builds on this: a trusted Rust core plus trusted frontend webview (`kelpie`), a separate restricted provider-execution process (`kelpie-provider-host`), untrusted remote webviews, and an optional `mpv` media subprocess (kelpie.md §11). The MVP may keep some of this in-process, but the interfaces are meant to be designed as though provider execution can move out-of-process later.

## Decision

Use Tauri 2 as Kelpie's desktop application shell, hosting a Rust application core and a React/TypeScript frontend webview, and rely on Tauri 2's per-window/per-webview capability model as the mechanism that keeps untrusted and semi-trusted content out of the trusted native IPC surface.

## Alternatives

kelpie.md does not enumerate rejected desktop-shell frameworks explicitly. The reasoned alternatives are the other common paths to a Rust+web-frontend desktop app:

- **Electron** — mature and widely used, but ships a full bundled Chromium/Node runtime per app and has a materially different (less granular) native-bridge security model than Tauri 2's per-webview capability system, which matters given Kelpie's requirement to strictly wall off untrusted remote content from native IPC (kelpie.md §8).
- **A hand-rolled native GUI** (e.g. GTK/Qt directly) — would avoid an embedded-webview dependency entirely, but would require reimplementing the React/TypeScript frontend as native UI and would lose the webview-based embedded-browser fallback that kelpie.md §91 depends on for DRM/subscription/unsupported-provider sources.

Given Kelpie's specific need for fine-grained, per-webview IPC isolation alongside a React/TypeScript frontend, Tauri 2 is the option the spec settles on.

## Consequences

- Kelpie's trusted UI, restricted provider execution, and untrusted remote browsing can be modeled as distinct webviews/processes with different capability grants, which is the foundation the provider sandbox (see [`0003-provider-sandbox.md`](./0003-provider-sandbox.md)) and the no-native-IPC-for-remote-content rule (see [`0004-no-remote-native-ipc.md`](./0004-no-remote-native-ipc.md)) both build on.
- The recommended process model (`kelpie`, `kelpie-provider-host`, remote webviews, optional `mpv`) is designed around Tauri's multi-webview/multi-window capabilities; see [`../architecture/OVERVIEW.md`](../architecture/OVERVIEW.md) and [`../architecture/PROVIDERS.md`](../architecture/PROVIDERS.md).
- The MVP may keep provider infrastructure in-process for simplicity, but application services must still communicate through typed commands/events (kelpie.md §10) so that moving provider execution out-of-process later does not require an architectural rewrite.
- Kelpie's security posture becomes partially dependent on Tauri 2's capability system behaving as documented; regression coverage for IPC exposure is tracked in [`../security/THREAT-MODEL.md`](../security/THREAT-MODEL.md) and [`../security/WEBVIEW-ISOLATION.md`](../security/WEBVIEW-ISOLATION.md).

## Status

Proposed — kelpie.md v4.0 documents this as an intended decision; no code exists yet that depends on it. This will move to Accepted once an implementation lands that relies on it, or Superseded/Rejected if reconsidered.
