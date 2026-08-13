//! kelpie-core::domain::progress: reading/playback progress (kelpie.md §136 Phase 2,
//! §84 Reading Progress).
//!
//! Tracks resumable playback/reading position for an item — distinct from `History`,
//! which only tracks access/open counts (a different Phase 2 work unit). A `Progress`
//! record applies either to a plain item (video/audio/etc: `item_uid` is the item's
//! own uid, `series_uid` is `None`) or to a manga/comic chapter within a series
//! (`item_uid` is the chapter's uid, `series_uid` is `Some`).

use serde::{Deserialize, Serialize};

/// Reading/playback progress for a single item or series chapter (kelpie.md §84).
///
/// `page_index` and `scroll_fraction` serve paginated/continuous readers
/// (manga, comics, galleries); `position_seconds` serves time-based media
/// (video, audio) as the natural equivalent of "page index" for playback
/// position. `positionSeconds` is not explicitly named in §84 but is the
/// natural time-based counterpart to page index, needed so this one table can
/// serve both paginated and time-based progress.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Progress {
    /// Primary key: the item's uid, or the chapter's uid when this progress
    /// record is for a chapter within a series.
    pub item_uid: String,
    /// Set when this progress is for a chapter within a series; `None` for a
    /// plain item (video/audio/etc).
    pub series_uid: Option<String>,
    /// Current page, for manga/comic/gallery readers.
    pub page_index: Option<u32>,
    /// Current scroll position within the page/chapter, 0.0-1.0, for
    /// continuous readers that need to restore precise scroll position.
    pub scroll_fraction: Option<f64>,
    /// Current playback position, in seconds, for video/audio media.
    pub position_seconds: Option<f64>,
    /// RFC3339 timestamp of the last update to this progress record.
    pub updated_at: String,
    /// Whether this item/chapter has been marked fully read/watched.
    pub completed: bool,
}
