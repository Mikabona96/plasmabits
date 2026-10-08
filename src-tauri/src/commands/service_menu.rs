use std::fs;
use std::path::{Path, PathBuf};

use crate::models::service_menu::{
    CreateDesktopActionResult,
    OperationStep,
    ServiceMenuAction,
};

#[tauri::command]
pub fn create_desktop_action(
    action: ServiceMenuAction,
) -> CreateDesktopActionResult {
    let mut steps = Vec::new();

    let home = match dirs::home_dir() {
        Some(home) => home,
        None => {
            steps.push(OperationStep {
                name: "Get home directory".to_string(),
                success: Some(false),
                message: "Could not determine home directory".to_string(),
            });

            return CreateDesktopActionResult {
                success: false,
                steps,
            };
        }
    };

    steps.push(OperationStep {
        name: "Get home directory".to_string(),
        success: Some(true),
        message: format!(
            "Home directory: {}",
            home.display()
        ),
    });

    // --------------------------------------------------
    // Service Menu directory
    // --------------------------------------------------

    let servicemenus_dir = home.join(".local/share/kio/servicemenus");

    if servicemenus_dir.exists() {
        steps.push(OperationStep {
            name: "Check service menu directory".to_string(),
            success: Some(true),
            message: format!(
                "Folder already exists: {}",
                servicemenus_dir.display()
            ),
        });

        steps.push(OperationStep {
            name: "Create service menu directory".to_string(),
            success: None,
            message: "Folder already exists, creation skipped".to_string(),
        });
    } else {
        steps.push(OperationStep {
            name: "Check service menu directory".to_string(),
            success: None,
            message: format!(
                "Folder does not exist: {}, creating the folder",
                servicemenus_dir.display()
            ),
        });

        if let Err(error) = fs::create_dir_all(&servicemenus_dir) {
            steps.push(OperationStep {
                name: "Create service menu directory".to_string(),
                success: Some(false),
                message: format!(
                    "Failed to create folder: {error}"
                ),
            });

            return CreateDesktopActionResult {
                success: false,
                steps,
            };
        }

        steps.push(OperationStep {
            name: "Create service menu directory".to_string(),
            success: Some(true),
            message: format!(
                "Folder created successfully: {}",
                servicemenus_dir.display()
            ),
        });
    }

    // --------------------------------------------------
    // File name
    // --------------------------------------------------

    let file_name = create_file_name(&action.name);

    let desktop_file = servicemenus_dir.join(
        format!("{file_name}.desktop")
    );

    steps.push(OperationStep {
        name: "Generate desktop file name".to_string(),
        success: Some(true),
        message: format!(
            "Generated file name: {}",
            desktop_file.display()
        ),
    });

    // --------------------------------------------------
    // Icon
    // --------------------------------------------------

    let icon = match prepare_icon(
        &home,
        &action.icon,
        &file_name,
        &mut steps,
    ) {
        Ok(icon) => icon,
        Err(()) => {
            return CreateDesktopActionResult {
                success: false,
                steps,
            };
        }
    };

    // --------------------------------------------------
    // Desktop file content
    // --------------------------------------------------

    fn normalize_list(value: &str) -> String {
        value
            .split(|character: char| character == ';' || character.is_whitespace())
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join(";")
    }

    let mime_type = normalize_list(&action.mime_type);
    let action_id = create_file_name(&action.name);

    let desktop_content = format!(
        "[Desktop Entry]\n\
        Type={}\n\
        X-KDE-ServiceTypes=KonqPopupMenu/Plugin\n\
        MimeType={};\n\
        Actions={};\n\
        X-KDE-Priority=TopLevel\n\
        \n\
        [Desktop Action {}]\n\
        Name={}\n\
        Icon={}\n\
        Exec={}\n",
        action.entry_type,
        mime_type,
        action_id,
        action_id,
        action.name,
        icon,
        action.exec,
    );

    // --------------------------------------------------
    // Create desktop file
    // --------------------------------------------------

    if let Err(error) = fs::write(
        &desktop_file,
        desktop_content,
    ) {
        steps.push(OperationStep {
            name: "Create desktop file".to_string(),
            success: Some(false),
            message: format!(
                "Failed to create {}: {error}",
                desktop_file.display()
            ),
        });

        return CreateDesktopActionResult {
            success: false,
            steps,
        };
    }

    steps.push(OperationStep {
        name: "Create desktop file".to_string(),
        success: Some(true),
        message: format!(
            "Created: {}",
            desktop_file.display()
        ),
    });

    // --------------------------------------------------
    // Make desktop file executable
    // --------------------------------------------------

    let mut permissions = match fs::metadata(&desktop_file) {
        Ok(metadata) => metadata.permissions(),

        Err(error) => {
            steps.push(OperationStep {
                name: "Read desktop file permissions".to_string(),
                success: Some(false),
                message: format!(
                    "Failed to read file permissions: {error}"
                ),
            });

            return CreateDesktopActionResult {
                success: false,
                steps,
            };
        }
    };

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        permissions.set_mode(0o755);

        if let Err(error) = fs::set_permissions(
            &desktop_file,
            permissions,
        ) {
            steps.push(OperationStep {
                name: "Make desktop file executable".to_string(),
                success: Some(false),
                message: format!(
                    "Failed to make desktop file executable: {error}"
                ),
            });

            return CreateDesktopActionResult {
                success: false,
                steps,
            };
        }
    }

    steps.push(OperationStep {
        name: "Make desktop file executable".to_string(),
        success: Some(true),
        message: "Desktop file is now executable".to_string(),
    });

    // --------------------------------------------------
    // Success
    // --------------------------------------------------

    CreateDesktopActionResult {
        success: true,
        steps,
    }
}

