use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct EvalPaper {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub cover: String,
    pub visits: u32,
    pub status: i32,
    pub created_at: i64,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct EvalQuestion {
    pub id: u64,
    pub paper_id: u64,
    pub content: String,
    pub options: serde_json::Value,
    pub sort: i32,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct EvalLog {
    pub id: u64,
    pub uid: u64,
    pub paper_id: u64,
    pub score: i32,
    pub answers: serde_json::Value,
    pub created_at: i64,
}
