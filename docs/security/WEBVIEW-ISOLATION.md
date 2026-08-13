# Webview Isolation

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

Kelpie's third trust domain — Untrusted Web Content — covers any remote page rendered inside the application: embedded browser sessions used for login, subscription content, CAPTCHAs, JavaScript-heavy pages, DRM content, provider breakage fallback, and manual browsing. This document describes how that domain is isolated from the Trusted Core and from the Restricted Provider Runtime, and the URL and rendering rules that back it up. The full trust-domain breakdown is documented in [`../architecture/OVERVIEW.md`](../architecture/OVERVIEW.md) §8; this document focuses specifically on the webview/browser boundary. Regression testing for this boundary is tracked in [`THREAT-MODEL.md`](./THREAT-MODEL.md) under "unexpected redirects," "script injection," "IPC exposure," and "URL scheme abuse."

## The Untrusted Web Content Boundary

Remote pages render in dedicated browser webviews, separate from the trusted application UI. These webviews receive no privileged native IPC — a remote page cannot call into Kelpie's Trusted Core commands regardless of what script it runs.

This is enforced through Tauri 2's capability model: permissions can be constrained to specific windows/webviews, and a webview without a matching capability receives no IPC access at all. Kelpie's design treats this as the hard boundary between the trusted application UI and remote sites — trusted application windows are granted the IPC capabilities they need, and webviews hosting remote content are deliberately left without a matching capability grant, rather than being trusted and then restricted after the fact.

## The Embedded Browser

The embedded browser is the primary surface where Untrusted Web Content is intentionally, routinely loaded. Its use cases are:

- Login
- Subscription content
- Unsupported provider
- CAPTCHA
- JavaScript-heavy page
- DRM content
- Provider breakage
- Manual browsing

It exposes a constrained set of browser-chrome controls to the user — back, forward, reload, address, tabs, open externally, and add to Kelpie — but the pages it renders are not treated as trusted application content, and are not granted native IPC regardless of what they do. For the broader embedded browser architecture (the "Add to Kelpie" extraction flow, browser session lifecycle, and how browser-sourced items become library items), the primary reference is [`../architecture/BROWSER.md`](../architecture/BROWSER.md); this document covers only the isolation guarantees.

## HTML Rendering Security

Kelpie also extracts and displays HTML content directly inside trusted application surfaces (for example, article/story text or feed content pulled from a provider). Because that extracted HTML is untrusted-source content being rendered in a trusted context, it must be sanitized before display.

Rejected by default:

- `script`
- `iframe`
- `object`
- `embed`
- event handlers
- unsafe URL schemes

Remote interactive web pages belong in browser webviews rather than in trusted application HTML. In other words, sanitization is not treated as a substitute for the webview boundary — content that needs to run as an interactive page (rather than be displayed as static, sanitized markup) is routed to the embedded browser, not rendered inline in trusted UI.

## URL Security

Navigation and link-following throughout the application are restricted by URL scheme.

Default externally navigable schemes:

- `https`
- `http`

Schemes that are rejected, or that require explicit review before being allowed:

- `javascript:`
- `data:`
- `file:`
- arbitrary custom schemes

This applies both to navigation initiated from trusted UI (e.g. following a link out of extracted HTML or a feed item) and to navigation inside embedded browser webviews, since an uncontrolled scheme is one of the ways untrusted content could otherwise reach outside its intended sandbox.

## Summary of the Boundary

| Concern | Mechanism |
|---|---|
| Native IPC from remote pages | No capability grant on untrusted webviews (Tauri 2 capability model) |
| Interactive remote content in trusted UI | Routed to embedded browser webviews instead of inline rendering |
| Extracted HTML shown in trusted UI | Sanitized: script/iframe/object/embed/event handlers/unsafe schemes rejected by default |
| Navigation target schemes | `https`/`http` allowed by default; `javascript:`, `data:`, `file:`, and custom schemes rejected or reviewed |

These rules are defensive baselines, not merely UX choices — they are what stands between a hostile provider page and the Trusted Core's IPC surface, secrets, and filesystem access.
