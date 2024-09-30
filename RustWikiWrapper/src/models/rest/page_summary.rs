use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PageSummary {
    pub title: String,
    pub extract: Option<String>,
    pub thumbnail: Option<Thumbnail>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Thumbnail {
    pub source: String,
    pub width: u32,
    pub height: u32,
}