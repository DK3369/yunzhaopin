//! Job-fair reservation (login required).

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{
    ApiJson, AppResult, AppState, AuthenticatedUser, ValidatedJson,
};
use phpyun_services::zph_service::{self, ReserveInput};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/zph/{id}/reserve", post(reserve))
        .route("/zph/{id}/my-reservation", get(my_reservation))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ReserveForm {
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

#[derive(Debug, Serialize, ToSchema)]
pub struct ReservedId {
    pub id: u64,
}

/// Reserve a job-fair slot
#[utoipa::path(
    post,
    path = "/v1/mcenter/zph/{id}/reserve",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    request_body = ReserveForm,
    responses((status = 200, description = "ok", body = ReservedId))
)]
pub async fn reserve(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
    ValidatedJson(f): ValidatedJson<ReserveForm>,
) -> AppResult<ApiJson<ReservedId>> {
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
    Ok(ApiJson(ReservedId { id: rid }))
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
#[utoipa::path(
    get,
    path = "/v1/mcenter/zph/{id}/my-reservation",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn my_reservation(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
) -> AppResult<ApiJson<Option<MyReservation>>> {
    let row = zph_service::my_reservation(&state, &user, id).await?;
    Ok(ApiJson(row.map(MyReservation::from)))
}
