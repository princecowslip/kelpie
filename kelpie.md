Kelpie

Complete Product, UX, Software Architecture, Provider Registry and Implementation Specification

Product: Kelpie
Category: Linux adult-media hub, aggregator, browser, reader, player and personal library
Platforms: Debian, Ubuntu, Arch Linux and compatible distributions
Architecture: Local-first desktop application with sandboxed provider integrations
Recommended stack: Rust + Tauri 2 + React/TypeScript + SQLite/FTS5 + optional mpv/libmpv
Document version: 4.0
Registry snapshot: August 2026
Target audience: Product, UI/UX, frontend, backend, security, QA, provider developers and maintainers

---

1. Product Summary

Kelpie is a private Linux desktop application for discovering, browsing, searching, viewing, reading, listening to and organizing lawful adult media distributed across many websites and local files.

It combines the useful characteristics of a:

- Media center.
- Feed reader.
- Multi-site search engine.
- Video browser.
- GIF and short-clip browser.
- Image/gallery viewer.
- Booru client.
- Manga reader.
- Comic reader.
- Webcomic reader.
- Adult cartoon/illustration browser.
- Hentai/adult manga library.
- Literature reader.
- Audio player.
- Imageboard/thread viewer.
- Creator-platform browser.
- Live-site launcher.
- Personal media library.
- Private web browser.
- Extensible provider/plugin platform.

The application is not designed around one website.

Its fundamental architecture is:

Remote sources
      ↓
Provider adapters
      ↓
Normalized media model
      ↓
Local database/index
      ↓
Feed / Search / Library
      ↓
Specialized viewers

The user therefore owns a stable Kelpie library even when individual websites change.

---

2. Product Thesis

The core product proposition is:

«One private Linux-native interface for adult media across the web, regardless of whether the source contains video, GIFs, images, manga, comics, stories, audio or mixed media.»

The long-term value of the project should come from:

1. The normalized media model.
2. The provider platform.
3. The global feed.
4. Unified search.
5. Specialized viewers.
6. Cross-site library organization.
7. Privacy controls.
8. Browser fallback.
9. Source-management tooling.
10. Linux-native integration.

Supporting a large number of sites is valuable, but provider count must not become the architecture.

---

3. Product Goals

3.1 Unified discovery

Allow many sources to participate in:

- Home.
- Latest.
- Following.
- Continue.
- Search.
- Discover.
- Saved searches.

3.2 Media-aware consumption

Each medium receives an appropriate viewer.

Video        → video player
GIF          → animation viewer
Image        → image viewer
Gallery      → gallery viewer
Manga        → sequential reader
Comic        → sequential reader
Story        → text reader
Audio        → audio player
Imageboard   → thread viewer
Unsupported  → isolated browser

3.3 Local ownership

Favorites, history, progress, collections and follow state belong to Kelpie rather than individual websites.

3.4 Resilience

Provider breakage must be isolated.

One source failing must not:

- Crash the application.
- Break the global feed.
- Prevent local search.
- Destroy previously indexed content.

3.5 Extensibility

Users should be able to add sources without recompiling Kelpie.

3.6 Privacy

Sensitive activity stays local unless the user explicitly enables some future synchronization feature.

---

4. Hard Product Boundaries

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

For subscription or DRM-protected services, browser access is a valid and expected provider mode.

---

5. Adult-Only Safety Model

Kelpie is an adults-only application.

Official presets undergo a registry review before release.

A built-in provider must receive one of:

APPROVED
BROWSER_ONLY
REVIEW_REQUIRED
DISABLED
BLOCKED

A source should not become an official preset merely because:

- It is popular.
- Its API is convenient.
- Someone has already written a scraper.
- It describes itself as an adult site.

Registry review should consider:

- Source ownership.
- Age/content policy.
- Moderation model.
- Content-reporting process.
- Non-consensual-content policy.
- Reliability.
- Technical integration risk.

User-added custom sources remain separate from official endorsement.

---

6. Design Principles

Local first

Core state resides locally.

Source agnostic

Frontend components consume normalized objects.

Media aware

Media type determines presentation, not website identity.

Graceful degradation

Integration can fall back to the browser.

Explicit permissions

Providers declare exactly which domains and capabilities they require.

Recoverable

Database and provider updates have rollback paths.

Explainable

Recommendations and filters can be understood and overridden by the user.

Discreet

The application should look like premium media software, not a stereotypical adult website.

---

7. User Modes

Kelpie should support several overlapping workflows.

Browse mode

For users who primarily want discovery and viewing.

Library mode

For users maintaining favorites, series and collections.

Reader mode

For manga, comics, webcomics and literature.

Power-user mode

For keyboard navigation, saved searches and filters.

Private mode

For sessions that should leave minimal persistent local state.

Developer mode

For creating and debugging provider adapters.

---

8. Fundamental Trust Architecture

The application has three execution domains.

Trusted Core

Rust application services.

Can access:

- SQLite.
- Local configuration.
- Approved filesystem locations.
- OS integration.
- Secrets.
- Media backend.

Restricted Provider Runtime

Can access:

- Explicitly permitted origins.
- Provider-specific storage.
- Provider-specific session state.
- Parsing APIs.
- Provider logs.

Cannot access:

- Arbitrary shell commands.
- Arbitrary filesystem.
- Other providers' sessions.
- Native application commands.

Untrusted Web Content

Remote pages render in dedicated browser webviews.

They receive no privileged native IPC.

Tauri 2's current capability model allows permissions to be constrained to specific windows/webviews; a webview without a matching capability receives no IPC access. That should be used as the hard boundary between Kelpie's trusted UI and remote sites.

---

9. Recommended Technology Stack

Native/application core

Rust
Tokio
SQLite

Desktop shell

Tauri 2

Frontend

React
TypeScript
Vite
TanStack Query
Zustand or Redux Toolkit

Media

HTML5/WebView playback
+
optional mpv/libmpv fallback

mpv remains suitable as a secondary playback backend because it supports a broad set of video/audio formats and codecs.

Search

SQLite FTS5

SQLite documents FTS5 as its full-text-search virtual-table module, making it appropriate for local title/tag/creator/story indexing.

---

10. Major Application Services

AppCore
├── ProviderService
├── FeedService
├── SearchService
├── LibraryService
├── SeriesService
├── HistoryService
├── PlaybackService
├── ReaderService
├── BrowserService
├── DownloadService
├── CacheService
├── SchedulerService
├── PrivacyService
├── SecurityService
├── RegistryService
└── SettingsService

Services communicate through typed commands/events rather than frontend components reaching directly into persistence.

---

11. Application Process Model

Recommended eventual model:

kelpie
   ├── trusted Rust core
   └── trusted frontend webview

kelpie-provider-host
   └── restricted provider execution

