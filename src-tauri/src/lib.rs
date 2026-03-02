mod commands;
mod db;
mod error;
mod ollama;
mod state;
mod vigil;

use tauri::Manager;
use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            // Set up vault path in user's home directory
            let home = dirs_next::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
            let vault_path = home.join(".noctuary").join("vault");

            let app_state =
                AppState::new(vault_path).expect("Failed to initialize Noctuary state");

            // Clone Arc handles for the Vigil before managing state
            let vigil_db = app_state.db.clone();
            let vigil_status = app_state.vigil_status.clone();

            app.manage(app_state);

            // Spawn the Vigil background task
            vigil::spawn_vigil(vigil_db, vigil_status);

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Offerings
            commands::offerings::create_offering,
            commands::offerings::create_offering_from_url,
            commands::offerings::create_offering_from_file,
            commands::offerings::list_offerings,
            commands::offerings::delete_offering,
            commands::offerings::get_offering_count,
            // Vault
            commands::vault::list_vault_items,
            commands::vault::get_vault_item,
            commands::vault::delete_vault_item,
            commands::vault::get_all_tags,
            commands::vault::add_user_tag,
            commands::vault::remove_tag,
            // Settings
            commands::settings::get_setting,
            commands::settings::set_setting,
            commands::settings::get_all_settings,
            commands::settings::get_vault_path,
            commands::settings::check_ollama,
            // Vigil
            commands::vigil::get_vigil_status,
            // Librarian
            commands::librarian::ask_librarian,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Noctuary");
}
