//! Kelpie desktop Tauri application entry point (kelpie.md §135 Phase 1: Desktop
//! Foundation). This file and `commands/mod.rs` are frozen for the duration of
//! Phase 1 — new commands register here, but the wiring itself does not change;
//! Stage B units only edit the bodies of their own `commands/<domain>.rs` files.

mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            commands::db::setup(app)?;
            commands::logging::setup(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::settings::get_settings,
            commands::settings::set_settings,
            commands::db::health_check,
            commands::logging::get_log_dir,
            commands::feed::get_fake_feed,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
