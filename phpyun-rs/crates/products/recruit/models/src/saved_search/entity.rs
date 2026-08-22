use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SavedSearch {
    pub id: u64,
    pub uid: u64,
    pub name: String,
    pub kind: String,
    pub params: serde_json::Value,
    pub notify: i32,
    pub last_notified_at: i64,
    pub created_at: i64,
    pub updated_at: i64,
}
