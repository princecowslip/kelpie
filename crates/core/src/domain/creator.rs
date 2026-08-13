//! kelpie-core::domain::creator: `Creator` domain type (kelpie.md §136 Phase 2 Core
//! Data Layer; §104 Database Domains groups `creators` under "Attribution &
//! tagging"; §97 Following lists "creator" as a followable entity type).
//!
//! `Creator` is a standalone, normalized attribution registry row — distinct from
//! the denormalized `creators: Vec<CreatorCredit>` JSON embedded directly on
//! `MediaItem`/`Series` rows (see [`crate::domain::CreatorCredit`], which other
//! Phase 2 entities store as their own per-row credit list; that is fine and
//! expected). This table exists so a creator can be looked up, followed, and
//! deduplicated across items independent of any single item's embedded credit
//! (kelpie.md §97).
//!
//! `id` is a stable identifier chosen by the caller at creation time — this type
//! does not generate one. The intended convention (documented here, not mandated
//! by kelpie.md, which does not specify creator id shape) is:
//!
//! - `"{providerId}:{remoteId}"` for creators that originate from a provider, so
//!   the same remote creator always maps to the same row (supports dedup/lookup).
//! - Any other caller-generated unique string (e.g. a UUIDv4) for locally-created
//!   creators with no provider origin.

use serde::{Deserialize, Serialize};

/// A normalized creator record (kelpie.md §104 Database Domains: Attribution;
/// §97 Following). See the module doc comment for the `id` convention.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Creator {
    /// Stable identifier. See module doc comment.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Alternate names/handles this creator is also known by. Persisted as a JSON
    /// array in a single TEXT column by the repository layer.
    pub aliases: Vec<String>,
    /// The provider this creator record originated from, if any. `None` for
    /// creators that were created locally rather than sourced from a provider.
    pub provider_id: Option<String>,
}

/// A creator credited on a specific item, as returned by joining `item_creators`
/// with `creators` (kelpie.md §104). Distinct from [`crate::domain::CreatorCredit`],
/// which is the denormalized credit embedded directly on `MediaItem`/`Series` rows.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatorLink {
    pub creator: Creator,
    /// The role this creator held on the linked item, e.g. `"author"`, `"artist"`.
    pub role: Option<String>,
}
