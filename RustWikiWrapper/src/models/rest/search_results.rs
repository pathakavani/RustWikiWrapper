use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchResults {
    pub query: Query,
    #[serde(default)]
    pub search_info: Option<SearchInfo>,
}

#[derive(Debug, Deserialize)]
pub struct Query {
    pub search: Vec<SearchResult>,
}

#[derive(Debug, Deserialize)]
pub struct SearchInfo {
    #[serde(rename = "totalhits")]
    pub total_hits: Option<u64>,
    #[serde(rename = "suggestion")]
    pub search_suggestion: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchResult {
    pub pageid: u64,
    pub title: String,
    pub snippet: String,
    #[serde(rename = "titlesnippet")]
    pub title_snippet: Option<String>,
    pub size: Option<u32>,
    pub wordcount: Option<u32>,
    pub timestamp: Option<String>,
}
