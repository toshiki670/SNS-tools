use std::{fs, path::PathBuf, sync::Arc};
use tauri::{api::path::app_data_dir as tauri_app_data_dir, Config};

pub fn app_data_dir(config: &Arc<Config>) -> PathBuf {
    let path = tauri_app_data_dir(config).expect("Failed to get application data path.");
    if !path.exists() {
        fs::create_dir_all(&path).unwrap();
    }
    return path;
}
