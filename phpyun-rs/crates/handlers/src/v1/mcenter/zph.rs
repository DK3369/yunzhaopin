//! Job-fair reservation (login required).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, ValidatedJson};
use phpyun_services::zph_service::{self, ReserveInput};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use phpyun_core::dto::{CreatedId, IdBody};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/zph/reserve", post(reserve))
        .route("/zph/my-reservation", post(my_reservation))
        .route("/zph/com-status", post(com_status))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ReserveForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,

    /// Comma-separated list of job ids
    #[validate(length(min = 0, max = 500))]
    #[serde(default)]
    pub job_ids: String,
    #[validate(length(min = 1, max = 64))]
    pub name: String,
    /// Contact phone (PHPYun field name moblie)
    #[validate(length(min = 6, max = 32))]
    pub moblie: String,
}

/// Reserve a job-fair slot
#[utoipa::path(post,
    path = "/v1/mcenter/zph/reserve",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = ReserveForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn reserve(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<ReserveForm>) -> AppResult<ApiJson<CreatedId>> {
    let id = f.id;
    user.require_jobseeker()?;
    let rid = zph_service::reserve(
        &state,
        &user,
        id,
        ReserveInput {
            job_ids: &f.job_ids,
            name: &f.name,
            mobile: &f.moblie,
        },
    )
    .await?;
    Ok(ApiJson(CreatedId { id: rid }))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MyReservation {
    pub id: u64,
    pub zid: u64,
    pub job_ids: String,
    pub name: String,
    pub mobile: String,
    pub status: i32,
    pub created_at: i64,
}

impl From<phpyun_models::zph::entity::ZphReservation> for MyReservation {
    fn from(r: phpyun_models::zph::entity::ZphReservation) -> Self {
        Self {
            id: r.id,
            zid: r.zid,
            job_ids: r.job_ids,
            name: r.name,
            mobile: r.mobile,
            status: r.status,
            created_at: r.created_at,
        }
    }
}

/// My reservation for a specific job fair
#[utoipa::path(post,
    path = "/v1/mcenter/zph/my-reservation",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn my_reservation(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiJson<Option<MyReservation>>> {
    let id = b.id;
    let row = zph_service::my_reservation(&state, &user, id).await?;
    Ok(ApiJson(row.map(MyReservation::from)))
}

// ==================== Pre-apply status (counterpart of `wap/ajax::ajaxComjob`) ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct OwnJobBrief {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ComStatusView {
    /// `applied` (already signed up) — see `status` for the application state.
    /// `not_applied` (eligible to apply) — see `jobs` for the picker list.
    /// `no_jobs` (must publish at least one job before applying).
    pub state: String,
    /// Only present when `state == "applied"`. 0 pending / 1 approved / 2 rejected.
    pub status: Option<i32>,
    /// Only present when `state == "not_applied"`.
    pub jobs: Option<Vec<OwnJobBrief>>,
}

/// Pre-apply status for an employer on a job fair.
#[utoipa::path(post,
    path = "/v1/mcenter/zph/com-status",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses(
        (status = 200, description = "ok", body = ComStatusView),
        (status = 403, description = "Not an employer"),
    )
)]
pub async fn com_status(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiJson<ComStatusView>> {
    let id = b.id;
    use zph_service::ComStatusOutcome;
    let view = match zph_service::com_status_for_fair(&state, &user, id).await? {
        ComStatusOutcome::Applied { status } => ComStatusView {
            state: "applied".into(),
            status: Some(status),
            jobs: None,
        },
        ComStatusOutcome::NotApplied { jobs } => ComStatusView {
            state: "not_applied".into(),
            status: None,
            jobs: Some(
                jobs.into_iter()
                    .map(|j| OwnJobBrief {
                        id: j.id,
                        name: j.name,
                    })
                    .collect(),
            ),
        },
        ComStatusOutcome::NoJobs => ComStatusView {
            state: "no_jobs".into(),
            status: None,
            jobs: Some(Vec::new()),
        },
    };
    Ok(ApiJson(view))
}

