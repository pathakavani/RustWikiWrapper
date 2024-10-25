use crate::api::MediaWikiClient as Client;
use crate::models::action::page_content::PageContent;
use std::error::Error;

pub async fn get_page_content(client: &Client, title: &str) -> Result<PageContent, Box<dyn Error>> {
    // prepare the parameters for the API request
    let params = &[
        ("action", "query"),
        ("prop", "revisions"),
        ("titles", title),
        ("format", "json"),
        ("formatversion", "2"),
        ("rvprop", "content|ids"),
        ("rvslots", "main"),
    ];

    // send the GET request to fetch page content
    let response = client
        .get("w/api.php", params)
        .await?
        .json::<serde_json::Value>()
        .await?;

    // extract the page from the response
    let page = response["query"]["pages"]
        .as_array()
        .and_then(|pages| pages.first())
        .ok_or("No page found in response")?;

    // Check if the page exists
    if page.get("missing").is_some() {
        return Err(format!("Page '{}' does not exist", title).into());
    }

    // extract content from revisions
    let content = page["revisions"]
        .as_array()
        .and_then(|revs| revs.first())
        .and_then(|rev| rev["slots"]["main"]["content"].as_str())
        .ok_or("Failed to extract page content")?;

    // construct the PageContent struct
    let page_content = PageContent {
        pageid: page["pageid"].as_i64().unwrap_or_default(),
        ns: page["ns"].as_i64().unwrap_or_default() as i32,
        title: page["title"].as_str().unwrap_or_default().to_string(),
        content: content.to_string(),
        content_model: page["revisions"][0]["slots"]["main"]["contentmodel"]
            .as_str()
            .unwrap_or("wikitext")
            .to_string(),
        page_language: page["pagelanguage"]
            .as_str()
            .unwrap_or("en")
            .to_string(),
        revid: page["revisions"][0]["revid"].as_i64(),
        parent_id: page["revisions"][0]["parentid"].as_i64(),
    };

    Ok(page_content)
}