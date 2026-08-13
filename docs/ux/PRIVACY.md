# Privacy

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Overview

This document covers Kelpie's privacy-related UX flows: Private Mode, the Safe Screen, Neutral Preview Mode, notification privacy, and Application Lock. These are described here from the **UX-flow angle** — what the user configures, what triggers each behavior, and what the user sees. For the underlying storage and security mechanics of the application lock (how a PIN, passphrase, or OS-backed secret is actually validated and stored), see [`../security/SECRET-STORAGE.md`](../security/SECRET-STORAGE.md); this document does not restate that.

All of these features exist in service of the "Privacy" design principle in `../PRODUCT.md` — sensitive activity stays local and, where these controls are engaged, is not even persisted locally.

## Private mode

While Private Mode is active, Kelpie is specified to not persist (kelpie.md §99):

- Searches
- Opened items
- Media progress
- Reader progress
- Browser history
- Feed interaction signals
- Temporary private-session state, after the session closes

Critically, Private Mode is additive privacy, not destructive privacy: **existing normal history remains untouched.** Entering Private Mode does not clear or hide what was recorded before it was turned on — it only stops new activity in the session from being written to persistent state. When the private session ends, its temporary state is discarded rather than merged into the user's normal history.

## Safe screen

The Safe Screen is a configurable global shortcut intended for the moment a user needs to instantly stop showing on-screen content — someone entering the room, a screen-share starting, and similar situations. Activating it is specified to (kelpie.md §100):

- Pause media
- Pause GIF (animated content)
- Mute audio
- Hide the current page/image/video
- Hide the browser
- Replace visible content with a neutral UI
- Optionally lock the application

A hard constraint governs what that neutral UI is allowed to look like: **it must not imitate another application or fake an OS screen.** The Safe Screen is meant to read, honestly, as Kelpie showing a neutral state — not as a decoy pretending to be a different program or a fake lock screen. This is a deliberate rejection of "panic button" patterns that disguise the application as something else; Kelpie's approach is to hide content, not to lie about what is on screen.

Safe Screen activation is one of the four triggers for Application Lock (see below), so the two features are meant to compose: a single shortcut can both hide content immediately and, optionally, require re-authentication before it is shown again.

## Neutral preview mode

Where Private Mode and the Safe Screen govern *when* content is hidden, Neutral Preview Mode governs a standing, configurable choice about how explicit previews are shown by default in cards, feeds, and browsing surfaces. When engaged, it replaces explicit visual previews with (kelpie.md §101):

- A media icon
- A source icon
- A neutral placeholder
- An optional generic title

Five distinct preview modes are specified, giving the user a spectrum rather than a single on/off switch:

| Mode | Behavior |
|---|---|
| **Never hide** | Previews always render normally. |
| **Blur** | Previews render blurred. |
| **Reveal on hover** | Previews stay hidden/neutral until the pointer hovers the card. |
| **Reveal on click** | Previews stay hidden/neutral until the user explicitly clicks/taps. |
| **Neutral covers** | Previews are replaced outright with the neutral icon/placeholder treatment described above. |

This spectrum lets a user choose a posture that matches their actual environment — from full previews (private, single-user machine) through to fully neutral covers (shared or public-facing screen) — without losing the ability to browse at all in the more conservative modes.

## Notification privacy

By default, OS-level notifications are specified to be generic rather than descriptive. The worked example (kelpie.md §102):

- Default: `Download complete`
- Not: the explicit title of what was downloaded

Detailed notifications (ones that would include a specific title or other identifying content) are **opt-in** — a user must deliberately choose to see more descriptive notification content; the safe, generic default is what ships without that choice being made.

## Application lock

Application Lock gates access to the app (or to unlocking it after the Safe Screen) behind an authentication step. Three lock methods are specified (kelpie.md §103):

- PIN
- Passphrase
- OS-backed secret

The security and storage mechanics behind these methods — how a PIN or passphrase is verified, and how an OS-backed secret is retrieved from the platform's secret store — are covered in [`../security/SECRET-STORAGE.md`](../security/SECRET-STORAGE.md), not here.

Four triggers are specified for when the lock engages:

- **Manual** — the user locks the app directly.
- **Inactivity** — the app locks itself after a period of no user activity.
- **System resume** — the app locks on waking from sleep/suspend.
- **Safe-screen activation** — locking is one of the optional effects of triggering the Safe Screen (see above).

## Related documents

- [`../security/SECRET-STORAGE.md`](../security/SECRET-STORAGE.md) — how Application Lock credentials and OS-backed secrets are actually stored and validated.
- [`INFORMATION-ARCHITECTURE.md`](./INFORMATION-ARCHITECTURE.md) — Settings → Privacy is the configuration entry point for all of the features on this page.
- [`FEEDS.md`](./FEEDS.md) — feed interaction signals, one of the categories Private Mode does not persist.
- [`ACCESSIBILITY.md`](./ACCESSIBILITY.md) — non-color-only status cues, relevant to how Neutral Preview Mode's states are communicated.
