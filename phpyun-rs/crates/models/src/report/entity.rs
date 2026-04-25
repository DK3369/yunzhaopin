use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Report {
    pub id: u64,
    pub reporter_uid: u64,
    pub target_kind: i32,
    pub target_id: u64,
    pub reason_code: String,
    pub detail: Option<String>,
    pub status: i32,
    pub created_at: i64,
}

pub const KIND_JOB: i32 = 1;
pub const KIND_COMPANY: i32 = 2;
pub const KIND_RESUME: i32 = 3;
pub const KIND_ARTICLE: i32 = 4;
pub const KIND_USER: i32 = 5;