// --------------------------------------------------
// Helpers
// --------------------------------------------------

fn create_file_name(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|character| {
            if character.is_alphanumeric() {
                character
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim_matches('_')
        .to_string()
}

fn prepare_icon(
    home: &Path,
    icon: &str,
    file_name: &str,
    steps: &mut Vec<OperationStep>,
) -> Result<String, ()> {
    let icon_path = PathBuf::from(icon);

    // --------------------------------------------------
    // Check whether icon is a custom file
    // --------------------------------------------------

    if !icon_path.is_absolute() || !icon_path.is_file() {
        steps.push(OperationStep {
            name: "Check custom icon".to_string(),
            success: None,
            message: format!(
                "Icon \"{}\" is a system icon, no copy required",
                icon
            ),
        });

        return Ok(icon.to_string());
    }

    steps.push(OperationStep {
        name: "Check custom icon".to_string(),
        success: Some(true),
        message: format!(
            "Custom icon found: {}",
            icon_path.display()
        ),
    });

    // --------------------------------------------------
    // Validate extension
    // --------------------------------------------------

    let extension = match icon_path
        .extension()
        .and_then(|extension| extension.to_str())
    {
        Some(extension) => extension.to_ascii_lowercase(),

        None => {
            steps.push(OperationStep {
                name: "Check custom icon format".to_string(),
                success: Some(false),
                message: "Icon has no file extension".to_string(),
            });

            return Err(());
        }
    };

    if extension != "svg" && extension != "png" {
        steps.push(OperationStep {
            name: "Check custom icon format".to_string(),
            success: Some(false),
            message: format!(
                "Unsupported icon format: .{extension}. Only SVG and PNG are supported"
            ),
        });

        return Err(());
    }

    steps.push(OperationStep {
        name: "Check custom icon format".to_string(),
        success: Some(true),
        message: format!(
            "Icon format .{extension} is supported"
        ),
    });

    // --------------------------------------------------
    // Icons directory
    // --------------------------------------------------

    let icons_dir = home.join(
        ".local/share/icons/hicolor/scalable/apps"
    );

    if icons_dir.exists() {
        steps.push(OperationStep {
            name: "Check icons directory".to_string(),
            success: Some(true),
            message: format!(
                "Folder already exists: {}",
                icons_dir.display()
            ),
        });

        steps.push(OperationStep {
            name: "Create icons directory".to_string(),
            success: None,
            message: "Folder already exists, creation skipped".to_string(),
        });
    } else {
        steps.push(OperationStep {
            name: "Check icons directory".to_string(),
            success: None,
            message: format!(
                "Folder does not exist: {}, creating the folder",
                icons_dir.display()
            ),
        });

        if let Err(error) = fs::create_dir_all(&icons_dir) {
            steps.push(OperationStep {
                name: "Create icons directory".to_string(),
                success: Some(false),
                message: format!(
                    "Failed to create folder: {error}"
                ),
            });

            return Err(());
        }

        steps.push(OperationStep {
            name: "Create icons directory".to_string(),
            success: Some(true),
            message: format!(
                "Folder created successfully: {}",
                icons_dir.display()
            ),
        });
    }

    // --------------------------------------------------
    // Copy icon
    // --------------------------------------------------

    let destination = icons_dir.join(
        format!("{file_name}.{extension}")
    );

    if let Err(error) = fs::copy(
        &icon_path,
        &destination,
    ) {
        steps.push(OperationStep {
            name: "Copy custom icon".to_string(),
            success: Some(false),
            message: format!(
                "Failed to copy icon: {error}"
            ),
        });

        return Err(());
    }

    steps.push(OperationStep {
        name: "Copy custom icon".to_string(),
        success: Some(true),
        message: format!(
            "Copied icon to: {}",
            destination.display()
        ),
    });

    Ok(destination.to_string_lossy().to_string())
}