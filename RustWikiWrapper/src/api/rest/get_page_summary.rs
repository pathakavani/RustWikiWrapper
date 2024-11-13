use crate::api::MediaWikiClient as Client;
use crate::models::rest::page_summary::PageSummary;
use std::error::Error;

pub async fn get_page_summary(client: &Client, title: &str) -> Result<PageSummary, Box<dyn Error>> {
    // encode the title to make it URL-safe
    let encoded_title = urlencoding::encode(title);

    // the API endpoint for page summary
    let endpoint = &format!("page/summary/{}", encoded_title);

    // prepare an empty list of parameters (since the summary endpoint might not need any)
    let params = &[];

    // send the GET request and deserialize the response into the PageSummary struct
    let response = client
        .get(endpoint, params)
        .await?
        .json::<PageSummary>()
        .await?;

    Ok(response)
}
