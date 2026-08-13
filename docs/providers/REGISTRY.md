# Provider Registry

> This document is derived from kelpie.md v4.0 (Registry snapshot: August 2026). Kelpie is pre-implementation — this describes intended/aspirational architecture, not built or verified behavior.

## Overview

This document is the canonical catalogue reference for Kelpie's official provider registry: which source families and named providers are recommended as launch/registry candidates, at what integration level, and under what governance rules. It reproduces the spec's own preset tables and recommendations (kelpie.md §34–§44, §49–§54) faithfully, because that catalogue data is itself the content this document exists to carry.

A note on scope: the real third-party sites named below (Pornhub, XVideos, FAKKU, Literotica, and others) are registry-planning references, exactly as they are in kelpie.md itself — they describe which source *categories and named candidates* the registry is planned to reason about, not confirmation that Kelpie integrates with, has scraped, or has verified any of them today. Kelpie is pre-implementation; nothing in this document is built or verified behavior. See `CLAUDE.md` at the repository root for the content boundaries that govern how this catalogue may be extended.

Several tables below use integration-level and safety-status terms (`Native`, `Integrated`, `Metadata`, `Browser+`, `Generic`, `Experimental`, `Quarantined`) that are defined in full in kelpie.md §30 (Provider Integration Levels) and covered architecturally in [`../architecture/PROVIDERS.md`](../architecture/PROVIDERS.md); they are used here as-is without being redefined.

## Official preset registry

