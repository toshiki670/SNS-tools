// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod controllers;
mod db;
mod entities;
mod gateways;
mod schema;
mod use_cases;
mod utility;

use db::ConnectionPool;
use tauri::Manager;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", log::Level::Trace.to_string());
    env_logger::init();

    tauri::Builder::default()
        .setup(|app: &mut tauri::App| {
            let app_path = utility::tauri::app_data_dir(&app.config());

            let connection_pool = db::establish_connection(app_path.clone());
            db::run_migration(&connection_pool);
            app.manage::<ConnectionPool>(connection_pool);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            controllers::command::get_settings,
            controllers::command::update_settings,
            controllers::command::validate_system_current_password,
            controllers::command::update_system_password,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
