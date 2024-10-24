use crate::api::MediaWikiClient as Client;
use crate::models::action::upload_response::UploadResponse;
use reqwest::multipart;
use reqwest::Body;
use std::error::Error;
use std::path::Path;
use tokio::fs::File as TokioFile;
use tokio_util::io::ReaderStream;

pub async fn upload_file(
    client: &Client,
    base_url: &str,
    filename: &str,
    filepath: &str,
    comment: Option<&str>,
) -> Result<UploadResponse, Box<dyn Error>> {
    // Step 1: Obtain a CSRF token
    let token_response = client
        .get(
            "w/api.php",
            &[("action", "query"), ("meta", "tokens"), ("format", "json")],
        )
        .await?
        .json::<serde_json::Value>()
        .await?;

    let csrf_token = token_response["query"]["tokens"]["csrftoken"]
        .as_str()
        .ok_or("Failed to obtain CSRF token")?;

    // Step 2: Open the file to be uploaded using async tokio file
    let file_path = Path::new(filepath);
    let file = TokioFile::open(file_path).await?;

    // Step 3: Convert the file into a stream using ReaderStream
    let file_stream = ReaderStream::new(file);

    // Convert the file stream into reqwest::Body
    let body = Body::wrap_stream(file_stream);

    // Step 4: Create the file part for multipart form
    let file_part = multipart::Part::stream(body).file_name(filename.to_string());

    // Step 5: Create a multipart form to upload the file
    let mut form = multipart::Form::new()
        .text("action", "upload")
        .text("filename", filename.to_string())
        .text("format", "json")
        .text("token", csrf_token.to_string())
        .text("ignorewarnings", "1")
        .part("file", file_part);

    // Optionally add a comment
    if let Some(comment_text) = comment {
        form = form.text("comment", comment_text.to_string());
    }

    // Step 6: Send the POST request to upload the file
    let upload_response = client
        .client
        .post(&format!("{}/w/api.php", base_url))
        .multipart(form)
        .send()
        .await?
        .json::<UploadResponse>()
        .await?;

    Ok(upload_response)
}
