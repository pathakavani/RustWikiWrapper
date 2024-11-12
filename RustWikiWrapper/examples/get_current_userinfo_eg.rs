// examples/get_current_user_eg.rs
use rust_wiki_wrapper::api::action::get_current_user::get_current_user;
use rust_wiki_wrapper::api::MediaWikiClient;
use std::error::Error;

async fn test_get_current_user() -> Result<(), Box<dyn Error>> {
    println!("=== Testing Get Current User Info API ===");

    // initialize the client
    let client = MediaWikiClient::new("https://en.wikipedia.org");

    // get current user info
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

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    test_get_current_user().await?;
    Ok(())
}