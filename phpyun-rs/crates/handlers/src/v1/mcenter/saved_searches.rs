//! Saved-search subscription management.

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Router,
};
use phpyun_core::json;
use phpyun_core::{
    ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_services::saved_search_service::{self, CreateInput};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/saved-searches", get(list).post(create))
        .route("/saved-searches/{id}/notify", post(set_notify))
        .route("/saved-searches/{id}", post(remove))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SavedItem {
    pub id: u64,
    pub uid: u64,
    pub name: String,
    pub kind: String,
    pub params: json::Value,
    pub notify: i32,
    pub last_notified_at: i64,
    pub last_notified_at_n: String,
    pub created_at: i64,
    pub created_at_n: String,
    pub updated_at: i64,
    pub updated_at_n: String,
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

impl From<phpyun_models::saved_search::entity::SavedSearch> for SavedItem {
    fn from(s: phpyun_models::saved_search::entity::SavedSearch) -> Self {
        Self {
            id: s.id,
            uid: s.uid,
            name: s.name,
            kind: s.kind,
            params: s.params,
            notify: s.notify,
            last_notified_at_n: fmt_dt(s.last_notified_at),
            last_notified_at: s.last_notified_at,
            created_at_n: fmt_dt(s.created_at),
            created_at: s.created_at,
            updated_at_n: fmt_dt(s.updated_at),
            updated_at: s.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateForm {
    #[validate(length(min = 1, max = 120))]
    pub name: String,
    #[validate(length(min = 1, max = 16))]
    pub kind: String,
    pub params: json::Value,
    #[serde(default = "default_notify")]
    pub notify: bool,
}
fn default_notify() -> bool { true }

#[derive(Debug, Serialize, ToSchema)]
pub struct CreatedId {
    pub id: u64,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct NotifyForm {
    pub notify: bool,
}

/// My saved searches
#[utoipa::path(
    get,
    path = "/v1/mcenter/saved-searches",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<SavedItem>>> {
    let r = saved_search_service::list(&state, &user, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(SavedItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Create a saved search
#[utoipa::path(
    post,
    path = "/v1/mcenter/saved-searches",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = CreateForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<CreateForm>,
) -> AppResult<ApiJson<CreatedId>> {
    let id = saved_search_service::create(
        &state,
        &user,
        CreateInput {
            name: &f.name,
            kind: &f.kind,
            params: &f.params,
            notify: f.notify,
        },
    )
    .await?;
    Ok(ApiJson(CreatedId { id }))
}

/// Toggle notification switch
#[utoipa::path(
    post,
    path = "/v1/mcenter/saved-searches/{id}/notify",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    request_body = NotifyForm,
    responses((status = 200, description = "ok"))
)]
pub async fn set_notify(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
    ValidatedJson(f): ValidatedJson<NotifyForm>,
) -> AppResult<ApiOk> {
    saved_search_service::set_notify(&state, &user, id, f.notify).await?;
    Ok(ApiOk("ok"))
}

/// Delete a saved search
#[utoipa::path(
    post,
    path = "/v1/mcenter/saved-searches/{id}",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn remove(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(id): Path<u64>,
) -> AppResult<ApiOk> {
    saved_search_service::delete(&state, &user, id).await?;
    Ok(ApiOk("deleted"))
}
