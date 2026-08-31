//! User management (admin only).

use axum::{extract::State, routing::post, Json, Router};
use phpyun_core::utils::fmt_dt;
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, ClientIp, Pagination, ValidatedJson,
};

use crate::dto::AdminPaged;
use phpyun_services::admin_service::{self, UserFilter};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/users", post(list))
        .route("/users/status", post(set_status))
        .route("/users/impersonate", post(impersonate))
        .route("/users/php-add", post(php_add))
        .route("/users/php-edit", post(php_edit))
        .route("/users/php-editsave", post(php_editsave))
        .route("/users/php-save-user", post(php_save_user))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct UserListQuery {
    #[validate(length(max = 100))]
    pub keyword: Option<String>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    #[validate(range(min = 0, max = 9))]
    pub usertype: Option<i32>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    #[validate(range(min = 0, max = 9))]
    pub status: Option<i32>,
}

fn usertype_name(t: i32) -> &'static str {
    match t {
        1 => "jobseeker",
        2 => "company",
        3 => "admin",
        _ => "unknown",
    }
}

fn user_status_name(s: i32) -> &'static str {
    match s {
        0 => "pending",
        1 => "active",
        2 => "locked",
        3 => "deleted",
        _ => "unknown",
    }
}

/// User management item — phpyun_member exposed columns (password / salt are not output) + derived usertype/status text + formatted timestamps.
#[derive(Debug, Serialize, ToSchema)]
pub struct AdminUserItem {
    pub uid: u64,
    pub username: String,
    pub email: Option<String>,
    pub moblie: Option<String>,
    pub usertype: i32,
    pub usertype_n: String,
    pub status: i32,
    pub status_n: String,
    pub did: u64,
    pub reg_date: i64,
    pub reg_date_n: String,
    pub login_date: Option<i64>,
    pub login_date_n: String,
}

impl From<phpyun_models::user::entity::Member> for AdminUserItem {
    fn from(m: phpyun_models::user::entity::Member) -> Self {
        Self {
            uid: m.uid,
            username: m.username,
            email: m.email,
            moblie: m.moblie,
            usertype_n: usertype_name(m.usertype).to_string(),
            usertype: m.usertype,
            status_n: user_status_name(m.status).to_string(),
            status: m.status,
            did: m.did,
            reg_date_n: fmt_dt(m.reg_date),
            reg_date: m.reg_date,
            login_date_n: fmt_dt(m.login_date.unwrap_or(0)),
            login_date: m.login_date,
        }
    }
}

/// User list (admin)
#[utoipa::path(
    post,
    path = "/v1/admin/users",
    tag = "admin",
    security(("bearer" = [])),
    params(UserListQuery),
    responses((status = 200, description = "ok"), (status = 403, description = "forbidden"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<UserListQuery>,
) -> AppResult<ApiResponse<AdminPaged<AdminUserItem>>> {
    user.require_admin()?;
    let filter = UserFilter {
        keyword: q.keyword.as_deref(),
        usertype: q.usertype,
        status: q.status,
    };
    let r = admin_service::list_users(&state, &filter, page).await?;
    Ok(ApiResponse::data(AdminPaged::from(phpyun_core::Paged::from_listing(
        r.list, r.total, page,
    ))))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SetStatusForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub uid: u64,
    /// 0=frozen / 1=active
    #[validate(range(min = 0, max = 1))]
    pub status: i32,
}

/// Freeze / unfreeze a user
#[utoipa::path(post,
    path = "/v1/admin/users/status",
    tag = "admin",
    security(("bearer" = [])),
    request_body = SetStatusForm,
    responses((status = 200, description = "ok"))
)]
pub async fn set_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SetStatusForm>,
) -> AppResult<ApiResponse> {
    let uid = f.uid;
    user.require_admin()?;
    admin_service::set_user_status(&state, &user, uid, f.status).await?;
    Ok(ApiResponse::message("ok"))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ImpersonateForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub uid: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ImpersonateData {
    pub uid: u64,
    pub usertype: u8,
    pub access_token: String,
}

/// Simulate login as a member (PHP 模拟登录). Returns a member access token; does not switch the admin session.
#[utoipa::path(
    post,
    path = "/v1/admin/users/impersonate",
    tag = "admin",
    security(("bearer" = [])),
    request_body = ImpersonateForm,
    responses((status = 200, description = "ok", body = ImpersonateData), (status = 403, description = "forbidden"))
)]
pub async fn impersonate(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    headers: axum::http::HeaderMap,
    ValidatedJson(f): ValidatedJson<ImpersonateForm>,
) -> AppResult<ApiResponse<ImpersonateData>> {
    user.require_admin()?;
    let ua = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    let r = phpyun_services::user_service::impersonate(
        &state,
        user.uid,
        f.uid,
        phpyun_services::user_service::LoginContext { ip: &ip, ua: &ua },
    )
    .await?;
    Ok(ApiResponse::data(ImpersonateData {
        uid: r.uid,
        usertype: r.usertype,
        access_token: r.access,
    }))
}

/// PHP `users_member::add_action`。
#[utoipa::path(post, path = "/v1/admin/users/php-add", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn php_add(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    let username = body.get("username").and_then(|v| v.as_str()).unwrap_or("");
    let add_on = match body.get("add") {
        None | Some(serde_json::Value::Null) => false,
        Some(serde_json::Value::Bool(b)) => *b,
        Some(serde_json::Value::Number(n)) => n.as_i64().unwrap_or(0) != 0,
        Some(serde_json::Value::String(s)) => {
            let t = s.trim();
            !t.is_empty() && t != "0"
        }
        _ => true,
    };
    let is_form = add_on || username.trim().is_empty();
    let data = phpyun_services::admin_longtail_service::member_php_add(&state, &user, &body).await?;
    if is_form {
        return Ok(ApiResponse::data(data));
    }
    Ok(ApiResponse::message_data("admin_model_00106", data))
}

/// PHP `users_member::edit_action`.
#[utoipa::path(post, path = "/v1/admin/users/php-edit", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn php_edit(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    let uid = body
        .get("uid")
        .and_then(|v| v.as_u64().or_else(|| v.as_str().and_then(|s| s.parse().ok())))
        .unwrap_or(0);
    Ok(ApiResponse::data(
        phpyun_services::admin_longtail_service::member_php_edit(&state, &user, uid).await?,
    ))
}

/// PHP `users_member::editSave_action`.
#[utoipa::path(post, path = "/v1/admin/users/php-editsave", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn php_editsave(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse> {
    phpyun_services::admin_longtail_service::member_edit_save(&state, &user, &body).await?;
    Ok(ApiResponse::message("admin_user_00083"))
}

/// PHP `users_member::saveUser_action`.
#[utoipa::path(post, path = "/v1/admin/users/php-save-user", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn php_save_user(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse> {
    phpyun_services::admin_longtail_service::company_save_user(&state, &user, &body).await?;
    Ok(ApiResponse::message("admin_user_00083"))
}
