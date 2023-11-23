use tauri::{command, State};

use super::client::XClient;

#[command]
pub async fn x_get_api<'r>(client: State<'r, XClient>) -> Result<String, String> {
    if let Ok(response) = client.tauri_api().await {
        println!("Status Code: {:?}", response);
    } else {
        println!("Something Happened!");
    };

    Ok("OK".to_string())
}
