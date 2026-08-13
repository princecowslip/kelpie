//! Provider manifest parsing and validation (kelpie.md §137 Phase 3 item 1,
//! §25 Provider Manifest; see `docs/providers/MANIFEST.md` for the
//! developer-facing reference this module implements in Rust).
//!
//! A manifest is read before any provider code runs, so this module only
//! concerns itself with the manifest's own declarative shape: JSON structure
//! plus the field-level rules kelpie.md §25 implies (non-empty identifiers, a
//! recognized schema version, a well-formed version string, origins that are
//! actually origins). It deliberately does **not** cross-check `capabilities`
//! against a fixed vocabulary (kelpie.md §24) or validate `origins` against
//! live network activity (kelpie.md §28) — those are the "capability system"
//! and "provider permissions" Phase 3 units, landing separately.

use std::collections::HashSet;
use std::fmt;

use serde::{Deserialize, Serialize};

/// The only `schema` value this parser currently understands (kelpie.md §25).
const SUPPORTED_SCHEMA: u32 = 1;

/// A parsed provider manifest, matching the JSON shape in kelpie.md §25.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderManifest {
    pub schema: u32,

    pub id: String,
    pub name: String,

    pub version: String,
    #[serde(rename = "providerApi")]
    pub provider_api: String,

    #[serde(rename = "siteType")]
    pub site_type: String,

    #[serde(rename = "contentKinds")]
    pub content_kinds: Vec<String>,

    pub capabilities: Vec<String>,

    pub origins: Vec<String>,

    pub authentication: String,
}

/// One violated validation rule, with a human-readable message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError(pub String);

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to parse provider manifest JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("provider manifest failed validation: {}", join_validation_errors(.0))]
    Validation(Vec<ValidationError>),
}

fn join_validation_errors(errors: &[ValidationError]) -> String {
    errors
        .iter()
        .map(|e| e.0.as_str())
        .collect::<Vec<_>>()
        .join("; ")
}

pub type Result<T> = std::result::Result<T, Error>;

/// `id` is used as a durable key elsewhere (registry, DB `providers` table in a
/// later unit), so it's restricted to a stable identifier charset rather than
/// allowed to be arbitrary text.
fn is_valid_id(id: &str) -> bool {
    !id.is_empty()
        && id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
}

/// A light manual check for "looks like a version string" — numeric,
/// dot-separated segments (e.g. `1`, `1.2`, `1.2.0`). Not full semver: this
/// repo doesn't otherwise depend on the `semver` crate, and the spec's own
/// example values are this simple.
fn is_valid_version(version: &str) -> bool {
    !version.is_empty()
        && version
            .split('.')
            .all(|segment| !segment.is_empty() && segment.chars().all(|c| c.is_ascii_digit()))
}

/// `origins` entries are origins, not URLs: `scheme://host` with no path,
/// query, or fragment. Implemented with plain `str` parsing rather than a new
/// `url` crate dependency, since the check is narrow.
fn is_valid_origin(origin: &str) -> bool {
    let Some(rest) = origin
        .strip_prefix("https://")
        .or_else(|| origin.strip_prefix("http://"))
    else {
        return false;
    };

    !rest.is_empty() && !rest.contains(['/', '?', '#'])
}

fn push_if_empty(errors: &mut Vec<ValidationError>, field: &str, value: &str) {
    if value.trim().is_empty() {
        errors.push(ValidationError(format!("`{field}` must not be empty")));
    }
}

/// Validate a list-of-strings field is non-empty and contains no empty or
/// duplicate entries.
fn validate_string_list(errors: &mut Vec<ValidationError>, field: &str, values: &[String]) {
    if values.is_empty() {
        errors.push(ValidationError(format!("`{field}` must not be empty")));
        return;
    }

    if values.iter().any(|v| v.trim().is_empty()) {
        errors.push(ValidationError(format!(
            "`{field}` must not contain empty entries"
        )));
    }

    let unique: HashSet<&str> = values.iter().map(String::as_str).collect();
    if unique.len() != values.len() {
        errors.push(ValidationError(format!(
            "`{field}` must not contain duplicate entries"
        )));
    }
}

impl ProviderManifest {
    /// Parse and validate a provider manifest from raw JSON text.
    ///
    /// Returns [`Error::Json`] if the input isn't valid JSON (or doesn't match
    /// the expected shape at all), or [`Error::Validation`] — carrying every
    /// violated rule, not just the first — if it parses but fails one or more
    /// field-level checks.
    pub fn parse(raw: &str) -> Result<Self> {
        let manifest: ProviderManifest = serde_json::from_str(raw)?;
        manifest.validate()?;
        Ok(manifest)
    }

