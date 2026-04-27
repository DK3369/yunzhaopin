//! System broadcasts (admin).

use axum::{
    extract::{Path, State},
    Router,
    routing::{get, post},
};
use phpyun_core::{ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson};
use phpyun_services::broadcast_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use phpyun_core::dto::{CreatedId, IdBody};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/broadcasts", post(create))
        .route("/broadcasts/list", post(list))
        .route("/broadcasts/delete", post(remove))
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

fn target_usertype_name(t: i32) -> &'static str {
    match t { 0 => "all", 1 => "jobseeker", 2 => "company", _ => "unknown" }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BroadcastItem {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub target_usertype: i32,
    pub target_usertype_n: String,
    pub status: i32,
    pub issuer_uid: u64,
    pub created_at: i64,
    pub created_at_n: String,
}

impl From<phpyun_models::broadcast::entity::Broadcast> for BroadcastItem {
    fn from(b: phpyun_models::broadcast::entity::Broadcast) -> Self {
        Self {
            id: b.id,
            title: b.title,
            body: b.body,
            target_usertype_n: target_usertype_name(b.target_usertype).to_string(),
            target_usertype: b.target_usertype,
            status: b.status,
            issuer_uid: b.issuer_uid,
            created_at_n: fmt_dt(b.created_at),
            created_at: b.created_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateForm {
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    #[validate(length(min = 1, max = 50_000))]
    pub body: String,
    /// 0=all 1=jobseeker 2=employer 3=admin
    #[validate(range(min = 0, max = 3))]
    #[serde(default)]
    pub target_usertype: i32,
}

#[utoipa::path(post, path = "/v1/admin/broadcasts/list", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<BroadcastItem>>> {
    user.require_admin()?;
    let r = broadcast_service::admin_list(&state, &user, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(BroadcastItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

#[utoipa::path(post, path = "/v1/admin/broadcasts", tag = "admin", security(("bearer" = [])), request_body = CreateForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<CreateForm>,
) -> AppResult<ApiJson<CreatedId>> {
    user.require_admin()?;
    let id =
        broadcast_service::admin_create(&state, &user, &f.title, &f.body, f.target_usertype)
            .await?;
    Ok(ApiJson(CreatedId { id }))
}

#[utoipa::path(post, path = "/v1/admin/broadcasts/delete", tag = "admin", security(("bearer" = [])), request_body = IdBody, responses((status = 200, description = "ok")))]
pub async fn remove(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiOk> {
    user.require_admin()?;
    broadcast_service::admin_delete(&state, &user, b.id).await?;
    Ok(ApiOk("deleted"))
}
