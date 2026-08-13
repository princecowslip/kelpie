//! kelpie-core::domain::collection: the `Collection` domain type (kelpie.md §136
//! Phase 2, §94 Library, §95 Collections).
//!
//! A collection is user-created and may mix any [`crate::domain::MediaKind`] — video,
//! GIF, gallery, manga, comic, story, audio, external page, or local file (§95). Kelpie
//! places no media-kind restriction on collection membership at the data layer; that is
//! a UI/domain concern layered on top. Item membership and manual ordering live in the
//! `collection_items` link table, backed by `migrations/0007_collections.sql`.

use serde::{Deserialize, Serialize};

/// How a collection's items are ordered for display (kelpie.md §95 Collections:
/// Sorting).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionSortMode {
    /// User-defined manual order, tracked by `collection_items.position`.
    Manual,
    /// Order by when the item was added to the collection.
    DateAdded,
    /// Order by the item's publication date.
    Published,
    /// Order by item title.
    Title,
    /// Order by item creator.
    Creator,
}

/// A user-created collection (kelpie.md §94 Library, §95 Collections).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub uid: String,
    pub name: String,
    pub sort_mode: CollectionSortMode,
    pub created_at: String,
    pub updated_at: String,
}

/// A single item's membership in a [`Collection`] (`collection_items` table). `uid`s
/// referenced here are opaque foreign identifiers into whichever entity table the item
/// actually belongs to (e.g. a `MediaItem` or `Series` uid) — collections do not
/// constrain what kind of item they hold.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionItem {
    pub collection_uid: String,
    pub item_uid: String,
    /// Manual ordering index, used when `sort_mode` is [`CollectionSortMode::Manual`].
    pub position: i64,
    pub added_at: String,
}