    /// Run field-level validation against an already-deserialized manifest.
    pub fn validate(&self) -> Result<()> {
        let mut errors = Vec::new();

        if self.schema != SUPPORTED_SCHEMA {
            errors.push(ValidationError(format!(
                "unsupported `schema` version {} (expected {SUPPORTED_SCHEMA})",
                self.schema
            )));
        }

        if !is_valid_id(&self.id) {
            errors.push(ValidationError(
                "`id` must be non-empty and contain only lowercase ASCII letters, digits, `-`, or `_`"
                    .to_string(),
            ));
        }

        push_if_empty(&mut errors, "name", &self.name);

        if !is_valid_version(&self.version) {
            errors.push(ValidationError(
                "`version` must be a non-empty, dot-separated numeric version string (e.g. `1.2.0`)"
                    .to_string(),
            ));
        }

        if !is_valid_version(&self.provider_api) {
            errors.push(ValidationError(
                "`providerApi` must be a non-empty, dot-separated numeric version string"
                    .to_string(),
            ));
        }

        push_if_empty(&mut errors, "siteType", &self.site_type);
        push_if_empty(&mut errors, "authentication", &self.authentication);

        validate_string_list(&mut errors, "contentKinds", &self.content_kinds);
        validate_string_list(&mut errors, "capabilities", &self.capabilities);
        validate_string_list(&mut errors, "origins", &self.origins);

        if let Some(bad_origin) = self.origins.iter().find(|o| !is_valid_origin(o)) {
            errors.push(ValidationError(format!(
                "`origins` entry {bad_origin:?} must be an `http://` or `https://` origin with no path, query, or fragment"
            )));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(Error::Validation(errors))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The exact example manifest from kelpie.md §25.
    const SPEC_EXAMPLE: &str = r#"{
  "schema": 1,

  "id": "example",
  "name": "Example",

  "version": "1.2.0",
  "providerApi": "1",

  "siteType": "manga",

  "contentKinds": [
    "manga",
    "illustration"
  ],

  "capabilities": [
    "feed",
    "search",
    "series",
    "chapters",
    "pages",
    "browser"
  ],

  "origins": [
    "https://example.com",
    "https://cdn.example.com"
  ],

  "authentication": "browser-cookie"
}"#;

    #[test]
    fn spec_example_manifest_parses_and_round_trips_every_field() {
        let manifest = ProviderManifest::parse(SPEC_EXAMPLE).expect("spec example should parse");

        assert_eq!(manifest.schema, 1);
        assert_eq!(manifest.id, "example");
        assert_eq!(manifest.name, "Example");
        assert_eq!(manifest.version, "1.2.0");
        assert_eq!(manifest.provider_api, "1");
        assert_eq!(manifest.site_type, "manga");
        assert_eq!(manifest.content_kinds, vec!["manga", "illustration"]);
        assert_eq!(
            manifest.capabilities,
            vec!["feed", "search", "series", "chapters", "pages", "browser"]
        );
        assert_eq!(
            manifest.origins,
            vec!["https://example.com", "https://cdn.example.com"]
        );
        assert_eq!(manifest.authentication, "browser-cookie");
    }

    #[test]
    fn spec_example_manifest_reserializes_with_original_wire_field_names() {
        let manifest = ProviderManifest::parse(SPEC_EXAMPLE).expect("spec example should parse");
        let value = serde_json::to_value(&manifest).expect("serialize");

        for key in [
            "schema",
            "id",
            "name",
            "version",
            "providerApi",
            "siteType",
            "contentKinds",
            "capabilities",
            "origins",
            "authentication",
        ] {
            assert!(value.get(key).is_some(), "missing wire field `{key}`");
        }
    }

    #[test]
    fn malformed_json_is_a_json_error_not_a_validation_error() {
        let err = ProviderManifest::parse("{ not json").unwrap_err();
        assert!(
            matches!(err, Error::Json(_)),
            "expected Error::Json, got {err:?}"
        );
    }

    fn valid_manifest_json() -> serde_json::Value {
        serde_json::from_str(SPEC_EXAMPLE).unwrap()
    }

    fn expect_validation_error(mut value: serde_json::Value, field: &str, bad: serde_json::Value) {
        value[field] = bad;
        let raw = serde_json::to_string(&value).unwrap();
        let err = ProviderManifest::parse(&raw).unwrap_err();
        match err {
            Error::Validation(errors) => {
                assert!(
                    !errors.is_empty(),
                    "expected at least one validation error for field `{field}`"
                );
            }
            other => panic!("expected Error::Validation for field `{field}`, got {other:?}"),
        }
    }

    #[test]
    fn rejects_unsupported_schema_version() {
        expect_validation_error(valid_manifest_json(), "schema", serde_json::json!(2));
    }

    #[test]
    fn rejects_empty_id() {
        expect_validation_error(valid_manifest_json(), "id", serde_json::json!(""));
    }

    #[test]
    fn rejects_id_with_uppercase_or_invalid_characters() {
        expect_validation_error(
            valid_manifest_json(),
            "id",
            serde_json::json!("Example Site!"),
        );
    }

    #[test]
    fn rejects_empty_name() {
        expect_validation_error(valid_manifest_json(), "name", serde_json::json!("   "));
    }

    #[test]
    fn rejects_malformed_version() {
        expect_validation_error(
            valid_manifest_json(),
            "version",
            serde_json::json!("v1.2.0"),
        );
    }

    #[test]
    fn rejects_malformed_provider_api() {
        expect_validation_error(
            valid_manifest_json(),
            "providerApi",
            serde_json::json!("latest"),
        );
    }

    #[test]
    fn rejects_empty_site_type() {
        expect_validation_error(valid_manifest_json(), "siteType", serde_json::json!(""));
    }

    #[test]
    fn rejects_empty_authentication() {
        expect_validation_error(
            valid_manifest_json(),
            "authentication",
            serde_json::json!(""),
        );
    }

    #[test]
    fn rejects_empty_content_kinds() {
        expect_validation_error(valid_manifest_json(), "contentKinds", serde_json::json!([]));
    }

    #[test]
    fn rejects_content_kinds_with_empty_entry() {
        expect_validation_error(
            valid_manifest_json(),
            "contentKinds",
            serde_json::json!(["manga", ""]),
        );
    }

    #[test]
    fn rejects_duplicate_content_kinds() {
        expect_validation_error(
            valid_manifest_json(),
            "contentKinds",
            serde_json::json!(["manga", "manga"]),
        );
    }

    #[test]
    fn rejects_empty_capabilities() {
        expect_validation_error(valid_manifest_json(), "capabilities", serde_json::json!([]));
    }

    #[test]
    fn rejects_duplicate_capabilities() {
        expect_validation_error(
            valid_manifest_json(),
            "capabilities",
            serde_json::json!(["feed", "feed"]),
        );
    }

    #[test]
    fn rejects_empty_origins() {
        expect_validation_error(valid_manifest_json(), "origins", serde_json::json!([]));
    }

    #[test]
    fn rejects_origin_without_scheme() {
        expect_validation_error(
            valid_manifest_json(),
            "origins",
            serde_json::json!(["example.com"]),
        );
    }

    #[test]
    fn rejects_origin_with_a_path() {
        expect_validation_error(
            valid_manifest_json(),
            "origins",
            serde_json::json!(["https://example.com/feed"]),
        );
    }

    #[test]
    fn rejects_origin_with_a_query_string() {
        expect_validation_error(
            valid_manifest_json(),
            "origins",
            serde_json::json!(["https://example.com?x=1"]),
        );
    }

    #[test]
    fn accepts_http_origin_for_local_or_dev_sources() {
        let mut value = valid_manifest_json();
        value["origins"] = serde_json::json!(["http://localhost:8080"]);
        let raw = serde_json::to_string(&value).unwrap();
        ProviderManifest::parse(&raw).expect("http origin should be accepted");
    }

    #[test]
    fn manifest_with_multiple_violations_reports_all_of_them() {
        let mut value = valid_manifest_json();
        value["schema"] = serde_json::json!(2);
        value["id"] = serde_json::json!("");
        value["name"] = serde_json::json!("");
        let raw = serde_json::to_string(&value).unwrap();

        let err = ProviderManifest::parse(&raw).unwrap_err();
        match err {
            Error::Validation(errors) => {
                assert!(
                    errors.len() >= 3,
                    "expected at least 3 validation errors, got {}: {errors:?}",
                    errors.len()
                );
            }
            other => panic!("expected Error::Validation, got {other:?}"),
        }
    }

    #[test]
    fn error_display_joins_every_validation_message() {
        let mut value = valid_manifest_json();
        value["id"] = serde_json::json!("");
        value["name"] = serde_json::json!("");
        let raw = serde_json::to_string(&value).unwrap();

        let err = ProviderManifest::parse(&raw).unwrap_err();
        let message = err.to_string();
        assert!(
            message.contains("id"),
            "message should mention `id`: {message}"
        );
        assert!(
            message.contains("name"),
            "message should mention `name`: {message}"
        );
    }
}