remote webviews
   └── untrusted websites

mpv
   └── optional media subprocess

The MVP may initially keep some provider infrastructure in-process, but the interfaces should be designed as though provider execution can move out-of-process later.

---

12. Core Media Taxonomy

Separate three concepts:

Structure

What the object technically is.

Style

How it is visually presented.

Classification

How the adult source categorizes it.

Example:

Structure: manga
Style: manga/anime
Classification: adult, hentai

Another:

Structure: animation
Style: cartoon
Classification: adult

This prevents an explosion of redundant media types.

---

13. Structural Media Types

type MediaKind =
  | "video"
  | "short_video"
  | "clip"

  | "gif"
  | "animated_image"
  | "animation"

  | "image"
  | "illustration"
  | "photograph"
  | "gallery"

  | "manga"
  | "comic"
  | "webcomic"
  | "comic_strip"

  | "story"
  | "serialized_text"
  | "article"

  | "audio"
  | "audio_story"

  | "booru_post"

  | "imageboard_thread"
  | "imageboard_post"

  | "live"

  | "game"
  | "visual_novel"

  | "external";

---

14. Media Style

type MediaStyle =
  | "photographic"
  | "realistic"
  | "illustrated"
  | "anime"
  | "manga"
  | "cartoon"
  | "3d"
  | "mixed"
  | "other";

Style does not control which viewer is selected.

---

15. Adult Classification

interface ContentClassification {
  adult: true;

  labels: string[];

  providerLabels: string[];

  style?: MediaStyle[];
}

Possible normalized labels:

adult
hentai
adult-manga
adult-comic
adult-cartoon
adult-animation
erotic-fiction
audio-erotica

---

16. Unified MediaItem

interface MediaItem {
  uid: string;

  providerId: string;
  remoteId?: string;

  canonicalUrl: string;

  kind: MediaKind;

  title: string;
  description?: string;

  creators: CreatorCredit[];

  thumbnails: ImageResource[];

  tags: TagReference[];

  classification: ContentClassification;

  publishedAt?: string;
  updatedAt?: string;

  durationSeconds?: number;

  dimensions?: {
    width?: number;
    height?: number;
  };

  series?: SeriesReference;

  sequence?: {
    volume?: SequenceNumber;
    chapter?: SequenceNumber;
    issue?: SequenceNumber;
    pageCount?: number;
  };

  availability: Availability;

  mediaSources?: MediaSource[];

  providerMetadata?: Record<string, unknown>;
}

---

17. Stable Identity

Preferred identity:

provider-id : object-type : remote-id

Example:

provider-x:manga-chapter:38172

Fallback:

SHA256(provider-id + canonical-url)

Title alone must never be treated as identity.

---

18. Availability Model

type Availability =
  | "direct"
  | "embedded"
  | "browser"
  | "login-required"
  | "subscription-required"
  | "temporarily-unavailable"
  | "removed"
  | "unknown";

Availability informs UI actions before the user attempts playback.

---

19. Media Sources

interface MediaSource {
  url: string;

  transport:
    | "http"
    | "file"
    | "hls"
    | "dash"
    | "browser";

  mimeType?: string;

  width?: number;
  height?: number;

  bitrate?: number;

  label?: string;

  expiresAt?: string;
}

Ephemeral signed URLs should be resolved on demand rather than persisted as canonical identity.

---

20. Series Model

Manga, comics, webcomics and episodic audio need hierarchical structures.

Series
 ├── Volume
 │    └── Chapter / Issue
 │          └── Page

or:

Audio Series
 └── Episode / Chapter

---

21. Series Object

interface Series {
  uid: string;

  providerId: string;
  remoteId?: string;

  canonicalUrl: string;

  title: string;
  alternateTitles: LocalizedTitle[];

  description?: string;

  creators: CreatorCredit[];

  cover?: ImageResource;

  tags: TagReference[];

  status?:
    | "ongoing"
    | "completed"
    | "hiatus"
    | "cancelled"
    | "unknown";

  readingDirection?:
    | "ltr"
    | "rtl"
    | "vertical";
}

---

22. Sequence Numbers

Do not assume chapters are integers.

Sources may expose:

10
10.5
10a
Special
Bonus
Prologue
Epilogue

Use:

interface SequenceNumber {
  raw: string;
  numeric?: number;
  sortKey: string;
}

The raw source label remains visible.

---

23. Provider Model

A provider converts a remote source into normalized Kelpie objects.

Provider categories include:

specific website
site family/protocol
generic HTML
generic feed
browser-only
local filesystem

---

24. Provider Capabilities

feeds
feed
search
suggest

item
creator
tags
related

series
volumes
chapters
issues
pages

thread
posts

resolve-url

media
captions

authentication
browser
download

The frontend checks capabilities before rendering actions.

---

25. Provider Manifest

{
  "schema": 1,

  "id": "example",
  "name": "Example",

  "version": "1.2.0",
  "providerApi": "1",

  "siteType": "manga",

  "contentKinds": [
    "manga",
    "illustration"
  ],

  "capabilities": [
    "feed",
    "search",
    "series",
    "chapters",
    "pages",
    "browser"
  ],

  "origins": [
    "https://example.com",
    "https://cdn.example.com"
  ],

  "authentication": "browser-cookie"
}

---

26. Provider Execution API

Conceptually:

interface Provider {
  initialize(context: ProviderContext): Promise<void>;

  feeds?(): Promise<FeedDescriptor[]>;

  feed?(
    request: FeedRequest
  ): Promise<Page<MediaItem>>;

  search?(
    request: SearchRequest
  ): Promise<Page<SearchResult>>;

  item?(
    request: ItemRequest
  ): Promise<MediaItem>;

  series?(
    request: SeriesRequest
  ): Promise<Series>;

  chapters?(
    request: ChapterRequest
  ): Promise<Page<Chapter>>;

  pages?(
    request: PageRequest
  ): Promise<PageManifest>;

  resolveUrl?(
    request: ResolveUrlRequest
  ): Promise<ResolvedObject | null>;

  media?(
    request: MediaRequest
  ): Promise<MediaSource[]>;

  related?(
    request: RelatedRequest
  ): Promise<Page<MediaItem>>;
}

Requests should support cancellation.

---

27. Provider Sandbox

Preferred future implementation:

WASM/WASI provider package

Alternative:

QuickJS provider host

Host APIs:

http.request
html.parse
xml.parse
json.parse

cookies.readOwn
cookies.writeOwn

cache.get
cache.put

log
clock

Denied APIs:

arbitrary filesystem
shell execution
process spawning
unrestricted network
environment secrets
other-provider sessions
native UI IPC

---

28. Provider Network Permissions

Every provider declares allowed origins.

example.com
api.example.com
cdn.example.com

The permission layer validates:

