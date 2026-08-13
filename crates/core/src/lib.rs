//! kelpie-core: shared domain types and application services (kelpie.md §9-10, §16 MediaItem).
//! Placeholder crate — most content lands in Phase 0/2. Currently provides XDG path
//! resolution (§121-122), used by the SQLite bootstrap and logging setup in Phase 1.

pub mod domain;
pub mod logging;
pub mod paths;
