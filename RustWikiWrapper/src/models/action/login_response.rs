use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub clientlogin: ClientLoginResult,
}

#[derive(Debug, Deserialize)]
pub struct ClientLoginResult {
    pub status: String,                 // The status of the login attempt (PASS, FAIL, UI, REDIRECT, RESTART)
    pub username: Option<String>,       // Username of the logged-in user (only present on success)
    pub message: Option<String>,        // Message explaining the result (often used for failure reasons)
    pub requests: Option<Vec<AuthRequest>>, // Additional requests for UI interaction (e.g., 2FA)
    pub token: Option<String>,          // A new login token in case of a continuation (e.g., for UI)
    pub logincontinue: Option<bool>,    // Whether the login process should be continued
    pub redirecttarget: Option<String>, // Redirect target URL in case of third-party login
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub id: String,                    // ID of the request (e.g., "OATH")
    pub fields: Vec<AuthField>,        // Fields that need to be filled in (e.g., OATHToken for 2FA)
}

#[derive(Debug, Deserialize)]
pub struct AuthField {
    pub name: String,                  // The name of the field (e.g., "OATHToken")
    pub type_field: String,            // Type of the field (e.g., "string", "password")
    pub label: Option<String>,         // Optional label describing the field
}