//! kelpie-core::domain::tag: the normalized tag registry entity (kelpie.md §136 Phase
//! 2, §89-90 Tag Normalization; kelpie.md §104 Database Domains: `tags`, `item_tags`).
//!
//! [`Tag`] is the normalized registry row — one per stable normalized identity — that
//! backs booru-style browsing (§88) by letting a tag be looked up, deduplicated, and
//! filtered across the whole library. It is distinct from
//! [`crate::domain::TagReference`], which is the per-reference/per-item shape that
//! other entities (e.g. `MediaItem.tags`, `Series.tags`) embed denormalized as JSON
//! directly on their own rows. A `TagReference`'s `normalized_id`, when present,
//! points at a [`Tag::normalized_id`] in this registry.

use serde::{Deserialize, Serialize};

/// A row in the normalized tag registry (`tags` table).
///
/// `normalized_id` is the stable normalized identity used to deduplicate and filter
/// tags across providers (kelpie.md §89 Tag Normalization: "normalized identity").
/// `aliases` collects the other raw source values (see
/// [`crate::domain::TagReference::source_value`]) that have been mapped onto this
/// normalized tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub normalized_id: String,
    pub display_name: String,
    pub category: Option<String>,
    pub aliases: Vec<String>,
}
