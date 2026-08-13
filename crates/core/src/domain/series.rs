//! kelpie-core::domain::series: the `Series` domain type (kelpie.md §136 Phase 2,
//! §20-21 Series Model / Series Object; see `docs/architecture/DOMAIN-MODEL.md`
//! "Series Object" for the canonical field-level reference this module implements
//! in Rust).
//!
//! `Series` represents the top of the hierarchical structure manga, comics,
//! webcomics, and episodic audio need above the individual `MediaItem` (kelpie.md
//! §20 Series Model): `Series -> Volume -> Chapter/Issue -> Page`, or for audio,
//! `Audio Series -> Episode/Chapter`.

use serde::{Deserialize, Serialize};

use super::{CreatorCredit, ImageResource, LocalizedTitle, TagReference};

/// Publication status of a `Series` (kelpie.md §21 `Series.status`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SeriesStatus {
    Ongoing,
    Completed,
    Hiatus,
    Cancelled,
    Unknown,
}

/// The direction a `Series`' pages should be read in (kelpie.md §21
/// `Series.readingDirection`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadingDirection {
    Ltr,
    Rtl,
    Vertical,
}

/// The `Series` object (kelpie.md §21 Series Object): the top of the hierarchical
/// structure above individual `MediaItem`s for manga, comics, webcomics, and
/// episodic audio.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Series {
    pub uid: String,

    pub provider_id: String,
    pub remote_id: Option<String>,

    pub canonical_url: String,

    pub title: String,
    pub alternate_titles: Vec<LocalizedTitle>,

    pub description: Option<String>,

    pub creators: Vec<CreatorCredit>,

    pub cover: Option<ImageResource>,

    pub tags: Vec<TagReference>,

    pub status: Option<SeriesStatus>,

    pub reading_direction: Option<ReadingDirection>,
}
