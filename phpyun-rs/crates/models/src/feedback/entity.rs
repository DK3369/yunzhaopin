use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Feedback {
    pub id: u64,
    pub uid: Option<u64>,
    pub category: String,
    pub content: String,
    pub contact: String,
    pub client_ip: String,
    pub status: i32,
    pub created_at: i64,
}
