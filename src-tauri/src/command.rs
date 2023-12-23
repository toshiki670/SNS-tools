use log::error;
use serde_json::{json, Value};
use tauri::command;

use crate::settings;

fn error_i18n(fn_name: &str, name: &str) -> String {
    format!("tauri.{}.{}", fn_name, name)
}

#[command]
pub async fn submit_settings() -> Result<Value, String> {
    let settings = settings::Setting::new();

    if let Ok(_) = settings.submit() {
        Ok(json!({"body": "abc"}))
    } else {
        Err(error_i18n("submit_settings", "failed"))
    }
}
