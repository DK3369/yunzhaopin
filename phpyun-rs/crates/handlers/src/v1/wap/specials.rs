//! Special recruitment events (aligned with PHPYun `wap/special`).

use axum::{
    extract::{Path, State},
    Router,
    routing::{get, post},
};
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson};
use phpyun_services::special_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use phpyun_core::dto::{IdBody};

fn fmt_date(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_default()
}

fn pic_n(state: &AppState, raw: &str) -> String {
    state
        .storage
        .normalize_legacy_url(raw, state.config.web_base_url.as_deref())
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/specials", post(list))
        .route("/specials/detail", post(detail))
        .route("/specials/companies", post(companies))
        .route("/specials/jobs", post(jobs))
        .route("/specials/apply", post(apply))
}

/// Special list item — aligned with all 20 columns of phpyun_special + CDN URL + formatted timestamps.
#[derive(Debug, Serialize, ToSchema)]
pub struct SpecialSummary {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub intro: String,
    pub body: String,

    // ---- Visuals + CDN ----
    pub banner: String,
    pub banner_n: String,
    pub background: String,
    pub background_n: String,
    pub wappic: String,
    pub wappic_n: String,
    pub wapback: String,
    pub wapback_n: String,

    // ---- Template / sort / rules ----
    pub tpl: String,
    pub sort: i32,
    pub max_count: i32,
    pub rating: String,
    pub com_bm: i32,
    pub integral: i32,

    // ---- Time / status / stats ----
    pub start_at: i64,
    pub start_at_n: String,
    pub end_at: i64,
    pub end_at_n: String,
    pub created_at: i64,
    pub created_at_n: String,
    pub view_count: i32,
    pub status: i32,
}

impl SpecialSummary {
    pub fn from_with_ctx(
        s: phpyun_models::special::entity::Special,
        state: &AppState,
    ) -> Self {
        let banner_n = pic_n(state, &s.banner);
        let background_n = pic_n(state, &s.background);
        let wappic_n = pic_n(state, &s.wappic);
        let wapback_n = pic_n(state, &s.wapback);
        Self {
            id: s.id,
            title: s.title,
            description: s.description,
            intro: s.intro,
            body: s.body,
            banner_n,
            banner: s.banner,
            background_n,
            background: s.background,
            wappic_n,
            wappic: s.wappic,
            wapback_n,
            wapback: s.wapback,
            tpl: s.tpl,
            sort: s.sort,
            max_count: s.max_count,
            rating: s.rating,
            com_bm: s.com_bm,
            integral: s.integral,
            start_at_n: fmt_date(s.start_at),
            start_at: s.start_at,
            end_at_n: fmt_date(s.end_at),
            end_at: s.end_at,
            created_at_n: fmt_date(s.created_at),
            created_at: s.created_at,
            view_count: s.view_count,
            status: s.status,
        }
    }
}

impl From<phpyun_models::special::entity::Special> for SpecialSummary {
    fn from(s: phpyun_models::special::entity::Special) -> Self {
        Self {
            id: s.id,
            title: s.title,
            description: s.description,
            intro: s.intro,
            body: s.body,
            banner: s.banner.clone(),
            banner_n: s.banner,
            background: s.background.clone(),
            background_n: s.background,
            wappic: s.wappic.clone(),
            wappic_n: s.wappic,
            wapback: s.wapback.clone(),
            wapback_n: s.wapback,
            tpl: s.tpl,
            sort: s.sort,
            max_count: s.max_count,
            rating: s.rating,
            com_bm: s.com_bm,
            integral: s.integral,
            start_at_n: fmt_date(s.start_at),
            start_at: s.start_at,
            end_at_n: fmt_date(s.end_at),
            end_at: s.end_at,
            created_at_n: fmt_date(s.created_at),
            created_at: s.created_at,
            view_count: s.view_count,
            status: s.status,
        }
    }
}

/// Special detail — same field set as SpecialSummary (special detail = list item + nested companies / jobs).
pub type SpecialDetail = SpecialSummary;

/// Participating company item — all 6 columns of phpyun_special_company + joined company info (name/logo/hy/pr/mun/city).
#[derive(Debug, Serialize, ToSchema)]
pub struct SpecialCompanyItem {
    pub id: u64,
    pub sid: u64,
    pub uid: u64,
    pub sort: i32,
    pub status: i32,
    pub created_at: i64,
    pub created_at_n: String,
    // ---- Joined with phpyun_company ----
    pub com_name: Option<String>,
    pub com_logo: Option<String>,
    pub com_logo_n: String,
    pub hy: i32,
    pub hy_n: String,
    pub pr: i32,
    pub pr_n: String,
    pub mun: i32,
    pub mun_n: String,
    pub province_id: i32,
    pub province_name: String,
    pub city_id: i32,
    pub city_name: String,
}

/// Job item inside a special — reuses wap::jobs::JobSummary (34 dictionary-translated fields)
pub type SpecialJob = super::jobs::JobSummary;

