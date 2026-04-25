use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CompanyProduct {
    pub id: u64,
    pub uid: u64,
    pub title: String,
    pub cover: String,
    pub body: String,
    pub status: i32,
    pub sort: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CompanyNews {
    pub id: u64,
    pub uid: u64,
    pub title: String,
    pub summary: String,
    pub body: String,
    pub status: i32,
    pub hits: u32,
    pub created_at: i64,
    pub updated_at: i64,
}
