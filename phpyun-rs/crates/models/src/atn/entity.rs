//! `phpyun_atn` — directional follow edges (jobseeker → company / jobseeker → user).
//!
//! PHP `atn.model.php::addAtnLt` is the canonical writer: it toggles
//! presence based on `(uid, sc_uid, sc_usertype)` and best-effort updates
//! the followee's display counter (`phpyun_company.ant_num` for sc_usertype=2 —
//! note the historical typo, the column is literally `ant_num`).

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Atn {
    pub id: u64,
    /// Follower uid (the logged-in user)
    pub uid: u64,
    /// Followee uid
    pub sc_uid: u64,
    /// Unix seconds when the follow was created
    pub time: i64,
    /// Follower usertype (PHP allows NULL but rows always have it set)
    pub usertype: Option<i32>,
    /// Followee usertype: 1=user, 2=company
    pub sc_usertype: Option<i32>,
    /// Optional teacher-id linkage (campus mode)
    pub tid: Option<i32>,
    /// Optional content/article id
    pub conid: Option<i32>,
    /// Optional job-fair (xjh) id
    pub xjhid: Option<i32>,
}

/// Followee kind values used by the toggle endpoint.
pub const KIND_USER: i32 = 1;
pub const KIND_COMPANY: i32 = 2;
