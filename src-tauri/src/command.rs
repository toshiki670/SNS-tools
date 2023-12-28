use log::{debug, error, info};
use serde_json::{json, Value};
use tauri::command;

use crate::settings;

fn i18n_path(fn_name: &str, name: &str) -> String {
    format!("tauri.{}.{}", fn_name, name)
}

#[command]
pub async fn submit_settings() -> Result<Value, String> {
    let settings = settings::Settings::new();

    if let Ok(_) = settings.submit() {
        Ok(json!({"body": "abc"}))
    } else {
        Err(i18n_path("submit_settings", "failed"))
    }
}

#[command]
pub fn get_settings() -> settings::Settings {
    let settings = settings::Settings::new();
    settings
}

#[command]
pub async fn update_settings(settings: settings::Settings) -> Result<String, String> {
    let mut current = settings::Settings::new();
    current.update(settings);

    match current.submit() {
        Ok(_) => Ok(i18n_path("update_settings", "success")),
        Err(e) => {
            error!("{}", e);
            Err(i18n_path("update_settings", "failed"))
        }
    }
}

#[command]
pub fn validate_system_current_password(current: &str) -> bool {
    debug!("{}", current);
    debug!("{}", current == "asdfASDF124!@#$");
    current == "asdfASDF124!@#$"
}

#[command]
pub fn update_system_password(current: &str, password: &str, confirm: &str) -> String {
    if validate_system_current_password(current) == false {
        unreachable!();
    }

    info!("{}", current);
    info!("{}", password);
    info!("{}", confirm);

    i18n_path("update_system_password", "success")
}
