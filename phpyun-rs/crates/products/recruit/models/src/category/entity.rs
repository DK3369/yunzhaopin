use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Category {
    pub id: u64,
    pub parent_id: u64,
    pub kind: String,
    pub name: String,
    pub sort: i32,
    pub status: i32,
    pub updated_at: i64,
}
