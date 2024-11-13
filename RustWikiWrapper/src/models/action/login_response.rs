use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub clientlogin: ClientLoginResult,
}

#[derive(Debug, Deserialize)]
pub struct ClientLoginResult {
    pub status: String,
    #[serde(default)]
    pub username: Option<String>,
}


#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub batchcomplete: String,
    pub query: QueryTokens,
}

#[derive(Debug, Deserialize)]
pub struct QueryTokens {
    pub tokens: Tokens,
}

#[derive(Debug, Deserialize)]
pub struct Tokens {
    pub logintoken: String,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub error: ErrorInfo,
}

#[derive(Debug, Deserialize)]
pub struct ErrorInfo {
    pub code: String,
    pub info: String,
}