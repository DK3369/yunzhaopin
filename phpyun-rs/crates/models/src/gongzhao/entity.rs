use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Gongzhao {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub cover: String,
    pub body: String,
    pub tag: String,
    pub status: i32,
    pub view_count: u32,
    pub start_at: i64,
    pub end_at: i64,
    pub created_at: i64,
}
