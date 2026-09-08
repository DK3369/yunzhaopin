use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserEntrustRow {
    pub id: u64,
    pub uid: u64,
    pub eid: u64,
    pub price: String,
    pub status: i32,
    pub add_time: i64,
    pub uname: String,
    pub name: String,
    #[sqlx(skip)]
    #[serde(default)]
    pub add_time_n: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustStat {
    #[serde(rename = "resumeAllNum")]
    pub resume_all_num: u64,
    #[serde(rename = "resumeStatusNum1")]
    pub resume_status_num1: u64,
    #[serde(rename = "resumeStatusNum2")]
    pub resume_status_num2: u64,
}
