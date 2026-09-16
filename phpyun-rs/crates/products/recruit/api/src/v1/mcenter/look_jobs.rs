//! Employer: who viewed my jobs. Jobseeker: jobs I viewed (PHP `look_job`).

use axum::{extract::State, routing::post, Router};
use phpyun_core::json;
use phpyun_core::utils::fmt_dt;
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_services::look_job_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/look-jobs/list", post(list_mine))
        .route("/look-jobs/mine", post(list_mine_seeker))
        .route("/look-jobs/delete", post(delete_mine_seeker))
}

#[derive(Debug, Deserialize, Validate, IntoParams, ToSchema, Default)]
pub struct LookJobsQuery {
    #[serde(default)]
    #[validate(length(max = 60))]
    pub keyword: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LookJobDeleteBody {
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999_999))]
    pub id: Option<u64>,
    #[serde(default)]
    #[validate(length(max = 200))]
    pub ids: Option<Vec<u64>>,
}

fn collect_ids(id: Option<u64>, ids: Option<Vec<u64>>) -> Vec<u64> {
    let mut out = ids.unwrap_or_default();
    if let Some(i) = id.filter(|x| *x > 0) {
        if !out.contains(&i) {
            out.push(i);
        }
    }
    out.retain(|x| *x > 0 && *x <= 99_999_999);
    out
}

fn keyword_opt(q: &LookJobsQuery) -> Option<&str> {
    q.keyword
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LookJobItem {
    pub id: u64,
    pub uid: u64,
    pub job_id: u64,
    pub job_name: String,
    pub com_name: String,
    pub minsalary: i32,
    pub maxsalary: i32,
    pub datetime: i64,
    pub datetime_n: String,
    pub uname: String,
    pub eid: i64,
}

impl From<phpyun_models::look_job::LookJob> for LookJobItem {
    fn from(r: phpyun_models::look_job::LookJob) -> Self {
        Self {
            id: r.id,
            uid: r.uid,
            job_id: r.jobid,
            job_name: r.job_name,
            com_name: r.com_name,
            minsalary: r.minsalary,
            maxsalary: r.maxsalary,
            datetime_n: fmt_dt(r.datetime),
            datetime: r.datetime,
            uname: r.uname,
            eid: r.eid,
        }
    }
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/look-jobs/list",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = LookJobsQuery,
    responses((status = 200, description = "ok"))
)]
pub async fn list_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<LookJobsQuery>,
) -> AppResult<ApiResponse<Paged<LookJobItem>>> {
    let r = look_job_service::list_mine(&state, &user, page, keyword_opt(&q)).await?;
    Ok(ApiResponse::data(Paged::from_listing(
        r.list.into_iter().map(LookJobItem::from).collect::<Vec<_>>(),
        r.total,
        page,
    )))
}

/// PHP `member/user/look_job` — jobs this jobseeker has viewed.
#[utoipa::path(
    post,
    path = "/v1/mcenter/look-jobs/mine",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_mine_seeker(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<LookJobItem>>> {
    let r = look_job_service::list_mine_seeker(&state, &user, page).await?;
    Ok(ApiResponse::data(Paged::from_listing(
        r.list.into_iter().map(LookJobItem::from).collect::<Vec<_>>(),
        r.total,
        page,
    )))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/look-jobs/delete",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = LookJobDeleteBody,
    responses((status = 200, description = "ok"))
)]
pub async fn delete_mine_seeker(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<LookJobDeleteBody>,
) -> AppResult<ApiResponse<json::Value>> {
    let ids = collect_ids(b.id, b.ids);
    let n = if user.usertype == 2 {
        look_job_service::hide_mine_employer(&state, &user, &ids).await?
    } else {
        look_job_service::hide_mine_seeker(&state, &user, &ids).await?
    };
    Ok(ApiResponse::data(json::json!({ "deleted": n })))
}
