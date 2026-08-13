//! kelpie-core::domain::chapter: the `Chapter` and `Page` domain types (kelpie.md
//! §136 Phase 2, §20 Series Model, §22 Sequence Numbers; see
//! `docs/architecture/DOMAIN-MODEL.md` for the canonical field-level reference).
//!
//! kelpie.md does not give a literal `Chapter`/`Page` TypeScript interface the way it
//! does for `Series` (§21) — the Series Model (§20) describes the hierarchy
//! `Series -> Volume -> Chapter/Issue -> Page` and leaves the leaf shapes to be
//! inferred from that hierarchy plus the `MediaItem.sequence` shape (§16). The field
//! shapes below are that inferred implementation choice, not a literal spec quote.

use crate::domain::{ImageResource, SequenceNumber};
use serde::{Deserialize, Serialize};

/// A single chapter (or issue) within a `Series`, per the `Series -> Volume ->
/// Chapter/Issue -> Page` hierarchy (kelpie.md §20 Series Model).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub uid: String,

    /// Foreign key to `Series.uid` (kelpie.md §21 Series Object).
    pub series_uid: String,

    pub provider_id: String,
    pub remote_id: Option<String>,

    pub canonical_url: String,

    pub title: Option<String>,

    /// The chapter/issue number itself (kelpie.md §22 Sequence Numbers). Raw source
    /// labels like `10`, `10.5`, `10a`, `Special` are all valid.
    pub sequence: SequenceNumber,

    /// The volume this chapter belongs to, when the source has volumes as an
    /// intermediate level between `Series` and `Chapter` (kelpie.md §20).
    pub volume_sequence: Option<SequenceNumber>,

    /// RFC3339 timestamp text, when the source exposes one.
    pub published_at: Option<String>,

    pub page_count: Option<u32>,
}

/// A single page within a `Chapter` (kelpie.md §20 Series Model).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    /// Foreign key to `Chapter.uid`.
    pub chapter_uid: String,

    /// 0-based ordering of this page within its chapter.
    pub page_index: u32,

    pub image: ImageResource,
}
