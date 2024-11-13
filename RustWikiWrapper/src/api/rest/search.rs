use crate::api::MediaWikiClient;
use crate::models::rest::search_results::SearchResults;
use std::error::Error;

impl MediaWikiClient {
    pub async fn get_search_results(
        &self,
        search_term: &str,
        limit: Option<u32>,
    ) -> Result<SearchResults, Box<dyn Error>> {
        // set default limit to 10 if not specified
        let limit = limit.unwrap_or(10);

        // set up query parameters for search
        let params = [
            ("action", "query"),
            ("list", "search"),
            ("srsearch", search_term),
            ("srlimit", &limit.to_string()),
            ("srwhat", "text"),
            ("srprop", "snippet|titlesnippet|wordcount|timestamp"),
            ("format", "json"),
        ];

        // send the GET request
        let response = self.get("w/api.php", &params).await?;

        // parse the response directly into our SearchResults struct
        let search_results: SearchResults = response.json().await?;

        Ok(search_results)
    }
}