- Initial request.
- Redirect destination.
- WebSocket destination if supported.
- Media host.
- API host.

Unexpected origins are rejected.

---

29. Provider Lifecycle

available
installed
enabled
disabled
updating
degraded
incompatible
broken
quarantined
removed

A provider becoming broken should not delete:

- History.
- Favorites.
- Collections.
- Indexed metadata.
- Source URLs.

---

30. Provider Integration Levels

Native

Reliable structured interface.

Integrated

Normal provider adapter with feed/search support.

Metadata

Metadata/indexing integration with browser consumption.

Browser+

Dedicated isolated browser preset with useful Kelpie integration.

Generic

Protocol/source-family adapter.

Experimental

Community or locally installed provider.

Quarantined

Provider package prevented from executing.

---

31. Provider Health

Record:

last_success
last_attempt
last_search_success
failure_count
average_latency
version
last_error

UI states:

Healthy
Refreshing
Degraded
Authentication Required
Rate Limited
Offline
Outdated
Broken
Disabled

---

32. Provider Updates

Application and provider releases must be decoupled.

Kelpie App
Provider Registry
Provider Package

This is important because site integration assumptions can change suddenly. RedGIFs, for example, currently states that it no longer offers third-party API access as of June 17, 2026.

A provider should therefore be able to move:

Native
→ Integrated
→ Metadata
→ Browser+

through a registry update without requiring a complete desktop release.

---

33. Provider Rollback

Maintain:

current provider
previous known-good provider

If post-update health tests fail:

disable new version
restore previous version
record rollback

---

34. Official Preset Registry

Kelpie should ship approximately 40–50 provider definitions/source families, but far fewer should be enabled automatically.

Every preset includes:

provider ID
display name
site type
integration level
capabilities
permissions
global-feed eligibility
last verification date
registry safety status
known limitations

---

35. Preset Registry Philosophy

A preset means:

«Kelpie knows how to represent or safely open this source.»

It does not necessarily mean:

«Kelpie extracts its media directly.»

This distinction allows strong support for subscription sources without defeating their business/access models.

---

36. Candidate Video Presets

Recommended launch catalogue:

Provider| Role| Initial integration
Pornhub| mainstream video| Browser+ / Metadata
XVideos| mainstream video| Browser+ / Metadata
XNXX| mainstream video| Browser+ / Metadata
xHamster| mainstream video| Browser+ / Metadata
Bellesa| curated video| Browser+
MakeLoveNotPorn| curated/independent video| Browser+
PinkLabel.TV| independent/queer cinema| Browser+
Lustery| creator/couple video| Browser+
Crash Pad Series| independent/queer video| Browser+

Major platforms including Pornhub, XVideos and XNXX remain active enough in 2026 to be subject to ongoing EU regulatory proceedings; because age-assurance requirements and availability can vary by jurisdiction, Kelpie should treat source accessibility as runtime state rather than assume universal availability.

---

37. GIF and Short-Clip Presets

Recommended:

Provider| Integration| Main structures
RedGIFs| Integrated/Browser+| GIF, clip, short video
EroMe| Integrated/Browser+| clip, gallery, image
Pornhub short-content view| provider sub-feed| short video
XVideos short-content view| provider sub-feed| short video

RedGIFs remains active in 2026, including its web-app surface, but its discontinued third-party API access means Kelpie should not make API availability an architectural assumption.

---

38. Image and Gallery Presets

Candidate set:

PornPics
EroMe
SuicideGirls
ImageFap — review-required

Recommended integration policy:

PornPics      → Integrated / Browser+
EroMe         → Integrated / Browser+
SuicideGirls  → Browser+
ImageFap      → Review Required

Large user-uploaded gallery services should be disabled by default unless registry review determines they are suitable for automatic global-feed participation.

---

39. Manga, Doujinshi and Hentai Presets

Primary official candidates:

FAKKU
Irodori Sakura / Irodori Comics adult catalogue

FAKKU currently presents an 18+ age gate and identifies its catalogue around adult manga, comics, doujin and games.

Irodori currently maintains a distinct R18 catalogue as well as a separate all-ages catalogue, which is exactly why Kelpie must represent catalogue classification independently from provider identity.

Recommended integration:

FAKKU
  Metadata / Browser+
  later Integrated where appropriate

Irodori R18
  Metadata / Browser+

Paid/subscription reading should use legitimate authenticated access.

---

40. Comic, Cartoon and Webcomic Presets

Candidate set:

Filthy Figments
Oglaf
Slipshine

These sources exercise different sequential-art structures and should not be forced through the manga assumptions.

Recommended:

Filthy Figments → Integrated / Browser+
Oglaf            → Integrated / Browser+
Slipshine        → Browser+ initially

Filthy Figments remains reachable as an adult-comics source and is useful as a reference provider for series/page-oriented integration.

---

41. Literature Presets

Primary:

Literotica
Lush Stories

Literotica is particularly valuable architecturally because its current service exposes stories, categories, tags, authors, audio and interactive story games within one ecosystem.

Recommended Literotica capabilities:

feed
search
story
creator
tags
series
audio
interactive-fiction
related
browser

This makes it one of the best early providers for testing mixed-media normalization.

---

42. Audio Presets

Recommended:

Quinn
Dipsea
femtasy
Literotica Audio
Bloom Stories

Quinn currently describes itself as an audio-erotica application.

Dipsea currently offers audio stories/audiobooks organized into multi-chapter series, making it suitable for Kelpie's Series → Chapter → Audio model.

Recommended integration:

Quinn       → Metadata / Browser+
Dipsea      → Metadata / Browser+
femtasy     → Browser+
Literotica  → Integrated through main provider
Bloom       → Browser+

---

43. Creator-Platform Presets

Add a dedicated source family:

creator_platform

Candidates:

OnlyFans
Fansly
ManyVids
JustForFans

Initial integration:

Browser+

Supported Kelpie functions:

isolated login
creator bookmark
follow shortcut
history
URL capture
manual library save
source metadata

Do not build around scraping authenticated creator feeds.

---

44. Live/Cam Presets

Site type:

live

Candidates:

Chaturbate
Stripchat
MyFreeCams

Initial integration:

Browser+

Kelpie may support:

creator bookmarks
live/offline state where legitimately available
history
provider browser

Do not:

record by default
cache private sessions
bypass paid/private-show controls

Stripchat is among the large adult services involved in current EU regulatory proceedings, reinforcing the need for jurisdiction-aware provider states.

---

45. Booru Support

Rather than initially endorsing numerous specific adult boorus, Kelpie should ship booru engine/provider families.

Built-ins:

Danbooru-compatible
Gelbooru-compatible
Shimmie/Shimmie2-compatible
Booru-on-Rails-compatible
Generic JSON Booru
Generic XML Booru

