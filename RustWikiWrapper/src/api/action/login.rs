use crate::api::MediaWikiClient as Client;
use std::error::Error;
use crate::models::action::login_response::LoginResponse;
use std::collections::HashMap;

pub async fn login(
    client: &Client,
    username: &str,
    password: &str,
    loginreturnurl: &str,  // URL to return to after login
) -> Result<LoginResponse, Box<dyn Error>> {

    // Step 1: Retrieve the login token (required for clientlogin)
    let token_response = client.get(
        "w/api.php",
        &[("action", "query"), ("meta", "tokens"), ("type", "login"), ("format", "json")]
    ).await?.json::<serde_json::Value>().await?;

    let login_token = token_response["query"]["tokens"]["logintoken"]
        .as_str()
        .ok_or("Failed to obtain login token")?;

    // Step 2: Prepare parameters for the clientlogin POST request
    let mut login_params = HashMap::new();
    login_params.insert("action", "clientlogin");
    login_params.insert("username", username);
    login_params.insert("password", password);
    login_params.insert("logintoken", login_token);
    login_params.insert("loginreturnurl", loginreturnurl);
    login_params.insert("format", "json");

    // Step 3: Send a POST request to login using clientlogin
    let login_response = client.post("w/api.php", &login_params)
        .await?
        .json::<LoginResponse>()
        .await?;

    // Step 4: Check the login response for success, failure, or further action
    match login_response.clientlogin.status.as_str() {
        "PASS" => {
            // Login successful
            Ok(login_response)
        }
        "FAIL" => {
            // Login failed
            Err("Login failed".into())
        }
        "UI" => {
            // UI response received (additional input required, like 2FA)
            // Here, you might have to prompt the user for additional fields such as 2FA code
            Err("Additional user input required (UI)".into())
        }
        "REDIRECT" => {
            // REDIRECT response received, handle as needed (e.g., open a browser for third-party authentication)
            Err("Redirect required to complete login".into())
        }
        "RESTART" => {
            // Authentication succeeded, but there is no linked user account
            Err("Authentication worked but no linked user account".into())
        }
        _ => Err("Unexpected response during login".into())
    }
}