//! `phpyun_talent_pool` — company talent pool (the company's "favorites + notes"
//! list of candidate resumes).
//!
//! Fields map one-to-one with the original PHPYun table.
//! - `eid`    Resume eid (in PHPYun this equals the job seeker's uid; semantics preserved)
//! - `cuid`   Company uid (the one who favorited)
//! - `uid`    Job seeker uid
//! - `remark` The company's private note

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct TalentPoolItem {
    pub id: u64,
    pub eid: u64,
    pub cuid: u64,
    pub uid: u64,
    #[serde(default)]
    pub remark: Option<String>,
    pub ctime: i64,
}