The user enters a domain and Kelpie attempts engine detection.

This provides excellent booru functionality without turning every compatible public board into an official preset.

---

46. Booru Auto-Detection

Flow:

Enter domain
     ↓
Probe known API signatures
     ↓
Detect engine
     ↓
Map endpoints
     ↓
Fetch sample posts
     ↓
Validate tags/media
     ↓
Show permission review
     ↓
Enable

Validation output:

API                    ✓
Search                 ✓
Tag metadata           ✓
Preview images         ✓
Original media         ✓
Pagination             ✓
Authentication         none

---

47. Imageboard Support

Ship protocol adapters instead of enabling arbitrary boards.

4chan JSON-compatible
vichan-compatible
Tinyboard-compatible
LynxChan-compatible
Generic JSON thread
Generic HTML thread

Setup:

Add Source
→ Imageboard
→ Enter host
→ Detect engine
→ Select boards
→ Review safety
→ Enable

Kelpie must never index every board automatically.

---

48. Adult Games and Visual Novels

Optional source family:

game
visual_novel
interactive_story

Candidate browser/metadata presets:

JAST
MangaGamer
FAKKU Games

This category can remain post-MVP because launching/organizing games is a distinct problem from media playback.

---

49. Generic Built-In Sources

Every installation should include:

RSS
Atom
JSON Feed

Generic HTML Feed
Generic HTML Gallery

Generic Video Source
Generic GIF Source

Generic Manga
Generic Comic
Generic Webcomic

Generic Booru
Generic Imageboard

Generic Literature

Browser-only Source

These generic providers are strategically more important than supporting an enormous number of fixed sites.

---

50. First-Run Preset Selection

Do not enable the complete provider registry automatically.

First-run screen:

Video
  Pornhub
  XVideos
  xHamster

GIFs
  RedGIFs

Images
  PornPics

Manga / Illustrated
  FAKKU

Comics
  Filthy Figments

Literature
  Literotica

Audio
  Quinn
  Dipsea

Independent
  PinkLabel.TV
  MakeLoveNotPorn

Then:

Browse all sources

Users explicitly choose.

---

51. Global Feed Eligibility

Recommended defaults:

Conventional video       opt-in
GIF                       opt-in
Images/gallery            opt-in
Manga/comics              opt-in
Literature                opt-in
Audio                     opt-in

Creator platforms         off
Live/cam                  off
Imageboards               off
User boorus               off until enabled
Browser-only providers    off

A provider can exist in Kelpie without being allowed to populate Home.

---

52. Registry Metadata

interface RegistryEntry {
  id: string;

  name: string;

  siteType: SiteType;

  integration:
    | "native"
    | "integrated"
    | "metadata"
    | "browser"
    | "generic"
    | "experimental";

  capabilities: Capability[];

  defaultEnabled: boolean;

  globalFeedEligible: boolean;

  lastVerifiedAt: string;

  safetyStatus:
    | "official-reviewed"
    | "community-content"
    | "review-required"
    | "browser-only"
    | "blocked";

  knownLimitations: string[];
}

---

53. Registry Governance

Every official provider should maintain:

last technical verification
last safety review
provider version
host/domain ownership snapshot
integration health
permission set

Registry review cadence:

High-volume integrations    monthly
Safety-sensitive sources    monthly
Browser-only sources        quarterly
Generic protocols           per release

---

54. Automatic Provider Quarantine

Trigger quarantine when:

provider signature invalid
package hash mismatch
unexpected domain change
unexpected permission expansion
persistent parser failure
security advisory
malicious redirect pattern

Quarantine stops provider execution while preserving user library state.

---

55. Custom Source Builder

Four levels:

Level 1 — Feed

RSS
Atom
JSON Feed

Level 2 — Generic Adapter

Gallery
Manga
Comic
Booru
Literature

Level 3 — Visual Site Builder

CSS selectors and visual extraction.

Level 4 — Provider SDK

Sandboxed code package.

---

56. Visual Site Builder

Recommended three-pane interface:

┌──────────────────┬────────────────────┬──────────────────┐
│ Mapping          │ Website Preview    │ Parsed Results   │
│                  │                    │                  │
│ Item selector    │ live page          │ Result 1         │
│ Title            │                    │ Result 2         │
│ Link             │                    │ Result 3         │
│ Thumbnail        │                    │                  │
│ Creator          │                    │                  │
│ Next page        │                    │                  │
└──────────────────┴────────────────────┴──────────────────┘

User clicks page elements to assign semantic roles.

---

57. Manga Source Builder

Mapping fields:

series URL
series title
alternate title
cover
description

chapter list
chapter label
chapter URL

page list
page image

next chapter
previous chapter

reading direction

Preview:

Series detected           1
Chapters detected        42
Pages in sample chapter  31
Reading direction        RTL

---

58. Global Feed Architecture

The global feed should primarily read from local normalized data.

Provider
  ↓
Refresh
  ↓
Normalize
  ↓
Validate
  ↓
Deduplicate
  ↓
Cluster
  ↓
SQLite
  ↓
Filter
  ↓
Rank
  ↓
UI

Do not make every scroll gesture generate dozens of remote requests.

---

59. Feed Modes

Home
Latest
Following
Continue
Unseen
Discover
Random

Home

Configurable mixed dashboard.

Latest

Chronological.

Following

Explicit follows only.

Continue

Partially consumed items.

Unseen

Unopened.

Discover

Diversity-biased recommendations.

Random

Intentional random browsing.

---

60. Home Screen

Suggested structure:

Search everything

Continue
──────────────
[card][card][card]

Following Updates
─────────────────
[series][creator][series]

New Manga & Comics
──────────────────
[cards]

Global Feed
───────────
[adaptive infinite feed]

All sections are:

reorderable
hideable
replaceable

---

61. Feed Filtering

Primary chips:

All
Video
GIFs
Images
Manga
Comics
Cartoons
Stories
Audio

Advanced:

Animation
Illustration
Photography
Booru
Imageboard
Webcomic
Live

---

62. Feed Ranking

Conceptual local score:

score =
    freshness
  + provider preference
  + media-family preference
  + follow bonus
  + preferred-tag match
  + saved-search match
  + unseen bonus
  + discovery bonus

  - seen penalty
  - provider repetition
  - creator repetition
  - series repetition
  - media-type repetition

No cloud recommendation engine should be necessary.

---

63. Ranking Modes

Chronological
Balanced
Personalized
Manual

Advanced controls:

Freshness            80
Followed series      90
Creators             70
Tags                 60
Provider priority    50
History influence    30
Discovery            20

Provide:

Why this item?

Example:

New chapter from a followed series
Matches two preferred tags
Published recently

---

64. Feed Clustering

Collapse related bursts.

Instead of:

