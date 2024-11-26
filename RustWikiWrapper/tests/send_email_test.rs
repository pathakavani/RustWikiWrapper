use rust_wiki_wrapper::
    api::MediaWikiClient
;

use std::env;
use tokio;
use dotenvy::dotenv;

// Helper function to get test credentials
fn get_test_credentials() -> Result<(String, String, String), String> {
    dotenv().ok();
    
    let username = env::var("WIKI_TEST_USERNAME")
        .map_err(|_| "WIKI_TEST_USERNAME not set")?;
    let password = env::var("WIKI_TEST_PASSWORD")
        .map_err(|_| "WIKI_TEST_PASSWORD not set")?;
    let target_user = env::var("WIKI_TEST_TARGET_USER")
        .map_err(|_| "WIKI_TEST_TARGET_USER not set")?;

    if username.is_empty() || password.is_empty() || target_user.is_empty() {
        return Err("Credentials cannot be empty".into());
    }

    Ok((username, password, target_user))
}

// Helper to get authenticated client with better error handling
async fn get_authenticated_client() -> Result<MediaWikiClient, String> {
    let (username, password, _) = get_test_credentials()
        .map_err(|e| format!("Failed to get credentials: {}", e))?;
    
    let client = MediaWikiClient::new("https://test.wikipedia.org");
    
    match client.login(&username, &password).await {
        Ok(_) => Ok(client),
        Err(e) => {
            println!("Login attempt failed: {:?}", e);
            Err(format!("Authentication failed: {}", e))
        }
    }
}


#[tokio::test]
async fn test_send_email_with_cc() {
    if let Ok((_, _, target_user)) = get_test_credentials() {
        if let Ok(client) = get_authenticated_client().await {
            let result = client.send_email(
                &target_user,
                "Test Email with CC",
                "This is a test message with CC enabled.",
                true
            ).await;
            
            assert!(result.is_ok(), "Email with CC should be sent successfully");
            if let Ok(response) = result {
                assert_eq!(response.emailuser.result, "Success");
            }
        }
    }
}

#[tokio::test]
async fn test_send_email_invalid_user() {
    if let Ok(client) = get_authenticated_client().await {
        let result = client.send_email(
            "nonexistent_user_12345",
            "Test Subject",
            "Test Message",
            false
        ).await;
        
        assert!(result.is_err(), "Should fail with invalid user");
        if let Err(e) = result {
            println!("Expected error received: {}", e);
            assert!(e.to_string().contains("error") || e.to_string().contains("fail"),
                   "Error should indicate invalid user");
        }
    }
}

#[tokio::test]
async fn test_send_email_empty_fields() {
    if let Ok((_, _, target_user)) = get_test_credentials() {
        if let Ok(client) = get_authenticated_client().await {
            // Test empty subject
            let result = client.send_email(
                &target_user,
                "",
                "Test message content",
                false
            ).await;
            assert!(result.is_err(), "Should fail with empty subject");

            // Test empty message
            let result = client.send_email(
                &target_user,
                "Test subject",
                "",
                false
            ).await;
            assert!(result.is_err(), "Should fail with empty message");
        }
    }
}

#[tokio::test]
async fn test_send_email_long_message() {
    if let Ok((_, _, target_user)) = get_test_credentials() {
        if let Ok(client) = get_authenticated_client().await {
            let long_message = "A".repeat(1000);
            let result = client.send_email(
                &target_user,
                "Long Message Test",
                &long_message,
                false
            ).await;
            
            match result {
                Ok(response) => {
                    assert_eq!(response.emailuser.result, "Success");
                    println!("Long email sent successfully");
                },
                Err(e) => {
                    println!("Long message error: {}", e);
                    // Check if error is due to length limitations
                    assert!(e.to_string().contains("length") || e.to_string().contains("limit"),
                           "Error should be related to message length");
                }
            }
        }
    }
}

#[tokio::test]
async fn test_rate_limit() {
    if let Ok((_, _, target_user)) = get_test_credentials() {
        if let Ok(client) = get_authenticated_client().await {
            let mut hit_rate_limit = false;
            
            for i in 1..=3 {
                let result = client.send_email(
                    &target_user,
                    &format!("Rate Limit Test {}", i),
                    "Test message",
                    false
                ).await;
                
                match result {
                    Ok(_) => {
                        println!("Email {} sent successfully", i);
                        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    },
                    Err(e) => {
                        if e.to_string().contains("rate") {
                            hit_rate_limit = true;
                            println!("Rate limit detected as expected");
                            break;
                        }
                        println!("Email {} failed: {}", i, e);
                    }
                }
            }
            
            if !hit_rate_limit {
                println!("Note: Rate limit was not hit during testing");
            }
        }
    }
}

#[tokio::test]
async fn test_token_handling() {
    if let Ok((_, _, target_user)) = get_test_credentials() {
        if let Ok(client) = get_authenticated_client().await {
            // Send two emails to test token handling
            let result1 = client.send_email(
                &target_user,
                "Token Test 1",
                "First test message",
                false
            ).await;
            
            assert!(result1.is_ok(), "First email should succeed");
            
            // Small delay between requests
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            
            let result2 = client.send_email(
                &target_user,
                "Token Test 2",
                "Second test message",
                false
            ).await;
            
            assert!(result2.is_ok(), "Second email should succeed");
        }
    }
}