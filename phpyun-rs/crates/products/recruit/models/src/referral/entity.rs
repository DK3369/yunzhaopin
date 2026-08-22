use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Referral {
    pub id: u64,
    pub inviter_uid: u64,
    pub invitee_uid: u64,
    pub points: i32,
    pub status: i32,
    pub created_at: i64,
}