Series A Ch 30
Series A Ch 31
Series A Ch 32
Series A Ch 33

show:

Series A

4 new chapters
Latest: Chapter 33

Cluster types:

series-update
gallery-batch
duplicate-set
provider-batch
thread-update

---

65. Search Architecture

Search in two stages.

Immediate Local Search

FTS index.

Streaming Remote Search

Provider searches occur concurrently.

UI:

Local results: 42

Remote
✓ Provider A — 14
… Provider B
✓ Provider C — 9
! Provider D — sign in required

Users can interact with results immediately.

---

66. Search Query Language

type:video
type:gif
type:manga
type:comic
type:story

source:provider
creator:"name"
series:"title"

tag:example
-tag:example

style:anime
style:cartoon

before:2026-01-01
after:2025-01-01

duration:>20m

saved:true
seen:false
following:true
offline:true

---

67. Saved Searches

A search can be:

saved
renamed
pinned
placed on Home
placed in sidebar
restricted to local index
restricted to selected providers

Example:

type:manga following:true seen:false

Display name:

Unread Followed Manga

---

68. Deduplication

Use increasingly weaker signals:

provider remote ID
canonical URL
declared cross-source canonical ID
creator + normalized title
duration similarity
thumbnail perceptual hash
optional media fingerprint

Confidence:

exact
high
possible

Only Exact/High should auto-collapse.

---

69. Source Variant Selection

If several providers contain a duplicate:

Priority:

user preferred source
authenticated source
higher quality
better metadata
recent provider reliability

UI:

Available from 3 sources

---

70. Navigation

HOME
  Home
  Discover
  Continue
  Search

MEDIA
  Video
  GIFs
  Images
  Manga
  Comics
  Cartoons
  Literature
  Audio

SOURCES
  All Sources
  Video
  Images & Booru
  Manga & Comics
  Literature
  Audio
  Creator Platforms
  Live
  + Add Source

LIBRARY
  Saved
  Series
  Collections
  Following
  History
  Downloads
  Local Files

SYSTEM
  Settings

Empty groups disappear.

---

71. Route Model

/
/discover
/continue

/search
/search/:savedSearch

/source/:provider
/source/:provider/search

/item/:uid

/series/:uid
/chapter/:uid

/library/saved
/library/series
/library/collections
/library/collection/:id
/library/following
/library/history
/library/downloads

/browser/:tab

