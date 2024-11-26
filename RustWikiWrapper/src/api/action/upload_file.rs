use crate::api::MediaWikiClient;
use std::collections::HashMap;
use std::error::Error;


impl MediaWikiClient{
    pub async fn upload_file(
        &self,
        filename: &str,
        file_url: &str,
        comment: Option<&str>,
    ) -> Result<String, Box<dyn Error>> {
        // Step 1: Retrieve a CSRF token
        let token_response = self
            .get(
                "w/api.php",
                &[("action", "query"), ("meta", "tokens"), ("format", "json")],
            )
            .await
            .map_err(|e| format!("Failed to fetch CSRF token: {}", e))?
            .json::<serde_json::Value>()
            .await
            .map_err(|e| format!("Failed to parse CSRF token response: {}", e))?;
    
        let csrf_token = token_response["query"]["tokens"]["csrftoken"]
            .as_str()
            .ok_or("Failed to obtain CSRF token from the API response")?;
    
        // Step 2: Prepare the form data
        let mut form_data = HashMap::new();
        form_data.insert("action", "upload");
        form_data.insert("filename", filename);
        form_data.insert("url", file_url);
        form_data.insert("format", "json");
        form_data.insert("token", csrf_token);
        form_data.insert("ignorewarnings", "1");
    
        // Optionally add a comment
        if let Some(comment_text) = comment {
            form_data.insert("comment", comment_text);
        }
    
        // Step 3: Send the POST request using MediaWikiClient's post method
        let upload_response = self
            .post("w/api.php", &form_data)
            .await
            .map_err(|e| format!("Failed to send upload request: {}", e))?;
    
        let status = upload_response.status();
        let response = upload_response
            .text()
            .await
            .map_err(|e| format!("Failed to read response text: {}", e))?;
    
        // Debugging: Print response details
        println!("Upload request status: {}", status);
        println!("Raw upload response: {}", response);
    
        // Check for success
        let response_json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| format!("Failed to parse upload response JSON: {}", e))?;
        if response_json["upload"]["result"] != "Success" {
            return Err(format!(
                "Upload failed: {:?}",
                response_json["upload"].get("warnings").unwrap_or(&response_json)
            )
            .into());
        }
    
        Ok(response)
    }

}

