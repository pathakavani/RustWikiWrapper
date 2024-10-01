use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub login: LoginResult,
}

#[derive(Debug, Deserialize)]
pub struct LoginResult {
    pub result: String,    // Indicates the result of the login operation (e.g., "Success", "Fail")
    pub lguserid: Option<u32>,  // User ID (only present on success)
    pub lgusername: Option<String>, // Username (only present on success)
    pub reason: Option<String>, // Reason for failure (only present on failure)
    pub token: Option<String>, // A new token may be provided for some cases
}