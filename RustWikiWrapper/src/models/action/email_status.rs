use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EmailUserResponse {
    pub emailuser: EmailUserResult,
}

#[derive(Debug, Deserialize)]
pub struct EmailUserResult {
    pub result: String,
    pub message: Option<String>,
}
