use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ResumeDownload {
    pub id: u64,
    pub com_id: u64,
    pub uid: u64,
    pub eid: u64,
    pub datetime: i64,
}
