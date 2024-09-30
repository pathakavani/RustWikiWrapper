use serde::Deserialize;

// Struct representing the thumbnail object within the page response
#[derive(Debug, Deserialize)]
pub struct Thumbnail {
    pub url: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

// Struct representing the license information within the page response
#[derive(Debug, Deserialize)]
pub struct License {
    pub url: Option<String>,
    pub title: Option<String>,
}

// Struct representing the main page object returned by the /page/{title}/bare API
#[derive(Debug, Deserialize)]
pub struct Page {
    pub title: String,
    pub key: Option<String>,
    pub latest_revision: Option<u32>,
    pub html_url: Option<String>,  // URL to fetch the latest content in HTML
    pub thumbnail: Option<Thumbnail>,
    pub license: Option<License>,
}