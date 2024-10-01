use crate::api::MediaWikiClient as Client;
//use serde::Deserialize;
use std::error::Error;
use crate::models::action::login_response::LoginResponse;

pub async fn login(
    _client: &Client,
    _base_url: &str,
    _username: &str,
    _password: &str,
) -> Result<LoginResponse, Box<dyn Error>> {
    unimplemented!()
    // Placeholder for login action API
}