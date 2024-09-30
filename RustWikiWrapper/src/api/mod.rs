// src/api/mod.rs

use reqwest::Client;

pub mod rest;
pub mod action;

// Common struct that can be used by all the API's
pub struct MediaWikiClient {
    client: Client,
    base_url: String,
}

impl MediaWikiClient {
pub fn new(base_url: &str) -> Self {
    MediaWikiClient {
        client: Client::new(),
        base_url: base_url.to_string(),
    }
}

// Common utility functions can be added here
}