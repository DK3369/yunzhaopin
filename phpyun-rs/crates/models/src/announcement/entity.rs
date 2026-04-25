use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Announcement {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub content: String,
    pub view_num: u32,
    pub datetime: i64,
    pub startime: i64,
    pub endtime: i64,
    pub did: u64,
    pub status: i32,
    pub created_at: i64,
}
