use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct InviteCode {
    pub id: u64,
    pub company_uid: u64,
    pub code: String,
    pub note: String,
    pub max_uses: u32,
    pub used_count: u32,
    pub expires_at: i64,
    pub status: i32,
    pub created_at: i64,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CompanyHr {
    pub company_uid: u64,
    pub hr_uid: u64,
    pub role: String,
    pub joined_at: i64,
    pub status: i32,
}
