use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

/// PHP `phpyun_wxnav` (weixinmenu). JSON `type` matches the table column.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct WxNav {
    pub id: u64,
    pub name: String,
    pub keyid: i32,
    pub key: String,
    pub url: String,
    #[serde(rename = "type")]
    pub nav_type: String,
    pub sort: i32,
    pub appid: String,
    pub apppage: String,
}
