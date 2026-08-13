# Embedded Browser

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

Kelpie includes an embedded browser for cases where full provider integration isn't possible or isn't appropriate. This document covers what that browser is for and how content encountered inside it can be pulled into Kelpie's normalized model. The embedded browser's underlying trust boundary — how it stays isolated from the trusted core — is covered in [`../security/WEBVIEW-ISOLATION.md`](../security/WEBVIEW-ISOLATION.md); see that document for the isolation/trust-boundary detail rather than this one.

## Embedded Browser

The embedded browser exists to handle situations Kelpie's normalized provider model can't (or shouldn't try to) fully absorb, including:

- login
- subscription content
- unsupported provider
- CAPTCHA
- JavaScript-heavy pages
- DRM content
- provider breakage
- manual browsing

Because these all involve loading arbitrary remote content, sessions in the embedded browser run as Untrusted Web Content per the trust architecture in [`OVERVIEW.md`](./OVERVIEW.md#fundamental-trust-architecture) — they receive no privileged native IPC.

The browser exposes the controls one would expect from a standard browsing surface, plus a Kelpie-specific action:

- back
- forward
- reload
- address
- tabs
- open externally
- add to Kelpie

## Add to Kelpie

The **Add to Kelpie** action is what bridges a manually-browsed page back into Kelpie's normalized model. Triggered from the browser, it runs the current URL through the provider resolver:

```
current URL
  ↓
provider resolver
  ↓
recognized?
 ├── yes → normalized item
 └── no  → generic external bookmark
```

If the URL is recognized by a provider (per [`PROVIDERS.md`](./PROVIDERS.md)), it resolves into a proper normalized `MediaItem` (see [`DOMAIN-MODEL.md`](./DOMAIN-MODEL.md)) with all the metadata, tagging, and library integration that implies. If it isn't recognized by any provider, it falls back to being saved as a generic external bookmark instead — still tracked, but without normalized metadata.
