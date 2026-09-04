//! Company claim (public: can be submitted without login).

use axum::{extract::State, routing::{get, post}, Router};
use phpyun_core::{dto::OkResp, ApiResponse, AppResult, AppState, ClientIp, ValidatedJson, ValidatedJsonOrQuery};
use phpyun_services::claim_service::{self, ClaimInput};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub const GET_ALLOWED_PATHS: &[&str] = &["/v1/wap/claim/check"];

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/claim", post(claim))
        .route("/claim/check", get(check).post(check))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ClaimForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub uid: u64,
    #[validate(length(min = 1, max = 64))]
    pub code: String,
    #[validate(length(min = 3, max = 32))]
    pub username: String,
    #[validate(length(min = 6, max = 64))]
    pub password: String,
}

/// Claim a company
#[utoipa::path(
    post,
    path = "/v1/wap/claim",
    tag = "wap",
    request_body = ClaimForm,
    responses((status = 200, description = "ok", body = OkResp))
)]
pub async fn claim(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<ClaimForm>,
) -> AppResult<ApiResponse<OkResp>> {
    claim_service::claim(
        &state,
        ClaimInput {
            uid: f.uid,
            code: &f.code,
            username: &f.username,
            password: &f.password,
            client_ip: &ip,
        },
    )
    .await?;
    Ok(ApiResponse::data(OkResp { ok: true }))
}

#[derive(Debug, Deserialize, Validate, IntoParams, ToSchema)]
pub struct ClaimCheckQuery {
    #[validate(range(min = 1, max = 99_999_999))]
    pub uid: u64,
    #[validate(length(min = 1, max = 64))]
    pub code: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ClaimCheckView {
    pub ok: bool,
}

#[utoipa::path(
    get,
    path = "/v1/wap/claim/check",
    tag = "wap",
    params(ClaimCheckQuery),
    responses((status = 200, description = "ok", body = ClaimCheckView))
)]
pub async fn check(
    State(state): State<AppState>,
    ValidatedJsonOrQuery(q): ValidatedJsonOrQuery<ClaimCheckQuery>,
) -> AppResult<ApiResponse<ClaimCheckView>> {
    let r = claim_service::check(&state, q.uid, &q.code).await?;
    Ok(ApiResponse::data(ClaimCheckView { ok: r.ok }))
}
