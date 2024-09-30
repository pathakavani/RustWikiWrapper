use reqwest::Client;
use std::error::Error;
use crate::models::rest::pages::Page;

// Function to get page information using the MediaWiki REST API
pub async fn get_page_info(
    _client: &Client,
    _title: &str,
) -> Result<Page, Box<dyn Error>> {
    
    unimplemented!()
}