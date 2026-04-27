//! Company claim (public: can be submitted without login).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{dto::OkResp, ApiJson, AppResult, AppState, ClientIp, ValidatedJson};
use phpyun_services::claim_service::{self, ClaimInput};
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new().route("/claim", post(claim))
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
) -> AppResult<ApiJson<OkResp>> {
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
    Ok(ApiJson(OkResp { ok: true }))
}
