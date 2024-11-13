use rust_wiki_wrapper::api::action::login;
use rust_wiki_wrapper::api::MediaWikiClient;
use std::error::Error;
use dotenv::dotenv;
use std::env;
use std::collections::HashMap;

async fn upload_file_with_session(
    filepath: &str,
    target_filename: &str,
    comment: Option<&str>
) -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let username = env::var("WIKI_USERNAME")
        .expect("WIKI_USERNAME must be set in .env file");
    let password = env::var("WIKI_PASSWORD")
        .expect("WIKI_PASSWORD must be set in .env file");

    let client = MediaWikiClient::new("https://test.wikipedia.org");

    // Step 1: Login and get session
    println!("Logging in as {}...", username);
    let login_result = login::login(&client, &username, &password).await?;
    
    if login_result.clientlogin.status != "PASS" {
        return Err("Login failed".into());
    }
    println!("Successfully logged in as {}", 
        login_result.clientlogin.username.unwrap_or_default());

    // Verify file exists
    let file = tokio::fs::File::open(filepath).await?;
    let metadata = file.metadata().await?;
    println!("File exists, size: {} bytes", metadata.len());

    // Step 2: Get CSRF token for upload
    let token_response = client.get(
        "w/api.php",
        &[
            ("action", "query"),
            ("meta", "tokens"),
            ("format", "json"),
        ],
    ).await?;
    
    let token_data: serde_json::Value = token_response.json().await?;
    let csrf_token = token_data["query"]["tokens"]["csrftoken"]
        .as_str()
        .ok_or("Failed to obtain CSRF token")?;

    println!("Got CSRF token: {}", csrf_token);

    // Step 3: Prepare upload parameters
    let mut upload_params = HashMap::new();
    upload_params.insert("action", "upload");
    upload_params.insert("filename", target_filename);
    upload_params.insert("token", csrf_token);
    upload_params.insert("format", "json");
    upload_params.insert("ignorewarnings", "1");

    if let Some(cmt) = comment {
        upload_params.insert("comment", cmt);
    }

    // Create form data
    let form = reqwest::multipart::Form::new()
        .text("action", "upload")
        .text("filename", target_filename.to_string())
        .text("format", "json")
        .text("token", csrf_token.to_string())
        .text("ignorewarnings", "1");

    // Add the file
    let file = tokio::fs::File::open(filepath).await?;
    let stream = tokio_util::codec::FramedRead::new(file, tokio_util::codec::BytesCodec::new());
    let file_stream = reqwest::Body::wrap_stream(stream);
    
    let form = form.part("file", 
        reqwest::multipart::Part::stream(file_stream)
            .file_name(target_filename.to_string())
            .mime_str("application/octet-stream")?
    );

    println!("Uploading file: {}", filepath);
    println!("Target filename on wiki: {}", target_filename);

    // Step 4: Send upload request
    let response = client.client
        .post(format!("{}/w/api.php", client.base_url))
        .multipart(form)
        .send()
        .await?;

    let status = response.status();
    let response_text = response.text().await?;

    // Step 5: Parse response
    if response_text.contains("\"result\":\"Success\"") {
        println!("Upload successful!");
        let response_json: serde_json::Value = serde_json::from_str(&response_text)?;
        if let Some(url) = response_json["upload"]["imageinfo"]["url"].as_str() {
            println!("File URL: {}", url);
        }
        Ok(())
    } else {
        Err(format!("Upload failed. Status: {}. Response: {}", status, response_text).into())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let filepath = "/Users/tejasunkara/Projects/secure_programming_project/Wikipediahelp/rust-2.jpg";
    let target_filename = "TestImage.jpg";
    let upload_comment = Some("Test upload via RustWikiWrapper");

    match upload_file_with_session(filepath, target_filename, upload_comment).await {
        Ok(_) => println!("File upload process completed successfully"),
        Err(e) => println!("Error during upload process: {}", e),
    }

    Ok(())
}