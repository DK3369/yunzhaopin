//! Core fields of the `phpyun_company` table (the original PHPYun table has
//! 60+ columns; this struct covers what the WAP detail page needs).
//!
//! Names mirror PHP for easy 1:1 mapping in the service layer.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Company {
    #[sqlx(try_from = "i32")]
    pub uid: u64,
    pub name: Option<String>,
    pub shortname: Option<String>,

    // ---- Industry / size / nature ----
    /// Industry id
    pub hy: i32,
    /// Company nature dictionary id (private/foreign/state-owned/...)
    #[sqlx(default)]
    pub pr: i32,
    /// Size dictionary id (under 50 / 50-100 / ...)
    #[sqlx(default)]
    pub mun: i32,
    /// Founding date (string; PHPYun stores '2020-01-01')
    #[serde(default)]
    pub sdate: Option<String>,
    /// Registered capital
    #[sqlx(default)]
    pub money: i32,
    /// Capital currency (CNY/USD/...)
    #[sqlx(default)]
    pub moneytype: i32,

    // ---- Address ----
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    #[serde(default)]
    pub address: Option<String>,
    #[serde(default)]
    pub zip: Option<String>,
    /// Map longitude
    #[serde(default)]
    pub x: Option<String>,
    /// Map latitude
    #[serde(default)]
    pub y: Option<String>,

    // ---- Contact ----
    pub linkman: Option<String>,
    pub linkjob: Option<String>,
    #[serde(default)]
    pub linkqq: Option<String>,
    pub linkphone: Option<String>,
    #[serde(default)]
    pub linktel: Option<String>,
    pub linkmail: Option<String>,
    #[serde(default)]
    pub website: Option<String>,

    // ---- Images ----
    pub logo: Option<String>,
    pub logo_status: i32,
    /// Storefront photo / business license
    #[serde(default)]
    pub firmpic: Option<String>,
    /// Company QR code
    #[serde(default)]
    pub comqcode: Option<String>,

    // ---- Body content ----
    pub content: Option<String>,

    // ---- Audit & status ----
    /// 0 under review / 1 approved / 2 locked / 3 rejected
    pub r_status: i32,
    pub rec: i32,
    pub hits: i32,
    #[sqlx(default)]
    pub expoure: i32,
    #[sqlx(default)]
    pub moblie_status: i32,
    #[sqlx(default)]
    pub email_status: i32,
    #[sqlx(default)]
    pub yyzz_status: i32,

    // ---- Membership tier ----
    #[sqlx(default)]
    pub rating: i32,
    #[serde(default)]
    pub rating_name: Option<String>,
    /// VIP start time
    #[sqlx(default)]
    pub vipstime: i64,
    /// VIP expiry time
    #[sqlx(default)]
    pub vipetime: i64,
    #[sqlx(default)]
    pub payd: i32,
    #[sqlx(default)]
    pub integral: i32,

    // ---- Timestamps ----
    /// `lastupdate` is varchar(10)! PHPYun stores either a "YYYY-MM-DD"
    /// string or a numeric timestamp.
    #[serde(default)]
    pub lastupdate: Option<String>,
    #[sqlx(default)]
    pub addtime: i64,
    #[sqlx(default)]
    pub login_date: i64,

    // ---- Verification ----
    #[sqlx(default)]
    pub fact_status: i32,

    #[sqlx(try_from = "i32")]
    pub did: u64,
}
