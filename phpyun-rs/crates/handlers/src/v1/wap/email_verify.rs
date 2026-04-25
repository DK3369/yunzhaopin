//! Email change callback (public endpoint reached by clicking the link in the email).

use axum::{
    extract::{Query, State},
    routing::get,
    Router,
};
use phpyun_core::{ApiOk, AppResult, AppState};
use phpyun_services::contact_cert_service;
use serde::Deserialize;
use utoipa::IntoParams;

pub fn routes() -> Router<AppState> {
    Router::new().route("/cert/email/verify", get(verify))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct VerifyQuery {
    pub token: String,
}

/// Email token callback; on success updates member.email
#[utoipa::path(
    get,
    path = "/v1/wap/cert/email/verify",
    tag = "wap",
    params(VerifyQuery),
    responses((status = 200, description = "ok"), (status = 400, description = "expired or invalid"))
)]
pub async fn verify(
    State(state): State<AppState>,
    Query(q): Query<VerifyQuery>,
) -> AppResult<ApiOk> {
    contact_cert_service::verify_email_token(&state, &q.token).await?;
    Ok(ApiOk("email_verified"))
}
