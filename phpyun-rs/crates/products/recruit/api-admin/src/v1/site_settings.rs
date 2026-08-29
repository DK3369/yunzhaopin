//! Site settings management (admin).

use axum::{extract::State, routing::post, Json, Router};
use phpyun_core::utils::fmt_dt;
use phpyun_core::{ApiResponse, AppResult, AppState, AuthenticatedUser, ValidatedJson};
use phpyun_services::site_setting_service::{self, UpsertInput};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/site-settings", post(upsert))
        .route("/site-settings/list", post(list))
        .route("/site-settings/delete", post(remove))
        .route("/site-settings/batch", post(batch))
        .route("/site-settings/payset", post(payset))
        .route("/site-settings/payset/alipay", post(payset_alipay))
        .route("/site-settings/payset/tenpay", post(payset_tenpay))
        .route("/site-settings/payset/bank", post(payset_bank))
        .route("/site-settings/payset/bank-delete", post(payset_bank_delete))
        .route("/site-settings/php-seo", post(php_seo))
        .route("/site-settings/php-seo-add", post(php_seo_add))
        .route("/site-settings/php-seo-save", post(php_seo_save))
        .route("/site-settings/php-seo-del", post(php_seo_del))
        .route("/site-settings/php-regset", post(php_regset))
        .route("/site-settings/php-regset-save", post(php_regset_save))
        .route("/site-settings/php-messageset", post(php_messageset))
        .route("/site-settings/php-hbconfig", post(php_hbconfig))
        .route("/site-settings/php-hb-saveset", post(php_hb_saveset))
        .route("/site-settings/php-hb-list", post(php_hb_list))
        .route("/site-settings/php-hb-save-open", post(php_hb_save_open))
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
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<Vec<SettingItem>>> {
    user.require_admin()?;
    let list = site_setting_service::admin_list(&state, &user).await?;
    Ok(ApiResponse::data(
        list.into_iter().map(SettingItem::from).collect(),
    ))
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
) -> AppResult<ApiResponse> {
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
    Ok(ApiResponse::message("ok"))
}

/// Delete setting
#[utoipa::path(post,
    path = "/v1/admin/site-settings/delete",
    tag = "admin",
    security(("bearer" = [])),
    request_body = RemoveBody,
    responses((status = 200, description = "ok"))
)]
pub async fn remove(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<RemoveBody>,
) -> AppResult<ApiResponse> {
    let key = b.key;
    phpyun_core::validators::ensure_path_key(&key)?;
    user.require_admin()?;
    site_setting_service::admin_delete(&state, &user, &key).await?;
    Ok(ApiResponse::message("deleted"))
}

#[derive(Debug, serde::Deserialize, validator::Validate, utoipa::ToSchema)]
pub struct RemoveBody {
    #[validate(
        length(min = 1, max = 64),
        custom(function = "phpyun_core::validators::path_token")
    )]
    pub key: String,
}

#[utoipa::path(
    post,
    path = "/v1/admin/site-settings/batch",
    tag = "admin",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn batch(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    let obj = body.as_object().cloned().unwrap_or_default();
    for (k, v) in obj {
        if k.is_empty()
            || k.len() > 64
            || k == "pytoken"
            || k == "m"
            || k == "c"
            || k == "a"
            || k == "config"
        {
            continue;
        }
        if phpyun_core::validators::path_token(&k).is_err() {
            continue;
        }
        let value = match v {
            serde_json::Value::String(s) => s,
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::Bool(b) => {
                if b {
                    "1".into()
                } else {
                    "0".into()
                }
            }
            serde_json::Value::Null => continue,
            other => other.to_string(),
        };
        if value.len() > 65_000 {
            continue;
        }
        site_setting_service::admin_upsert(
            &state,
            &user,
            UpsertInput {
                key: &k,
                value: &value,
                description: "",
                is_public: false,
            },
        )
        .await?;
    }
    Ok(ApiResponse::message("ok"))
}

