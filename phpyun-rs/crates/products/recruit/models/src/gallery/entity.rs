//! `phpyun_company_show` / `phpyun_resume_show` -- company environment showcase /
//! personal portfolio images.
//!
//! Both tables share the same schema (uid/title/picurl/sort) and reuse one entity.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct GalleryItem {
    pub id: u64,
    pub uid: u64,
    pub title: String,
    pub picurl: String,
    pub sort: i32,
}

/// Which table: company environment / personal portfolio.
#[derive(Debug, Clone, Copy)]
pub enum GalleryKind {
    Company,
    Resume,
}

impl GalleryKind {
    pub fn table(&self) -> &'static str {
        match self {
            Self::Company => "phpyun_company_show",
            Self::Resume => "phpyun_resume_show",
        }
    }
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "company" => Some(Self::Company),
            "resume" => Some(Self::Resume),
            _ => None,
        }
    }
}
