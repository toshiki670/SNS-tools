// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod x;

fn main() {
    tauri::Builder::default()
        .manage(x::client::XClient::new())
        .invoke_handler(tauri::generate_handler![x::command::x_get_api])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
