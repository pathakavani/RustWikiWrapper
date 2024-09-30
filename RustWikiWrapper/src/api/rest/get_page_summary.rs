use crate::api::MediaWikiClient;
use crate::models::rest::page_summary::PageSummary;
use std::error::Error;

impl MediaWikiClient {
    pub async fn get_page_summary(&self, title: &str) -> Result<PageSummary, Box<dyn Error>> {
        let url = format!("{}/page/summary/{}", self.base_url, title);
        let response = self.client.get(&url).send().await?.json::<PageSummary>().await?;

        Ok(response)
    }
}