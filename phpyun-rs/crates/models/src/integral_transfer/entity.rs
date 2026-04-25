use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct IntegralTransfer {
    pub id: u64,
    pub from_uid: u64,
    pub to_uid: u64,
    pub points: u32,
    pub note: String,
    pub created_at: i64,
}
