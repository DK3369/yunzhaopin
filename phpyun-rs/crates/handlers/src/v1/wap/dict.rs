//! Dictionary endpoints -- used to populate frontend dropdowns.
//!
//! PHPYun stores this data in dynamic tables such as `phpyun_category` / `phpyun_city` / `phpyun_industry`,
//! admin-configurable. For now we return **static built-in values** (the initial dictionaries most large
//! sites use) so the frontend can be wired up; if we later switch to a DB-backed dynamic dictionary, only
//! the handler implementations here need to change -- the routes and response shapes remain stable.
//!
//! ## Internationalization
//!
//! Strings are not hard-coded in the const tables -- the tables only store (id, i18n_key); during
//! serialization we call `phpyun_core::i18n::t()` to translate using the current request language.
//! Translation entries are maintained under the `dict.*` namespace of `locales/<lang>.json`.

use axum::{extract::Path, routing::get, Router};
use phpyun_core::i18n::{current_lang, t, Lang};
use phpyun_core::{ApiJson, AppResult, AppState};
use serde::Serialize;
use utoipa::ToSchema;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/dict/cities", get(cities))
        .route("/dict/industries", get(industries))
        .route("/dict/job-categories", get(job_categories))
        .route("/dict/educations", get(educations))
        .route("/dict/experiences", get(experiences))
        .route("/dict/salaries", get(salaries))
        .route("/dict/job-types", get(job_types))
        .route("/dict/cities/{province_id}", get(cities_of_province))
}

/// Dictionary item as seen by the client. `name` is a string resolved using the current request language.
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct DictItem {
    pub id: i32,
    pub name: String,
}

/// Static dictionary definition: (id, i18n key).
/// `key` looks like `dict.province.5` -- `<bucket>.<id>` is composed by the caller.
#[derive(Debug, Clone, Copy)]
struct DictEntry {
    id: i32,
    key: &'static str,
}

impl DictEntry {
    const fn new(id: i32, key: &'static str) -> Self {
        Self { id, key }
    }
}

fn render(entries: &[DictEntry], lang: Lang) -> Vec<DictItem> {
    entries
        .iter()
        .map(|e| DictItem {
            id: e.id,
            name: t(e.key, lang),
        })
        .collect()
}

/// Province / centrally-administered municipality dictionary
#[utoipa::path(
    get,
    path = "/v1/wap/dict/cities",
    tag = "wap",
    responses((status = 200, description = "ok"))
)]
pub async fn cities() -> AppResult<ApiJson<Vec<DictItem>>> {
    Ok(ApiJson(render(PROVINCES, current_lang())))
}

/// Cities under a given province
#[utoipa::path(
    get,
    path = "/v1/wap/dict/cities/{province_id}",
    tag = "wap",
    params(("province_id" = i32, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn cities_of_province(
    Path(pid): Path<i32>,
) -> AppResult<ApiJson<Vec<DictItem>>> {
    let lang = current_lang();
    let v = match pid {
        1 => render(BEIJING_DISTRICTS, lang),
        2 => render(SHANGHAI_DISTRICTS, lang),
        _ => vec![DictItem {
            id: 0,
            name: t("dict.all", lang),
        }],
    };
    Ok(ApiJson(v))
}

/// Industry categories
#[utoipa::path(
    get,
    path = "/v1/wap/dict/industries",
    tag = "wap",
    responses((status = 200, description = "ok"))
)]
pub async fn industries() -> AppResult<ApiJson<Vec<DictItem>>> {
    Ok(ApiJson(render(INDUSTRIES, current_lang())))
}

/// Top-level job categories
#[utoipa::path(
    get,
    path = "/v1/wap/dict/job-categories",
    tag = "wap",
    responses((status = 200, description = "ok"))
)]
pub async fn job_categories() -> AppResult<ApiJson<Vec<DictItem>>> {
    Ok(ApiJson(render(JOB_CATEGORIES, current_lang())))
}

