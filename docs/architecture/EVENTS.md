# Events & Cross-Service Communication

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

**A note on this document's provenance:** unlike the other files in `architecture/`, kelpie.md has no single dedicated section describing an event system. This document is an *inferred synthesis*, assembled from several cross-cutting references elsewhere in the spec — the statement that services communicate through typed commands/events (§10), the process model (§11), provider health states (§31), the refresh scheduler (§112), and the error model/UX (§113–114). It describes the *shape* of an event-driven architecture that those sections imply, without inventing specific event names, payload schemas, or channels that kelpie.md does not itself specify. Treat this document as more speculative than its siblings, and prefer kelpie.md directly for anything load-bearing.

## Why an events document exists at all

Section 10 of kelpie.md (Major Application Services) states plainly: "Services communicate through typed commands/events rather than frontend components reaching directly into persistence." That single sentence, combined with the process model in §11 and several state machines described elsewhere in the spec, implies an event-driven backbone connecting `AppCore`'s services (`ProviderService`, `FeedService`, `SearchService`, `LibraryService`, `SeriesService`, `HistoryService`, `PlaybackService`, `ReaderService`, `BrowserService`, `DownloadService`, `CacheService`, `SchedulerService`, `PrivacyService`, `SecurityService`, `RegistryService`, `SettingsService` — see [`OVERVIEW.md`](./OVERVIEW.md#major-application-services)) to each other and to the frontend. kelpie.md does not, however, enumerate a concrete event catalog, so none is invented here.

## Typed commands and events between services

