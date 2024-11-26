use crate::api::MediaWikiClient;
use crate::models::action::user_info::UserInfoResponse;
use std::error::Error;

impl MediaWikiClient{
    pub async fn get_current_user(&self) -> Result<UserInfoResponse, Box<dyn Error>> {
        // prepare the parameters for userinfo query
        let params = &[
            ("action", "query"),
            ("meta", "userinfo"),
            ("format", "json"),
            ("uiprop", "blockinfo|groups|rights|editcount|registration|email|gender"),
        ];
    
        // send the GET request
        let response = self
            .get("w/api.php", params)
            .await?
            .json::<UserInfoResponse>()
            .await?;
    
        Ok(response)
    }

}
