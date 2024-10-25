use rust_wiki_wrapper::api::rest::get_page_summary::get_page_summary;
use rust_wiki_wrapper::api::MediaWikiClient;

use serde_json;

#[tokio::main]
async fn main() {
    let client = MediaWikiClient::new("https://en.wikipedia.org/api/rest_v1");

    match get_page_summary(&client, "Rust (programming language)").await {
        Ok(summary) => {
            // Pretty print the SummaryResponse as JSON
            match serde_json::to_string_pretty(&summary) {
                Ok(pretty_json) => println!("{}", pretty_json),
                Err(e) => println!("Failed to convert to JSON: {}", e),
            }
        }
        Err(e) => {
            println!("Failed to fetch the page summary: {}", e);
        }
    }

    // Main function would test all the api's similar to the example
    // demonstrated above.
}
