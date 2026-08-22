//! `phpyun_job_tellog` -- job contact-phone click log.
//!
//! Aligns with PHPYun `app/model/job.model.php::addTelLog`.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct JobTelLog {
    pub id: u64,
    pub jobid: u64,
    pub comid: u64,
    #[serde(default)]
    pub uid: u64,
    #[serde(default)]
    pub source: i32,
    pub ip: String,
    pub ctime: i64,
}
