//! Public site settings (keys with is_public=1).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::error::InfraError;
use phpyun_core::{ApiJson, AppError, AppResult, AppState, ValidatedJson};
use phpyun_services::site_setting_service;
use serde::Serialize;
use utoipa::ToSchema;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/site/settings", post(list))
        .route("/site/settings/get", post(get_one))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SettingView {
    pub key: String,
    pub value: String,
    pub description: String,
}

impl From<phpyun_models::site_setting::entity::SiteSetting> for SettingView {
    fn from(s: phpyun_models::site_setting::entity::SiteSetting) -> Self {
        Self {
            key: s.key_name,
            value: s.value,
            description: s.description,
        }
    }
}

/// List public settings
#[utoipa::path(post, path = "/v1/wap/site/settings/get", tag = "wap", responses((status = 200, description = "ok")))]
pub async fn list(State(state): State<AppState>) -> AppResult<ApiJson<Vec<SettingView>>> {
    let list = site_setting_service::list_public(&state).await?;
    Ok(ApiJson(list.into_iter().map(SettingView::from).collect()))
}

/// Single public setting
#[utoipa::path(post,
    path = "/v1/wap/site/settings",
    tag = "wap",
    request_body = GetOneBody,
    responses((status = 200, description = "ok", body = SettingView), (status = 404))
)]
pub async fn get_one(State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<GetOneBody>) -> AppResult<ApiJson<SettingView>> {
    let key = b.key;
    phpyun_core::validators::ensure_path_key(&key)?;
    let row = site_setting_service::get(&state, &key)
        .await?
        .filter(|s| s.is_public == 1)
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("setting_not_found".into())))?;
    Ok(ApiJson(SettingView::from(row)))
}

#[derive(Debug, serde::Deserialize, validator::Validate, utoipa::ToSchema)]
pub struct GetOneBody {
    #[validate(length(min = 1, max = 64), custom(function = "phpyun_core::validators::path_token"))]
    pub key: String,
}
