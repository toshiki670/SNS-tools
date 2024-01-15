use crate::gateways::settings_repository;
use crate::use_cases::{load_settings_use_case, update_settings_use_case};
use crate::utility::app_data_dir;
use serde_json::Value;

pub fn load(app: tauri::AppHandle) -> Value {
    let app_path = app_data_dir(&app.config());
    let repository = settings_repository::SettingsRepository::new(app_path);
    let settings = load_settings_use_case::run(&repository);

    serde_json::to_value(settings).unwrap()
}

pub fn update(app: tauri::AppHandle, value: Value) -> bool {
    let app_path = app_data_dir(&app.config());
    let repository = settings_repository::SettingsRepository::new(app_path);

    let current = load_settings_use_case::run(&repository);
    let new = serde_json::from_value(value).unwrap();
    update_settings_use_case::run(&repository, current, new)
}
