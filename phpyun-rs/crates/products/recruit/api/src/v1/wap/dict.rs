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

use axum::{extract::State, routing::get, Router};
use phpyun_core::i18n::{current_lang, t, Lang};
use phpyun_core::ValidatedJsonOrQuery;
use phpyun_core::{ApiResponse, AppResult, AppState};
use phpyun_services::dict_service;
use serde::Deserialize;
use serde::Serialize;
use utoipa::ToSchema;
use validator::Validate;

pub const GET_ALLOWED_PATHS: &[&str] = &[
    "/v1/wap/dict/cities",
    "/v1/wap/dict/cities/by-province",
    "/v1/wap/dict/industries",
    "/v1/wap/dict/job-categories",
    "/v1/wap/dict/educations",
    "/v1/wap/dict/experiences",
    "/v1/wap/dict/salaries",
    "/v1/wap/dict/job-types",
    "/v1/wap/dict/welfares",
    "/v1/wap/dict/reports",
    "/v1/wap/dict/tags",
];

pub fn routes() -> Router<AppState> {
    // Two cities routes are explicitly deprecated; we still register them so
    // existing clients stay green while they migrate to /v1/wap/regions.
    #[allow(deprecated)]
    let r = Router::new()
        .route("/dict/cities", get(cities).post(cities))
        .route(
            "/dict/cities/by-province",
            get(cities_of_province).post(cities_of_province),
        );
    r.route("/dict/industries", get(industries).post(industries))
        .route(
            "/dict/job-categories",
            get(job_categories).post(job_categories),
        )
        .route("/dict/educations", get(educations).post(educations))
        .route("/dict/experiences", get(experiences).post(experiences))
        .route("/dict/salaries", get(salaries).post(salaries))
        .route("/dict/job-types", get(job_types).post(job_types))
        .route("/dict/welfares", get(welfares).post(welfares))
        .route("/dict/reports", get(reports).post(reports))
        .route("/dict/tags", get(tags).post(tags))
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

fn named_items(rows: Vec<(i32, String)>) -> Vec<DictItem> {
    rows.into_iter()
        .map(|(id, name)| DictItem { id, name })
        .collect()
}

#[derive(Debug, Deserialize, Validate, utoipa::ToSchema, Default)]
pub struct DictSourceQuery {
    /// `job` (comclass, default) or `user` (userclass) — PHP job vs resume filters.
    #[validate(length(max = 16))]
    pub source: Option<String>,
}

/// Province dictionary — PHP `$city_index` / `$city_name` from city.cache.php
#[utoipa::path(
    post,
    path = "/v1/wap/dict/cities",
    tag = "wap",
    responses((status = 200, description = "ok"))
)]
pub async fn cities(State(state): State<AppState>) -> AppResult<ApiResponse<Vec<DictItem>>> {
    let dicts = dict_service::get(&state).await?;
    let rows = dicts.city_provinces();
    if rows.is_empty() {
        return Ok(ApiResponse::data(render(PROVINCES, current_lang())));
    }
    Ok(ApiResponse::data(named_items(rows)))
}

#[derive(Debug, Deserialize, Validate, utoipa::ToSchema)]
pub struct ProvinceBody {
    #[validate(range(min = 0, max = 9_999_999))]
    pub province_id: i32,
}

#[utoipa::path(
    post,
    path = "/v1/wap/dict/cities/by-province",
    tag = "wap",
    request_body = ProvinceBody,
    responses((status = 200, description = "ok"))
)]
pub async fn cities_of_province(
    State(state): State<AppState>,
    ValidatedJsonOrQuery(b): ValidatedJsonOrQuery<ProvinceBody>,
) -> AppResult<ApiResponse<Vec<DictItem>>> {
    let dicts = dict_service::get(&state).await?;
    let rows = dicts.city_of_parent(b.province_id);
    if rows.is_empty() {
        let lang = current_lang();
        let v = match b.province_id {
            1 => render(BEIJING_DISTRICTS, lang),
            2 => render(SHANGHAI_DISTRICTS, lang),
            _ => Vec::new(),
        };
        return Ok(ApiResponse::data(v));
    }
    Ok(ApiResponse::data(named_items(rows)))
}

