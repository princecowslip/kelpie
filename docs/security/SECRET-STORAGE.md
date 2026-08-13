# Secret Storage

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

Kelpie's Trusted Core is the only domain with access to secrets (see [`../architecture/OVERVIEW.md`](../architecture/OVERVIEW.md) §8). This document covers the storage and security mechanics around that custody: how authentication is designed to avoid collecting plaintext credentials in the first place, what a diagnostic bundle must never contain, and how the application lock protects the local application state at rest. It is one of the surfaces exercised by the "secret leakage" and "cookie leakage" items in the regression checklist in [`THREAT-MODEL.md`](./THREAT-MODEL.md).

For the user-facing privacy flows built on top of these mechanics (what the user sees and controls around locking, diagnostics export, and similar), see [`../ux/PRIVACY.md`](../ux/PRIVACY.md).

## Authentication

Kelpie's preferred authentication flow avoids ever taking custody of a user's site credentials directly:

```
Sign In
  ↓
provider browser
  ↓
website login page
  ↓
user authenticates directly
  ↓
site creates session
```

The user signs in through the provider's own login page inside the embedded browser (see [`WEBVIEW-ISOLATION.md`](./WEBVIEW-ISOLATION.md)), typing their credentials directly into the site's own page rather than into a Kelpie-owned form. The site then creates its own session, which Kelpie can hold onto as session state, without Kelpie's provider adapters ever having handled the raw password.

**Kelpie provider adapters should not collect plaintext passwords.** This is a design constraint on how authentication is implemented, not merely a recommendation about how credentials happen to be stored — the goal is that there is no plaintext password for storage mechanics to protect in the first place for provider-adapter-mediated logins, because the browser-mediated flow above never routes one through Kelpie.

## Diagnostics

Diagnostic bundles exist so a user can share application state for troubleshooting without that bundle becoming a secret-leakage vector. A safe diagnostic bundle is scoped to:

- `app-info.json`
- `environment.json`
- `provider-status.json`
- `errors.log`
- `redacted-settings.json`

The following must never be exported in a diagnostic bundle, under any circumstance:

- Passwords
- Tokens
- Cookies
- Authorization headers
- Secret API keys

The presence of `redacted-settings.json` (rather than raw settings) in the safe list, alongside the explicit never-export list, reflects the same principle: diagnostics are built by construction to exclude secret material, rather than by filtering a general-purpose export after the fact.

## Application Lock

The Application Lock is the mechanism that protects local application state — including the secrets and session data the Trusted Core holds — from anyone with physical or session access to the machine but not to the lock itself.

Supported lock methods:

- PIN
- Passphrase
- OS-backed secret

Triggers that engage the lock:

- Manual
- Inactivity
- System resume
- Safe-screen activation

Offering an OS-backed secret option ties the lock, where used, to the platform's own credential storage rather than requiring Kelpie to invent its own at-rest protection scheme from scratch. The inactivity, system-resume, and safe-screen triggers mean the lock is not purely opt-in-per-session — it re-engages automatically around the moments where an unattended or resumed session is most likely to be exposed to someone other than the user.

## Summary

| Mechanism | What it protects against |
|---|---|
| Browser-mediated authentication (no plaintext password collection) | Kelpie provider adapters becoming a store of raw site credentials |
| Diagnostic bundle scoping + never-export list | Secrets leaving the device inside a support/diagnostics export |
| Application Lock (PIN / passphrase / OS-backed secret, with automatic triggers) | Local access to application state, secrets, and sessions by someone other than the user |

Together these mean secrets are, by design, collected as narrowly as possible, never included in exports meant to leave the device, and gated behind a lock that re-engages at the moments an unattended session is most exposed.
