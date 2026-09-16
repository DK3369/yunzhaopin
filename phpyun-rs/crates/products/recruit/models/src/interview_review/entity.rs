use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct InterviewReview {
    pub id: u64,
    pub yqms_id: u64,
    pub rater_uid: u64,
    pub ratee_uid: u64,
    pub dimensions: String,
    pub total: u32,
    pub comment: String,
    pub created_at: i64,
    pub updated_at: i64,
}