The one architectural commitment kelpie.md makes explicitly is that inter-service and frontend-to-backend communication goes through **typed commands and events**, not direct persistence access. Given the process model in [`OVERVIEW.md`](./OVERVIEW.md#application-process-model) — a trusted Rust core, a trusted frontend webview, an out-of-process (eventually) provider host, and untrusted remote webviews — this implies at minimum:

- Frontend → Trusted Core: typed commands (e.g. driven by Tauri's IPC), gated by the capability model described in [`OVERVIEW.md`](./OVERVIEW.md#fundamental-trust-architecture) so that untrusted webviews cannot issue them.
- Trusted Core services → Frontend: typed events, used to push state changes (health transitions, refresh completion, errors) rather than requiring the frontend to poll.
- Trusted Core ↔ Provider Runtime: some command/event-shaped boundary is implied by the restricted-runtime model, though kelpie.md does not specify its concrete transport once provider execution moves out-of-process into `kelpie-provider-host`.

No specific command or event names beyond what is described below are asserted here, since kelpie.md itself does not name them.

## Health state transitions as events

Provider health (see [`PROVIDERS.md`](./PROVIDERS.md#provider-health)) is inherently a state machine, and by nature its states are reached via transitions — i.e. events. The recorded health fields (`last_success`, `last_attempt`, `last_search_success`, `failure_count`, `average_latency`, `version`, `last_error`) read as the kind of fields that get updated in response to discrete occurrences (a refresh attempt succeeding or failing, a search completing, an update landing), rather than being recomputed from scratch. The resulting UI-facing states — Healthy, Refreshing, Degraded, Authentication Required, Rate Limited, Offline, Outdated, Broken, Disabled — are consistent with a model where each provider has current state that transitions in response to typed events (e.g. a refresh attempt outcome, a rate-limit response, an update/rollback per [`PROVIDERS.md`](./PROVIDERS.md#provider-updates) and [`PROVIDERS.md`](./PROVIDERS.md#provider-rollback)) and where the UI simply reflects current state rather than re-deriving it live.

## Lifecycle state as events

Provider *health* (above) is not the only provider state machine kelpie.md describes. §29 Provider Lifecycle names a separate set of states governing install/update/removal rather than refresh/search outcomes:

- `available`
- `installed`
- `enabled`
- `disabled`
- `updating`
- `degraded`
- `incompatible`
- `broken`
- `quarantined`
- `removed`

These are distinct from the health states above (a provider can be `enabled` and simultaneously `Degraded`), and transitions between them are driven by different occurrences — installation, manual enable/disable, an update landing, a registry quarantine action (see [`PROVIDERS.md`](./PROVIDERS.md#provider-integration-levels) and [`../providers/REGISTRY.md`](../providers/REGISTRY.md#automatic-provider-quarantine)), or removal — rather than by refresh/search attempts. §29 is also explicit that a provider becoming `broken` must not delete history, favorites, collections, indexed metadata, or source URLs, which implies the lifecycle-state transition itself is the event, while the data it might otherwise threaten is deliberately left untouched by that transition.

That both a health state machine (§31) and a lifecycle state machine (§29) exist as adjacent, distinct sections of kelpie.md's Provider Model chapter reinforces the "typed commands/events" framing above: these read as two different families of events landing on the same provider entity, not one undifferentiated status field.

## Download state machine

§108 Downloads is the one place in kelpie.md that names an explicit, unambiguous state list end to end, which makes it useful ballast for an otherwise-inferred document:

```
queued → resolving → downloading → paused → complete → failed
```

alongside pause/resume, retry, concurrency, bandwidth limiting, a free-space check, and a configurable destination. Each of those transitions and features reads naturally as an event a `DownloadService` (see [`OVERVIEW.md`](./OVERVIEW.md#major-application-services)) would emit or react to — a download queued, resolved, progressing, paused by the user, completed, or failed — even though, consistent with this document's discipline, kelpie.md does not name a corresponding event type for each transition.

## State is persisted, not just computed live

§104 Database Domains lists `provider_state` and `provider_errors` as first-class tables, alongside `feeds`/`feed_entries`/`feed_clusters` and `downloads`. This corroborates the state-machine framing above: provider health/lifecycle state and errors are recorded, queryable data, not values recomputed on demand — consistent with a model where events update persisted state rather than the UI deriving status live on every render.

## Scheduled refresh as a trigger source

The Refresh Scheduler (§112) is an explicit source of triggers into this system. Configurable schedules are: Manual, On Launch, 15 Minutes, 30 Minutes, Hourly, Every 6 Hours, Every 12 Hours, Daily. The scheduler is expected to:

- apply jitter
- respect rate limits
- pause offline
- avoid duplicate jobs
- back off after failures
- optionally pause on battery

Each scheduled firing is naturally the kind of occurrence that would produce a "refresh started" / "refresh completed" event pair feeding into the feed pipeline (see [`FEEDS.md`](./FEEDS.md#global-feed-architecture)) and into the provider health updates described above. kelpie.md does not name these events explicitly; they are described here only as the shape the scheduler implies.

## Error propagation

kelpie.md defines a structured error model (§113) rather than allowing arbitrary provider strings to reach the frontend. The full set of normalized error types is:

- `NetworkError`
- `Timeout`
- `AuthenticationRequired`
- `SubscriptionRequired`
- `RateLimited`
- `PermissionDenied`
- `ProviderOutdated`
- `ProviderParseError`
- `UnsupportedMedia`
- `ContentRemoved`
- `DatabaseError`
- `SecurityViolation`
- `Unknown`

The explicit statement that "the frontend receives structured errors rather than arbitrary provider strings" implies these errors are themselves propagated as typed data — consistent with the typed-command/event model above, though again kelpie.md does not specify a distinct "error event" channel versus errors carried as part of a command's response.

The corresponding user-facing behavior (§114, Error UX) gives a sense of how these errors are expected to resolve into UI treatments once propagated. For a provider that appears to have changed:

```
This source appears to have changed.

[Check Update]
[Open Website]
[Diagnostics]
```

For authentication:

```
Sign in to continue.

[Sign In]
[Open Website]
```

For offline conditions:

```
You're offline.

Showing indexed content.
```

## Notifications as a user-visible endpoint

§102 Notification Privacy and §121 Linux Integration both mention OS-level notifications (the default example given is "Download complete," deliberately not an explicit title, with detailed notifications opt-in; §121 lists `notifications` among the Linux desktop standards Kelpie should follow via XDG/Wayland/X11 integration). A download reaching `complete` (§108, above) or a provider transitioning to a state worth surfacing are the kinds of internal events plausible as the trigger for such a notification — this is the one point where kelpie.md hints at events becoming visible outside the application window, though it does not specify which internal occurrences map to which notifications.

## Summary

Taken together, kelpie.md's cross-cutting references support describing Kelpie's runtime as built around: typed commands issued from the frontend into trusted-core services; typed events pushed back out as state changes; a provider health state machine driven by refresh/search/update outcomes; a separate provider lifecycle state machine driven by install/update/quarantine/removal actions; a download state machine with an explicit, named state list; persisted state and error tables backing both; a scheduler that is the primary internal trigger source for refreshes; a structured error model that propagates failures as typed data rather than opaque strings; and OS notifications as one plausible user-visible endpoint of that event chain. No event names, payload shapes, or additional channels beyond what is described above should be assumed — if a future implementation needs a concrete event catalog, that catalog belongs in a real specification update to kelpie.md, not as an extrapolation from this document.
