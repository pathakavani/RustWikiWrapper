use crate::api::MediaWikiClient;
use crate::models::action::create_account::CreateAccountResponse;
use std::error::Error;
use std::collections::HashMap;

impl MediaWikiClient {
    pub async fn create_account(
        &self,
        username: &str,
        password: &str,
        email: Option<&str>,  // Email is optional
    ) -> Result<CreateAccountResponse, Box<dyn Error>> {

        // Step 1: Retrieve the createaccount token
        let token_response = self.get(
            "w/api.php",
            &[("action", "query"), ("meta", "tokens"), ("type", "createaccount"), ("format", "json")]
        ).await?.json::<serde_json::Value>().await?;

        let create_token = token_response["query"]["tokens"]["createaccounttoken"]
            .as_str()
            .ok_or("Failed to obtain createaccount token")?;

        // Step 2: Prepare parameters for the account creation request
        let mut create_params = HashMap::new();
        create_params.insert("action", "createaccount");
        create_params.insert("createtoken", create_token);
        create_params.insert("username", username);
        create_params.insert("password", password);
        create_params.insert("retype", password);  // Retyping password for confirmation
        create_params.insert("createreturnurl", "http://example.com");  // Return URL after account creation
        create_params.insert("format", "json");

        // Optionally add email if provided
        if let Some(email_address) = email {
            create_params.insert("email", email_address);
        }

        // Step 3: Send the POST request to create an account
        let account_creation_response = self.post("w/api.php", &create_params)
            .await?
            .json::<CreateAccountResponse>()
            .await?;

        // Return the parsed account creation response
        Ok(account_creation_response)
    }
} 