// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod router;
mod x;

#[tokio::main]
async fn main() {
    // let router = <Router>::new().build();

    tauri::Builder::default()
        .plugin(rspc::integrations::tauri::plugin(router::router(), || ()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
