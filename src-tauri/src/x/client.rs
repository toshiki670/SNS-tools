// https://docs.rs/tauri/1.5.2/tauri/api/http/struct.HttpRequestBuilder.html

use log::{error, info};
use serde_json::Value;
use tauri::api::{
    http::{Client, ClientBuilder, HttpRequestBuilder, Response, ResponseType},
    Error,
};

pub struct XClient {
    client: Client,
}

impl XClient {
    pub fn new() -> Self {
        let client = ClientBuilder::new()
            .max_redirections(3)
            .connect_timeout(std::time::Duration::from_secs(5))
            .build()
            .unwrap();

        Self { client }
    }

    pub async fn tauri_api(&self) -> Result<Value, Error> {
        let request = request_builder("GET", "https://httpbin.org/ip")?;

        info!("request info: {:?}", request);

        let response = self.client.send(request).await?;

        process_response(response).await
    }
}

fn request_builder(method: &str, url: &str) -> Result<HttpRequestBuilder, Error> {
    match HttpRequestBuilder::new(method, url) {
        Ok(builder) => Ok(builder.response_type(ResponseType::Json)),
        Err(err) => {
            error!("method: {}, url: {}", method, url);
            Err(err)
        }
    }
}

async fn process_response(response: Response) -> Result<Value, Error> {
    info!("Status Code: {:?}", response.status());

    let data = response.read().await?.data;

    info!("Response Data: {:?}", data);
    Ok(data)
}
