//! `phpyun_company_news` / `phpyun_company_product` -- company news / products.
//!
//! The two tables share essentially the same schema; we use one entity and
//! dispatch on `kind` to choose the table name.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CompanyContent {
    pub id: u64,
    pub uid: u64,
    pub title: String,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub file: Option<String>,
    /// 0 = pending review / 1 = approved / 2 = rejected
    pub status: i32,
    #[serde(default)]
    pub statusbody: Option<String>,
    pub ctime: i64,
    pub did: u32,
    pub usertype: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum ContentKind {
    News,
    Product,
}

impl ContentKind {
    pub fn table(&self) -> &'static str {
        match self {
            Self::News => "phpyun_company_news",
            Self::Product => "phpyun_company_product",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "news" => Some(Self::News),
            "product" => Some(Self::Product),
            _ => None,
        }
    }
}
