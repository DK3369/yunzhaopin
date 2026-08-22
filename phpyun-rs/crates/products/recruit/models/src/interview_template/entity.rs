use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct InterviewTemplate {
    pub id: u64,
    pub uid: u64,
    pub name: String,
    pub content: String,
    pub address: String,
    pub linkman: String,
    pub linktel: String,
    pub intertime: i64,
    pub status: i32,
    pub created_at: i64,
    pub updated_at: i64,
}
