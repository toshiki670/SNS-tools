use log::error;
use serde_json::{json, Value};
use tauri::command;


#[command]
pub async fn x_get_api() -> Result<Value, Value> {
    Err(json!({"Error": "API"}))
}
