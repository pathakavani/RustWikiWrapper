use dotenv::dotenv;
use rust_wiki_wrapper::{
    api::action::get_current_user::get_current_user, api::action::login::login,
    api::MediaWikiClient,
};
use std::env;

async fn setup_authenticated_client() -> MediaWikiClient {
    dotenv().ok();

    let username = env::var("WIKI_USERNAME").expect("WIKI_USERNAME must be set in environment");
    let password = env::var("WIKI_PASSWORD").expect("WIKI_PASSWORD must be set in environment");

    let client = MediaWikiClient::new("https://en.wikipedia.org");

    login(&client, &username, &password)
        .await
        .expect("Failed to login");

    client
}

#[tokio::test]
async fn test_get_authenticated_user_info() {
    let client = setup_authenticated_client().await;
    let result = get_current_user(&client).await;

    match result {
        Ok(user_info) => {
            println!("Successfully retrieved user info:");
            println!("Username: {:?}", user_info.query.userinfo.name);
            println!("ID: {:?}", user_info.query.userinfo.id);

            if let Some(groups) = &user_info.query.userinfo.groups {
                println!("Groups: {:?}", groups);
            }
            if let Some(rights) = &user_info.query.userinfo.rights {
                println!("Rights: {:?}", rights);
            }

            // Verify user properties
            let name = user_info.query.userinfo.name.expect("Name should exist");
            assert!(!name.is_empty(), "Username should not be empty");

            let id = user_info.query.userinfo.id.expect("ID should exist");
            assert!(id > 0, "User ID should be positive");

            // Check for email if user has email set
            if let Some(email) = &user_info.query.userinfo.email {
                assert!(email.contains('@'), "Email should be valid");
            }

            // Check registration date if present
            if let Some(registration) = &user_info.query.userinfo.registration {
                assert!(
                    !registration.is_empty(),
                    "Registration date should not be empty"
                );
            }

            // Check edit count if present
            if let Some(editcount) = user_info.query.userinfo.editcount {
                assert!(editcount >= 0, "Edit count should be non-negative");
            }
        }
        Err(e) => {
            panic!("Failed to get user info: {}", e);
        }
    }
}

#[tokio::test]
async fn test_user_rights() {
    let client = setup_authenticated_client().await;
    let result = get_current_user(&client).await;

    match result {
        Ok(user_info) => {
            if let Some(rights) = user_info.query.userinfo.rights {
                println!("User rights: {:?}", rights);

                // Check for basic rights that all registered users should have
                assert!(
                    rights.contains(&String::from("read")),
                    "User should have read rights"
                );
                assert!(
                    rights.contains(&String::from("edit")),
                    "User should have edit rights"
                );
            } else {
                panic!("User rights not found");
            }
        }
        Err(e) => {
            panic!("Failed to check user rights: {}", e);
        }
    }
}

#[tokio::test]
async fn test_user_groups() {
    let client = setup_authenticated_client().await;
    let result = get_current_user(&client).await;

    match result {
        Ok(user_info) => {
            if let Some(groups) = user_info.query.userinfo.groups {
                println!("User groups: {:?}", groups);

                // Check for basic groups that registered users should be in
                assert!(
                    groups.contains(&String::from("*")),
                    "User should be in * group"
                );
                assert!(
                    groups.contains(&String::from("user")),
                    "User should be in user group"
                );
            } else {
                panic!("User groups not found");
            }
        }
        Err(e) => {
            panic!("Failed to check user groups: {}", e);
        }
    }
}

#[tokio::test]
async fn test_rate_limit_handling() {
    let client = setup_authenticated_client().await;

    for i in 1..=5 {
        let result = get_current_user(&client).await;

        match result {
            Ok(_) => println!("Request {} succeeded", i),
            Err(e) => {
                println!("Request {} failed: {}", i, e);
                if e.to_string().contains("rate") {
                    println!("Rate limit detected as expected");
                    return;
                }
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
}

#[tokio::test]
async fn test_comprehensive_user_info() {
    let client = setup_authenticated_client().await;
    let result = get_current_user(&client).await;

    match result {
        Ok(user_info) => {
            println!("Full user info:");
            println!("Name: {:?}", user_info.query.userinfo.name);
            println!("ID: {:?}", user_info.query.userinfo.id);

            if let Some(editcount) = user_info.query.userinfo.editcount {
                println!("Edit count: {}", editcount);
            }
            if let Some(registration) = &user_info.query.userinfo.registration {
                println!("Registration: {}", registration);
            }
            if let Some(gender) = &user_info.query.userinfo.gender {
                println!("Gender: {}", gender);
            }
            if let Some(blockid) = user_info.query.userinfo.blockid {
                println!("Block ID: {}", blockid);
            }

            // Basic assertions
            let name = user_info.query.userinfo.name.expect("Name should exist");
            assert!(!name.is_empty(), "Username should not be empty");

            let id = user_info.query.userinfo.id.expect("ID should exist");
            assert!(id > 0, "User ID should be positive");
        }
        Err(e) => panic!("Failed to get user info: {}", e),
    }
}
