//! Admin dashboard aggregate.

use axum::{extract::State, routing::post, Router};
use phpyun_core::utils::fmt_dt;
use phpyun_core::{ApiResponse, AppResult, AppState, AuthenticatedUser, ValidatedJson};
use phpyun_services::{admin_dashboard_service, category_service, dict_service, site_setting_service};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/dashboard/overview", post(overview))
        .route("/dashboard/recent-signups", post(recent_signups))
        .route("/dashboard/msg-num", post(msg_num))
        .route("/dashboard/home-data", post(home_data))
        .route("/dashboard/ajax-statis", post(ajax_statis))
        .route("/dashboard/month-statis", post(month_statis))
        .route("/dashboard/ajax-right", post(ajax_right))
        .route("/dashboard/chart", post(chart))
        .route("/cache/clear", post(cache_clear))
        .route("/cache/php-dicts", post(php_dicts))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OverviewView {
    pub pending_company_certs: u64,
    pub pending_jobs: u64,
    pub pending_reports: u64,
    pub pending_feedback: u64,
    pub total_users: u64,
    pub active_companies: u64,
    pub active_jobs: u64,
    pub active_resumes: u64,
    pub today_new_jobs: u64,
    pub today_new_resumes: u64,
}

/// Review queue + activity snapshot
#[utoipa::path(
    post,
    path = "/v1/admin/dashboard/overview",
    tag = "admin",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = OverviewView))
)]
pub async fn overview(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<OverviewView>> {
    user.require_admin()?;
    let o = admin_dashboard_service::overview(&state, &user).await?;
    Ok(ApiResponse::data(OverviewView {
        pending_company_certs: o.pending_company_certs,
        pending_jobs: o.pending_jobs,
        pending_reports: o.pending_reports,
        pending_feedback: o.pending_feedback,
        total_users: o.total_users,
        active_companies: o.active_companies,
        active_jobs: o.active_jobs,
        active_resumes: o.active_resumes,
        today_new_jobs: o.today_new_jobs,
        today_new_resumes: o.today_new_resumes,
    }))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct RecentQuery {
    #[serde(default = "default_limit")]
    #[validate(range(min = 1, max = 200))]
    pub limit: u64,
}
fn default_limit() -> u64 {
    10
}

fn usertype_name(t: i32) -> &'static str {
    match t {
        1 => "jobseeker",
        2 => "company",
        3 => "admin",
        _ => "unknown",
    }
}

fn user_status_name(s: i32) -> &'static str {
    match s {
        0 => "pending",
        1 => "active",
        2 => "locked",
        3 => "deleted",
        _ => "unknown",
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RecentUser {
    pub uid: u64,
    pub username: String,
    pub email: Option<String>,
    pub moblie: Option<String>,
    pub usertype: i32,
    pub usertype_n: String,
    pub status: i32,
    pub status_n: String,
    pub did: u64,
    pub reg_date: i64,
    pub reg_date_n: String,
    pub login_date: Option<i64>,
    pub login_date_n: String,
}

/// Recent signups
#[utoipa::path(
    post,
    path = "/v1/admin/dashboard/recent-signups",
    tag = "admin",
    security(("bearer" = [])),
    params(RecentQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn recent_signups(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(q): ValidatedJson<RecentQuery>,
) -> AppResult<ApiResponse<Vec<RecentUser>>> {
    user.require_admin()?;
    let list = admin_dashboard_service::recent_signups(&state, &user, q.limit).await?;
    Ok(ApiResponse::data(
        list.into_iter()
            .map(|m| RecentUser {
                uid: m.uid,
                username: m.username,
                email: m.email,
                moblie: m.moblie,
                usertype_n: usertype_name(m.usertype).to_string(),
                usertype: m.usertype,
                status_n: user_status_name(m.status).to_string(),
                status: m.status,
                did: m.did,
                reg_date_n: fmt_dt(m.reg_date),
                reg_date: m.reg_date,
                login_date_n: fmt_dt(m.login_date.unwrap_or(0)),
                login_date: m.login_date,
            })
            .collect(),
    ))
}

/// PHP `msgNum()` pending-review badges.
#[utoipa::path(
    post,
    path = "/v1/admin/dashboard/msg-num",
    tag = "admin",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn msg_num(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<phpyun_models::admin_msg::repo::AdminMsgNum>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_dashboard_service::msg_num(&state, &user).await?,
    ))
}

#[derive(Debug, Deserialize, Default, Validate, ToSchema)]
#[serde(default)]
pub struct DashboardFilter {
    #[validate(length(max = 16))]
    pub r#type: Option<String>,
    #[validate(length(max = 16))]
    pub kind: Option<String>,
    #[validate(length(max = 16))]
    pub area: Option<String>,
    #[validate(length(max = 32))]
    pub sdate: Option<String>,
    #[validate(length(max = 32))]
    pub edate: Option<String>,
}

#[utoipa::path(post, path = "/v1/admin/dashboard/home-data", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn home_data(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<serde_json::Value>> {
    Ok(ApiResponse::data(
        admin_dashboard_service::home_data(&state, &user).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/dashboard/ajax-statis", tag = "admin", security(("bearer" = [])), request_body = DashboardFilter, responses((status = 200, description = "ok")))]
pub async fn ajax_statis(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(q): ValidatedJson<DashboardFilter>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    Ok(ApiResponse::data(
        admin_dashboard_service::ajax_statis(
            &state,
            &user,
            admin_dashboard_service::AjaxStatisQuery {
                r#type: q.r#type,
                area: q.area,
            },
        )
        .await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/dashboard/month-statis", tag = "admin", security(("bearer" = [])), request_body = DashboardFilter, responses((status = 200, description = "ok")))]
pub async fn month_statis(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(q): ValidatedJson<DashboardFilter>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    Ok(ApiResponse::data(
        admin_dashboard_service::month_statis(
            &state,
            &user,
            admin_dashboard_service::MonthStatisQuery {
                sdate: q.sdate,
                edate: q.edate,
            },
        )
        .await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/dashboard/ajax-right", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn ajax_right(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<serde_json::Value>> {
    Ok(ApiResponse::data(
        admin_dashboard_service::ajax_right(&state, &user).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/dashboard/chart", tag = "admin", security(("bearer" = [])), request_body = DashboardFilter, responses((status = 200, description = "ok")))]
pub async fn chart(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(q): ValidatedJson<DashboardFilter>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    Ok(ApiResponse::data(
        admin_dashboard_service::chart(
            &state,
            &user,
            q.kind.as_deref().or(q.r#type.as_deref()).unwrap_or(""),
            admin_dashboard_service::MonthStatisQuery {
                sdate: q.sdate,
                edate: q.edate,
            },
        )
        .await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/cache/clear", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn cache_clear(
    user: AuthenticatedUser,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    Ok(ApiResponse::message("ok"))
}

/// PHP `common/cache` + `getCacheData` (job/city cascader + search_list).
#[utoipa::path(post, path = "/v1/admin/cache/php-dicts", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn php_dicts(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<serde_json::Value>> {
    user.require_admin()?;
    let dicts = dict_service::get(&state).await?;
    let jobs = category_service::list(&state, "job").await?;
    let cities = category_service::list(&state, "city").await?;
    let job_nodes: Vec<(u64, u64, String)> = jobs
        .iter()
        .map(|c| (c.id, c.parent_id, c.name.clone()))
        .collect();
    let city_nodes: Vec<(u64, u64, String)> = cities
        .iter()
        .map(|c| (c.id, c.parent_id, c.name.clone()))
        .collect();
    let mut data = admin_dashboard_service::php_cache_payload(
        &job_nodes,
        &city_nodes,
        &dicts.comclass_by_variable("job_edu"),
        &dicts.comclass_by_variable("job_exp"),
    );
    let map_key = site_setting_service::get(&state, "map_key")
        .await?
        .map(|s| s.value)
        .unwrap_or_default();
    let map_secret = site_setting_service::get(&state, "map_secret")
        .await?
        .map(|s| s.value)
        .unwrap_or_default();
    admin_dashboard_service::attach_amap(&mut data, &map_key, &map_secret);
    Ok(ApiResponse::data(data))
}
