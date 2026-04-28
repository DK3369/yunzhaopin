//! Site settings management (admin).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, ValidatedJson};
use phpyun_services::site_setting_service::{self, UpsertInput};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use phpyun_core::utils::{fmt_dt};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/site-settings", post(upsert))
        .route("/site-settings/list", post(list))
        .route("/site-settings/delete", post(remove))
}


#[derive(Debug, Serialize, ToSchema)]
pub struct SettingItem {
    pub key: String,
    pub value: String,
    pub description: String,
    pub is_public_int: i32,
    pub is_public: bool,
    pub updated_at: i64,
    pub updated_at_n: String,
}

impl From<phpyun_models::site_setting::entity::SiteSetting> for SettingItem {
    fn from(s: phpyun_models::site_setting::entity::SiteSetting) -> Self {
        Self {
            key: s.key_name,
            value: s.value,
            description: s.description,
            is_public: s.is_public == 1,
            is_public_int: s.is_public,
            updated_at_n: fmt_dt(s.updated_at),
            updated_at: s.updated_at,
        }
    }
}

/// All settings (including non-public)
#[utoipa::path(
    post,
    path = "/v1/admin/site-settings/list",
    tag = "admin",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<Vec<SettingItem>>> {
    user.require_admin()?;
    let list = site_setting_service::admin_list(&state, &user).await?;
    Ok(ApiJson(list.into_iter().map(SettingItem::from).collect()))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpsertForm {
    #[validate(length(min = 1, max = 64))]
    pub key: String,
    #[validate(length(max = 65_000))]
    pub value: String,
    #[validate(length(max = 500))]
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub is_public: bool,
}

/// Create / update setting
#[utoipa::path(
    post,
    path = "/v1/admin/site-settings",
    tag = "admin",
    security(("bearer" = [])),
    request_body = UpsertForm,
    responses((status = 200, description = "ok"))
)]
pub async fn upsert(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<UpsertForm>,
) -> AppResult<ApiOk> {
    user.require_admin()?;
    site_setting_service::admin_upsert(
        &state,
        &user,
        UpsertInput {
            key: &f.key,
            value: &f.value,
            description: &f.description,
            is_public: f.is_public,
        },
    )
    .await?;
    Ok(ApiOk("ok"))
}

/// Delete setting
#[utoipa::path(post,
    path = "/v1/admin/site-settings/delete",
    tag = "admin",
    security(("bearer" = [])),
    request_body = RemoveBody,
    responses((status = 200, description = "ok"))
)]
pub async fn remove(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<RemoveBody>) -> AppResult<ApiOk> {
    let key = b.key;
    phpyun_core::validators::ensure_path_key(&key)?;
    user.require_admin()?;
    site_setting_service::admin_delete(&state, &user, &key).await?;
    Ok(ApiOk("deleted"))
}

#[derive(Debug, serde::Deserialize, validator::Validate, utoipa::ToSchema)]
pub struct RemoveBody {
    #[validate(length(min = 1, max = 64), custom(function = "phpyun_core::validators::path_token"))]
    pub key: String,
}
