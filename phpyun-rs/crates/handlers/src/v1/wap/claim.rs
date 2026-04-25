//! Company claim (public: can be submitted without login).

use axum::{extract::State, routing::post, Router};
use phpyun_core::{ApiJson, AppResult, AppState, ClientIp, ValidatedJson};
use phpyun_services::claim_service::{self, ClaimInput};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new().route("/claim", post(claim))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ClaimForm {
    pub uid: u64,
    #[validate(length(min = 1, max = 64))]
    pub code: String,
    #[validate(length(min = 3, max = 32))]
    pub username: String,
    #[validate(length(min = 6, max = 64))]
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ClaimResult {
    pub ok: bool,
}

/// Claim a company
#[utoipa::path(
    post,
    path = "/v1/wap/claim",
    tag = "wap",
    request_body = ClaimForm,
    responses((status = 200, description = "ok", body = ClaimResult))
)]
pub async fn claim(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<ClaimForm>,
) -> AppResult<ApiJson<ClaimResult>> {
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
    Ok(ApiJson(ClaimResult { ok: true }))
}
