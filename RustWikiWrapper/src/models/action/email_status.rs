use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct EmailUserResponse {
    pub emailuser: EmailUserResult,
}

// Struct for the email result
#[derive(Debug, Deserialize)]
pub struct EmailUserResult {
    pub result: String,    // The result of the email attempt (e.g., "Success", "Failure")
    pub error: Option<String>, // Error message if any
}