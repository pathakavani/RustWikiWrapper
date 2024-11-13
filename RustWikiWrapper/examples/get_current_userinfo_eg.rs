use rust_wiki_wrapper::api::action::get_current_user::get_current_user;
use rust_wiki_wrapper::api::action::login::login;  // Add this
use rust_wiki_wrapper::MediaWikiClient;
use std::error::Error;
use dotenv::dotenv;
use std::env;

async fn test_get_current_user() -> Result<(), Box<dyn Error>> {
    println!("=== Testing Get Current User Info API ===");

    // load environment variables
    dotenv().ok();
    
    // get credentials
    let username = env::var("WIKI_USERNAME")
        .expect("WIKI_USERNAME not set in .env file");
    let password = env::var("WIKI_PASSWORD")
        .expect("WIKI_PASSWORD not set in .env file");

    // initialize the client
    let client = MediaWikiClient::new("https://en.wikipedia.org");

    match login(&client, &username, &password).await {
        Ok(login_response) => {
            println!("Login successful: {:#?}", login_response);
            
            // now get current user info using the same client
            match get_current_user(&client).await {
                Ok(user_info) => {
                    println!("\nUser Information:");
                    if let Some(name) = &user_info.query.userinfo.name {
                        println!("Username: {}", name);
                    }
                    
                    if user_info.query.userinfo.anon.is_some() {
                        println!("Status: Anonymous");
                    } else {
                        println!("Status: Logged in");
                    }

                    if let Some(id) = user_info.query.userinfo.id {
                        println!("User ID: {}", id);
                    }

                    if let Some(groups) = &user_info.query.userinfo.groups {
                        println!("\nUser Groups:");
                        for group in groups {
                            println!("- {}", group);
                        }
                    }

                    if let Some(rights) = &user_info.query.userinfo.rights {
                        println!("\nUser Rights:");
                        for right in rights {
                            println!("- {}", right);
                        }
                    }

                    if let Some(count) = user_info.query.userinfo.editcount {
                        println!("\nEdit Count: {}", count);
                    }

                    if let Some(registration) = &user_info.query.userinfo.registration {
                        println!("Registration Date: {}", registration);
                    }

                    // print raw response for debugging
                    println!("\nRaw Response:");
                    println!("{:#?}", user_info);
                },
                Err(e) => {
                    println!("❌ Error getting user info: {}", e);
                }
            }
        },
        Err(e) => {
            println!("❌ Login failed: {}", e);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    test_get_current_user().await?;
    Ok(())
}