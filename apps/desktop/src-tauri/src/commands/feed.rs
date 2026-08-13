//! Fake feed command handlers (kelpie.md §135 Phase 1: "Use fake providers only").
//! Owned by Stage B unit 6 (fake provider feed data) — this stub returns no items.

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct MediaItemStub {
    pub uid: String,
    pub title: String,
    pub kind: String,
    pub thumbnail_url: Option<String>,
    pub provider_label: String,
}

#[tauri::command]
pub fn get_fake_feed() -> Vec<MediaItemStub> {
    Vec::new()
}