kelpie.md §34 states that Kelpie should ship approximately 40–50 provider definitions/source families, but that far fewer should be enabled automatically (see [First-run preset selection](#first-run-preset-selection) below).

Every preset in the registry is expected to include:

- Provider ID
- Display name
- Site type
- Integration level
- Capabilities
- Permissions
- Global-feed eligibility
- Last verification date
- Registry safety status
- Known limitations

## Preset registry philosophy

kelpie.md §35 draws a deliberate distinction that governs everything else in this document:

> A preset means: "Kelpie knows how to represent or safely open this source."
> It does not necessarily mean: "Kelpie extracts its media directly."

This distinction is what allows the registry to offer strong support for subscription sources without the design implying that Kelpie defeats their business or access models — a preset can be nothing more than "Kelpie knows this site exists, what kind of content it holds, and how to open it safely," at `Browser+` or `Metadata` integration, rather than a full scraping integration.

## Candidate presets by category

### Video presets

kelpie.md §36 (Candidate Video Presets) gives the following recommended launch catalogue:

| Provider | Role | Initial integration |
|---|---|---|
| Pornhub | mainstream video | Browser+ / Metadata |
| XVideos | mainstream video | Browser+ / Metadata |
| XNXX | mainstream video | Browser+ / Metadata |
| xHamster | mainstream video | Browser+ / Metadata |
| Bellesa | curated video | Browser+ |
| MakeLoveNotPorn | curated/independent video | Browser+ |
| PinkLabel.TV | independent/queer cinema | Browser+ |
| Lustery | creator/couple video | Browser+ |
| Crash Pad Series | independent/queer video | Browser+ |

The spec notes that major platforms including Pornhub, XVideos, and XNXX remain active enough in 2026 to be subject to ongoing EU regulatory proceedings; because age-assurance requirements and availability can vary by jurisdiction, Kelpie is meant to treat source accessibility as runtime state rather than assume universal availability.

### GIF and short-clip presets

kelpie.md §37 recommends:

| Provider | Integration | Main structures |
|---|---|---|
| RedGIFs | Integrated/Browser+ | GIF, clip, short video |
| EroMe | Integrated/Browser+ | clip, gallery, image |
| Pornhub short-content view | provider sub-feed | short video |
| XVideos short-content view | provider sub-feed | short video |

RedGIFs remains active in 2026, including its web-app surface, but its discontinued third-party API access (see [`MANIFEST.md`](./MANIFEST.md) for how the registry handles this kind of change without a new Kelpie release) means Kelpie should not make API availability an architectural assumption.

### Image and gallery presets

kelpie.md §38 gives a candidate set and a recommended integration policy:

Candidates: PornPics, EroMe, SuicideGirls, ImageFap (review-required).

| Provider | Recommended integration |
|---|---|
| PornPics | Integrated / Browser+ |
| EroMe | Integrated / Browser+ |
| SuicideGirls | Browser+ |
| ImageFap | Review Required |

Large user-uploaded gallery services should be disabled by default unless registry review determines they are suitable for automatic global-feed participation.

### Manga, doujinshi, and hentai presets

kelpie.md §39 gives the primary official candidates as FAKKU and Irodori Sakura / Irodori Comics' adult catalogue.

FAKKU currently presents an 18+ age gate and identifies its catalogue around adult manga, comics, doujin, and games. Irodori currently maintains a distinct R18 catalogue as well as a separate all-ages catalogue — which the spec calls out as exactly why Kelpie must represent catalogue classification independently from provider identity, rather than assuming a provider is uniformly one classification.

Recommended integration:

- **FAKKU** — Metadata / Browser+ initially, later Integrated where appropriate.
- **Irodori R18** — Metadata / Browser+.

Paid/subscription reading is expected to use legitimate authenticated access rather than any workaround.

### Comic, cartoon, and webcomic presets

kelpie.md §40 gives a candidate set of Filthy Figments, Oglaf, and Slipshine — sources that exercise different sequential-art structures and should not be forced through the manga assumptions used above.

| Provider | Recommended integration |
|---|---|
| Filthy Figments | Integrated / Browser+ |
| Oglaf | Integrated / Browser+ |
| Slipshine | Browser+ initially |

Filthy Figments remains reachable as an adult-comics source and is called out as a useful reference provider for series/page-oriented integration.

### Literature presets

kelpie.md §41 names Literotica and Lush Stories as primary candidates.

Literotica is called out as particularly valuable architecturally because its current service exposes stories, categories, tags, authors, audio, and interactive story games within one ecosystem. The spec's recommended Literotica capability set is:

- `feed`
- `search`
- `story`
- `creator`
- `tags`
- `series`
- `audio`
- `interactive-fiction`
- `related`
- `browser`

This breadth is what makes Literotica, per the spec, one of the best early providers for testing mixed-media normalization.

### Audio presets

kelpie.md §42 recommends Quinn, Dipsea, femtasy, Literotica Audio, and Bloom Stories.

Quinn currently describes itself as an audio-erotica application. Dipsea currently offers audio stories/audiobooks organized into multi-chapter series, making it suitable for Kelpie's Series → Chapter → Audio model (kelpie.md §20–§22).

| Provider | Recommended integration |
|---|---|
| Quinn | Metadata / Browser+ |
| Dipsea | Metadata / Browser+ |
| femtasy | Browser+ |
| Literotica | Integrated through main provider |
| Bloom | Browser+ |

### Creator-platform presets

kelpie.md §43 recommends adding a dedicated `creator_platform` source family, with candidates OnlyFans, Fansly, ManyVids, and JustForFans, all at initial integration **Browser+**.

Supported Kelpie functions for this family:

- Isolated login
- Creator bookmark
- Follow shortcut
- History
- URL capture
- Manual library save
- Source metadata

The spec is explicit: **do not build around scraping authenticated creator feeds.**

### Live/cam presets

kelpie.md §44 defines a `live` site type, with candidates Chaturbate, Stripchat, and MyFreeCams, all at initial integration **Browser+**.

Kelpie may support, for this family:

- Creator bookmarks
- Live/offline state where legitimately available
- History
- Provider browser

Kelpie must **not**:

- Record by default
- Cache private sessions
- Bypass paid/private-show controls

Stripchat is noted as among the large adult services involved in current EU regulatory proceedings, reinforcing the need for jurisdiction-aware provider states across this family generally.

## Generic built-in sources

kelpie.md §49 specifies that every installation should include the following generic (protocol/site-family, not single-site) providers:

- RSS
- Atom
- JSON Feed
- Generic HTML Feed
- Generic HTML Gallery
- Generic Video Source
- Generic GIF Source
- Generic Manga
- Generic Comic
- Generic Webcomic
- Generic Booru
- Generic Imageboard
- Generic Literature
- Browser-only Source

The spec's framing is unambiguous: these generic providers are strategically **more important** than supporting an enormous number of fixed sites. See [`GENERIC-HTML.md`](./GENERIC-HTML.md), [`GENERIC-BOORU.md`](./GENERIC-BOORU.md), and [`GENERIC-MANGA.md`](./GENERIC-MANGA.md) for the builder mechanics behind several of these.

## First-run preset selection

kelpie.md §50 is explicit that the complete provider registry should **not** be enabled automatically. Instead, a first-run screen offers a curated starting set:

```
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
```

Followed by a "Browse all sources" option. Users explicitly choose what gets enabled beyond this starting set.

## Global feed eligibility

kelpie.md §51 gives recommended defaults for whether a provider category is allowed to populate the global feed (Home) automatically:

| Category | Default |
|---|---|
| Conventional video | opt-in |
| GIF | opt-in |
| Images/gallery | opt-in |
| Manga/comics | opt-in |
| Literature | opt-in |
| Audio | opt-in |
| Creator platforms | off |
| Live/cam | off |
| Imageboards | off |
| User boorus | off until enabled |
| Browser-only providers | off |

A provider can exist in Kelpie — installed and usable — without being allowed to populate Home.

## Registry metadata

kelpie.md §52 specifies the following shape for a registry entry:

```ts
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
```

`capabilities` here draws from the same vocabulary documented in [`CAPABILITIES.md`](./CAPABILITIES.md).

## Registry governance

kelpie.md §53 specifies that every official provider should maintain:

- Last technical verification
- Last safety review
- Provider version
- Host/domain ownership snapshot
- Integration health
- Permission set

Registry review cadence:

| Provider category | Review cadence |
|---|---|
| High-volume integrations | monthly |
| Safety-sensitive sources | monthly |
| Browser-only sources | quarterly |
| Generic protocols | per release |

## Automatic provider quarantine

kelpie.md §54 specifies that quarantine is triggered when any of the following occur:

- Provider signature invalid
- Package hash mismatch
- Unexpected domain change
- Unexpected permission expansion
- Persistent parser failure
- Security advisory
- Malicious redirect pattern

Quarantine stops provider execution while preserving user library state — quarantining a provider is not meant to delete a user's history, favorites, collections, indexed metadata, or source URLs tied to it (kelpie.md §29, Provider Lifecycle).

## Related documents

- [`MANIFEST.md`](./MANIFEST.md) — the manifest shape each registry entry is backed by.
- [`CAPABILITIES.md`](./CAPABILITIES.md) — the capability vocabulary referenced in registry entries.
- [`PERMISSIONS.md`](./PERMISSIONS.md) — the sandbox/network model registry permission review evaluates.
- [`GENERIC-HTML.md`](./GENERIC-HTML.md), [`GENERIC-BOORU.md`](./GENERIC-BOORU.md), [`GENERIC-MANGA.md`](./GENERIC-MANGA.md) — the generic built-in source families referenced above.
- [`../architecture/PROVIDERS.md`](../architecture/PROVIDERS.md) — provider lifecycle, integration levels, health, updates, and rollback.
