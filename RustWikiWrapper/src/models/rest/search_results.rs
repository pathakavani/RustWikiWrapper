use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Page {
    pub id: u32,
    pub key: String,
    pub title: String,
    pub excerpt: Option<String>,
    pub description: Option<String>,
    pub thumbnail: Option<Thumbnail>,
}

#[derive(Debug, Deserialize)]
pub struct Thumbnail {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Deserialize)]
pub struct SearchResults {
    pub pages: Vec<Page>,
}