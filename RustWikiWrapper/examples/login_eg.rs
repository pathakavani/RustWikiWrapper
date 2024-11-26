use rust_wiki_wrapper::api::MediaWikiClient;
use std::env;
use std::error::Error;

async fn test_login(client: &MediaWikiClient) -> Result<(), Box<dyn Error>> {
    println!("=== Testing Wikipedia Login ===");
    
    // Get credentials from environment variables
    let username = env::var("WIKI_USERNAME").expect("WIKI_USERNAME not set");
    let password = env::var("WIKI_PASSWORD").expect("WIKI_PASSWORD not set");
    
    println!("Attempting to login with username: {}", username);
    
    // Using the login function with correct number of parameters
    match client.login( &username, &password).await {
        Ok(response) => {
            println!("\nLogin Response: {:?}", response );
            println!("Status: {}", response.clientlogin.status);
            
            // Handle username if present
            if let Some(username) = response.clientlogin.username {
                println!("Successfully logged in as: {}", username);
            }

            // Check login status
            match response.clientlogin.status.as_str() {
                "PASS" => println!("✅ Login successful!"),
                status => println!("❌ Login failed with status: {}", status),
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
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    
    // Create client
    let client = MediaWikiClient::new("https://test.wikipedia.org");
    
    // Test login
    test_login(&client).await?;
    
    Ok(())
}