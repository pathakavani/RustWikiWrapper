use crate::api::MediaWikiClient;
use std::error::Error;
use std::collections::HashMap;
use crate::models::action::email_status::EmailUserResponse;

pub async fn send_email(
    client: &MediaWikiClient,
    target_user: &str,
    subject: &str,
    message: &str,
    cc_me: bool,
) -> Result<EmailUserResponse, Box<dyn Error>> {

    // Step 1: Obtain the email token (equal to edit token)
    let token_response = client.get(
        "w/api.php",
        &[("action", "query"), ("meta", "tokens"), ("format", "json")]
    ).await?.json::<serde_json::Value>().await?;
    
    let email_token = token_response["query"]["tokens"]["csrftoken"]
        .as_str()
        .ok_or("Failed to obtain email token")?;

    // Step 2: Prepare the parameters for sending the email
    let mut params = HashMap::new();
    params.insert("action", "emailuser");
    params.insert("target", target_user);
    params.insert("subject", subject);
    params.insert("text", message);
    params.insert("token", email_token); // Token obtained in the previous request
    params.insert("ccme", if cc_me { "true" } else { "false" });
    params.insert("format", "json");

    // Step 3: Send the email request using the post method
    let email_response = client.post("w/api.php", &params).await?.json::<EmailUserResponse>().await?;

    Ok(email_response)
}