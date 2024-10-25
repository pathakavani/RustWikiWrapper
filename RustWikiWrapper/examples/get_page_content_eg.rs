use rust_wiki_wrapper::api::action::get_page_content::get_page_content;
use rust_wiki_wrapper::api::MediaWikiClient;
use std::error::Error;

async fn test_get_page_content(client: &MediaWikiClient, title: &str) -> Result<(), Box<dyn Error>> {
    println!("=== Testing Action API: Get Page Content ===");
    println!("Fetching content for page: {}", title);

    match get_page_content(client, title).await {
        Ok(content) => {
            println!("\nPage Details:");
            println!("Title: {}", content.title);
            println!("Page ID: {}", content.pageid);
            println!("Namespace: {}", content.ns);
            println!("Content Model: {}", content.content_model);
            println!("Language: {}", content.page_language);
            
            if let Some(rev_id) = content.revid {
                println!("Revision ID: {}", rev_id);
            }
            
            println!("\nContent Preview (first 500 characters):");
            let preview = if content.content.len() > 500 {
                format!("{}...", &content.content[..500])
            } else {
                content.content.clone()
            };
            println!("{}", preview);
        }
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // initialize client with action API base URL
    let client = MediaWikiClient::new("https://en.wikipedia.org");

    // test with existing page
    println!("\nTesting with existing page:");
    test_get_page_content(&client, "Rust_(programming_language)").await?;

    // test with non-existent page
    println!("\nTesting with non-existent page:");
    test_get_page_content(&client, "This_Page_Does_Not_Exist_12345").await?;

    Ok(())
}