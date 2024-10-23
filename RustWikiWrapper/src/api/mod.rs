// src/api/mod.rs

use reqwest::{Client, Url};
use std::error::Error;
use std::collections::HashMap;

pub mod rest;
pub mod action;

// Common struct that can be used by all the API's
pub struct MediaWikiClient {
    client: Client,
    base_url: String,
}

impl MediaWikiClient {
    // Constructor for MediaWikiClient
    pub fn new(base_url: &str) -> Self {
        MediaWikiClient {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    // Implement a GET method that sends a request to the provided URL
    pub async fn get(&self, endpoint: &str, params: &[(&str, &str)]) -> Result<reqwest::Response, Box<dyn Error>> {
        let url = Url::parse_with_params(
            &format!("{}/{}", self.base_url, endpoint),
            params,
        )?;

        let response = self.client.get(url).send().await?;
        Ok(response)
    }

    // Implement a POST method that sends a POST request to the provided URL with form parameters
    pub async fn post(&self, endpoint: &str, params: &HashMap<&str, &str>) -> Result<reqwest::Response, Box<dyn Error>> {
        let url = format!("{}/{}", self.base_url, endpoint);

        let response = self.client
            .post(&url)
            .form(params)  // Sending form data
            .send()
            .await?;

        Ok(response)
    }
}