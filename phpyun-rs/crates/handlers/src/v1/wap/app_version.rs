//! App version check (public).

use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};
use phpyun_core::{ApiJson, AppResult, AppState};
use phpyun_services::app_version_service;
use serde::Serialize;
use utoipa::ToSchema;

pub fn routes() -> Router<AppState> {
    Router::new().route("/app-version/{platform}", get(latest))
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
#[utoipa::path(
    get,
    path = "/v1/wap/app-version/{platform}",
    tag = "wap",
    params(("platform" = String, Path, description = "ios/android/windows/mac")),
    responses((status = 200, description = "ok"))
)]
pub async fn latest(
    State(state): State<AppState>,
    Path(platform): Path<String>,
) -> AppResult<ApiJson<Option<VersionView>>> {
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
