//! `phpyun_resumetpl` — resume template catalog (purchased/selected by jobseekers).

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ResumeTpl {
    pub id: u64,
    pub name: String,
    pub pic: Option<String>,
    pub price: i32,
    /// 0 = disabled / 1 = enabled
    pub status: i32,
    pub sort: i32,
}
