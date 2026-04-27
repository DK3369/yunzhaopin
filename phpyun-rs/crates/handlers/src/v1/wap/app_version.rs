//! App version check (public).

use axum::{
    extract::{Path, State},
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, AppResult, AppState, ValidatedJson};
use phpyun_services::app_version_service;
use serde::Serialize;
use utoipa::ToSchema;
use phpyun_core::dto::{};

pub fn routes() -> Router<AppState> {
    Router::new().route("/app-version", post(latest))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VersionView {
    pub platform: String,
    pub version: String,
    pub version_code: u32,
    pub is_force: bool,
    pub download_url: String,
    pub changelog: String,
    pub released_at: i64,
}

/// Get the latest version for a platform
#[utoipa::path(post,
    path = "/v1/wap/app-version",
    tag = "wap",
    request_body = LatestBody,
    responses((status = 200, description = "ok"))
)]
pub async fn latest(State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<LatestBody>) -> AppResult<ApiJson<Option<VersionView>>> {
    let platform = b.platform;
    phpyun_core::validators::ensure_path_token(&platform)?;
    let v = app_version_service::latest(&state, &platform).await?;
    Ok(ApiJson(v.map(|v| VersionView {
        platform: v.platform,
        version: v.version,
        version_code: v.version_code,
        is_force: v.is_force == 1,
        download_url: v.download_url,
        changelog: v.changelog,
        released_at: v.released_at,
    })))
}

#[derive(Debug, serde::Deserialize, validator::Validate, utoipa::ToSchema)]
pub struct LatestBody {
    #[validate(length(min = 1, max = 64), custom(function = "phpyun_core::validators::path_token"))]
    pub platform: String,
}
