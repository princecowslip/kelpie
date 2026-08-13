# Content Safety

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

This is the most load-bearing document in Kelpie's security documentation. It states the hard, non-negotiable boundaries on what Kelpie is allowed to be and do, the review model that keeps official provider presets inside those boundaries, and the technical rules that keep rendered and decoded content from becoming an attack vector. Nothing in this document is a soft guideline, a default that can be reconfigured away, or a target to be balanced against other priorities — these are the constraints the rest of the product design in `kelpie.md` operates within. The high-level product framing for these boundaries lives in [`../PRODUCT.md`](../PRODUCT.md); this document gives the fuller technical and policy treatment.

## Hard Product Boundaries

Kelpie must not be designed to:

- Circumvent DRM.
- Circumvent authentication.
- Break subscription restrictions.
- Defeat paywalls.
- Break CAPTCHA systems.
- Evade technical access controls.
- Circumvent geographic restrictions.
- Publicly re-host third-party libraries.
- Facilitate non-consensual sexual imagery.
- Aggregate sexual material involving minors.
- Treat fictional/illustrated minor sexualization as acceptable simply because it is drawn.
- Execute untrusted provider code with native privileges.

For subscription or DRM-protected services, browser access is a valid and expected provider mode — that is, when a source requires a login, subscription, or DRM-gated playback, the correct integration is to let the user authenticate and view the content through the embedded browser (see [`WEBVIEW-ISOLATION.md`](./WEBVIEW-ISOLATION.md)), not to build a provider adapter that circumvents the protection.

These boundaries are absolute. They are not subject to per-provider exception, per-user configuration, or "advanced mode" override. A feature, provider integration, or design proposal that would require crossing one of these lines is out of scope for Kelpie regardless of its other merits.

## Adult-Only Safety Model

Kelpie is an adults-only application. Its entire safety model — registry review, provider status gating, and the technical content-handling rules below — exists in service of the hard boundaries above, particularly the prohibitions on non-consensual imagery and any sexual material involving minors, in any form, including fictional or illustrated depictions.

### Registry Review

Official presets undergo a registry review before release. A built-in provider must receive one of the following review statuses:

- `APPROVED`
- `BROWSER_ONLY`
- `REVIEW_REQUIRED`
- `DISABLED`
- `BLOCKED`

A source should **not** become an official preset merely because:

- It is popular.
- Its API is convenient.
- Someone has already written a scraper.
- It describes itself as an adult site.

None of those factors substitute for review. Registry review instead considers:

- Source ownership.
- Age/content policy.
- Moderation model.
- Content-reporting process.
- Non-consensual-content policy.
- Reliability.
- Technical integration risk.

User-added custom sources remain separate from official endorsement — a user may add a custom source themselves, but doing so does not grant it any of the review statuses above, and it is not presented to the user as vetted or endorsed by Kelpie the way an `APPROVED` or `BROWSER_ONLY` official preset would be.

## HTML Rendering Security

Extracted HTML — content pulled from a provider and displayed inside trusted application surfaces — must be sanitized before it is rendered. This is a content-safety rule as well as an isolation rule: unsanitized markup from an external source is exactly the kind of content that could otherwise be used to misrepresent what a user is looking at or to smuggle unwanted behavior into a trusted surface.

Rejected by default:

- `script`
- `iframe`
- `object`
- `embed`
- event handlers
- unsafe URL schemes

Remote interactive web pages belong in browser webviews rather than trusted application HTML. The isolation mechanics behind this rule (webview capability boundaries, URL scheme rules) are documented in [`WEBVIEW-ISOLATION.md`](./WEBVIEW-ISOLATION.md); this document states it as a content-safety requirement on what may reach trusted rendering surfaces at all.

## Malformed Content and Resource-Exhaustion Handling

Because Kelpie decodes media (images, animations, video, archives) and processes responses originating from untrusted providers, its security testing explicitly requires that malformed or adversarially-crafted content be handled safely rather than assumed well-formed. The full regression checklist this belongs to is tracked in [`THREAT-MODEL.md`](./THREAT-MODEL.md); the items specifically relevant to content safety are:

- **Oversized responses** — provider or remote responses large enough to exhaust memory or degrade the application must be bounded rather than processed unconditionally.
- **Decompression bombs** — compressed payloads (images, archives, etc.) that expand to a disproportionate size on decode must not be allowed to exhaust memory or disk.
- **Malformed image** — crafted or corrupt image data must not be able to crash or exploit the image decoding pipeline.
- **Malformed animation** — the same requirement applies to animated image/GIF data.
- **Malformed video** — the same requirement applies to video data.
- **Malformed archive** — the same requirement applies to archive data, such as manga/comic chapter packages.

These rules apply regardless of a provider's registry review status — even an `APPROVED` provider's content is untrusted input from the perspective of the decoding and rendering pipeline, and must be handled under these same safety rules.

## How These Rules Relate

The Hard Product Boundaries and Adult-Only Safety Model define *what Kelpie will never be or do*, at the product and policy level. The HTML rendering and malformed-content rules define *how content that is otherwise in-scope is safely handled* at the technical level once it reaches the application. Both are necessary: policy boundaries without technical enforcement would be aspirational only, and technical sanitization without policy boundaries would not prevent Kelpie from being built to do something it must never do in the first place.
