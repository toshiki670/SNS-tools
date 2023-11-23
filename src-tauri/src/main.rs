// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod base_ctx;
mod router;
mod x;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .manage(x::client::XClient::new())
        .invoke_handler(tauri::generate_handler![x::command::x_get_api])
        .plugin(rspc::integrations::tauri::plugin(router::router(), || {
            base_ctx::BaseCtx::init()
        }))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
