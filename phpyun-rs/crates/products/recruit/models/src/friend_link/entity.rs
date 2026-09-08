use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct FriendLink {
    pub id: u64,
    pub name: String,
    pub url: String,
    pub logo: String,
    pub category: String,
    pub sort: i32,
    pub status: i32,
    pub created_at: i64,
}

/// PHP `set_friendlink::getInfo_action` column names.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct FriendLinkPhpRow {
    pub id: u64,
    pub link_name: String,
    pub link_url: String,
    pub pic: String,
    pub link_type: String,
    pub link_sorting: i32,
    pub did: i32,
    pub tem_type: i32,
    pub img_type: i32,
    pub link_state: i32,
    pub link_time: String,
    pub statusbody: String,
}
