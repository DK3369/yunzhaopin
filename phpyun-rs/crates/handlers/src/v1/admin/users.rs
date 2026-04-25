//! User management (admin only).

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{
    ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_services::admin_service::{self, UserFilter};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(list))
        .route("/users/{uid}/status", post(set_status))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct UserListQuery {
    pub keyword: Option<String>,
    pub usertype: Option<i32>,
    pub status: Option<i32>,
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

fn usertype_name(t: i32) -> &'static str {
    match t { 1 => "jobseeker", 2 => "company", 3 => "admin", _ => "unknown" }
}

fn user_status_name(s: i32) -> &'static str {
    match s { 0 => "pending", 1 => "active", 2 => "locked", 3 => "deleted", _ => "unknown" }
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
    get,
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
    Query(q): Query<UserListQuery>,
) -> AppResult<ApiJson<Paged<AdminUserItem>>> {
    user.require_admin()?;
    let filter = UserFilter {
        keyword: q.keyword.as_deref(),
        usertype: q.usertype,
        status: q.status,
    };
    let r = admin_service::list_users(&state, &filter, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(AdminUserItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SetStatusForm {
    /// 0=frozen / 1=active
    #[validate(range(min = 0, max = 1))]
    pub status: i32,
}

/// Freeze / unfreeze a user
#[utoipa::path(
    post,
    path = "/v1/admin/users/{uid}/status",
    tag = "admin",
    security(("bearer" = [])),
    params(("uid" = u64, Path)),
    request_body = SetStatusForm,
    responses((status = 200, description = "ok"))
)]
pub async fn set_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(uid): Path<u64>,
    ValidatedJson(f): ValidatedJson<SetStatusForm>,
) -> AppResult<ApiOk> {
    user.require_admin()?;
    admin_service::set_user_status(&state, &user, uid, f.status).await?;
    Ok(ApiOk("ok"))
}
