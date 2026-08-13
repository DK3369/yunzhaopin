//! GET /v1/wap/captcha — issue an image CAPTCHA (PNG base64 data URI).

use axum::{extract::State, routing::post, Router};
use phpyun_core::{ApiResponse, AppResult, AppState};
use phpyun_services::captcha_service;
use serde::Serialize;
use utoipa::ToSchema;

pub fn routes() -> Router<AppState> {
    Router::new().route("/captcha", post(issue))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CaptchaData {
    /// CAPTCHA cid; send back to the server when submitting registration/login
    pub cid: String,
    /// Data URI in `data:image/png;base64,...` format;
    /// the frontend can render it directly via `<img src={image} />`.
    pub image: String,
}

/// Issue an image CAPTCHA
#[utoipa::path(
    post,
    path = "/v1/wap/captcha",
    tag = "auth",
    responses(
        (status = 200, description = "Issued", body = CaptchaData),
    )
)]
pub async fn issue(State(state): State<AppState>) -> AppResult<ApiResponse<CaptchaData>> {
    let r = captcha_service::issue(&state).await?;
    Ok(ApiResponse::data(CaptchaData {
        cid: r.cid,
        image: r.image,
    }))
}