/// PHP `set_payset::index`.
#[utoipa::path(
    post,
    path = "/v1/admin/site-settings/payset",
    tag = "admin",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn payset(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<serde_json::Value>> {
    Ok(ApiResponse::data(
        site_setting_service::payset_index(&state, &user).await?,
    ))
}

/// PHP `set_payset::alipay`.
#[utoipa::path(
    post,
    path = "/v1/admin/site-settings/payset/alipay",
    tag = "admin",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn payset_alipay(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse> {
    site_setting_service::payset_alipay(&state, &user, &body).await?;
    Ok(ApiResponse::message("admin_01397"))
}

/// PHP `set_payset::tenpay`.
#[utoipa::path(
    post,
    path = "/v1/admin/site-settings/payset/tenpay",
    tag = "admin",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn payset_tenpay(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse> {
    site_setting_service::payset_tenpay(&state, &user, &body).await?;
    Ok(ApiResponse::message("admin_01398"))
}

/// PHP `set_payset::bank`.
#[utoipa::path(
    post,
    path = "/v1/admin/site-settings/payset/bank",
    tag = "admin",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn payset_bank(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse> {
    let name = body
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim();
    let bank_name = body
        .get("bank_name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim();
    let bank_number = body
        .get("bank_number")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim();
    let bank_address = body
        .get("bank_address")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim();
    if name.is_empty() || bank_name.is_empty() || bank_number.is_empty() || bank_address.is_empty()
    {
        return Err(phpyun_core::ApiError::param_invalid("bank_fields"));
    }
    let id = match body.get("id") {
        Some(serde_json::Value::Number(n)) => n.as_u64(),
        Some(serde_json::Value::String(s)) if !s.is_empty() => s.parse().ok(),
        _ => None,
    };
    site_setting_service::payset_bank_upsert(
        &state,
        &user,
        site_setting_service::BankIn {
            id,
            name,
            bank_name,
            bank_number,
            bank_address,
        },
    )
    .await?;
    Ok(ApiResponse::message("ok"))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct BankDelBody {
    #[serde(default, alias = "del")]
    pub id: u64,
}

/// PHP `set_payset::del`.
#[utoipa::path(
    post,
    path = "/v1/admin/site-settings/payset/bank-delete",
    tag = "admin",
    security(("bearer" = [])),
    request_body = BankDelBody,
    responses((status = 200, description = "ok"))
)]
pub async fn payset_bank_delete(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<BankDelBody>,
) -> AppResult<ApiResponse> {
    site_setting_service::payset_bank_delete(&state, &user, b.id).await?;
    Ok(ApiResponse::message("ok"))
}

fn body_u64(body: &serde_json::Value, key: &str) -> u64 {
    match body.get(key) {
        Some(serde_json::Value::Number(n)) => n.as_u64().unwrap_or(0),
        Some(serde_json::Value::String(s)) => s.trim().parse().unwrap_or(0),
        _ => 0,
    }
}

fn body_str<'a>(body: &'a serde_json::Value, key: &str) -> &'a str {
    body.get(key).and_then(|v| v.as_str()).unwrap_or("")
}

#[utoipa::path(post, path = "/v1/admin/site-settings/php-seo", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn php_seo(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    Ok(ApiResponse::data(
        site_setting_service::seo_index(&state, &user, body_str(&body, "action")).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/site-settings/php-seo-add", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn php_seo_add(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    Ok(ApiResponse::data(
        site_setting_service::seo_add_form(&state, &user, body_u64(&body, "id")).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/site-settings/php-seo-save", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn php_seo_save(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    let id = site_setting_service::seo_save(&state, &user, &body).await?;
    let key = if body_u64(&body, "id") > 0 {
        "admin_model_00100"
    } else {
        "admin_model_00101"
    };
    Ok(ApiResponse::message_data(key, serde_json::json!({ "id": id })))
}

#[utoipa::path(post, path = "/v1/admin/site-settings/php-seo-del", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn php_seo_del(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse> {
    site_setting_service::seo_del(&state, &user, body_u64(&body, "id")).await?;
    Ok(ApiResponse::message("admin_model_00104"))
}

#[utoipa::path(post, path = "/v1/admin/site-settings/php-regset", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn php_regset(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<serde_json::Value>> {
    Ok(ApiResponse::data(
        site_setting_service::regset_index(&state, &user).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/site-settings/php-regset-save", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn php_regset_save(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse> {
    site_setting_service::regset_save(&state, &user, &body).await?;
    Ok(ApiResponse::message("admin_model_00072"))
}

#[utoipa::path(post, path = "/v1/admin/site-settings/php-messageset", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn php_messageset(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<serde_json::Value>> {
    Ok(ApiResponse::data(
        site_setting_service::messageset_index(&state, &user).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/site-settings/php-hbconfig", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn php_hbconfig(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<serde_json::Value>> {
    Ok(ApiResponse::data(
        site_setting_service::hbconfig_index(&state, &user).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/site-settings/php-hb-saveset", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn php_hb_saveset(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse> {
    site_setting_service::hbconfig_save_set(&state, &user, &body).await?;
    Ok(ApiResponse::message("admin_01450"))
}

#[utoipa::path(post, path = "/v1/admin/site-settings/php-hb-list", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn php_hb_list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    Ok(ApiResponse::data(
        site_setting_service::hbconfig_list(&state, &user, body_u64(&body, "type") as i32).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/site-settings/php-hb-save-open", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn php_hb_save_open(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse> {
    site_setting_service::hbconfig_save_open(&state, &user, &body).await?;
    Ok(ApiResponse::message("admin_01451"))
}
