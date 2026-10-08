mod commands;
mod models;

use commands::app_metadata::get_app_metadata;
use commands::service_menu::create_desktop_action;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_app_metadata,
            create_desktop_action
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}