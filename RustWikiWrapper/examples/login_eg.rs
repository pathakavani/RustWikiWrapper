use rust_wiki_wrapper::api::action::login::login;
use rust_wiki_wrapper::api::MediaWikiClient;
use std::env;
use std::error::Error;

async fn test_login(client: &MediaWikiClient) -> Result<(), Box<dyn Error>> {
    println!("=== Testing Wikipedia Login ===");
    
    // get credentials from environment variables
    let username = env::var("WIKI_USERNAME").expect("WIKI_USERNAME not set");
    let password = env::var("WIKI_PASSWORD").expect("WIKI_PASSWORD not set");
    let return_url = "https://en.wikipedia.org";
    
    println!("Attempting to login with username: {}", username);
    
    // Using the login function from your module
    match login(client, &username, &password, return_url).await {
        Ok(response) => {
            println!("\nLogin Response:");
            println!("Status: {}", response.clientlogin.status);
            
            if let Some(username) = &response.clientlogin.username {
                println!("Username: {}", username);
            }
            if let Some(message) = &response.clientlogin.message {
                println!("Message: {}", message);
            }
        }
        Err(e) => {
            println!("\n❌ Error during login process: {}", e);
        }
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let client = MediaWikiClient::new("https://en.wikipedia.org");
    test_login(&client).await?;
    Ok(())
}