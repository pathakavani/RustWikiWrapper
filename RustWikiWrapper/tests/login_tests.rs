use dotenv::dotenv;
use rust_wiki_wrapper:: api::MediaWikiClient;
use std::env;

// Helper function to get test credentials from environment variables
fn get_test_credentials() -> (String, String) {
    dotenv().ok(); // Load .env file if present
    let username =
        env::var("WIKI_TEST_USERNAME").expect("WIKI_TEST_USERNAME must be set in environment");
    let password =
        env::var("WIKI_TEST_PASSWORD").expect("WIKI_TEST_PASSWORD must be set in environment");
    (username, password)
}

#[tokio::test]
async fn test_successful_login() {
    let (username, password) = get_test_credentials();
    let client = MediaWikiClient::new("https://test.wikipedia.org");

    let result = client.login(&username, &password).await;

    match result {
        Ok(response) => {
            println!("status: {}", response.clientlogin.status);
            assert_eq!(response.clientlogin.status.as_str(), "PASS");
            println!(
                "Login successful with message: {:?}",
                response.clientlogin.status
            );
        }
        Err(e) => {
            panic!("Login failed: {}", e);
        }
    }
}

#[tokio::test]
async fn test_failed_login() {
    let client = MediaWikiClient::new("https://test.wikipedia.org");

    let result = client.login("invalid_user", "invalid_password").await;

    assert!(
        result.is_err(),
        "Login should fail with invalid credentials"
    );
    if let Err(e) = result {
        println!("Expected error occurred: {}", e);
        assert!(
            e.to_string().contains("FAIL"),
            "Error should indicate failed login"
        );
    }
}

#[tokio::test]
async fn test_empty_credentials() {
    let client = MediaWikiClient::new("https://test.wikipedia.org");

    // Test empty username
    let empty_user_result = client.login("", "some_password").await;
    assert!(
        empty_user_result.is_err(),
        "Should fail with empty username"
    );

    // Test empty password
    let empty_pass_result = client.login("some_user", "").await;
    assert!(
        empty_pass_result.is_err(),
        "Should fail with empty password"
    );
}

#[tokio::test]
async fn test_rate_limit_handling() {
    let client = MediaWikiClient::new("https://test.wikipedia.org");
    let (username, password) = get_test_credentials();

    // Attempt multiple logins in quick succession to trigger rate limiting
    for i in 0..5 {
        let result = client.login(&username, &password).await;
        match result {
            Ok(_) => println!("Login attempt {} succeeded", i + 1),
            Err(e) => {
                println!("Login attempt {} failed: {}", i + 1, e);
                // Don't fail the test if we hit rate limiting - it's expected
                if e.to_string().contains("rate limit") {
                    println!("Rate limit detected as expected");
                    return;
                }
            }
        }

        // Add a small delay between attempts
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

