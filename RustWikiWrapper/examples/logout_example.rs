use rust_wiki_wrapper::MediaWikiClient;
use std::error::Error;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables
    dotenv().expect("Failed to load .env file");
    
    // Initialize client
    let client = MediaWikiClient::new("https://test.wikipedia.org");
    
    // Get credentials from environment variables
    let username = env::var("WIKI_USERNAME")
        .expect("WIKI_USERNAME not set in .env file");
    let password = env::var("WIKI_PASSWORD")
        .expect("WIKI_USERNAME not set in .env file");

    println!("=== MediaWiki Login/Logout Test ===");
    
    // First, try to login
    println!("\nStep 1: Logging in...");
    println!("Username: {}", username);
    
    match client.login(&username, &password).await {
        Ok(login_response) => {
            println!("Login successful!");
            if let Some(username) = login_response.clientlogin.username {
                println!("Logged in as: {}", username);
            }

            // Wait a moment before logout (optional)
            println!("\nWaiting briefly before logout...");
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

            // Now attempt to logout
            println!("\nStep 2: Logging out...");
            match client.logout().await {
                Ok(_) => {
                    println!("Logout successful!");
                    
                    // Verify logout by attempting to access protected resource
                    println!("\nStep 3: Verifying logout...");
                    let verify_response = client.get(
                        "w/api.php",
                        &[
                            ("action", "query"),
                            ("meta", "userinfo"),
                            ("format", "json"),
                        ],
                    ).await?;

                    let verify_data: serde_json::Value = verify_response.json().await?;
                    println!("Verification response: {}", 
                        serde_json::to_string_pretty(&verify_data)?);

                    if verify_data["query"]["userinfo"].get("anon").is_some() {
                        println!("Verification confirmed: Successfully logged out!");
                    } else {
                        println!("Warning: Session might still be active!");
                    }
                },
                Err(e) => {
                    eprintln!("Logout failed: {}", e);
                    eprintln!("Details: ");
                    eprintln!("- Make sure you're still logged in");
                    eprintln!("- Check if the CSRF token is valid");
                }
            }
        },
        Err(e) => {
            eprintln!("Login failed: {}", e);
            eprintln!("Cannot proceed with logout test");
        }
    }

    Ok(())
}