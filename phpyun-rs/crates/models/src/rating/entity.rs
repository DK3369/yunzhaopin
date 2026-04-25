use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Rating {
    pub id: u64,
    pub rater_uid: u64,
    pub target_uid: u64,
    pub target_kind: i32,
    pub stars: i32,
    pub comment: String,
    pub status: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct RatingAggregate {
    pub target_uid: u64,
    pub target_kind: i32,
    pub count: u32,
    pub sum_stars: u32,
    pub avg_x100: u32,
    pub updated_at: i64,
}

pub const RATING_COMPANY: i32 = 1;
pub const RATING_RESUME: i32 = 2;
pub const RATING_JOB: i32 = 3;
