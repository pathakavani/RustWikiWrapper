use crate::api::MediaWikiClient as Client;
//use serde::Deserialize;
use std::error::Error;
use crate::models::action::email_status::EmailUserResponse;

pub async fn send_email(
    _client: &Client,
   _base_url: &str,
   _target_user: &str,
   _subject: &str,
    _message: &str,
    _cc_me: bool,) -> Result<EmailUserResponse, Box<dyn Error>> {
   
    unimplemented!()
    // Placeholder for send email action API
}