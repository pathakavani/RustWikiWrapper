use crate::api::MediaWikiClient;
use crate::models::action::user_info::UserInfoResponse;
use std::error::Error;

pub async fn get_current_user(client: &MediaWikiClient) -> Result<UserInfoResponse, Box<dyn Error>> {
    // prepare the parameters for userinfo query
    let params = &[
        ("action", "query"),
        ("meta", "userinfo"),
        ("format", "json"),
        ("uiprop", "blockinfo|groups|rights|editcount|registration|email|gender"),
    ];

    // send the GET request
    let response = client
        .get("w/api.php", params)
        .await?
        .json::<UserInfoResponse>()
        .await?;

    Ok(response)
}