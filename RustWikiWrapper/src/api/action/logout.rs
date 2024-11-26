use crate::api::MediaWikiClient;
use std::error::Error;
use std::collections::HashMap;

impl MediaWikiClient{
    pub async fn logout(&self) -> Result<(), Box<dyn Error>> {

        // retrieve the CSRF token (used for logout)
        let token_response = self.get(
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
        self.post("w/api.php", &params).await?;
    
        Ok(())
    }
}

