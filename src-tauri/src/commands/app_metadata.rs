use std::fs;

use crate::models::service_menu::AppMetadata;

#[tauri::command]
pub fn get_app_metadata(desktop_file: String) -> Option<AppMetadata> {
    let content = fs::read_to_string(desktop_file).ok()?;

    let mut in_desktop_entry = false;

    let mut name = None;
    let mut icon = None;
    let mut exec = None;

    for line in content.lines() {
        let line = line.trim();

        if line.starts_with('[') && line.ends_with(']') {
            in_desktop_entry = line == "[Desktop Entry]";
            continue;
        }

        if !in_desktop_entry {
            continue;
        }

        if let Some(value) = line.strip_prefix("Name=") {
            name = Some(value.to_string());
        } else if let Some(value) = line.strip_prefix("Icon=") {
            icon = Some(value.to_string());
        } else if let Some(value) = line.strip_prefix("Exec=") {
            let value = value.trim();

            let command = value
                .split_whitespace()
                .next()
                .unwrap_or("");

            if !command.is_empty() {
                exec = Some(command.to_string());
            }
        }
    }

    Some(AppMetadata {
        name,
        icon,
        exec,
    })
}