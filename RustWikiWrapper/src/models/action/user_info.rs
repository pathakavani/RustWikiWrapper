// Model for User Info Response
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfoResponse {
    pub query: QueryResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResult {
    pub userinfo: UserInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub editcount: Option<u32>,
    pub registration: Option<String>,
    pub groups: Option<Vec<String>>,
    pub rights: Option<Vec<String>>,

    // A placeholder for current user info
}