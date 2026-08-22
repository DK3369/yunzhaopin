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
