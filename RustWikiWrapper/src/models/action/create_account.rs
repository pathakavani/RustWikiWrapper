// Model for Create Account Response
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAccountResponse {
    pub status: String,
    pub username: Option<String>,
    pub user_id: Option<u64>,
    pub message: Option<String>,
    // A placeholder for account creation structure
}