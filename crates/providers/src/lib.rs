//! kelpie-providers: provider execution runtime (kelpie.md §23-33 Provider Model).
//! Placeholder crate — the sandboxed provider platform lands in Phase 3 (§137).
//! Phase 1 (§135) uses fake providers only; no real network/provider code.
//!
//! This module provides a deterministic, hardcoded fake media feed used to
//! exercise the Tauri command / frontend pipeline end-to-end before any real
//! provider integration exists. All titles, provider labels, and thumbnail
//! URLs below are synthetic placeholders (see repository CLAUDE.md "Content
//! boundaries") — no real provider, site, or piece of media is referenced.

use serde::{Deserialize, Serialize};

/// Mirrors `MediaKind` in `packages/normalized-types/src/index.ts`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaKind {
    Video,
    Image,
    Gif,
    Manga,
    Comic,
    Literature,
    Audio,
}

impl MediaKind {
    /// All variants, in a stable order — used by tests to assert coverage.
    pub const ALL: [MediaKind; 7] = [
        MediaKind::Video,
        MediaKind::Image,
        MediaKind::Gif,
        MediaKind::Manga,
        MediaKind::Comic,
        MediaKind::Literature,
        MediaKind::Audio,
    ];

    /// The wire string used by `MediaItemStub.kind` (and the TS `MediaKind` union).
    pub fn as_str(&self) -> &'static str {
        match self {
            MediaKind::Video => "video",
            MediaKind::Image => "image",
            MediaKind::Gif => "gif",
            MediaKind::Manga => "manga",
            MediaKind::Comic => "comic",
            MediaKind::Literature => "literature",
            MediaKind::Audio => "audio",
        }
    }
}

/// A single fake feed entry produced by the Phase 1 fake-provider generator.
///
/// Field shape mirrors `MediaItem` in `packages/normalized-types/src/index.ts`
/// (minus the fields Phase 1 doesn't need yet); the Tauri command layer in
/// `apps/desktop/src-tauri/src/commands/feed.rs` maps this into the
/// wire-serialized `MediaItemStub`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FakeMediaItem {
    pub uid: String,
    pub title: String,
    pub kind: MediaKind,
    pub thumbnail_url: Option<String>,
    pub provider_label: String,
    pub description: Option<String>,
}

/// Generic, non-network placeholder thumbnail. Points at a generic image
/// placeholder service (not any real content host) purely so the frontend
/// has something to render an `<img>` tag against; Phase 1 does no real
/// network fetching of its own regardless of what this string contains.
fn placeholder_thumbnail(seed: &str) -> String {
    format!("https://placehold.co/320x180?text={seed}")
}

/// One synthetic fake-provider catalog entry: (uid suffix, title, provider
/// label, has a thumbnail, description).
struct Entry {
    uid_suffix: &'static str,
    title: &'static str,
    provider_label: &'static str,
    has_thumbnail: bool,
    description: Option<&'static str>,
}

const VIDEO: &[Entry] = &[
    Entry { uid_suffix: "video-0001", title: "Lorem Ipsum Dolor Sit Amet Video", provider_label: "Fake Provider A", has_thumbnail: true, description: Some("Placeholder synopsis text for a synthetic sample video.") },
    Entry { uid_suffix: "video-0002", title: "Placeholder Video Sample One", provider_label: "Fake Provider B", has_thumbnail: true, description: None },
    Entry { uid_suffix: "video-0003", title: "Synthetic Test Clip Alpha", provider_label: "Fake Provider A", has_thumbnail: false, description: Some("Another placeholder description, generated for pipeline testing.") },
    Entry { uid_suffix: "video-0004", title: "Example Video Title Beta", provider_label: "Fake Provider C", has_thumbnail: true, description: None },
];

const IMAGE: &[Entry] = &[
    Entry { uid_suffix: "image-0001", title: "Placeholder Image Set One", provider_label: "Fake Provider B", has_thumbnail: true, description: None },
    Entry { uid_suffix: "image-0002", title: "Lorem Ipsum Gallery Sample", provider_label: "Fake Provider A", has_thumbnail: true, description: Some("Placeholder gallery description.") },
    Entry { uid_suffix: "image-0003", title: "Synthetic Photo Placeholder", provider_label: "Fake Provider C", has_thumbnail: false, description: None },
    Entry { uid_suffix: "image-0004", title: "Example Image Collection Two", provider_label: "Fake Provider B", has_thumbnail: true, description: None },
];

const GIF: &[Entry] = &[
    Entry { uid_suffix: "gif-0001", title: "Looping Placeholder Gif One", provider_label: "Fake Provider A", has_thumbnail: true, description: None },
    Entry { uid_suffix: "gif-0002", title: "Synthetic Animated Sample", provider_label: "Fake Provider C", has_thumbnail: true, description: Some("Placeholder animation description.") },
    Entry { uid_suffix: "gif-0003", title: "Example Gif Placeholder Two", provider_label: "Fake Provider B", has_thumbnail: false, description: None },
    Entry { uid_suffix: "gif-0004", title: "Lorem Ipsum Animation Sample", provider_label: "Fake Provider A", has_thumbnail: true, description: None },
];

const MANGA: &[Entry] = &[
    Entry { uid_suffix: "manga-0001", title: "Placeholder Manga Chapter One", provider_label: "Fake Provider C", has_thumbnail: true, description: Some("Placeholder chapter summary.") },
    Entry { uid_suffix: "manga-0002", title: "Synthetic Manga Sample Alpha", provider_label: "Fake Provider A", has_thumbnail: true, description: None },
    Entry { uid_suffix: "manga-0003", title: "Example Manga Volume Two", provider_label: "Fake Provider B", has_thumbnail: false, description: None },
    Entry { uid_suffix: "manga-0004", title: "Lorem Ipsum Manga Placeholder", provider_label: "Fake Provider C", has_thumbnail: true, description: None },
];

