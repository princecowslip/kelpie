//! kelpie-core::domain::media_item: the `MediaItem` entity (kelpie.md §136 Phase 2,
//! §16 Unified MediaItem, §19 Media Sources; see `docs/architecture/DOMAIN-MODEL.md`
//! for the canonical field-level reference this module implements in Rust).
//!
//! `MediaDimensions`, `MediaSequence`, `MediaTransport`, and `MediaSource` are the
//! small nested types `MediaItem` needs that aren't shared with any other Phase 2
//! entity, so (unlike `CreatorCredit`, `ImageResource`, `TagReference`, etc.) they
//! live here rather than in `domain::mod`.
//!
//! `providerMetadata` is specified in kelpie.md §16 as `Record<string, unknown>`
//! (i.e. arbitrary JSON), which would naturally map to `Option<serde_json::Value>`.
//! `kelpie-core`'s `Cargo.toml` is frozen shared wiring for Phase 2 and does not
//! depend on `serde_json` (only `serde`), so that field is instead represented here
//! as `Option<String>` holding the already-serialized JSON text verbatim — which is
//! also exactly the shape it takes on disk, since `kelpie-database` persists it as a
//! JSON TEXT column without needing to interpret its contents.

use serde::{Deserialize, Serialize};

use super::{
    Availability, ContentClassification, CreatorCredit, ImageResource, MediaKind, SequenceNumber,
    SeriesReference, TagReference,
};

/// Intrinsic width/height of a `MediaItem`, when known (kelpie.md §16 `dimensions`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MediaDimensions {
    pub width: Option<u32>,
    pub height: Option<u32>,
}

/// Volume/chapter/issue position within a series, plus page count (kelpie.md §16
/// `sequence`). Each position uses `SequenceNumber` (kelpie.md §22) rather than a
/// plain integer, since sources may expose values like `10`, `10.5`, or `Special`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MediaSequence {
    pub volume: Option<SequenceNumber>,
    pub chapter: Option<SequenceNumber>,
    pub issue: Option<SequenceNumber>,
    pub page_count: Option<u32>,
}

/// How a `MediaSource` is delivered (kelpie.md §19 Media Sources).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaTransport {
    Http,
    File,
    Hls,
    Dash,
    Browser,
}

/// A single playable/renderable source resolved for a `MediaItem` (kelpie.md §19
/// Media Sources). A `MediaItem` may have zero or more of these.
///
/// Per kelpie.md §19, `url` may be an ephemeral signed URL that expires (see
/// `expiresAt`) and should be re-resolved through the provider rather than treated
/// as permanent identity — `MediaSource` is a resolved-source record, not identity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaSource {
    pub url: String,
    pub transport: MediaTransport,
    pub mime_type: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub bitrate: Option<u32>,
    pub label: Option<String>,
    pub expires_at: Option<String>,
}

/// The unified normalized media representation every provider adapter produces
/// (kelpie.md §16 Unified MediaItem). Backed by the `items` + `media_sources`
/// tables (kelpie.md §104 Database Domains: Media) via
/// `kelpie_database::repositories::media_item`.
///
/// Stable identity (kelpie.md §17): the preferred `uid` shape is
/// `provider-id:object-type:remote-id`, falling back to
/// `SHA256(provider-id + canonical-url)` when a provider exposes no stable
/// `remote_id`. Title is never identity.
///
/// Timestamps (`published_at`, `updated_at`) are stored as RFC3339 text, matching
/// this codebase's convention of storing all timestamps as TEXT (no `chrono`/`time`
/// dependency).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaItem {
    pub uid: String,

    pub provider_id: String,
    pub remote_id: Option<String>,

    pub canonical_url: String,

    pub kind: MediaKind,

    pub title: String,
    pub description: Option<String>,

    pub creators: Vec<CreatorCredit>,

    pub thumbnails: Vec<ImageResource>,

    pub tags: Vec<TagReference>,

    pub classification: ContentClassification,

    pub published_at: Option<String>,
    pub updated_at: Option<String>,

    pub duration_seconds: Option<f64>,

    pub dimensions: Option<MediaDimensions>,

    pub series: Option<SeriesReference>,

    pub sequence: Option<MediaSequence>,

    pub availability: Availability,

    pub media_sources: Vec<MediaSource>,

    /// Arbitrary provider-specific JSON (kelpie.md §16 `providerMetadata`), stored
    /// as already-serialized JSON text — see the module doc comment for why this
    /// isn't `serde_json::Value`.
    pub provider_metadata: Option<String>,
}
