// Dashboard
// https://developer.twitter.com/en/portal/dashboard

// Document(X)
// https://developer.twitter.com/ja/docs/authentication/oauth-1-0a
// https://developer.twitter.com/en/docs/authentication/oauth-1-0a/authorizing-a-request

// Document(Qiita)
// https://qiita.com/TsutomuNakamura/items/a99b588df7035f772e02
// https://qiita.com/kerupani129/items/ee9d894cc67101f16c3f

use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use percent_encoding::{utf8_percent_encode, AsciiSet, PercentEncode};
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};
use std::collections::HashMap;
use tauri::api::http::header::{HeaderMap, AUTHORIZATION};

pub struct OAuth1 {
    consumer_key: String,
    consumer_secret: String,
    callback: String,
}

impl OAuth1 {
    pub fn new(consumer_key: String, consumer_secret: String, callback: Option<String>) -> Self {
        let callback = callback.unwrap_or("http://127.0.0.1".to_string());

        Self {
            consumer_key,
            consumer_secret,
            callback,
        }
    }

    pub fn get_oauth_header(&self, method: &str, url: &str) -> HeaderMap {
        let oauth_nonce = &Alphanumeric.sample_string(&mut thread_rng(), 32);
        let oauth_callback = &self.callback;
        let oauth_signature_method = "HMAC-SHA1";
        let oauth_timestamp = &Utc::now().timestamp().to_string();
        let oauth_consumer_key = &self.consumer_key;
        let oauth_consumer_secret = &self.consumer_secret;
        let oauth_version = "1.0";

        let mut params = HashMap::<&str, &str>::new();
        params.insert("oauth_nonce", oauth_nonce);
        params.insert("oauth_callback", oauth_callback);
        params.insert("oauth_signature_method", oauth_signature_method);
        params.insert("oauth_timestamp", oauth_timestamp);
        params.insert("oauth_consumer_key", oauth_consumer_key);
        params.insert("oauth_version", oauth_version);

        println!("params: {:?}", &params);

        let oauth_signature: &str =
            &Self::create_oauth_signature(&method, &url, oauth_consumer_secret, "", &params);

        params.insert("oauth_signature", oauth_signature);

        let mut params: Vec<(&str, &str)> = params.into_iter().collect();
        params.sort();

        let oauth_nonce = Self::percent_encode(oauth_nonce);
        let oauth_callback = Self::percent_encode(oauth_callback);
        let oauth_signature_method = Self::percent_encode(oauth_signature_method);
        let oauth_timestamp = Self::percent_encode(oauth_timestamp);
        let oauth_consumer_key = Self::percent_encode(oauth_consumer_key);
        let oauth_signature = Self::percent_encode(oauth_signature);
        let oauth_version = Self::percent_encode(oauth_version);
        let val = format!(
            "OAuth {}, {}, {}, {}, {}, {}, {}",
            format!("oauth_nonce=\"{oauth_nonce}\""),
            format!("oauth_callback=\"{oauth_callback}\""),
            format!("oauth_signature_method=\"{oauth_signature_method}\""),
            format!("oauth_timestamp=\"{oauth_timestamp}\""),
            format!("oauth_consumer_key=\"{oauth_consumer_key}\""),
            format!("oauth_signature=\"{oauth_signature}\""),
            format!("oauth_version=\"{oauth_version}\"")
        );

        let mut hedears = HeaderMap::new();
        hedears.insert(AUTHORIZATION, val.parse().unwrap());
        hedears
    }
}

/// Private methods
impl OAuth1 {
    const FRAGMENT: &AsciiSet = &percent_encoding::NON_ALPHANUMERIC
        .remove(b'~')
        .remove(b'-')
        .remove(b'.')
        .remove(b'_');

    fn percent_encode<'a>(input: &'a str) -> PercentEncode<'a> {
        utf8_percent_encode(input, Self::FRAGMENT)
    }

    fn create_oauth_signature(
        method: &str,
        url: &str,
        oauth_consumer_secret: &str,
        oauth_token_secret: &str,
        params: &HashMap<&str, &str>,
    ) -> String {
        let cs_encoded = Self::percent_encode(oauth_consumer_secret);
        let ts_encoded = Self::percent_encode(oauth_token_secret);
        let key: String = format!("{}&{}", cs_encoded, ts_encoded);

        let mut params: Vec<(&&str, &&str)> = params.into_iter().collect();
        params.sort();

        let param = params
            .into_iter()
            .map(|(k, v)| format!("{}={}", Self::percent_encode(k), Self::percent_encode(v)))
            .collect::<Vec<String>>()
            .join("&");

        let method_encoded = Self::percent_encode(method);
        let url_encoded = Self::percent_encode(url);
        let param_encoded = Self::percent_encode(&param);

        let data = format!("{}&{}&{}", method_encoded, url_encoded, param_encoded);

        let hash = hmacsha1::hmac_sha1(key.as_bytes(), data.as_bytes());
        general_purpose::STANDARD.encode(&hash)
    }
}
