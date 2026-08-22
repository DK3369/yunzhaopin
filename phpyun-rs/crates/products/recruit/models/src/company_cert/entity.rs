use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CompanyCert {
    pub uid: u64,
    pub license_photo: String,
    pub id_photo: String,
    pub status: i32,
    pub note: String,
    pub submitted_at: i64,
    pub reviewed_at: i64,
    pub reviewer_uid: u64,
    pub created_at: i64,
    pub updated_at: i64,
}

pub const STATUS_DRAFT: i32 = 0;
pub const STATUS_PENDING: i32 = 1;
pub const STATUS_APPROVED: i32 = 2;
pub const STATUS_REJECTED: i32 = 3;
