use crate::api::MediaWikiClient as Client;
use std::error::Error;
use std::collections::HashMap;

pub async fn logout(client: &Client) -> Result<(), Box<dyn Error>> {

    // retrieve the CSRF token (used for logout)
    let token_response = client.get(
        "w/api.php",
        &[("action", "query"), ("meta", "tokens"), ("format", "json")]
    ).await?.json::<serde_json::Value>().await?;
    
    let csrf_token = token_response["query"]["tokens"]["csrftoken"]
        .as_str()
        .ok_or("Failed to obtain CSRF token")?;

    // prepare parameters for the logout action
    let mut params = HashMap::new();
    params.insert("action", "logout");
    params.insert("token", csrf_token);
    params.insert("format", "json");

    // send the POST request to logout
    client.post("w/api.php", &params).await?;

    Ok(())
}