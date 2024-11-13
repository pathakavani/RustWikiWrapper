use crate::api::MediaWikiClient as client;
use crate::models::rest::metadata::Metadata;
use std::error::Error;

pub async fn get_metadata(client: &client, title: &str) -> Result<Metadata, Box<dyn Error>> {
    // prepare the parameters as a slice of tuples
    let params = &[
        ("action", "query"),
        ("prop", "info"),
        ("titles", title),
        ("format", "json"),
        ("inprop", "url|talkid"),
    ];

    // send the GET request to fetch metadata
    let metadata_response = client
        .get("w/api.php", params)
        .await?
        .json::<serde_json::Value>()
        .await?;

    // parse the response into the Metadata struct
    let pages = metadata_response["query"]["pages"]
        .as_object()
        .ok_or("Failed to parse pages")?;

    // extract the first page's metadata
    let metadata_value = pages
        .values()
        .next()
        .ok_or("No metadata found for the page")?;

    // convert the JSON value into the Metadata struct
    let metadata: Metadata = serde_json::from_value(metadata_value.clone())?;

    Ok(metadata)
}
