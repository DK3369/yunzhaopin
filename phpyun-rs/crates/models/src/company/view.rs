//! Plain-data view types for `Company`.
//!
//! `from_with_dict` constructors live in `phpyun_handlers::v1::wap::companies`
//! since they need `LocalizedDicts` (which depends on this crate).

use serde::Serialize;
use utoipa::ToSchema;

/// Company list item — mirrors PHPYun `companyM::getList()` output. Shared
/// across `wap/companies`, `wap/search`, `wap/home` to keep the card UI
/// consistent across listing surfaces.
#[derive(Debug, Serialize, ToSchema)]
pub struct CompanySummary {
    pub uid: u64,
    pub name: Option<String>,
    pub shortname: Option<String>,

    // ---- Raw ids ----
    pub hy: i32,
    pub pr: i32,
    pub mun: i32,
    pub province_id: i32,
    pub city_id: i32,

    // ---- Dict-translated names (filled by `company_summary_from_dict`) ----
    pub hy_n: String,
    pub pr_n: String,
    pub mun_n: String,
    pub city_one: String,
    pub city_two: String,

    // ---- Other ----
    pub logo: Option<String>,
    pub rec: i32,
    pub hits: i32,
    pub rating: i32,
    pub rating_name: Option<String>,
}
