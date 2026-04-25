use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Broadcast {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub target_usertype: i32,
    pub status: i32,
    pub issuer_uid: u64,
    pub created_at: i64,
}
