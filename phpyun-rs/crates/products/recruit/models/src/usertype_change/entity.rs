//! `phpyun_change` — user-type switch application (personal <-> company).
//!
//! Aligned with PHPYun `wap/ajax::applytype_action` + `userinfo.model::checkChangeApply`.
//!
//! status: 1 = pending approval / 2 = approved / 3 = rejected.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UsertypeChange {
    pub id: u64,
    pub uid: u64,
    /// Current usertype
    pub usertype: i32,
    /// The usertype being applied to switch to
    pub applyusertype: i32,
    #[serde(default)]
    pub applybody: Option<String>,
    /// 1 = pending review / 2 = approved / 3 = rejected
    pub status: i32,
    pub ctime: i64,
}
