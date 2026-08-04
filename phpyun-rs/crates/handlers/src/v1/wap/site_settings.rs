//! Public site settings (keys with is_public=1).

use axum::{
    extract::State,
    Json,
    Router,
    routing::post,
};
use phpyun_core::error::InfraError;
use phpyun_core::{ApiJson, AppError, AppResult, AppState, Lang, ValidatedJson};
use phpyun_models::report::repo as report_repo;
use phpyun_services::site_setting_service;
use serde::Serialize;
use serde_json::{Value, json};
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

/// List public settings, or return selectable report reasons when
/// `key=report_reasons`.
#[utoipa::path(
    post,
    path = "/v1/wap/site/settings",
    tag = "wap",
    request_body = GetOneBody,
    responses((status = 200, description = "Public settings, or report reason options for report_reasons"))
)]
pub async fn list(
    State(state): State<AppState>,
    lang: Lang,
    body: Option<Json<GetOneBody>>,
) -> AppResult<ApiJson<Value>> {
    if body.as_ref().is_some_and(|b| b.key == "report_reasons") {
        let reasons = report_repo::list_reasons(state.db.reader()).await?;
        let data: Vec<ReportReasonView> = reasons
            .into_iter()
            .map(|reason| ReportReasonView {
                id: reason.id,
                code: reason.id.to_string(),
                name: localize_reason(reason.id, &reason.name, lang).to_owned(),
            })
            .collect();
        return Ok(ApiJson(json!(data)));
    }

    let list = site_setting_service::list_public(&state).await?;
    let data: Vec<SettingView> = list.into_iter().map(SettingView::from).collect();
    Ok(ApiJson(json!(data)))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReportReasonView {
    pub id: u64,
    /// Pass this value as `reason_code` when submitting a report.
    pub code: String,
    pub name: String,
}

fn localize_reason<'a>(id: u64, database_name: &'a str, lang: Lang) -> &'a str {
    match lang {
        Lang::ZhCN => database_name,
        Lang::ZhTW => match id {
            1 => "非建設性提問",
            2 => "不友善言論、垃圾內容與不適宜討論的內容",
            3 => "不構成提問或問題表意不明確",
            4 => "問題已失效或過期",
            5 => "廣告等垃圾資訊",
            6 => "違法違規內容",
            7 => "不宜公開討論的政治內容",
            _ => database_name,
        },
        Lang::En => match id {
            1 => "Non-constructive content",
            2 => "Abusive, spam, or inappropriate content",
            3 => "Unclear or invalid question",
            4 => "Outdated or no longer relevant",
            5 => "Advertising or spam",
            6 => "Illegal or prohibited content",
            7 => "Sensitive political content",
            _ => database_name,
        },
    }
}

/// Single public setting
#[utoipa::path(post,
    path = "/v1/wap/site/settings/get",
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
