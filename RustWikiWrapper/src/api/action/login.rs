use crate::api::MediaWikiClient;
use crate::models::action::login_response::LoginResponse;
use std::error::Error;
use std::collections::HashMap;
use serde_json::Value;

pub async fn login(
    client: &MediaWikiClient,
    username: &str,
    password: &str,
) -> Result<LoginResponse, Box<dyn Error>> {
    // Step 1: Get login token
    let token_response = client.get(
        "w/api.php",
        &[
            ("action", "query"),
            ("meta", "tokens"),
            ("type", "login"),
            ("format", "json"),
        ],
    ).await?;

    let token_data: Value = token_response.json().await?;
    let login_token = token_data["query"]["tokens"]["logintoken"]
        .as_str()
        .ok_or("Failed to obtain login token")?;

    // Step 2: Prepare login parameters
    let mut login_params = HashMap::new();
    login_params.insert("action", "clientlogin");
    login_params.insert("username", username);
    login_params.insert("password", password);
    login_params.insert("logintoken", login_token);
    login_params.insert("loginreturnurl", "http://localhost:3000");
    login_params.insert("format", "json");

    // Step 3: Send login request
    let login_response = client.post("w/api.php?format=json", &login_params).await?;
    let response_text = login_response.text().await?;
    let login_result: LoginResponse = serde_json::from_str(&response_text)?;
    
    match login_result.clientlogin.status.as_str() {
        "PASS" => Ok(login_result),
        status => Err(format!("Login failed with status: {}", status).into())
    }
}