use crate::api::MediaWikiClient;
use crate::models::action::email_status::EmailUserResponse;
use std::collections::HashMap;
use std::error::Error;

impl MediaWikiClient {
    pub async fn send_email(
        &self,
        target_user: &str,
        subject: &str,
        message: &str,
        cc_me: bool,
    ) -> Result<EmailUserResponse, Box<dyn Error>> {
        // get CSRF token
        let token_response = self
            .get(
                "w/api.php",
                &[("action", "query"), ("meta", "tokens"), ("format", "json")],
            )
            .await?;

        let token_data: serde_json::Value = token_response.json().await?;
        let email_token = token_data["query"]["tokens"]["csrftoken"]
            .as_str()
            .ok_or("Failed to obtain email token")?;

        // prepare parameters
        let mut params = HashMap::new();
        params.insert("action", "emailuser");
        params.insert("target", target_user);
        params.insert("subject", subject);
        params.insert("text", message);
        params.insert("token", email_token);
        params.insert("ccme", if cc_me { "1" } else { "0" });
        params.insert("format", "json");

        // send the email request
        let response = self.post("w/api.php", &params).await?;
        let email_response: EmailUserResponse = response.json().await?;

        Ok(email_response)
    }
}
