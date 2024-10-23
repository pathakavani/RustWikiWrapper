use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateAccountResponse {
    pub createaccount: CreateAccountResult,
}

#[derive(Debug, Deserialize)]
pub struct CreateAccountResult {
    pub result: String,  // e.g., "Success", "Failure"
    pub username: Option<String>,  // Username of the created account (on success)
    pub reason: Option<String>,  // Reason for failure (if any)
}