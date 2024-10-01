// Model for Page Content Response
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PageContentResponse {
    pub query: QueryResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResult {
    pub pages: Vec<Page>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    pub pageid: Option<u64>,
    pub title: Option<String>,
    pub content: Option<String>,
    // Placeholder for Page content response structure
}