const COMIC: &[Entry] = &[
    Entry { uid_suffix: "comic-0001", title: "Placeholder Comic Issue One", provider_label: "Fake Provider A", has_thumbnail: true, description: None },
    Entry { uid_suffix: "comic-0002", title: "Synthetic Comic Sample Alpha", provider_label: "Fake Provider B", has_thumbnail: false, description: Some("Placeholder issue blurb.") },
    Entry { uid_suffix: "comic-0003", title: "Example Comic Placeholder Two", provider_label: "Fake Provider C", has_thumbnail: true, description: None },
];

const LITERATURE: &[Entry] = &[
    Entry { uid_suffix: "literature-0001", title: "Placeholder Story Chapter One", provider_label: "Fake Provider B", has_thumbnail: false, description: Some("Placeholder chapter excerpt text.") },
    Entry { uid_suffix: "literature-0002", title: "Synthetic Literature Sample", provider_label: "Fake Provider A", has_thumbnail: false, description: None },
    Entry { uid_suffix: "literature-0003", title: "Example Text Placeholder Two", provider_label: "Fake Provider C", has_thumbnail: false, description: None },
];

const AUDIO: &[Entry] = &[
    Entry { uid_suffix: "audio-0001", title: "Placeholder Audio Track One", provider_label: "Fake Provider C", has_thumbnail: false, description: None },
    Entry { uid_suffix: "audio-0002", title: "Synthetic Audio Sample Alpha", provider_label: "Fake Provider B", has_thumbnail: true, description: Some("Placeholder track description.") },
    Entry { uid_suffix: "audio-0003", title: "Example Audio Placeholder Two", provider_label: "Fake Provider A", has_thumbnail: false, description: None },
];

fn build(kind: MediaKind, entries: &[Entry]) -> Vec<FakeMediaItem> {
    entries
        .iter()
        .map(|e| FakeMediaItem {
            uid: format!("fake:{}", e.uid_suffix),
            title: e.title.to_string(),
            kind,
            thumbnail_url: e.has_thumbnail.then(|| placeholder_thumbnail(kind.as_str())),
            provider_label: e.provider_label.to_string(),
            description: e.description.map(str::to_string),
        })
        .collect()
}

/// Deterministic fake media feed for Phase 1 (kelpie.md §135). No randomness,
/// no network access — a fixed, hardcoded catalog of synthetic placeholder
/// items spanning every `MediaKind`, so downstream consumers (the Tauri
/// command layer, the frontend feed view) can be exercised end-to-end before
/// any real provider integration exists.
pub fn fake_feed() -> Vec<FakeMediaItem> {
    let mut items = Vec::new();
    items.extend(build(MediaKind::Video, VIDEO));
    items.extend(build(MediaKind::Image, IMAGE));
    items.extend(build(MediaKind::Gif, GIF));
    items.extend(build(MediaKind::Manga, MANGA));
    items.extend(build(MediaKind::Comic, COMIC));
    items.extend(build(MediaKind::Literature, LITERATURE));
    items.extend(build(MediaKind::Audio, AUDIO));
    items
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn produces_a_reasonable_number_of_items() {
        let items = fake_feed();
        assert!(
            (15..=30).contains(&items.len()),
            "expected 15-30 fake feed items, got {}",
            items.len()
        );
    }

    #[test]
    fn every_item_has_non_empty_required_fields() {
        for item in fake_feed() {
            assert!(!item.uid.is_empty(), "uid must not be empty");
            assert!(!item.title.trim().is_empty(), "title must not be empty");
            assert!(
                !item.provider_label.trim().is_empty(),
                "provider_label must not be empty"
            );
            if let Some(url) = &item.thumbnail_url {
                assert!(!url.trim().is_empty(), "thumbnail_url, if set, must not be empty");
            }
        }
    }

    #[test]
    fn uids_are_unique() {
        let items = fake_feed();
        let unique: HashSet<&str> = items.iter().map(|i| i.uid.as_str()).collect();
        assert_eq!(unique.len(), items.len(), "uids must be unique");
    }

    #[test]
    fn covers_every_media_kind() {
        let items = fake_feed();
        let kinds_present: HashSet<MediaKind> = items.iter().map(|i| i.kind).collect();
        for kind in MediaKind::ALL {
            assert!(
                kinds_present.contains(&kind),
                "expected fake feed to cover MediaKind::{kind:?}"
            );
        }
    }

    #[test]
    fn covers_several_distinct_provider_labels() {
        let items = fake_feed();
        let providers: HashSet<&str> = items.iter().map(|i| i.provider_label.as_str()).collect();
        assert!(
            providers.len() >= 2,
            "expected at least 2 distinct provider labels, got {}",
            providers.len()
        );
    }

    #[test]
    fn media_kind_as_str_matches_serde_lowercase_rename() {
        for kind in MediaKind::ALL {
            let json = serde_json::to_string(&kind).unwrap();
            assert_eq!(json, format!("\"{}\"", kind.as_str()));
        }
    }

    #[test]
    fn is_deterministic_across_calls() {
        assert_eq!(fake_feed(), fake_feed());
    }

    #[test]
    fn no_real_adult_site_placeholder_hosts() {
        // Content-boundary guard: thumbnail URLs must never point at a real
        // third-party host, only at the generic placeholder image service.
        for item in fake_feed() {
            if let Some(url) = &item.thumbnail_url {
                assert!(
                    url.starts_with("https://placehold.co/"),
                    "unexpected thumbnail host: {url}"
                );
            }
        }
    }
}
