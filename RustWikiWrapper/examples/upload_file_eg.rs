use std::error::Error;
use dotenv::dotenv;
use std::env;
use rust_wiki_wrapper::api::MediaWikiClient;
use rust_wiki_wrapper::api::action::{login, upload_file};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    // Read configuration
    let base_url = "https://test.wikipedia.org"; // Replace with your MediaWiki instance URL
    let username = env::var("WIKI_USERNAME").expect("WIKI_USERNAME must be set in .env file");
    let password = env::var("WIKI_PASSWORD").expect("WIKI_PASSWORD must be set in .env file");
    let file_to_upload = env::var("UPLOAD_FILEPATH").expect("file path or url should be set"); // Replace with the path to your local file
    let file_name = env::var("UPLOAD_FILENAME").expect("file Name should be set"); // Replace with the desired file name on the wiki

    // Step 1: Initialize the MediaWiki client
    let client = MediaWikiClient::new(base_url);

    // Step 2: Log in using the login API
    println!("Logging in as {}...", username);
    let login_result = login::login(&client, &username, &password).await?;

    // Verify login success
    match login_result.clientlogin.status.as_str() {
        "PASS" => {
            println!(
                "Successfully logged in as {}",
                login_result.clientlogin.username.unwrap_or_default()
            );
        }
        _ => {
            eprintln!("Login failed: {:?}", login_result);
            return Err("Login failed".into());
        }
    }

    // Step 3: Upload a file
    println!("Uploading file: {}", file_name);
    let upload_result = upload_file::upload_file(
        &client,
        &file_name,
        &file_to_upload,
        Some("Uploaded using RustWikiWrapper"),
    )
    .await;

    // Handle the upload result
    match upload_result {
        Ok(response) => {
            println!("Upload successful! Response:\n{}", response);
        }
        Err(e) => {
            eprintln!("Upload failed: {}", e);
        }
    }

    Ok(())
}