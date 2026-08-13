//! kelpie-core::domain: the normalized media domain model (kelpie.md §12-23 Core Media
//! Taxonomy / Unified MediaItem / Series Model; see `docs/architecture/DOMAIN-MODEL.md`
//! for the canonical field-level reference this module implements in Rust).
//!
//! Phase 2 (§136) scope. The types defined directly in this file are the ones shared
//! across more than one entity module below, so they are defined exactly once here
//! rather than duplicated per-entity. Each submodule owns exactly one Phase 2 entity
//! and is filled in independently — this file's contents (the `pub mod` declarations
//! and the shared types) are frozen for Phase 2 so that parallel entity work never
//! needs to touch this file.
//!
//! `CreatorCredit`, `ImageResource`, and `LocalizedTitle` are referenced by kelpie.md's
//! TypeScript interfaces (§16, §21) but no field-level shape is given anywhere in
//! kelpie.md or docs/ — the shapes below are an implementation choice inferred from how
//! each type is used, not a literal spec quote.

pub mod chapter;
pub mod collection;
pub mod creator;
pub mod history;
pub mod media_item;
pub mod progress;
pub mod series;
pub mod tag;

use serde::{Deserialize, Serialize};

/// Structural media type (kelpie.md §13 Structural Media Types). Only the subset
/// already in use by the Phase 1 fake-provider feed plus the manga/comic/audio kinds
/// Phase 2's entities need is enumerated so far; the remaining `MediaKind` variants
/// from §13 are added by whichever later phase first needs to store them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaKind {
    Video,
    ShortVideo,
    Clip,
    Gif,
    AnimatedImage,
    Animation,
    Image,
    Illustration,
    Photograph,
    Gallery,
    Manga,
    Comic,
    Webcomic,
    ComicStrip,
    Story,
    SerializedText,
    Article,
    Audio,
    AudioStory,
    BooruPost,
    ImageboardThread,
    ImageboardPost,
    Live,
    Game,
    VisualNovel,
    External,
}

/// How an item is visually presented, independent of `MediaKind` (kelpie.md §14).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaStyle {
    Photographic,
    Realistic,
    Illustrated,
    Anime,
    Manga,
    Cartoon,
    ThreeD,
    Mixed,
    Other,
}

/// Whether and how an item can currently be accessed (kelpie.md §18 Availability Model).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Availability {
    Direct,
    Embedded,
    Browser,
    LoginRequired,
    SubscriptionRequired,
    TemporarilyUnavailable,
    Removed,
    Unknown,
}

/// Adult classification metadata (kelpie.md §15). Every item in Kelpie is adult
/// content; this records how the source itself categorizes it.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ContentClassification {
    pub labels: Vec<String>,
    pub provider_labels: Vec<String>,
    pub style: Option<Vec<MediaStyle>>,
}

/// Chapter/volume/issue numbering is not assumed to be a plain integer — sources may
/// expose values like `10`, `10.5`, `10a`, `Special`, `Prologue` (kelpie.md §22).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceNumber {
    pub raw: String,
    pub numeric: Option<f64>,
    pub sort_key: String,
}

/// A normalized tag reference, preserving traceability back to the source
/// (kelpie.md §90 Tag Normalization). Populates `MediaItem.tags` / `Series.tags`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagReference {
    pub normalized_id: Option<String>,
    pub provider_id: String,
    pub source_value: String,
    pub display_name: String,
    pub category: Option<String>,
}

/// A creator credit on an item or series (kelpie.md §16, §21 `creators`). Field shape
/// is an inferred implementation choice — see module doc comment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatorCredit {
    pub name: String,
    pub role: Option<String>,
    pub provider_id: Option<String>,
    pub url: Option<String>,
}

/// An image resource (thumbnail, cover, page image). Field shape is an inferred
/// implementation choice — see module doc comment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageResource {
    pub url: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

/// A title in a specific locale (kelpie.md §21 `alternateTitles`). Field shape is an
/// inferred implementation choice — see module doc comment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizedTitle {
    pub locale: String,
    pub value: String,
}

/// A lightweight reference from a `MediaItem` back to its parent `Series`
/// (kelpie.md §16 `MediaItem.series`), without embedding the full `Series` object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesReference {
    pub uid: String,
    pub title: String,
}
