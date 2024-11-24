#![no_main]

use reqwest::{Response, StatusCode};
use async_trait::async_trait;
use serde::Serialize;
use libfuzzer_sys::fuzz_target;
use tokio::runtime::Runtime;
//use serde_json;
//use http::response::Builder;

// Define a trait for the client behavior
#[async_trait]
pub trait WikiClient {
    async fn get(
        &self,
        endpoint: &str,
        params: &[(&str, &str)],
    ) -> Result<reqwest::Response, reqwest::Error>;

    async fn post<T: Serialize + Send + Sync>(
        &self,
        endpoint: &str,
        form: &T,
    ) -> Result<reqwest::Response, reqwest::Error>;
}

// Mock client implementation
pub struct MockClient;

#[async_trait]
impl WikiClient for MockClient {
    async fn get(
        &self,
        _endpoint: &str,
        _params: &[(&str, &str)],
    ) -> Result<reqwest::Response, reqwest::Error> {
        // Create a response using reqwest's response builder
        let response_builder = http::Response::builder()
            .status(StatusCode::OK)
            .body("Mock GET Response")
            .unwrap();
        
        Ok(Response::from(response_builder))
    }

    async fn post<T: Serialize + Send + Sync>(
        &self,
        _endpoint: &str,
        _form: &T,
    ) -> Result<reqwest::Response, reqwest::Error> {
        // Create a response using reqwest's response builder
        let response_builder = http::Response::builder()
            .status(StatusCode::OK)
            .body("Mock POST Response")
            .unwrap();
        
        Ok(Response::from(response_builder))
    }
}

async fn login(
    client: &impl WikiClient,
    username: &str,
    password: &str,
) -> Result<(), reqwest::Error> {
    let _ = client.post(
        "api.php",
        &serde_json::json!({
            "username": username,
            "password": password,
        }),
    ).await?;
    
    Ok(())
}

fuzz_target!(|data: &[u8]| {
    if let Ok(fuzz_input) = std::str::from_utf8(data) {
        let runtime = Runtime::new().unwrap();

        runtime.block_on(async {
            let client = MockClient;
            let username = fuzz_input;
            let password = fuzz_input;

            let _ = login(&client, username, password).await;
        });
    }
});