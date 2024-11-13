use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub anon: Option<String>,  // present for anonymous users
    pub rights: Option<Vec<String>>,
    pub groups: Option<Vec<String>>,
    pub blockid: Option<i32>,
    pub blockedby: Option<String>,
    pub blockreason: Option<String>,
    pub blockexpiry: Option<String>,
    pub hastalked: Option<bool>,
    pub messages: Option<bool>,
    pub editcount: Option<i32>,
    pub registration: Option<String>,
    pub implicitgroups: Option<Vec<String>>,
    pub email: Option<String>,
    pub emailable: Option<bool>,
    pub gender: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfoResponse {
    pub batchcomplete: Option<String>,
    pub query: QueryResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResponse {
    pub userinfo: UserInfo,
}