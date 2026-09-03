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
    /// Homepage famous-company banner (`phpyun_hotjob.hot_pic`). Absent on regular lists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hot_pic: Option<String>,
    pub rec: i32,
    pub hits: i32,
    pub rating: i32,
    pub rating_name: Option<String>,
    /// Open jobs (`state=1 AND status=0 AND r_status=1`). Additive; 0 when unknown.
    pub job_num: u64,
    /// Business-license verified (`yyzz_status=1`). Additive.
    pub yyzz_status: i32,
    /// On-site verification (PHP `fact_status`).
    pub fact_status: i32,
    /// PHP `$com.welfare_n`.
    pub welfare_n: Vec<String>,
    /// Homepage famous-company hover (PHP `hotjob` 插件最多 3 条在招).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub open_jobs: Vec<CompanyOpenJob>,
}

/// Brief job row for the famous-company hover panel.
#[derive(Debug, Serialize, ToSchema, Clone)]
pub struct CompanyOpenJob {
    pub id: u64,
    pub name: String,
}
