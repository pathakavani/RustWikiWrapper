// Model for Create/Edit Page Response
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEditPageResponse {
    pub edit: EditResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EditResult {
    pub result: String,
    pub page_id: Option<u64>,
    pub title: Option<String>,
    pub content_model: Option<String>,
    pub old_revid: Option<u64>,
    pub new_revid: Option<u64>,
    pub nochange: Option<bool>,
}