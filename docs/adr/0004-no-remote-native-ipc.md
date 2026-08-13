# ADR 0004: Remote/Untrusted Web Content Never Receives Native IPC Access

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Context

Kelpie's embedded browser and provider-facing surfaces necessarily render pages from arbitrary, adversarial third-party websites — login pages, subscription content, unsupported providers, CAPTCHA challenges, JavaScript-heavy pages, DRM-gated content, and general manual browsing (kelpie.md §91). kelpie.md's trust-domain model treats this category, "Untrusted Web Content," as fully distinct from both the trusted core and the restricted provider runtime: "Remote pages render in dedicated browser webviews. They receive no privileged native IPC." (kelpie.md §8). It further notes that "Tauri 2's current capability model allows permissions to be constrained to specific windows/webviews; a webview without a matching capability receives no IPC access. That should be used as the hard boundary between Kelpie's trusted UI and remote sites" (kelpie.md §8).

This boundary is reinforced downstream at the content layer: extracted HTML must be sanitized, rejecting script, iframe, object, embed tags, event handlers, and unsafe URL schemes by default, with the explicit rule that "Remote interactive web pages belong in browser webviews rather than trusted application HTML" (kelpie.md §119). URL navigation is likewise restricted — only `https`/`http` are externally navigable by default, while `javascript:`, `data:`, `file:`, and arbitrary custom schemes must be rejected or explicitly reviewed (kelpie.md §120).

## Decision

Remote and otherwise untrusted web content — anything rendered in a browser webview rather than originating from the trusted Rust core or a permitted provider-sandbox call — never receives privileged native IPC access. This is enforced structurally via Tauri 2's per-window/per-webview capability model (a webview without a matching capability grant gets no IPC surface at all), and is reinforced by sanitizing any extracted HTML before it can reach trusted application UI, and by restricting externally navigable URL schemes to `https`/`http` by default.

## Alternatives

kelpie.md frames this as the hard boundary rather than presenting a named alternative; the reasoned rejected alternative is:

- **Granting remote webviews a filtered/limited native IPC surface** (e.g. allowing a small allowlisted set of commands) — rejected implicitly, since the spec's language is categorical ("no privileged native IPC," used "as the hard boundary" — kelpie.md §8) rather than describing a reduced-but-present IPC surface. Any such surface would reintroduce exactly the sandbox-escape risk the trust-domain split exists to prevent, since remote pages are adversarial by default and not reviewed the way official provider packages are.
- **Trusting sanitized HTML enough to render it inside privileged application UI** — rejected in favor of routing all remote interactive content through isolated browser webviews instead (kelpie.md §119), since sanitization reduces but does not eliminate the risk of a bypass reaching a trusted rendering context.

## Consequences

- The embedded browser (see [`0005-browser-fallback.md`](./0005-browser-fallback.md)) can safely be used for login, subscription/DRM content, CAPTCHA, and unsupported-provider browsing precisely because compromise of a remote page cannot, by construction, reach native commands.
- "Add to Kelpie" and similar bridges from browser content back into the trusted app must go through an explicit resolver step (URL → provider resolver → recognized/unrecognized — kelpie.md §92) rather than any direct IPC call originating from the remote page itself.
- HTML sanitization (rejecting script/iframe/object/embed/event handlers/unsafe URL schemes — kelpie.md §119) and URL scheme restriction (kelpie.md §120) become mandatory, continuously-tested controls, not one-time defaults; see [`../security/THREAT-MODEL.md`](../security/THREAT-MODEL.md) and [`../security/WEBVIEW-ISOLATION.md`](../security/WEBVIEW-ISOLATION.md).
- This boundary depends on Tauri 2's capability model behaving as documented (see [`0001-tauri.md`](./0001-tauri.md)); any weakening or bypass of that model is a critical-severity concern for Kelpie specifically because of how much untrusted content the app is designed to render.

## Status

Proposed — kelpie.md v4.0 documents this as an intended decision; no code exists yet that depends on it. This will move to Accepted once an implementation lands that relies on it, or Superseded/Rejected if reconsidered.
