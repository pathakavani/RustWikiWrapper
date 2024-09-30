// src/lib.rs
// src/lib.rs
pub mod api;
pub mod models;

#[cfg(test)]
mod tests {
    use super::api::MediaWikiClient; // Move the import inside the test module

    #[tokio::test]
    async fn test_get_page_summary() {
        // Now the `MediaWikiClient` struct should be accessible
        let client = MediaWikiClient::new("https://en.wikipedia.org/api/rest_v1");
        let result = client.get_page_summary("Rust_(programming_language)").await;

        match result {
            Ok(summary) => {
                println!("Title: {}", summary.title);
                println!("Extract: {:?}", summary.extract);
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}