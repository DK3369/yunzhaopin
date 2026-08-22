//! `phpyun_banner` -- banner images on the company home page.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CompanyBanner {
    pub id: u64,
    pub uid: u64,
    pub pic: String,
    #[serde(default)]
    pub link: Option<String>,
    #[serde(default)]
    pub sort: i32,
    pub addtime: i64,
}
