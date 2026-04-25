use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct HrDoc {
    pub id: u64,
    pub cid: u64,
    pub name: String,
    pub url: String,
    pub body: String,
    pub hits: u32,
    pub is_show: i32,
    pub created_at: i64,
    pub updated_at: i64,
}
