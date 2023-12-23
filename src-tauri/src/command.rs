use log::{error, info};
use serde_json::{json, Value};
use tauri::command;

use crate::settings;

fn i18n_path(fn_name: &str, name: &str) -> String {
    format!("tauri.{}.{}", fn_name, name)
}

#[command]
pub async fn submit_settings() -> Result<Value, String> {
    let settings = settings::Setting::new();

    if let Ok(_) = settings.submit() {
        Ok(json!({"body": "abc"}))
    } else {
        Err(i18n_path("submit_settings", "failed"))
    }
}

#[command]
pub fn validate_system_current_password(current: &str) -> bool {
    true
}

#[command]
pub fn update_system_password(current: &str, password: &str, confirm: &str) -> String {
    if false == validate_system_current_password(current) {
        unreachable!();
    }

    info!("{}", current);
    info!("{}", password);
    info!("{}", confirm);

    i18n_path("update_system_password", "success")
}
