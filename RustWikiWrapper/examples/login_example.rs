use rust_wiki_wrapper::MediaWikiClient;
use std::error::Error;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load .env file
    dotenv().expect("Failed to load .env file");
    
    // Create MediaWiki client with URL from environment
    let wiki_url = env::var("WIKI_URL")
        .unwrap_or_else(|_| "https://test.wikipedia.org".to_string());
    
    let client = MediaWikiClient::new(&wiki_url);
    
    // Get credentials from environment variables
    let username = env::var("WIKI_USERNAME")
        .expect("WIKI_USERNAME not set in .env file");
    let password = env::var("WIKI_PASSWORD")
        .expect("WIKI_PASSWORD not set in .env file");

    println!("Attempting to login to {}...", wiki_url);
    println!("Username: {}", username);

    match client.login(&username, &password).await {
        Ok(response) => {
            println!("\nLogin Result:");
            println!("Status: {}", response.clientlogin.status);
            if let Some(username) = response.clientlogin.username {
                println!("Logged in as: {}", username);
            }
        },
        Err(e) => {
            eprintln!("\nLogin failed: {}", e);
            eprintln!("\nPlease check your credentials in the .env file.");
        }
    }

    Ok(())
}