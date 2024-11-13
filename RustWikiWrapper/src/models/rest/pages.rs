use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Thumbnail {
    pub url: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}


#[derive(Debug, Deserialize)]
pub struct License {
    pub url: Option<String>,
    pub title: Option<String>,
}


#[derive(Debug, Deserialize)]
pub struct Page {
    pub title: String,
    pub key: Option<String>,
    pub latest_revision: Option<u32>,
    pub html_url: Option<String>,  
    pub thumbnail: Option<Thumbnail>,
    pub license: Option<License>,
}