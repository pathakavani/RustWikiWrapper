// Model for page content response
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PageContent {
    pub pageid: i64,
    pub ns: i32,
    pub title: String,
    pub content: String,
    #[serde(rename = "contentmodel")]
    pub content_model: String,
    #[serde(rename = "pagelanguage")]
    pub page_language: String,
    pub revid: Option<i64>,
    #[serde(rename = "parentid")]
    pub parent_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: String,
    pub info: String,
    #[serde(rename = "*")]
    pub details: Option<String>,
}