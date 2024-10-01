use crate::api::MediaWikiClient;
use crate::models::action::create_edit_page::CreateEditPageResponse;
use std::error::Error;

impl MediaWikiClient {
    pub async fn create_edit_page(
        &self,
        _title: &str,
        _content: &str,
        _summary: Option<&str>,
        _token: &str,
    ) -> Result<CreateEditPageResponse, Box<dyn Error>> {
        unimplemented!()
    }
}
