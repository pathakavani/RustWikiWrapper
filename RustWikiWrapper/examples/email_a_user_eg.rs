use dotenv::dotenv;
use rust_wiki_wrapper::api::MediaWikiClient;
use std::env;
use std::error::Error;

async fn send_wiki_email(target: &str, subject: &str, message: &str) -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let username = env::var("WIKI_USERNAME").expect("WIKI_USERNAME must be set in .env file");
    let password = env::var("WIKI_PASSWORD").expect("WIKI_PASSWORD must be set in .env file");

    let client = MediaWikiClient::new("https://test.wikipedia.org");

    // first login using the login function
    println!("Logging in as {}...", username);
    let login_result = client.login(&username, &password).await?;

    match login_result.clientlogin.status.as_str() {
        "PASS" => {
            println!(
                "Successfully logged in as {}",
                login_result.clientlogin.username.unwrap_or_default()
            );

            // now send email
            let result = client
                .send_email(
                    target, subject, message, true, // cc me
                )
                .await?;

            match result.emailuser.result.as_str() {
                "Success" => println!("Email sent successfully!"),
                _ => println!(
                    "Failed to send email: {}",
                    result
                        .emailuser
                        .message
                        .unwrap_or_else(|| "Unknown error".to_string())
                ),
            }
            Ok(())
        }
        _ => Err("Login failed".into()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let target = "Sree0521";
    let subject = "Hello from Rust";
    let message = "This is a test email sent via the MediaWiki API";

    match send_wiki_email(target, subject, message).await {
        Ok(_) => println!("Email process completed successfully"),
        Err(e) => println!("Error during email process: {}", e),
    }

    Ok(())
}
