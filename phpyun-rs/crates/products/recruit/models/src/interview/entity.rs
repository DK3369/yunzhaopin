use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Interview {
    pub id: u64,
    pub apply_id: u64,
    pub com_id: u64,
    pub uid: u64,
    pub job_id: u64,
    pub inter_time: i64,
    pub address: String,
    pub linkman: String,
    pub linktel: String,
    pub remark: Option<String>,
    /// 0 = pending / 1 = accepted / 2 = declined / 3 = cancelled
    pub status: i32,
    pub created_at: i64,
}
