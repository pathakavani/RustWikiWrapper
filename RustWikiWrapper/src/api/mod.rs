// src/api/mod.rs
use reqwest::Client;
use std::error::Error;

use crate::models::SummaryResponse;

pub struct MediaWikiClient {
    client: Client,
    base_url: String,
}
``
impl MediaWikiClient {
    pub fn new(base_url: &str) -> Self {
        MediaWikiClient {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn get_page_summary(&self, title: &str) -> Result<SummaryResponse, Box<dyn Error>> {
        let url = format!("{}/page/summary/{}", self.base_url, title);
        let response = self.client.get(&url).send().await?.json::<SummaryResponse>().await?;
        println!("Parsed Summary Response: {:?}", response);

        Ok(response)
    }
}