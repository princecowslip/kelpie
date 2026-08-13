//! Fake feed command handlers (kelpie.md §135 Phase 1: "Use fake providers only").
//! Owned by Stage B unit 6 (fake provider feed data). Delegates the actual
//! catalog generation to `kelpie_providers::fake_feed()` and maps it onto the
//! wire-serialized `MediaItemStub` shape consumed by the frontend.

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct MediaItemStub {
    pub uid: String,
    pub title: String,
    pub kind: String,
    pub thumbnail_url: Option<String>,
    pub provider_label: String,
    pub description: Option<String>,
}

impl From<kelpie_providers::FakeMediaItem> for MediaItemStub {
    fn from(item: kelpie_providers::FakeMediaItem) -> Self {
        Self {
            uid: item.uid,
            title: item.title,
            kind: item.kind.as_str().to_string(),
            thumbnail_url: item.thumbnail_url,
            provider_label: item.provider_label,
            description: item.description,
        }
    }
}

#[tauri::command]
pub fn get_fake_feed() -> Vec<MediaItemStub> {
    kelpie_providers::fake_feed()
        .into_iter()
        .map(MediaItemStub::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_fake_feed_into_stubs_with_valid_kind_strings() {
        let items = get_fake_feed();
        assert!(!items.is_empty(), "expected get_fake_feed to return items");

        const VALID_KINDS: &[&str] = &[
            "video",
            "image",
            "gif",
            "manga",
            "comic",
            "literature",
            "audio",
        ];
        for item in &items {
            assert!(!item.uid.is_empty());
            assert!(!item.title.is_empty());
            assert!(!item.provider_label.is_empty());
            assert!(
                VALID_KINDS.contains(&item.kind.as_str()),
                "unexpected kind string: {}",
                item.kind
            );
        }
    }
}
