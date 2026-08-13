//! kelpie-database::repositories: SQLite-backed repositories for the Phase 2 (§136)
//! domain entities defined in `kelpie_core::domain`.
//!
//! One submodule per entity, each independently filled in by its own Phase 2 work
//! unit. This file's `pub mod` list is frozen for Phase 2 — entity work fills in the
//! body of its own submodule file and never needs to touch this file.
//!
//! Repositories take `&rusqlite::Connection` per call rather than owning a connection
//! or pool, matching the connection lifecycle already established by
//! [`crate::bootstrap`] / [`crate::open_at`] in the parent module.

pub mod chapter;
pub mod collection;
pub mod creator;
pub mod history;
pub mod media_item;
pub mod progress;
pub mod series;
pub mod tag;