/// Education levels
#[utoipa::path(
    get,
    path = "/v1/wap/dict/educations",
    tag = "wap",
    responses((status = 200, description = "ok"))
)]
pub async fn educations() -> AppResult<ApiJson<Vec<DictItem>>> {
    Ok(ApiJson(render(EDUCATIONS, current_lang())))
}

/// Work experience
#[utoipa::path(
    get,
    path = "/v1/wap/dict/experiences",
    tag = "wap",
    responses((status = 200, description = "ok"))
)]
pub async fn experiences() -> AppResult<ApiJson<Vec<DictItem>>> {
    Ok(ApiJson(render(EXPERIENCES, current_lang())))
}

/// Salary ranges
#[utoipa::path(
    get,
    path = "/v1/wap/dict/salaries",
    tag = "wap",
    responses((status = 200, description = "ok"))
)]
pub async fn salaries() -> AppResult<ApiJson<Vec<DictItem>>> {
    Ok(ApiJson(render(SALARIES, current_lang())))
}

/// Job types (full-time / part-time / internship / ...)
#[utoipa::path(
    get,
    path = "/v1/wap/dict/job-types",
    tag = "wap",
    responses((status = 200, description = "ok"))
)]
pub async fn job_types() -> AppResult<ApiJson<Vec<DictItem>>> {
    Ok(ApiJson(render(JOB_TYPES, current_lang())))
}

// ==================== Static data: (id, i18n key) ====================
//
// The `key` suffix maps to the `dict.*` node in locales/<lang>.json.
// To update translations just edit the JSON -- no need to touch Rust code.

const PROVINCES: &[DictEntry] = &[
    DictEntry::new(1,  "dict.province.1"),
    DictEntry::new(2,  "dict.province.2"),
    DictEntry::new(3,  "dict.province.3"),
    DictEntry::new(4,  "dict.province.4"),
    DictEntry::new(5,  "dict.province.5"),
    DictEntry::new(6,  "dict.province.6"),
    DictEntry::new(7,  "dict.province.7"),
    DictEntry::new(8,  "dict.province.8"),
    DictEntry::new(9,  "dict.province.9"),
    DictEntry::new(10, "dict.province.10"),
    DictEntry::new(11, "dict.province.11"),
    DictEntry::new(12, "dict.province.12"),
    DictEntry::new(13, "dict.province.13"),
    DictEntry::new(14, "dict.province.14"),
    DictEntry::new(15, "dict.province.15"),
    DictEntry::new(16, "dict.province.16"),
    DictEntry::new(17, "dict.province.17"),
    DictEntry::new(18, "dict.province.18"),
    DictEntry::new(19, "dict.province.19"),
    DictEntry::new(20, "dict.province.20"),
    DictEntry::new(21, "dict.province.21"),
    DictEntry::new(22, "dict.province.22"),
    DictEntry::new(23, "dict.province.23"),
    DictEntry::new(24, "dict.province.24"),
    DictEntry::new(25, "dict.province.25"),
    DictEntry::new(26, "dict.province.26"),
    DictEntry::new(27, "dict.province.27"),
    DictEntry::new(28, "dict.province.28"),
    DictEntry::new(29, "dict.province.29"),
    DictEntry::new(30, "dict.province.30"),
    DictEntry::new(31, "dict.province.31"),
    DictEntry::new(32, "dict.province.32"),
    DictEntry::new(33, "dict.province.33"),
    DictEntry::new(34, "dict.province.34"),
];

const BEIJING_DISTRICTS: &[DictEntry] = &[
    DictEntry::new(101, "dict.district_bj.101"),
    DictEntry::new(102, "dict.district_bj.102"),
    DictEntry::new(103, "dict.district_bj.103"),
    DictEntry::new(104, "dict.district_bj.104"),
    DictEntry::new(105, "dict.district_bj.105"),
    DictEntry::new(106, "dict.district_bj.106"),
    DictEntry::new(107, "dict.district_bj.107"),
    DictEntry::new(108, "dict.district_bj.108"),
    DictEntry::new(109, "dict.district_bj.109"),
    DictEntry::new(110, "dict.district_bj.110"),
];

