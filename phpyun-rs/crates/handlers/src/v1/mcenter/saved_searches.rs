//! Saved-search subscription management.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::json;
use phpyun_core::{ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson};
use phpyun_services::saved_search_service::{self, CreateInput};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use phpyun_core::dto::{CreatedId, IdBody};
use phpyun_core::utils::{fmt_dt};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/saved-searches", post(create))
        .route("/saved-searches/list", post(list))
        .route("/saved-searches/notify", post(set_notify))
        .route("/saved-searches/delete", post(remove))
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
    /// Arbitrary search-filter JSON. Capped at ~16 KB serialized to prevent
    /// memory bombs / SQL-payload smuggling via huge nested objects.
    #[validate(custom(function = "validate_params_size"))]
    pub params: json::Value,
    #[serde(default = "default_notify")]
    pub notify: bool,
}

fn validate_params_size(v: &json::Value) -> Result<(), validator::ValidationError> {
    let s = match phpyun_core::json::to_string(v) {
        Ok(s) => s,
        Err(_) => return Err(validator::ValidationError::new("params_unserializable")),
    };
    if s.len() > 16 * 1024 {
        return Err(validator::ValidationError::new("params_too_large"));
    }
    Ok(())
}
fn default_notify() -> bool { true }

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct NotifyForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,

    pub notify: bool,
}

/// My saved searches
#[utoipa::path(
    post,
    path = "/v1/mcenter/saved-searches/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<SavedItem>>> {
    let r = saved_search_service::list(&state, &user, page).await?;
    Ok(ApiJson(Paged::from_listing(r.list, r.total, page)))
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
#[utoipa::path(post,
    path = "/v1/mcenter/saved-searches/notify",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = NotifyForm,
    responses((status = 200, description = "ok"))
)]
pub async fn set_notify(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<NotifyForm>) -> AppResult<ApiOk> {
    let id = f.id;
    saved_search_service::set_notify(&state, &user, id, f.notify).await?;
    Ok(ApiOk("ok"))
}

/// Delete a saved search
#[utoipa::path(post,
    path = "/v1/mcenter/saved-searches/delete",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn remove(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiOk> {
    let id = b.id;
    saved_search_service::delete(&state, &user, id).await?;
    Ok(ApiOk("deleted"))
}