/settings/*

---

72. Card System

All cards share:

visual preview
media indicator
title
creator
source
secondary metadata

Media-specific badges:

VIDEO     21:43
GIF       GIF
MANGA     Ch. 18 · 32p
COMIC     Issue #7
STORY     18 min read
AUDIO     26 min
GALLERY   42 images
LIVE      LIVE

---

73. Display Modes

Comfortable
Compact
List
Mosaic

Preferences may be stored:

globally
per provider
per media type

---

74. Visual Design

Design target:

«Discreet premium media software.»

Avoid:

red/black adult-site stereotype
banner advertising aesthetics
flashing UI
gratuitous sexualized chrome
excessive gradients

Prefer:

neutral surfaces
media-forward cards
subtle borders
strong typography
short motion

---

75. Visual Tokens

Example dark theme:

Canvas          #090B0F
Surface         #101419
Raised          #181D24
Border          #29313A

Primary Text    #F2F4F3
Secondary       #A7B0AE
Muted           #687370

Accent          #9B7BFF
Accent          #7ED6C4
Accent          #C47AFF
Accent          #FF6F91
Accent          #769CFF
Accent          #B7E05A
Success         #55C995
Warning         #E7B45D
Danger          #E46772

Also ship a complete light theme.

---

76. Video Viewer

Functions:

play
pause
seek
volume
mute
speed
quality
subtitle track
audio track
fullscreen
PiP
loop
queue
previous/next
source variants
open original

Use browser-native playback where appropriate.

Use mpv/libmpv as an optional fallback for eligible direct media sources.

---

77. GIF Viewer

GIF behavior is presentation-based rather than file-extension-based.

May play:

GIF
animated WebP
animated AVIF
WebM
MP4

Feed options:

Never animate
Animate on hover
Animate when visible
Always animate

Recommended default:

Animate on hover

---

78. GIF Performance Rules

Animated cards should:

pause outside viewport
pause when unfocused
respect reduced motion
limit concurrent playback
prefer preview resources
respect battery mode

---

79. Image Viewer

Fit
Actual size
Zoom
Pan
Rotate
Fullscreen
Metadata
Previous
Next
Save
Open original

---

80. Gallery Viewer

Modes:

Single
Filmstrip
Grid
Continuous
Slideshow

Use:

lazy loading
near-image prefetch
memory-bounded decoding

---

81. Manga/Comic Reader

Reader modes:

Single Page
Double Page
Continuous Vertical
Webtoon

Future:

Guided Panel

---

82. Reading Direction

Source Default
LTR
RTL
Vertical

Direction affects:

page pairing
keyboard navigation
next/previous semantics
spread order

---

83. Sequential Reader Performance

Baseline decoded pages:

previous page
current page
next two pages

Adaptive preloading should consider:

RAM
page size
network speed
reading speed
performance mode

---

84. Reading Progress

Store:

series UID
chapter/issue UID
page index
scroll fraction
updated time
completed status

Continuous readers should restore precise scroll position.

---

85. Series Page

┌──────────────────────────────────────┐
│ Cover   Series Name                  │
│         Creator                      │
│         Ongoing                      │
│                                      │
│         Continue: Ch. 20 page 14     │
│                                      │
│ [Continue] [Follow] [Save]           │
├──────────────────────────────────────┤
│ Chapters                             │
│ ✓ 18                                 │
│ ✓ 19                                 │
│ ● 20                                 │
│   21                                 │
└──────────────────────────────────────┘

Functions:

follow
continue
mark read
mark unread
mark previous read
filter unread
sort
alternate source

---

86. Literature Reader

Features:

reader mode
font
size
line height
paragraph spacing
column width
theme
find
bookmark
chapter navigation
progress
reading-time estimate
open original

Provider-supplied HTML must be sanitized.

---

87. Audio Player

Persistent mini-player:

artwork
title
creator
play/pause
seek
speed
queue
volume

Expanded view:

chapters
description
transcript
sleep timer
series navigation

Integrate Linux media controls through MPRIS where practical.

---

88. Booru Browser

Booru UI should be highly information dense.

Search tags...

+tag1 +tag2 -tag3

┌────┬────┬────┬────┬────┐
│img │img │img │img │img │
├────┼────┼────┼────┼────┤
│img │img │img │img │img │
└────┴────┴────┴────┴────┘

Optional tag categories:

creator
series
character
general
technical

---

89. Tag Normalization

Preserve:

normalized identity
provider-local identity
display value
category
aliases

interface TagReference {
  normalizedId?: string;
  providerId: string;
  sourceValue: string;
  displayName: string;
  category?: string;
}

---

90. Imageboard Thread Viewer

Structure:

Original Post
 ├── attachment
 └── text

Reply
 ├── quote
 └── attachment

Reply

Functions:

inline attachments
expand
quote navigation
hide
collapse
refresh
save attachment
open original

---

91. Embedded Browser

Browser use cases:

login
subscription content
unsupported provider
CAPTCHA
JavaScript-heavy page
DRM content
provider breakage
manual browsing

Controls:

back
forward
reload
address
tabs
open externally
add to Kelpie

---

92. Add to Kelpie

From the browser:

Add to Kelpie

runs:

current URL
  ↓
provider resolver
  ↓
recognized?
 ├── yes → normalized item
 └── no  → generic external bookmark

---

93. Authentication

Preferred:

Sign In
  ↓
provider browser
  ↓
website login page
  ↓
user authenticates directly
  ↓
site creates session

Kelpie provider adapters should not collect plaintext passwords.

---

94. Library

Saved
Series
Collections
Following
History
Downloads
Local Files

The library is provider independent.

---

95. Collections

Collections may mix:

video
GIF
gallery
manga
comic
story
audio
external page
local file

Sorting:

manual
date added
published
title
creator

---

96. Smart Collections

Future query-backed collections.

Example:

type:manga following:true seen:false

becomes:

Unread Manga

---

97. Following

Allow following:

provider
creator
series
tag
saved search

Following gets a dedicated feed and may also influence personalized ranking.

---

98. History

Track:

first opened
last opened
open count
progress
completed

Retention:

Forever
90 days
30 days
7 days
Session only
Never

---

99. Private Mode

During private mode do not persist:

searches
opened items
media progress
reader progress
browser history
feed interaction signals
temporary private session state after session close

Existing normal history remains untouched.

---

100. Safe Screen

Configurable global shortcut:

pause media
pause GIF
mute audio
hide page/image/video
hide browser
replace content with neutral UI
optionally lock

Do not imitate another application or fake an OS screen.

---

101. Neutral Preview Mode

Replace explicit visual previews with:

media icon
source icon
neutral placeholder
optional generic title

Modes:

Never hide
Blur
Reveal on hover
Reveal on click
Neutral covers

---

102. Notification Privacy

Default:

Download complete

not:

explicit title downloaded

Detailed notifications are opt-in.

---

103. Application Lock

Methods:

PIN
Passphrase
OS-backed secret

Triggers:

manual
inactivity
system resume
safe-screen activation

---

104. Database Domains

providers
provider_permissions
provider_state

items
media_sources

creators
tags
item_tags

series
chapters
pages

feeds
feed_entries
feed_clusters

favorites

collections
collection_items

follows

history
progress

saved_searches
search_history

downloads

cache_entries

provider_errors

---

105. Search Database

SQLite FTS index:

CREATE VIRTUAL TABLE items_fts USING fts5(
    uid UNINDEXED,
    title,
    description,
    creators,
    tags,
    series_title,
    body
);

Full story text should only be indexed when local storage of that text is permitted and appropriate.

---

106. Migration Policy

Every migration:

numbered
immutable
transactional
tested

Major upgrade:

database
→ backup
→ migration
→ integrity check
→ launch

Failure:

restore backup
launch recovery mode
show diagnostics

---

107. Cache Architecture

Separate:

memory cache
metadata cache
thumbnail cache
page cache
temporary media cache
persistent downloads

Cache entries can carry:

expiry
sensitivity
provider
size

---

108. Downloads

Provider must explicitly expose download capability.

States:

queued
resolving
downloading
paused
complete
failed

Features:

pause/resume
retry
concurrency
bandwidth limit
free-space check
destination

Downloading must not bypass DRM or access controls.

---

109. Offline Manga/Comics

Where permitted:

Keep Chapter Offline
Keep Issue Offline
Keep Volume Offline

Temporary page cache is distinct from persistent offline storage.

---

110. Storage Manager

Show space used by:

database
thumbnails
GIF previews
image cache
comic/manga cache
video cache
audio cache
downloads
provider data

Each can be cleared independently where safe.

---

111. Network Stack

Provider
   ↓
Origin Permission
   ↓
Rate Limiter
   ↓
Provider HTTP Client
   ↓
Provider Cookie Jar
   ↓
Cache
   ↓
Internet

Per provider:

allowed origins
rate limit
timeout
retry policy
cookie state
custom headers
proxy override

---

112. Refresh Scheduler

Schedules:

Manual
On Launch
15 Minutes
30 Minutes
Hourly
Every 6 Hours
Every 12 Hours
Daily

Scheduler should:

apply jitter
respect rate limits
pause offline
avoid duplicate jobs
back off after failures
optionally pause on battery

---

113. Error Model

NetworkError
Timeout
AuthenticationRequired
SubscriptionRequired
RateLimited
PermissionDenied
ProviderOutdated
ProviderParseError
UnsupportedMedia
ContentRemoved
DatabaseError
SecurityViolation
Unknown

The frontend receives structured errors rather than arbitrary provider strings.

---

114. Error UX

Provider changed:

This source appears to have changed.

[Check Update]
[Open Website]
[Diagnostics]

Authentication:

Sign in to continue.

[Sign In]
[Open Website]

Offline:

You're offline.

Showing indexed content.

---

115. Diagnostics

Safe diagnostic bundle:

app-info.json
environment.json
provider-status.json
errors.log
redacted-settings.json

Never export:

passwords
tokens
cookies
authorization headers
secret API keys

---

116. Provider Developer Mode

Tools:

provider console
network inspector
HTML inspector
normalized output viewer
manifest viewer
permission viewer
fixture recorder
parser tester

Example:

GET feed
200 OK
324 ms

Parsed
  24 items

Warnings
  2 missing dates
  1 missing thumbnail

---

117. Testing Strategy

Unit

query parser
ranking
deduplication
chapter sorting
tag normalization
database models
permissions

Provider fixtures

feed HTML
search JSON
item page
chapter page
expected normalized result

Integration

provider runtime
scoped HTTP
database migrations
feed pipeline
browser bridge
media resolution

End-to-End

launch
add source
refresh
search
play
read
save
resume
private mode
provider failure

---

118. Security Testing

Required regression areas:

provider sandbox escape
unexpected redirects
HTML injection
script injection
IPC exposure
path traversal
URL scheme abuse
cookie leakage
secret leakage
oversized responses
decompression bombs
malformed image
malformed animation
malformed video
malformed archive
SQL injection

---

119. HTML Rendering Security

Extracted HTML must be sanitized.

Reject by default:

script
iframe
object
embed
event handlers
unsafe URL schemes

Remote interactive web pages belong in browser webviews rather than trusted application HTML.

---

120. URL Security

Default externally navigable schemes:

https
http

Reject or explicitly review:

javascript:
data:
file:
arbitrary custom schemes

---

121. Linux Integration

Follow Linux desktop standards for:

XDG paths
desktop file
Wayland
X11
system theme
file dialogs
notifications
MPRIS
media keys
Flatpak portals

The XDG Base Directory Specification defines standard locations for user configuration, data, state and cache and should guide Kelpie's storage layout.

---

122. Filesystem Layout

$XDG_CONFIG_HOME/kelpie/
    settings.toml

$XDG_DATA_HOME/kelpie/
    kelpie.sqlite
    providers/
    downloads/

$XDG_CACHE_HOME/kelpie/
    thumbnails/
    pages/
    media/
    responses/

$XDG_STATE_HOME/kelpie/
    logs/

---

123. Packaging

Debian/Ubuntu

.deb
AppImage

Arch

AUR PKGBUILD
AppImage

Cross-distribution

Flatpak

Later:

native repositories
community distribution packaging

---

124. Performance Targets

Target on a typical modern Linux desktop:

Cold launch             < 2 s
Cached Home             < 300 ms after DB open
Local search            < 100 ms typical
Feed scrolling          60 fps target
Indexed items           100,000+
Providers               100 without redesign

---

125. UI Virtualization

Virtualize:

global feed
search
history
collections
booru grid
downloads
large chapter lists

Never mount thousands of cards simultaneously.

---

126. Image Pipeline

viewport request
     ↓
memory cache
     ↓
disk cache
     ↓
network
     ↓
decode
     ↓
resize
     ↓
render

Priorities:

visible
near viewport
off-screen

Cancel low-value work during fast scrolling.

---

127. Performance Profiles

Battery Saver
Balanced
High Performance

Battery Saver:

disable animated previews
reduce prefetch
reduce thumbnail resolution
reduce provider refresh rate
lower concurrency

---

128. Accessibility

Required:

full keyboard navigation
visible focus
semantic controls
screen-reader labels
reduced motion
UI scaling
high contrast
reader text resizing
non-color-only status cues

---

129. Keyboard Model

Global:

Ctrl+K       Search / Command Palette
Ctrl+,       Settings
Ctrl+R       Refresh
Esc          Back / Close

Browser:

Ctrl+L
Ctrl+T
Ctrl+W
Alt+Left
Alt+Right

Media:

Space
Left/Right
M
F

Reader:

Left/Right
Page Up/Down
Home/End
F

All bindings are configurable.

---

130. Settings

General
Appearance
Home
Feeds
Sources
Search

Video
GIFs
Images
Manga & Comics
Literature
Audio

Library
Downloads

Privacy
Network
Storage
Keyboard

Advanced
About

Provide settings search.

---

131. Repository Layout

kelpie/
├── apps/
│   └── desktop/
│
├── crates/
│   ├── core/
│   ├── database/
│   ├── providers/
│   ├── feed/
│   ├── search/
│   ├── media/
│   ├── browser/
│   ├── downloads/
│   ├── security/
│   └── networking/
│
├── packages/
│   ├── ui/
│   ├── normalized-types/
│   └── provider-sdk/
│
├── providers/
│   ├── official/
│   ├── generic/
│   └── examples/
│
├── registry/
├── migrations/
├── tests/
├── packaging/
└── docs/

---

132. Documentation Architecture

docs/

PRODUCT.md
FEATURES.md
ROADMAP.md

architecture/
  OVERVIEW.md
  DOMAIN-MODEL.md
  PROVIDERS.md
  FEEDS.md
  SEARCH.md
  BROWSER.md
  DATABASE.md
  EVENTS.md

media/
  VIDEO.md
  GIF.md
  IMAGES.md
  BOORU.md
  IMAGEBOARD.md
  MANGA.md
  COMICS.md
  CARTOONS.md
  LITERATURE.md
  AUDIO.md

providers/
  SDK.md
  MANIFEST.md
  CAPABILITIES.md
  PERMISSIONS.md
  REGISTRY.md
  GENERIC-HTML.md
  GENERIC-BOORU.md
  GENERIC-MANGA.md
  TESTING.md

security/
  THREAT-MODEL.md
  WEBVIEW-ISOLATION.md
  PROVIDER-SANDBOX.md
  CONTENT-SAFETY.md
  SECRET-STORAGE.md

ux/
  INFORMATION-ARCHITECTURE.md
  CARDS.md
  FEEDS.md
  VIEWERS.md
  PRIVACY.md
  ACCESSIBILITY.md

platform/
  LINUX.md
  DEBIAN.md
  UBUNTU.md
  ARCH.md
  FLATPAK.md

development/
  BUILDING.md
  TESTING.md
  RELEASES.md
  CONTRIBUTING.md

---

133. Architecture Decision Records

Use ADRs for irreversible decisions.

0001-tauri.md
0002-sqlite.md
0003-provider-sandbox.md
0004-no-remote-native-ipc.md
0005-browser-fallback.md
0006-separate-provider-registry.md
0007-structural-media-taxonomy.md
0008-local-first-history.md

ADR format:

Context
Decision
Alternatives
Consequences
Status

---

134. Implementation Plan — Phase 0

Architecture Definition

Finish:

domain vocabulary
MediaItem
Series model
provider API
capabilities
security model
UI information architecture
registry schema

Exit condition:

«Core types and security boundaries are stable enough to implement without redesigning them every sprint.»

---

135. Phase 1 — Desktop Foundation

Build:

Tauri shell
React UI
routing
navigation
theme
settings
SQLite bootstrap
logging
CI
.deb
AppImage

Use fake providers only.

---

136. Phase 2 — Core Data Layer

Build:

MediaItem
Series
Chapter
Creator
Tag
Collection
History
Progress
database migrations
repositories

Add comprehensive unit tests.

---

137. Phase 3 — Provider Platform

Build:

manifest parser
capability system
provider lifecycle
provider permissions
scoped HTTP
provider storage
fixture runtime
diagnostics

No large real-source effort yet.

---

138. Phase 4 — Feed and Search

Build:

refresh scheduler
normalization pipeline
validation
deduplication
clustering
FTS
local search
remote search
ranking
filters

Stress with synthetic datasets.

---

139. Phase 5 — Core Viewers

Build:

video
GIF
image
gallery
literature
audio

Add history and progress.

---

140. Phase 6 — Manga and Comics

Build:

series
chapter list
single page
double page
LTR
RTL
continuous
webtoon
prefetch
progress
following

---

141. Phase 7 — Isolated Browser

Build:

remote webview
tabs
address bar
cookie isolation
provider sessions
login
URL resolver
Add to Kelpie
external browser handoff

Perform a dedicated security review before scaling provider count.

---

142. Phase 8 — Generic Providers

Build:

RSS
Atom
JSON Feed
Generic HTML
Generic Gallery
Generic Manga
Generic Comic
Generic Booru
Generic Imageboard
Browser-only

---

143. Phase 9 — Reference Providers

Implement providers deliberately chosen to test different parts of the architecture:

Literotica
  → text + tags + creators + audio

RedGIFs
  → animation/clip integration

Filthy Figments
  → sequential comics

PornPics
  → galleries

FAKKU
  → adult manga + authentication/browser

Dipsea or Quinn
  → audio-first

one mainstream video provider
  → video/browser

EroMe
  → mixed galleries and clips

This provides more architectural coverage than implementing ten similar video sources first.

---

144. Phase 10 — Privacy and Library

Build:

private mode
app lock
safe screen
neutral previews
history retention
following
series library
collections
storage management

---

145. Phase 11 — Downloads and Offline

Build:

download manager
resume
storage quota
offline chapters
offline audio/video where permitted
local file imports

---

146. Phase 12 — Provider Developer Experience

Build CLI:

kelpie-provider new
kelpie-provider test
kelpie-provider lint
kelpie-provider pack

Developer UI:

fixture recorder
parser inspector
network inspector
permission inspector
normalized output

---

147. Phase 13 — Preset Catalogue Expansion

Target:

12–18 integrated/metadata providers
15–20 Browser+ presets
8–12 generic source families

Approximate bundled registry:

40–50 definitions

Do not make 50 simultaneously enabled sources the default.

---

148. Phase 14 — Beta Hardening

Test:

Debian stable
Ubuntu LTS
Arch current

GNOME
KDE

Wayland
X11

HiDPI
multi-monitor

offline
low-memory
provider outage
registry outage
database migration failure

---

149. MVP Scope

Required:

desktop shell
SQLite
provider system

global feed
provider feed

local search
remote search

favorites
collections
history
progress

video
GIF
image/gallery
manga/comic
literature
audio

browser fallback

RSS/Atom
generic HTML
generic gallery
generic manga

3–5 real providers

private mode
safe screen

.deb
AppImage

---

150. Explicit MVP Deferrals

Do not block MVP on:

cloud sync
mobile
browser extension
community marketplace
guided comic panels
OCR
advanced fingerprints
complex AI recommendations
huge provider count
full game library support

---

151. MVP Acceptance Criteria

MVP passes when:

Kelpie installs on target Linux systems.

Cached Home works offline.

One provider failure cannot break Home.

Local search responds while remote searches continue.

Video resumes correctly.

Audio resumes correctly.

Literature resumes correctly.

Manga/comic page progress restores correctly.

RTL/LTR reading works.

GIFs stop when outside viewport.

Mixed collections work.

Private mode persists no browsing history.

Remote webviews cannot access privileged IPC.

RSS source can be added without restart.

Generic manga source can be added without restart.

A provider can be disabled without losing library references.

---

152. Version 1.0 Gate

1.0 requires:

stable provider API
signed official providers
provider rollback
registry signatures
migration recovery
security review
accessibility pass
provider development docs
complete source review workflow
Debian/Ubuntu/Arch validation

---

153. Definition of Done — Application Feature

A feature is complete only when it has:

happy path
loading state
empty state
failure state
offline behavior
private-mode behavior
keyboard behavior
accessibility behavior
persistence behavior
tests
documentation

---

154. Definition of Done — Provider

Provider completion requires:

manifest validation
permission review
fixture tests
pagination tests
missing-field tests
malformed response tests
rate-limit behavior
login handling
browser fallback
diagnostic redaction
last-tested date
registry review

---

155. Key Risks

Provider churn

Mitigation: independent provider packages, Browser+ fallback.

Anti-bot systems

Mitigation: do not design around bypassing them; downgrade provider functionality.

Regulation/region changes

Mitigation: runtime provider availability state and registry updates.

Unsafe custom provider

Mitigation: sandbox + origin permissions.

Database corruption

Mitigation: transactions, backups, migration recovery.

Huge image/GIF feeds

Mitigation: virtualization, memory budgets, viewport-aware loading.

Provider monopoly over feed

Mitigation: diversity penalties and feed clusters.

Sensitive desktop exposure

Mitigation: neutral previews, safe screen, lock, discreet notifications.

---

156. Success Metrics

Product success should not be measured only by site count.

Better engineering/product metrics include:

provider failure isolation rate
feed cold-start time
search latency
crash-free sessions
provider update recovery rate
database migration success
memory usage during large galleries
average time to add a new provider
percentage of browsing possible without browser fallback
keyboard-accessibility coverage

---

157. Final Architecture

                           kelpie

                     ┌──── Discovery ────┐
                     │                   │
                     │ Home              │
                     │ Search            │
                     │ Following         │
                     │ Continue          │
                     └─────────┬─────────┘
                               │
                     Normalized Media Model
                               │
      ┌───────────┬────────────┼───────────┬────────────┐
      │           │            │           │            │
    Video        GIF        Images      Sequential      Text
                                           │
                                      Manga/Comic
                                           │
                                         Pages
      │           │            │           │            │
      └───────────┴────────────┼───────────┴────────────┘
                               │
                              Audio
                               │
                     Local Library / FTS
                               │
                      Provider Platform
                               │
      ┌──────────┬─────────┬─────────┬────────┬─────────┐
      │          │         │         │        │         │
    APIs        RSS       HTML     Booru    Manga     Browser
      │          │         │         │        │         │
      └──────────┴─────────┴─────────┴────────┴─────────┘
                               │
                         External Sources

---

158. Final Product Direction

Kelpie should not become:

«A giant scraper with a desktop UI.»

It should become:

«A private Linux media platform with a provider ecosystem.»

The distinction determines whether the project remains maintainable.

Individual sites will:

- Redesign.
- Change APIs.
- Introduce authentication.
- Restrict regions.
- Remove endpoints.
- Become unavailable.
- Change ownership.

Kelpie's user experience should survive those events because its core concepts are independent:

Media
Series
Creator
Tag
Feed
Search
Collection
Follow
History
Progress
Provider

The project's strongest long-term assets are therefore:

1. The provider contract.

2. The normalized media and series models.

3. The local feed/search/library engine.

4. The specialized video, GIF, image, manga/comic, text and audio experiences.

5. The isolated browser fallback.

6. The provider registry and governance system.

7. The privacy/security model.

8. The ability for users to add new sources without waiting for the Kelpie core application to be redesigned.

That architecture makes Kelpie capable of covering mainstream video, GIFs, image galleries, manga, comics, cartoons, hentai/adult illustration, literature, audio, creator platforms, live sites, boorus, imageboards and future media categories without turning each one into a separate application.
