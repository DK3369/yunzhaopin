//! `phpyun_resumeout` — resume outbound records (manually sending one's own resume to an external company's email).

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ResumeOut {
    pub id: u64,
    pub uid: u64,
    pub resume: u64,
    pub email: String,
    pub comname: String,
    pub jobname: String,
    pub resumename: Option<String>,
    pub addtime: i64,
}
