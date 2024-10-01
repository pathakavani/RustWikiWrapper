use crate::api::MediaWikiClient;
use crate::models::rest::pages::Page;
use std::error::Error;

impl MediaWikiClient {
    pub async fn get_page_info(&self, title: &str) -> Result<Page, Box<dyn Error>> {
        unimplemented!()
    }
}