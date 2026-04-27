//! `phpyun_domain` — multi-site / sub-site definitions.
//!
//! PHP `wap/site::cache_action` and `domain_action` read this table to
//! present the user with a city-domain switcher. `fz_type = 1` is the
//! "city-based" sub-site (matched on province/city/three_cityid) while
//! `fz_type = 2` is the industry-based variant (`hy`).

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DomainSite {
    pub id: u64,
    pub title: String,
    /// Hostname (without scheme). Example: `bj.example.com`.
    pub domain: String,
    pub province: Option<i32>,
    pub city_id: Option<i32>,
    pub three_city_id: Option<i32>,
    /// `1` city-based, `2` industry-based, `0` site-wide.
    pub fz_type: i32,
    pub hy: Option<i32>,
    pub style: Option<String>,
    pub tpl: Option<String>,
    pub web_title: Option<String>,
    pub web_keyword: Option<String>,
    pub web_meta: Option<String>,
    pub web_logo: Option<String>,
    /// 1 = subdomain mode, 2 = sub-directory mode (PHP `mode`).
    pub mode: i32,
    pub indexdir: Option<String>,
}