/// Special list
#[utoipa::path(post, path = "/v1/wap/specials/detail", tag = "wap", responses((status = 200, description = "ok")))]
pub async fn list(
    State(state): State<AppState>,
    page: Pagination,
) -> AppResult<ApiJson<Paged<SpecialSummary>>> {
    let r = special_service::list(&state, page).await?;
    Ok(ApiJson(Paged::new(
        r.list
            .into_iter()
            .map(|s| SpecialSummary::from_with_ctx(s, &state))
            .collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Special detail
#[utoipa::path(post,
    path = "/v1/wap/specials",
    tag = "wap",
    request_body = IdBody,
    responses((status = 200, description = "ok", body = SpecialDetail))
)]
pub async fn detail(State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiJson<SpecialDetail>> {
    let id = b.id;
    let s = special_service::get(&state, id).await?;
    Ok(ApiJson(SpecialDetail::from_with_ctx(s, &state)))
}

/// Participating companies (phpyun_special_company JOIN phpyun_company)
#[utoipa::path(post,
    path = "/v1/wap/specials/companies",
    tag = "wap",
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn companies(State(state): State<AppState>,
    page: Pagination,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiJson<Paged<SpecialCompanyItem>>> {
    let id = b.id;
    let r = special_service::list_companies(&state, id, page).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    let uids: Vec<u64> = r.list.iter().map(|c| c.uid).collect();
    let mut com_map: std::collections::HashMap<
        u64,
        (Option<String>, Option<String>, i32, i32, i32, i32, i32),
    > = std::collections::HashMap::new();
    if !uids.is_empty() {
        let placeholders = std::iter::repeat("?")
            .take(uids.len())
            .collect::<Vec<_>>()
            .join(",");
        let sql = format!(
            "SELECT CAST(uid AS UNSIGNED), name, logo, \
                CAST(COALESCE(hy,0) AS SIGNED), \
                CAST(COALESCE(pr,0) AS SIGNED), \
                CAST(COALESCE(mun,0) AS SIGNED), \
                CAST(COALESCE(provinceid,0) AS SIGNED), \
                CAST(COALESCE(cityid,0) AS SIGNED) \
             FROM phpyun_company WHERE uid IN ({placeholders})"
        );
        let mut q = sqlx::query_as::<
            _,
            (u64, Option<String>, Option<String>, i32, i32, i32, i32, i32),
        >(&sql);
        for u in &uids {
            q = q.bind(*u as i64);
        }
        let rows = q.fetch_all(state.db.reader()).await.unwrap_or_default();
        for (uid, name, logo, hy, pr, mun, prov, city) in rows {
            com_map.insert(uid, (name, logo, hy, pr, mun, prov, city));
        }
    }
    let items: Vec<SpecialCompanyItem> = r
        .list
        .into_iter()
        .map(|c| {
            let info = com_map.get(&c.uid).cloned();
            let (com_name, com_logo, hy, pr, mun, prov, city) =
                info.unwrap_or((None, None, 0, 0, 0, 0, 0));
            let logo_full = pic_n(&state, com_logo.as_deref().unwrap_or(""));
            SpecialCompanyItem {
                id: c.id,
                sid: c.sid,
                uid: c.uid,
                sort: c.sort,
                status: c.status,
                created_at_n: fmt_date(c.created_at),
                created_at: c.created_at,
                com_name,
                com_logo_n: logo_full,
                com_logo,
                hy,
                hy_n: dicts.industry(hy).to_string(),
                pr,
                pr_n: dicts.comclass(pr).to_string(),
                mun,
                mun_n: dicts.comclass(mun).to_string(),
                province_id: prov,
                province_name: dicts.city(prov).to_string(),
                city_id: city,
                city_name: dicts.city(city).to_string(),
            }
        })
        .collect();
    Ok(ApiJson(Paged::new(
        items,
        r.total,
        page.page,
        page.page_size,
    )))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct JobQuery {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,

    #[serde(default = "default_limit")]
    #[validate(range(min = 1, max = 200))]
    pub limit: u64,
}
fn default_limit() -> u64 { 50 }

// ==================== Sign-up ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct ApplyResp {
    pub id: u64,
    /// Points deducted from the company's balance (0 if the event was free).
    pub integral_spent: i32,
}

/// Company signs up to a special event — counterpart of PHP
/// `wap/special::apply_action`. Requires `usertype=2` (employer); validation
/// errors return 400 with a stable `param_invalid: <code>` tag (e.g.
/// `special_signup_closed`, `special_already_applied`, `special_full`,
/// `company_no_active_job`, `company_rating_not_eligible`,
/// `insufficient_integral`).
#[utoipa::path(post,
    path = "/v1/wap/specials/apply",
    tag = "wap",
    security(("bearer" = [])),
    request_body = IdBody,
    responses(
        (status = 200, description = "ok", body = ApplyResp),
        (status = 400, description = "Validation failed (see tag)"),
        (status = 403, description = "Only employers (usertype=2) may apply"),
    )
)]
pub async fn apply(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiJson<ApplyResp>> {
    let id = b.id;
    let r = special_service::apply(&state, &user, id).await?;
    Ok(ApiJson(ApplyResp {
        id: r.id,
        integral_spent: r.integral_spent,
    }))
}

/// Jobs inside a special (reuses the rich JobSummary: 34 fields with dictionary translations)
#[utoipa::path(post,
    path = "/v1/wap/specials/jobs",
    tag = "wap",
    params(("id" = u64, Path), JobQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn jobs(State(state): State<AppState>,
    ValidatedJson(q): ValidatedJson<JobQuery>) -> AppResult<ApiJson<Vec<SpecialJob>>> {
    let id = q.id;
    let list = special_service::list_jobs(&state, id, q.limit.clamp(1, 200)).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    let now = phpyun_core::clock::now_ts();
    Ok(ApiJson(
        list.into_iter()
            .map(|j| crate::v1::wap::jobs::job_summary_from_dict(j, &dicts, now))
            .collect(),
    ))
}

