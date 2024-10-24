use crate::api::MediaWikiClient as client;
use crate::models::action::create_edit_page::CreateEditPageResponse;
use std::collections::HashMap;
use std::error::Error;

pub async fn create_edit_page(
    client: &client,
    title: &str,
    content: &str,
    summary: Option<&str>,
    token: &str,
) -> Result<CreateEditPageResponse, Box<dyn Error>> {
    // Prepare the parameters for the page creation/edit request
    let mut params = HashMap::new();
    params.insert("action", "edit");
    params.insert("title", title);
    params.insert("text", content);
    params.insert("token", token); // Editing requires a CSRF token
    params.insert("format", "json");

    // Optionally add the edit summary if provided
    if let Some(summary_text) = summary {
        params.insert("summary", summary_text);
    }

    // Send the POST request to create or edit the page
    let edit_response = client
        .post("w/api.php", &params)
        .await?
        .json::<CreateEditPageResponse>()
        .await?;

    Ok(edit_response)
}
