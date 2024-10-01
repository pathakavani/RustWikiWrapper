use crate::api::MediaWikiClient;
use crate::models::rest::metadata::Metadata;
use std::error::Error;

impl MediaWikiClient {
    pub async fn get_metadata(&self, _title: &str) -> Result<Metadata, Box<dyn Error>> {
        unimplemented!()
    }
}