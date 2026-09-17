//! Public site settings (keys with is_public=1).

use axum::{extract::State, routing::post, Router};
use phpyun_core::{ApiError, ApiResponse, AppResult, AppState, ValidatedJson};
use phpyun_models::report::repo as report_repo;
use phpyun_services::site_setting_service;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use utoipa::ToSchema;

pub fn routes() -> Router<AppState> {
    Router::new().route("/site/settings/get", post(get_one))
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

pub(crate) async fn public_settings_map(
    state: &AppState,
) -> AppResult<BTreeMap<String, String>> {
    let list = site_setting_service::list_public(state).await?;
    let mut data = BTreeMap::new();
    for s in list {
        data.insert(s.key_name, s.value);
    }
    Ok(data)
}

pub(crate) async fn overlay_client_ip_banned(
    state: &AppState,
    ip: &str,
    settings: &mut BTreeMap<String, String>,
) {
    if phpyun_services::site_gate_service::ensure_ip_allowed(state, ip)
        .await
        .is_err()
    {
        settings.insert("sy_client_ip_banned".into(), "1".into());
    }
}

pub(crate) async fn report_reasons(state: &AppState) -> AppResult<Vec<ReportReasonView>> {
    let reasons = report_repo::list_reasons(state.db.reader()).await?;
    Ok(reasons
        .into_iter()
        .map(|reason| ReportReasonView {
            id: reason.id,
            code: reason.id.to_string(),
            name: phpyun_services::enum_labels::report_reason_name(reason.id, &reason.name),
        })
        .collect())
}


#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ReportReasonView {
    pub id: u64,
    /// Pass this value as `reason_code` when submitting a report.
    pub code: String,
    pub name: String,
}

/// Single public setting
#[utoipa::path(post,
    path = "/v1/wap/site/settings/get",
    tag = "wap",
    request_body = GetOneBody,
    responses((status = 200, description = "ok", body = SettingView), (status = 404))
)]
pub async fn get_one(
    State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<GetOneBody>,
) -> AppResult<ApiResponse<SettingView>> {
    let key = b.key;
    phpyun_core::validators::ensure_path_key(&key)?;
    let row = site_setting_service::get(&state, &key)
        .await?
        .filter(|s| s.is_public == 1)
        .ok_or_else(|| ApiError::param_invalid("setting_not_found"))?;
    Ok(ApiResponse::data(SettingView::from(row)))
}

#[derive(Debug, serde::Deserialize, validator::Validate, utoipa::ToSchema)]
pub struct GetOneBody {
    #[validate(
        length(min = 1, max = 64),
        custom(function = "phpyun_core::validators::path_token")
    )]
    pub key: String,
}
