//! kelpie-core::domain::history: the `History` entity (kelpie.md §136 Phase 2,
//! §98 History).
//!
//! Tracks per-item access/completion state: when the item was first opened, when it
//! was last opened, how many times it has been opened, and whether it has been
//! marked completed. The "progress" concept named alongside these fields in §98 is
//! only a pointer here — `History` knows an item *has* progress associated with it,
//! but the progress data itself (page index, scroll fraction, etc.) is owned by the
//! separate `Progress` entity (`kelpie_core::domain::progress`) and is not
//! duplicated in this struct.
//!
//! Retention policy (Forever/90 days/30 days/7 days/Session only/Never, §99 Private
//! Mode) is later-phase product behavior built on top of this storage primitive —
//! see `kelpie_database::repositories::history::prune_before`.

use serde::{Deserialize, Serialize};

/// One row per item: access/completion bookkeeping for kelpie.md §98 History.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct History {
    /// Stable UID of the item this history row tracks (kelpie.md §17 Stable
    /// Identity). One history row per item — this is the primary key.
    pub item_uid: String,
    /// RFC3339 timestamp of the first time this item was opened. Never changes
    /// after the row is created.
    pub first_opened_at: String,
    /// RFC3339 timestamp of the most recent time this item was opened.
    pub last_opened_at: String,
    /// Number of times this item has been opened.
    pub open_count: u32,
    /// Whether the item has been marked completed.
    pub completed: bool,
}
