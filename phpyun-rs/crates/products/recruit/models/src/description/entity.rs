use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DescClass {
    pub id: u64,
    pub name: String,
    pub sort: i32,
    pub created_at: i64,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Description {
    pub id: u64,
    pub class_id: u64,
    pub title: String,
    pub content: String,
    pub is_type: i32,
    pub link_url: String,
    pub sort: i32,
    pub status: i32,
    pub created_at: i64,
    pub updated_at: i64,
}
