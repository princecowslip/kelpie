//! Settings command handlers (kelpie.md §130 Settings). Owned by Stage B unit 1
//! (frontend shell) — this stub returns defaults with no persistence.

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

#[tauri::command]
pub fn get_settings() -> Settings {
    Settings::default()
}

#[tauri::command]
pub fn set_settings(_settings: Settings) -> Result<(), String> {
    Ok(())
}
