// https://docs.rs/tauri/1.5.2/tauri/api/http/struct.HttpRequestBuilder.html

use log::info;
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
        let request =
            Self::common_request(HttpRequestBuilder::new("GET", Self::url("ip")).unwrap());

        let response = self.client.send(request).await?;

        Self::process_response(response).await
    }
}

/// Private methods
impl XClient {
    const DOMAIN: &str = "httpbin.org";

    fn url(path: &str) -> String {
        format!("https://{}/{}", Self::DOMAIN, path)
    }

    fn common_request(request: HttpRequestBuilder) -> HttpRequestBuilder {
        info!("request info: {:?}", request);
        request.response_type(ResponseType::Json)
    }

    async fn process_response(response: Response) -> Result<Value, Error> {
        info!("Status Code: {:?}", response.status());

        let data = response.read().await?.data;

        info!("Response Data: {:?}", data);
        Ok(data)
    }
}