/// Industry categories from `phpyun_industry` (PHP `$industry_name`)
#[utoipa::path(
    post,
    path = "/v1/wap/dict/industries",
    tag = "wap",
    responses((status = 200, description = "ok"))
)]
pub async fn industries(State(state): State<AppState>) -> AppResult<ApiResponse<Vec<DictItem>>> {
    let dicts = dict_service::get(&state).await?;
    let rows = dicts.industry_all();
    if rows.is_empty() {
        return Ok(ApiResponse::data(render(INDUSTRIES, current_lang())));
    }
    Ok(ApiResponse::data(named_items(rows)))
}

/// Education levels — `source=user` uses resume userclass; default is job comclass `job_edu`.
#[utoipa::path(
    post,
    path = "/v1/wap/dict/educations",
    tag = "wap",
    responses((status = 200, description = "ok"))
)]
pub async fn educations(
    State(state): State<AppState>,
    ValidatedJsonOrQuery(q): ValidatedJsonOrQuery<DictSourceQuery>,
) -> AppResult<ApiResponse<Vec<DictItem>>> {
    let dicts = dict_service::get(&state).await?;
    let rows = if q.source.as_deref() == Some("user") {
        dicts.userclass_by_variable("user_edu")
    } else {
        dicts.comclass_by_variable("job_edu")
    };
    if rows.is_empty() {
        return Ok(ApiResponse::data(render(EDUCATIONS, current_lang())));
    }
    Ok(ApiResponse::data(named_items(rows)))
}

/// Work experience — `source=user` uses resume `user_word`; default job `job_exp`.
#[utoipa::path(
    post,
    path = "/v1/wap/dict/experiences",
    tag = "wap",
    responses((status = 200, description = "ok"))
)]
pub async fn experiences(
    State(state): State<AppState>,
    ValidatedJsonOrQuery(q): ValidatedJsonOrQuery<DictSourceQuery>,
) -> AppResult<ApiResponse<Vec<DictItem>>> {
    let dicts = dict_service::get(&state).await?;
    let rows = if q.source.as_deref() == Some("user") {
        dicts.userclass_by_variable("user_word")
    } else {
        dicts.comclass_by_variable("job_exp")
    };
    if rows.is_empty() {
        return Ok(ApiResponse::data(render(EXPERIENCES, current_lang())));
    }
    Ok(ApiResponse::data(named_items(rows)))
}

#[utoipa::path(
    post,
    path = "/v1/wap/dict/welfares",
    tag = "wap",
    responses((status = 200, description = "ok"))
)]
pub async fn welfares(State(state): State<AppState>) -> AppResult<ApiResponse<Vec<DictItem>>> {
    let dicts = dict_service::get(&state).await?;
    Ok(ApiResponse::data(named_items(
        dicts.comclass_by_variable("job_welfare"),
    )))
}

/// Top-level job categories
#[utoipa::path(
    post,
    path = "/v1/wap/dict/job-categories",
    tag = "wap",
    responses((status = 200, description = "ok"))
)]
pub async fn job_categories() -> AppResult<ApiResponse<Vec<DictItem>>> {
    Ok(ApiResponse::data(render(JOB_CATEGORIES, current_lang())))
}

/// Salary ranges
#[utoipa::path(
    post,
    path = "/v1/wap/dict/salaries",
    tag = "wap",
    responses((status = 200, description = "ok"))
)]
pub async fn salaries() -> AppResult<ApiResponse<Vec<DictItem>>> {
    Ok(ApiResponse::data(render(SALARIES, current_lang())))
}

/// Job types (full-time / part-time / internship / temporary / remote).
/// `source=user` uses resume `user_type`.
#[utoipa::path(
    post,
    path = "/v1/wap/dict/job-types",
    tag = "wap",
    responses((status = 200, description = "ok"))
)]
pub async fn job_types(
    State(state): State<AppState>,
    ValidatedJsonOrQuery(q): ValidatedJsonOrQuery<DictSourceQuery>,
) -> AppResult<ApiResponse<Vec<DictItem>>> {
    if q.source.as_deref() == Some("user") {
        let dicts = dict_service::get(&state).await?;
        let rows = dicts.userclass_by_variable("user_type");
        if !rows.is_empty() {
            return Ok(ApiResponse::data(named_items(rows)));
        }
    }
    Ok(ApiResponse::data(render(JOB_TYPES, current_lang())))
}

