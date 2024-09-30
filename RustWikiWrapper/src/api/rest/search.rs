use reqwest::Client;
use std::error::Error;
use crate::models::rest::search_results::SearchResults;

pub async fn get_search_results(_client: &Client,
    _search_term: &str,
    _limit: Option<u32>,) -> Result<SearchResults, Box<dyn Error>> {
        unimplemented!()
}