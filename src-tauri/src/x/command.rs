use log::error;
use serde_json::{json, Value};
use tauri::{command, State};

use super::client::XClient;

#[command]
pub async fn x_get_api<'r>(client: State<'r, XClient>) -> Result<Value, String> {
    match client.tauri_api().await {
        Ok(response) => Ok(json!(response)),
        Err(err) => {
            json!(err.to_string());
            error!("{:?}", err);
            Err("Something Err".to_string())
        }
    }
}
