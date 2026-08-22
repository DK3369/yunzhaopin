//! `phpyun_entrust` — jobseeker ↔ headhunter binding (PHP truth: 5 columns).
//!
//! NOT a "post my desired job" table — that was a misread of the schema.
//! Each row = one jobseeker (`uid`) entrusted with a headhunter (`lt_uid`,
//! a member with `usertype=4`). A seeker can bind multiple headhunters.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Entrust {
    pub id: u64,
    pub uid: u64,
    pub lt_uid: u64,
    pub datetime: i64,
    pub remind_status: i32,
}
