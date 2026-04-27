//! Job review (admin).

use axum::{
    extract::{Path, State},
    Router,
    routing::{get, post},
};
use phpyun_core::{ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson};
use phpyun_services::admin_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/jobs", post(list))
        .route("/jobs/state", post(set_state))
        .route("/jobs/batch/state", post(batch_set_state))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct JobListQuery {
    /// 0=pending / 1=approved / 2=rejected
    #[validate(range(min = 0, max = 99))]
    pub state: Option<i32>,
}

/// Admin job review item — **reuses** `wap::jobs::JobSummary` (34 fields, all dict translations + promo derivations + time formatting).
///
/// Single field convention: review admin, public list / home / global search / employer self-admin all use the same Summary.
pub type AdminJobItem = crate::v1::wap::jobs::JobSummary;

/// Job review queue
#[utoipa::path(
    post,
    path = "/v1/admin/jobs",
    tag = "admin",
    security(("bearer" = [])),
    params(JobListQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<JobListQuery>,
) -> AppResult<ApiJson<Paged<AdminJobItem>>> {
    user.require_admin()?;
    let r = admin_service::list_jobs(&state, q.state, page).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    let now = phpyun_core::clock::now_ts();
    Ok(ApiJson(Paged::new(
        r.list
            .into_iter()
            .map(|j| crate::v1::wap::jobs::job_summary_from_dict(j, &dicts, now))
            .collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SetJobStateForm {
    #[validate(range(min = 1, max = 999_999_999))]
    pub id: u64,
    /// 1=approved / 2=rejected
    #[validate(range(min = 1, max = 2))]
    pub state: i32,
}

/// Review a job
#[utoipa::path(post,
    path = "/v1/admin/jobs/state",
    tag = "admin",
    security(("bearer" = [])),
    request_body = SetJobStateForm,
    responses((status = 200, description = "ok"))
)]
pub async fn set_state(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SetJobStateForm>) -> AppResult<ApiOk> {
    let id = f.id;
    user.require_admin()?;
    admin_service::set_job_state(&state, &user, id, f.state).await?;
    Ok(ApiOk("ok"))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct BatchStateForm {
    #[validate(length(min = 1, max = 200))]
    pub ids: Vec<u64>,
    #[validate(range(min = 1, max = 2))]
    pub state: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BatchResult {
    pub requested: usize,
    pub affected: u64,
}

/// Batch review jobs
#[utoipa::path(
    post,
    path = "/v1/admin/jobs/batch/state",
    tag = "admin",
    security(("bearer" = [])),
    request_body = BatchStateForm,
    responses((status = 200, description = "ok", body = BatchResult))
)]
pub async fn batch_set_state(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<BatchStateForm>,
) -> AppResult<ApiJson<BatchResult>> {
    user.require_admin()?;
    let r = admin_service::batch_set_job_state(&state, &user, &f.ids, f.state).await?;
    Ok(ApiJson(BatchResult {
        requested: r.requested,
        affected: r.affected,
    }))
}
