// https://docs.rs/tauri/1.5.2/tauri/api/http/struct.HttpRequestBuilder.html
// https://developer.twitter.com/en/portal/dashboard

// X認証、OAuth 1.0a のドキュメント
// https://developer.twitter.com/ja/docs/authentication/oauth-1-0a

// OAuth 1.0
// https://qiita.com/TsutomuNakamura/items/a99b588df7035f772e02
use log::info;
use serde_json::Value;
use tauri::api::{
    http::{Client, ClientBuilder, HttpRequestBuilder, Response, ResponseType},
    Error,
};

pub mod oauth1;
use oauth1::OAuth1;

pub struct XClient {
    client: Client,
    oauth1: OAuth1,
}

impl XClient {
    pub fn new(oauth1: OAuth1) -> Self {
        let client = ClientBuilder::new()
            .max_redirections(3)
            .connect_timeout(std::time::Duration::from_secs(5))
            .build()
            .unwrap();

        Self { client, oauth1 }
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
    const DOMAIN: &str = "api.twitter.com";

    fn url(path: &str) -> String {
        format!("https://{}/{}", Self::DOMAIN, path)
    }

    fn common_request(request: HttpRequestBuilder) -> HttpRequestBuilder {
        let request = request.response_type(ResponseType::Json);
        info!("request info: {:?}", request);
        request
    }

    async fn process_response(response: Response) -> Result<Value, Error> {
        info!("Status Code: {:?}", response.status());

        let data = response.read().await?.data;

        info!("Response Data: {:?}", data);
        Ok(data)
    }
}
