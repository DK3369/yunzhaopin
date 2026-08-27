//! PHP `adminCommon::login` / `index` menu / `me`. `/login` is mounted
//! **outside** the admin JWT guard.

use axum::{
    extract::State,
    http::HeaderMap,
    routing::post,
    Router,
};
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson,
};
use phpyun_services::admin_auth_service::{self, AdminMe, AdminMenuItem};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn public_routes() -> Router<AppState> {
    Router::new().route("/login", post(admin_login))
}

pub fn guarded_routes() -> Router<AppState> {
    Router::new()
        .route("/me", post(admin_me))
        .route("/menu", post(admin_menu))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AdminLoginForm {
    #[validate(length(min = 1, max = 64))]
    pub username: String,
    #[validate(length(min = 1, max = 128))]
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminLoginData {
    pub uid: u64,
    pub usertype: u8,
    pub username: String,
    pub name: String,
    pub group_name: String,
    pub path: String,
    pub access_token: String,
}

fn ua_from(headers: &HeaderMap) -> String {
    headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string()
}

/// PHP `index::login_action` against `phpyun_admin_user` (`md5(md5(pw))`).
#[utoipa::path(
    post,
    path = "/v1/admin/login",
    tag = "admin",
    request_body = AdminLoginForm,
    responses((status = 200, description = "ok", body = AdminLoginData))
)]
pub async fn admin_login(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    headers: HeaderMap,
    ValidatedJson(form): ValidatedJson<AdminLoginForm>,
) -> AppResult<ApiResponse<AdminLoginData>> {
    let ua = ua_from(&headers);
    let r = admin_auth_service::login(
        &state,
        &form.username,
        &form.password,
        phpyun_services::user_service::LoginContext { ip: &ip, ua: &ua },
    )
    .await?;
    Ok(ApiResponse::data(AdminLoginData {
        uid: r.uid,
        usertype: r.usertype,
        username: r.username,
        name: r.name,
        group_name: r.group_name,
        path: r.path,
        access_token: r.access,
    }))
}

#[utoipa::path(
    post,
    path = "/v1/admin/me",
    tag = "admin",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = AdminMeView))
)]
pub async fn admin_me(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<AdminMeView>> {
    let m = admin_auth_service::me(&state, &user).await?;
    Ok(ApiResponse::data(AdminMeView::from(m)))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminMeView {
    pub uid: u64,
    pub usertype: u8,
    pub username: String,
    pub name: String,
    pub group_name: String,
    pub m_id: i32,
}

impl From<AdminMe> for AdminMeView {
    fn from(m: AdminMe) -> Self {
        Self {
            uid: m.uid,
            usertype: m.usertype,
            username: m.username,
            name: m.name,
            group_name: m.group_name,
            m_id: m.m_id,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminMenuView {
    pub id: i64,
    pub keyid: i64,
    pub name: String,
    pub url: String,
    pub path: String,
    pub classname: String,
    pub menu: i32,
    pub sort: i32,
    pub route: String,
}

impl From<AdminMenuItem> for AdminMenuView {
    fn from(m: AdminMenuItem) -> Self {
        Self {
            id: m.id,
            keyid: m.keyid,
            name: m.name,
            url: m.url,
            path: m.path,
            classname: m.classname,
            menu: m.menu,
            sort: m.sort,
            route: m.route,
        }
    }
}

#[utoipa::path(
    post,
    path = "/v1/admin/menu",
    tag = "admin",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn admin_menu(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<Vec<AdminMenuView>>> {
    let list = admin_auth_service::menu(&state, &user).await?;
    Ok(ApiResponse::data(list.into_iter().map(AdminMenuView::from).collect()))
}
