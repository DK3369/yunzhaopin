//! Announcements. PHP fields include `startime` (not starttime).

use axum::{extract::State, routing::post, Json, Router};
use phpyun_core::dto::{CreatedId, IdBody};
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_models::announcement::entity::Announcement;
use phpyun_services::admin_cms_service::{self, AnnouncementUpsertIn};
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/announcements", post(upsert))
        .route("/announcements/list", post(list))
        .route("/announcements/delete", post(delete))
        .route("/announcements/php-add", post(php_add))
}

#[utoipa::path(post, path = "/v1/admin/announcements/list", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<Announcement>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_cms_service::list_announcements(&state, page).await?,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AnnouncementForm {
    pub id: Option<u64>,
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    #[serde(default)]
    pub keyword: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub startime: i64,
    #[serde(default)]
    pub endtime: i64,
    #[serde(default)]
    pub did: u64,
}

#[utoipa::path(post, path = "/v1/admin/announcements", tag = "admin", security(("bearer" = [])), request_body = AnnouncementForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn upsert(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<AnnouncementForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    user.require_admin()?;
    let id = admin_cms_service::upsert_announcement(
        &state,
        &user,
        AnnouncementUpsertIn {
            id: f.id,
            title: &f.title,
            keyword: &f.keyword,
            description: &f.description,
            content: &f.content,
            startime: f.startime,
            endtime: f.endtime,
            did: f.did,
        },
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

#[utoipa::path(post, path = "/v1/admin/announcements/delete", tag = "admin", security(("bearer" = [])), request_body = IdBody, responses((status = 200, description = "ok")))]
pub async fn delete(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_cms_service::delete_announcement(&state, &user, f.id).await?;
    Ok(ApiResponse::message("ok"))
}

/// PHP `announcement::add_action`：GET 表单 / POST `submit` 写入。
#[utoipa::path(post, path = "/v1/admin/announcements/php-add", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn php_add(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    let submit = body.get("submit").is_some()
        && body.get("submit") != Some(&serde_json::Value::Bool(false))
        && body.get("submit") != Some(&serde_json::json!(0))
        && body.get("submit") != Some(&serde_json::Value::String("0".into()));
    let data = admin_cms_service::announcement_php_add(&state, &user, &body).await?;
    if submit {
        return Ok(ApiResponse::message_data("admin_model_00189", data));
    }
    Ok(ApiResponse::data(data))
}
