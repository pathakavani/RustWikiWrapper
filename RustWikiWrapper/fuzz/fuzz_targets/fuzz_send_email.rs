#![no_main]

use reqwest::{Response, StatusCode};
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use libfuzzer_sys::fuzz_target;
use tokio::runtime::Runtime;
use std::collections::HashMap;
use serde_json::{json, Value};

// Mock structs with proper derive attributes
#[derive(Debug, Serialize, Deserialize)]
struct EmailUserResponse {
    emailuser: EmailUser,
}

#[derive(Debug, Serialize, Deserialize)]
struct EmailUser {
    result: String,
}

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

pub struct MockClient;

#[async_trait]
impl WikiClient for MockClient {
    async fn get(
        &self,
        _endpoint: &str,
        _params: &[(&str, &str)],
    ) -> Result<reqwest::Response, reqwest::Error> {
        let token_response = json!({
            "query": {
                "tokens": {
                    "csrftoken": "mock_token+\\"
                }
            }
        });

        let response_builder = http::Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "application/json")
            .body(token_response.to_string())
            .unwrap();
        
        Ok(Response::from(response_builder))
    }

    async fn post<T: Serialize + Send + Sync>(
        &self,
        _endpoint: &str,
        _form: &T,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let email_response = EmailUserResponse {
            emailuser: EmailUser {
                result: "Success".to_string(),
            },
        };

        let response_builder = http::Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "application/json")
            .body(serde_json::to_string(&email_response).unwrap())
            .unwrap();
        
        Ok(Response::from(response_builder))
    }
}

async fn send_email(
    client: &impl WikiClient,
    target_user: &str,
    subject: &str,
    message: &str,
    cc_me: bool,
) -> Result<EmailUserResponse, Box<dyn std::error::Error>> {
    // Get CSRF token
    let token_response = client
        .get(
            "w/api.php",
            &[("action", "query"), ("meta", "tokens"), ("format", "json")],
        )
        .await?;

    let token_data: Value = token_response.json().await?;
    let email_token = token_data["query"]["tokens"]["csrftoken"]
        .as_str()
        .ok_or("Failed to obtain email token")?;

    // Prepare parameters
    let mut params = HashMap::new();
    params.insert("action", "emailuser");
    params.insert("target", target_user);
    params.insert("subject", subject);
    params.insert("text", message);
    params.insert("token", email_token);
    params.insert("ccme", if cc_me { "1" } else { "0" });
    params.insert("format", "json");

    // Send email request
    let response = client.post("w/api.php", &params).await?;
    let email_response: EmailUserResponse = response.json().await?;

    Ok(email_response)
}

// Fuzz target implementation
fuzz_target!(|data: &[u8]| {
    if let Ok(fuzz_input) = std::str::from_utf8(data) {
        let runtime = Runtime::new().unwrap();

        runtime.block_on(async {
            let client = MockClient;
            
            // Split input into parts for different parameters
            let parts: Vec<&str> = fuzz_input.split(',').collect();
            
            // Extract parts with proper length checks
            let target_user = parts.get(0).map_or("default_user", |&s| s);
            let subject = parts.get(1).map_or("default_subject", |&s| s);
            let message = parts.get(2).map_or("default_message", |&s| s);
            let cc_me = parts.get(3).map_or(false, |&s| s == "1");

            // Try sending email with fuzzed input
            match send_email(&client, target_user, subject, message, cc_me).await {
                Ok(response) => {
                    assert_eq!(response.emailuser.result, "Success");
                },
                Err(e) => {
                    // Log error but don't panic
                    println!("Error occurred: {}", e);
                }
            }
        });
    }
});