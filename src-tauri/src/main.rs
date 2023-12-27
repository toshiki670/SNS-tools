// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
mod settings;

fn main() {
    std::env::set_var("RUST_LOG", log::Level::Trace.to_string());
    env_logger::init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command::submit_settings,
            command::get_settings,
            command::validate_system_current_password,
            command::update_system_password,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
