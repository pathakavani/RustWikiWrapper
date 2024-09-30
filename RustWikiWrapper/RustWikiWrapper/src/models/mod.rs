// src/models/mod.rs
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SummaryResponse {
    pub title: String,
    pub extract: Option<String>,
    pub thumbnail: Option<Thumbnail>,
}

#[derive(Debug, Deserialize)]
pub struct Thumbnail {
    pub source: String,
    pub width: u32,
    pub height: u32,
}