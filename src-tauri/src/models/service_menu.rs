use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ServiceMenuAction {
    pub name: String,
    pub icon: String,
    pub exec: String,

    #[serde(rename = "type")]
    pub entry_type: String,

    #[serde(rename = "mimeType")]
    pub mime_type: String,
}

#[derive(Debug, Serialize)]
pub struct AppMetadata {
    pub name: Option<String>,
    pub icon: Option<String>,
    pub exec: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OperationStep {
    pub name: String,

    // true  = успешно
    // false = ошибка
    // null  = информационный шаг
    pub success: Option<bool>,

    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct CreateDesktopActionResult {
    pub success: bool,
    pub steps: Vec<OperationStep>,
}