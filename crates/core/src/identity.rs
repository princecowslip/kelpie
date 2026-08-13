//! Stable identity (kelpie.md §17 Stable Identity).
//!
//! Preferred identity: `provider-id:object-type:remote-id`. Fallback (when a
//! provider doesn't expose a remote id): `SHA256(provider-id + canonical-url)`.
//! Title alone must never be treated as identity.

use sha2::{Digest, Sha256};

/// Compute a stable uid for a provider-sourced object, per §17.
///
/// `object_type` is a short, stable label for what's being identified (e.g.
/// `"item"`, `"series"`, `"chapter"`, `"creator"`, `"tag"`) — it exists so the
/// same `remote_id` from a provider can't collide across unrelated object
/// kinds.
pub fn stable_id(
    provider_id: &str,
    object_type: &str,
    remote_id: Option<&str>,
    canonical_url: &str,
) -> String {
    match remote_id {
        Some(remote_id) if !remote_id.is_empty() => {
            format!("{provider_id}:{object_type}:{remote_id}")
        }
        _ => {
            let mut hasher = Sha256::new();
            hasher.update(provider_id.as_bytes());
            hasher.update(canonical_url.as_bytes());
            format!("{:x}", hasher.finalize())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefers_provider_object_type_remote_id() {
        let id = stable_id(
            "provider-x",
            "manga-chapter",
            Some("38172"),
            "https://example.test/x",
        );
        assert_eq!(id, "provider-x:manga-chapter:38172");
    }

    #[test]
    fn falls_back_to_sha256_when_remote_id_missing() {
        let id = stable_id("provider-x", "item", None, "https://example.test/x");
        assert_eq!(id.len(), 64);
        assert!(id.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn falls_back_to_sha256_when_remote_id_empty() {
        let id = stable_id("provider-x", "item", Some(""), "https://example.test/x");
        assert_eq!(id.len(), 64);
    }

    #[test]
    fn fallback_is_deterministic() {
        let a = stable_id("provider-x", "item", None, "https://example.test/x");
        let b = stable_id("provider-x", "item", None, "https://example.test/x");
        assert_eq!(a, b);
    }

    #[test]
    fn fallback_differs_by_canonical_url() {
        let a = stable_id("provider-x", "item", None, "https://example.test/a");
        let b = stable_id("provider-x", "item", None, "https://example.test/b");
        assert_ne!(a, b);
    }

    #[test]
    fn object_type_prevents_cross_kind_collisions() {
        let a = stable_id("provider-x", "item", Some("1"), "https://example.test/x");
        let b = stable_id("provider-x", "series", Some("1"), "https://example.test/x");
        assert_ne!(a, b);
    }
}
