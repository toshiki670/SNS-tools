// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
mod db;
mod schema;
mod settings;

use db::ConnectionPool;
use tauri::{api::path::app_data_dir, Manager};

fn main() {
    std::env::set_var("RUST_LOG", log::Level::Trace.to_string());
    env_logger::init();

    tauri::Builder::default()
        .setup(|app: &mut tauri::App| {
            let app_path =
                app_data_dir(&app.config()).expect("Failed to get application data path.");
            if !app_path.exists() {
                std::fs::create_dir_all(&app_path).unwrap();
            }

            let connection_pool = db::establish_connection(app_path.clone());
            db::run_migration(&connection_pool);
            app.manage::<ConnectionPool>(connection_pool);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            command::submit_settings,
            command::get_settings,
            command::update_settings,
            command::validate_system_current_password,
            command::update_system_password,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
