// https://docs.rs/tauri/1.5.2/tauri/api/http/struct.HttpRequestBuilder.html
use tauri::api::http::{Client, ClientBuilder, HttpRequestBuilder, ResponseType};

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

    pub async fn tauri_api(&self) -> Result<serde_json::value::Value, tauri::api::Error> {
        let request = HttpRequestBuilder::new("GET", "https://httpbin.org/ip")
            .unwrap()
            .response_type(ResponseType::Json);

        let response = self.client.send(request).await?;
        println!("Status Code: {:?}", response.status());

        let response_data = response.read().await?;
        println!("Status Code: {:?}", response_data.data);

        Ok(response_data.data)
    }
}
