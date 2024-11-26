use crate::api::MediaWikiClient;
use crate::models::action::create_edit_page::CreateEditPageResponse;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

impl MediaWikiClient {
    pub async fn create_edit_page(
        &self,
        title: &str,
        content: &str,
        summary: Option<&str>,
    ) -> Result<CreateEditPageResponse, Box<dyn Error>> {
        // get CSRF token
        let token_response = self
            .get(
                "w/api.php",
                &[
                    ("action", "query"),
                    ("meta", "tokens"),
                    ("type", "csrf"),
                    ("format", "json"),
                ],
            )
            .await?;

        let token_data: Value = token_response.json().await?;
        let csrf_token = token_data["query"]["tokens"]["csrftoken"]
            .as_str()
            .ok_or("Failed to obtain CSRF token")?;

        // prepare edit parameters
        let mut edit_params = HashMap::new();
        edit_params.insert("action", "edit");
        edit_params.insert("title", title);
        edit_params.insert("text", content);
        edit_params.insert("token", csrf_token);

        // add summary if provided
        if let Some(edit_summary) = summary {
            edit_params.insert("summary", edit_summary);
        }

        // send edit request
        let edit_response = self
            .post("w/api.php?format=json", &edit_params)
            .await?;
            
        println!("Raw Response: {:?}", edit_response);    
            
        let parsed_response= edit_response
            .json::<CreateEditPageResponse>()
            .await?;

        

        // return the response
        Ok(parsed_response)
    }
}

// additional helper function for watching/unwatching pages
impl MediaWikiClient {
    pub async fn set_page_watched(&self, title: &str, watch: bool) -> Result<(), Box<dyn Error>> {
        // get CSRF token
        let token_response = self
            .get(
                "w/api.php",
                &[
                    ("action", "query"),
                    ("meta", "tokens"),
                    ("type", "csrf"),
                    ("format", "json"),
                ],
            )
            .await?;

        let token_data: Value = token_response.json().await?;
        let csrf_token = token_data["query"]["tokens"]["csrftoken"]
            .as_str()
            .ok_or("Failed to obtain CSRF token")?;

        // prepare watch parameters
        let mut watch_params = HashMap::new();
        watch_params.insert("action", if watch { "watch" } else { "unwatch" });
        watch_params.insert("title", title);
        watch_params.insert("token", csrf_token);

        // send watch request
        self.post("w/api.php?format=json", &watch_params).await?;

        Ok(())
    }
}
