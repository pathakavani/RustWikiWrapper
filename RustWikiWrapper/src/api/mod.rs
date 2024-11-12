pub mod action;  // Add this line to make the action module public
pub mod rest;    // Add this line if you're using REST APIs



use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::Serialize;


pub struct MediaWikiClient {
    pub client: Client,
    pub base_url: String,
}

impl MediaWikiClient {
    pub fn new(base_url: &str) -> Self {
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (compatible; RustWikiBot/1.0)")
            .cookie_store(true)
            .build()
            .unwrap_or_else(|_| Client::new());

        MediaWikiClient {
            client,
            base_url: base_url.to_string(),
        }
    }

    pub async fn get(
        &self,
        endpoint: &str,
        params: &[(&str, &str)],
    ) -> reqwest::Result<reqwest::Response> {
        let url = format!("{}/{}", self.base_url, endpoint);
        
        // Print request details
        println!("\nGET Request:");
        println!("URL: {}", url);
        println!("Parameters:");
        for (key, value) in params {
            println!("{}: {}", key, value);
        }

        self.client
            .get(&url)
            .query(params)
            .send()
            .await
    }

    pub async fn post<T: Serialize + ?Sized>(
        &self,
        endpoint: &str,
        form: &T,
    ) -> reqwest::Result<reqwest::Response> {
        let url = format!("{}/{}", self.base_url, endpoint);
        
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        // Print request details
        println!("\nPOST Request:");
        println!("URL: {}", url);
        println!("Headers:");
        for (key, value) in headers.iter() {
            println!("{}: {}", key, value.to_str().unwrap_or(""));
        }

        self.client
            .post(&url)
            .headers(headers)
            .form(form)
            .send()
            .await
    }
}