const SHANGHAI_DISTRICTS: &[DictEntry] = &[
    DictEntry::new(201, "dict.district_sh.201"),
    DictEntry::new(202, "dict.district_sh.202"),
    DictEntry::new(203, "dict.district_sh.203"),
    DictEntry::new(204, "dict.district_sh.204"),
    DictEntry::new(205, "dict.district_sh.205"),
    DictEntry::new(206, "dict.district_sh.206"),
    DictEntry::new(207, "dict.district_sh.207"),
    DictEntry::new(208, "dict.district_sh.208"),
    DictEntry::new(209, "dict.district_sh.209"),
    DictEntry::new(210, "dict.district_sh.210"),
];

const INDUSTRIES: &[DictEntry] = &[
    DictEntry::new(1,  "dict.industry.1"),
    DictEntry::new(2,  "dict.industry.2"),
    DictEntry::new(3,  "dict.industry.3"),
    DictEntry::new(4,  "dict.industry.4"),
    DictEntry::new(5,  "dict.industry.5"),
    DictEntry::new(6,  "dict.industry.6"),
    DictEntry::new(7,  "dict.industry.7"),
    DictEntry::new(8,  "dict.industry.8"),
    DictEntry::new(9,  "dict.industry.9"),
    DictEntry::new(10, "dict.industry.10"),
    DictEntry::new(11, "dict.industry.11"),
    DictEntry::new(12, "dict.industry.12"),
    DictEntry::new(13, "dict.industry.13"),
    DictEntry::new(99, "dict.industry.99"),
];

const JOB_CATEGORIES: &[DictEntry] = &[
    DictEntry::new(1,  "dict.job_category.1"),
    DictEntry::new(2,  "dict.job_category.2"),
    DictEntry::new(3,  "dict.job_category.3"),
    DictEntry::new(4,  "dict.job_category.4"),
    DictEntry::new(5,  "dict.job_category.5"),
    DictEntry::new(6,  "dict.job_category.6"),
    DictEntry::new(7,  "dict.job_category.7"),
    DictEntry::new(8,  "dict.job_category.8"),
    DictEntry::new(9,  "dict.job_category.9"),
    DictEntry::new(10, "dict.job_category.10"),
    DictEntry::new(11, "dict.job_category.11"),
    DictEntry::new(12, "dict.job_category.12"),
    DictEntry::new(99, "dict.job_category.99"),
];

const EDUCATIONS: &[DictEntry] = &[
    DictEntry::new(1, "dict.education.1"),
    DictEntry::new(2, "dict.education.2"),
    DictEntry::new(3, "dict.education.3"),
    DictEntry::new(4, "dict.education.4"),
    DictEntry::new(5, "dict.education.5"),
    DictEntry::new(6, "dict.education.6"),
    DictEntry::new(7, "dict.education.7"),
    DictEntry::new(8, "dict.education.8"),
];

const EXPERIENCES: &[DictEntry] = &[
    DictEntry::new(0, "dict.experience.0"),
    DictEntry::new(1, "dict.experience.1"),
    DictEntry::new(2, "dict.experience.2"),
    DictEntry::new(3, "dict.experience.3"),
    DictEntry::new(4, "dict.experience.4"),
    DictEntry::new(5, "dict.experience.5"),
    DictEntry::new(6, "dict.experience.6"),
];

const SALARIES: &[DictEntry] = &[
    DictEntry::new(0, "dict.salary.0"),
    DictEntry::new(1, "dict.salary.1"),
    DictEntry::new(2, "dict.salary.2"),
    DictEntry::new(3, "dict.salary.3"),
    DictEntry::new(4, "dict.salary.4"),
    DictEntry::new(5, "dict.salary.5"),
    DictEntry::new(6, "dict.salary.6"),
    DictEntry::new(7, "dict.salary.7"),
    DictEntry::new(8, "dict.salary.8"),
];

const JOB_TYPES: &[DictEntry] = &[
    DictEntry::new(1, "dict.job_type.1"),
    DictEntry::new(2, "dict.job_type.2"),
    DictEntry::new(3, "dict.job_type.3"),
    DictEntry::new(4, "dict.job_type.4"),
];
