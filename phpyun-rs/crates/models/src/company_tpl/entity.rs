//! `phpyun_company_tpl` -- catalogue of company templates (purchasable themes/skins).

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CompanyTpl {
    pub id: u64,
    pub name: String,
    pub url: String,
    /// `pic` relative path (frontend rendering requires checkpic).
    pub pic: Option<String>,
    /// 1 = points / 2 = balance (aligns with PHP `type`).
    pub r#type: i32,
    pub price: i32,
    /// 0 = disabled / 1 = enabled.
    pub status: i32,
    /// Sort weight (larger = earlier).
    pub sort: i32,
}
