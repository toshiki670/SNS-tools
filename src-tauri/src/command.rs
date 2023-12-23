use log::{error, info};
use serde_json::{json, Value};
use tauri::command;
use tauri::regex::Regex;

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
pub fn validate_system_password(password: &str) -> Result<(), String> {
    match password.len() {
        0 => Err(i18n_path("validate_system_password", "empty")),
        1..=8 => Err(i18n_path("validate_system_password", "less_then_8")),
        _ => {
            let a: &str = r#"[a-z]"#;

            let regexes = vec![r#"[a-z]"#, r#"[A-Z]"#, r#"\d"#, r#"[^a-zA-Z0-9]"#];
            if regexes
                .iter()
                .find(|r| false == Regex::new(r).unwrap().is_match(password))
                .is_none()
            {
                Ok(())
            } else {
                Err(i18n_path("validate_system_password", "week_password"))
            }
        }
    }
}

#[command]
pub fn validate_system_current_password(password: &str) -> Result<(), String> {
    Ok(())
}

#[command]
pub fn validate_system_confirm_password(newpsw: &str, confirm: &str) -> Result<(), String> {
    if newpsw == confirm {
        Ok(())
    } else {
        Err(i18n_path("validate_system_confirm_password", "not_match"))
    }
}

#[command]
pub fn update_system_password(current: &str, newpsw: &str, confirm: &str) -> String {
    validate_system_password(current).unwrap();
    validate_system_current_password(current).unwrap();
    validate_system_password(newpsw).unwrap();
    validate_system_password(confirm).unwrap();
    validate_system_confirm_password(newpsw, confirm).unwrap();

    info!("{}", current);
    info!("{}", newpsw);
    info!("{}", confirm);

    i18n_path("update_system_password", "success")
}
