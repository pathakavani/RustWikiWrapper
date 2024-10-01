use crate::api::MediaWikiClient;
use crate::models::action::create_account::CreateAccountResponse;
use std::error::Error;

impl MediaWikiClient {
    pub async fn create_account(
        &self,
        _username: &str,
        _password: &str,
        _email: Option<&str>,
    ) -> Result<CreateAccountResponse, Box<dyn Error>> {
        unimplemented!()
    }
}