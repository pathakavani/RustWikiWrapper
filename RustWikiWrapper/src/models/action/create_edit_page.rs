use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEditPageResponse {
    pub edit: EditResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditResult {
    pub result: String,
    #[serde(rename = "pageid")]
    pub page_id: Option<u64>,
    pub title: String,
    #[serde(rename = "contentmodel")]
    pub content_model: String,
    #[serde(rename = "oldrevid")]
    pub old_rev_id: u64,
    #[serde(rename = "newrevid")]
    pub new_rev_id: u64,
    #[serde(rename = "newtimestamp")]
    pub new_timestamp: String,
    pub watched: Option<String>,
    pub new: Option<String>,
}
