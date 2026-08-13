//! Settings command handlers (kelpie.md §130 Settings). Owned by Stage B unit 1
//! (frontend shell) — reads/writes a `settings.toml` file under
//! `kelpie_core::paths::config_dir()`.

use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub theme: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: "system".to_string(),
        }
    }
}

const SETTINGS_FILE_NAME: &str = "settings.toml";

fn settings_path() -> Result<PathBuf, String> {
    kelpie_core::paths::config_dir()
        .map(|dir| dir.join(SETTINGS_FILE_NAME))
        .map_err(|err| format!("failed to resolve config directory: {err}"))
}

/// Reads `settings.toml`, falling back to defaults if the file is missing or
/// unreadable/unparseable (e.g. corrupted by hand-editing).
#[tauri::command]
pub fn get_settings() -> Settings {
    let Ok(path) = settings_path() else {
        return Settings::default();
    };

    match fs::read_to_string(&path) {
        Ok(contents) => toml::from_str(&contents).unwrap_or_default(),
        Err(_) => Settings::default(),
    }
}

/// Writes `settings.toml`, overwriting any previous contents.
#[tauri::command]
pub fn set_settings(settings: Settings) -> Result<(), String> {
    let path = settings_path()?;
    let contents =
        toml::to_string_pretty(&settings).map_err(|err| format!("failed to serialize settings: {err}"))?;
    fs::write(&path, contents).map_err(|err| format!("failed to write {path:?}: {err}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_theme_is_system() {
        assert_eq!(Settings::default().theme, "system");
    }

    #[test]
    fn settings_round_trip_through_toml() {
        let settings = Settings {
            theme: "dark".to_string(),
        };
        let serialized = toml::to_string_pretty(&settings).unwrap();
        let deserialized: Settings = toml::from_str(&serialized).unwrap();
        assert_eq!(deserialized.theme, "dark");
    }
}
