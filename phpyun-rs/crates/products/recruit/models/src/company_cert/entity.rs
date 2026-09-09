//! `phpyun_company_cert` — PHP columns as-is.
//!
//! Enterprise qualification is `type = 3` (`check` = license image,
//! `social_credit` is the 18-char credit code **text**, not a photo).
//! Status: `0` pending / `1` approved / `2` rejected.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CompanyCert {
    pub id: u64,
    pub uid: u64,
    pub usertype: i32,
    pub cert_type: i32,
    pub status: i32,
    pub step: i32,
    pub check: String,
    pub check2: String,
    pub social_credit: String,
    pub owner_cert: String,
    pub wt_cert: String,
    pub other_cert: String,
    pub ctime: i64,
    pub statusbody: String,
    pub did: u64,
}

/// PHP `company_cert.type` for 企业资质.
pub const TYPE_LICENSE: i32 = 3;
/// PHP claim-code row (`check2`).
pub const TYPE_CLAIM: i32 = 6;

pub const STATUS_PENDING: i32 = 0;
pub const STATUS_APPROVED: i32 = 1;
pub const STATUS_REJECTED: i32 = 2;
