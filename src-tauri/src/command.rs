use log::{debug, error, info};
use serde_json::{json, Value};
use tauri::command;

use crate::settings;


#[command]
pub fn get_settings() -> settings::Settings {
    let settings = settings::Settings::new();
    settings
}

#[command]
pub async fn update_settings(settings: settings::Settings) -> bool {
    let mut current = settings::Settings::new();
    current.update(settings);

    match current.submit() {
        Ok(_) => true,
        Err(e) => {
            error!("{}", e);
            false
        }
    }
}

#[command]
pub fn validate_system_current_password(current_password: &str) -> bool {
    debug!("{}", current_password);
    true
}

#[command]
pub fn update_system_password(
    current_password: &str,
    new_password: &str,
    confirm_password: &str,
) -> bool {
    if validate_system_current_password(current_password) == false {
        unreachable!();
    }

    info!("{}", current_password);
    info!("{}", new_password);
    info!("{}", confirm_password);

    true
}