/// Salary cycle / report-time. `source=user` uses resume `user_report`.
#[utoipa::path(
    post,
    path = "/v1/wap/dict/reports",
    tag = "wap",
    responses((status = 200, description = "ok"))
)]
pub async fn reports(
    State(state): State<AppState>,
    ValidatedJsonOrQuery(q): ValidatedJsonOrQuery<DictSourceQuery>,
) -> AppResult<ApiResponse<Vec<DictItem>>> {
    let dicts = dict_service::get(&state).await?;
    let rows = if q.source.as_deref() == Some("user") {
        dicts.userclass_by_variable("user_report")
    } else {
        dicts.comclass_by_variable("job_report")
    };
    Ok(ApiResponse::data(named_items(rows)))
}

/// Resume person tags — PHP `$userdata.user_tag`.
#[utoipa::path(
    post,
    path = "/v1/wap/dict/tags",
    tag = "wap",
    responses((status = 200, description = "ok"))
)]
pub async fn tags(State(state): State<AppState>) -> AppResult<ApiResponse<Vec<DictItem>>> {
    let dicts = dict_service::get(&state).await?;
    Ok(ApiResponse::data(named_items(
        dicts.userclass_by_variable("user_tag"),
    )))
}

// ==================== Static data: (id, i18n key) ====================
//
// The `key` suffix maps to the `dict.*` node in locales/<lang>.json.
// To update translations just edit the JSON -- no need to touch Rust code.

const PROVINCES: &[DictEntry] = &[
    DictEntry::new(1, "dict.province.1"),
    DictEntry::new(2, "dict.province.2"),
    DictEntry::new(3, "dict.province.3"),
    DictEntry::new(4, "dict.province.4"),
    DictEntry::new(5, "dict.province.5"),
    DictEntry::new(6, "dict.province.6"),
    DictEntry::new(7, "dict.province.7"),
    DictEntry::new(8, "dict.province.8"),
    DictEntry::new(9, "dict.province.9"),
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
    DictEntry::new(1, "dict.industry.1"),
    DictEntry::new(2, "dict.industry.2"),
    DictEntry::new(3, "dict.industry.3"),
    DictEntry::new(4, "dict.industry.4"),
    DictEntry::new(5, "dict.industry.5"),
    DictEntry::new(6, "dict.industry.6"),
    DictEntry::new(7, "dict.industry.7"),
    DictEntry::new(8, "dict.industry.8"),
    DictEntry::new(9, "dict.industry.9"),
    DictEntry::new(10, "dict.industry.10"),
    DictEntry::new(11, "dict.industry.11"),
    DictEntry::new(12, "dict.industry.12"),
    DictEntry::new(13, "dict.industry.13"),
    DictEntry::new(99, "dict.industry.99"),
];

const JOB_CATEGORIES: &[DictEntry] = &[
    DictEntry::new(1, "dict.job_category.1"),
    DictEntry::new(2, "dict.job_category.2"),
    DictEntry::new(3, "dict.job_category.3"),
    DictEntry::new(4, "dict.job_category.4"),
    DictEntry::new(5, "dict.job_category.5"),
    DictEntry::new(6, "dict.job_category.6"),
    DictEntry::new(7, "dict.job_category.7"),
    DictEntry::new(8, "dict.job_category.8"),
    DictEntry::new(9, "dict.job_category.9"),
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
    DictEntry::new(5, "dict.job_type.5"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn job_types_include_remote_in_all_supported_languages() {
        let zh_cn = render(JOB_TYPES, Lang::ZhCN);
        let zh_tw = render(JOB_TYPES, Lang::ZhTW);
        let en = render(JOB_TYPES, Lang::En);

        assert_eq!(
            zh_cn.last().map(|item| (item.id, item.name.as_str())),
            Some((5, "远程"))
        );
        assert_eq!(
            zh_tw.last().map(|item| (item.id, item.name.as_str())),
            Some((5, "遠端"))
        );
        assert_eq!(
            en.last().map(|item| (item.id, item.name.as_str())),
            Some((5, "Remote"))
        );
    }
}
