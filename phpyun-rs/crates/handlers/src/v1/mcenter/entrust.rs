//! Jobseeker → headhunter bindings (`phpyun_entrust`).
//!
//! Aligned with PHPYun's actual 5-column schema:
//!   id, uid (seeker), lt_uid (headhunter), datetime, remind_status
//!
//! - GET   /v1/mcenter/entrust              — list my bindings (paginated)
//! - POST  /v1/mcenter/entrust              — bind a headhunter `{lt_uid}` (idempotent)
//! - POST  /v1/mcenter/entrust/delete       — unbind by `{lt_uid}` or `{id}`

use axum::{
    extract::{State},
    Router,
    routing::{get, post},
};
use phpyun_core::{ApiJson, ApiMsg, AppError, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson};
use phpyun_services::entrust_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/entrust", post(bind))
        .route("/entrust/list", post(list))
        .route("/entrust/delete", post(unbind))
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EntrustItem {
    pub id: u64,
    pub uid: u64,
    /// Headhunter uid (member with `usertype=4`).
    pub lt_uid: u64,
    /// Unix seconds when the binding was created.
    pub datetime: i64,
    pub datetime_n: String,
    pub remind_status: i32,
}

impl From<phpyun_models::entrust::entity::Entrust> for EntrustItem {
    fn from(e: phpyun_models::entrust::entity::Entrust) -> Self {
        Self {
            id: e.id,
            uid: e.uid,
            lt_uid: e.lt_uid,
            datetime_n: fmt_dt(e.datetime),
            datetime: e.datetime,
            remind_status: e.remind_status,
        }
    }
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ListQuery {}

/// List my headhunter bindings (paginated, newest first)
#[utoipa::path(
    post,
    path = "/v1/mcenter/entrust/list",
    tag = "mcenter",
    security(("bearer" = [])),
    params(ListQuery),
    responses((status = 200, description = "ok"))
)]pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(_q): ValidatedJson<ListQuery>,
) -> AppResult<ApiJson<Paged<EntrustItem>>> {
    let r = entrust_service::list_mine(&state, &user, page).await?;
    let items: Vec<EntrustItem> = r.list.into_iter().map(EntrustItem::from).collect();
    Ok(ApiJson(Paged::new(items, r.total, page.page, page.page_size)))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct BindForm {
    /// Headhunter uid to entrust (target member must have usertype=4).
    #[validate(range(min = 1))]
    pub lt_uid: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BindResp {
    pub id: u64,
    pub lt_uid: u64,
}

/// Bind a headhunter (idempotent — re-binding returns the existing row's id)
#[utoipa::path(
    post,
    path = "/v1/mcenter/entrust",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = BindForm,
    responses((status = 200, description = "ok", body = BindResp))
)]
pub async fn bind(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<BindForm>,
) -> AppResult<ApiJson<BindResp>> {
    let id = entrust_service::bind(&state, &user, f.lt_uid).await?;
    Ok(ApiJson(BindResp { id, lt_uid: f.lt_uid }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UnbindForm {
    /// Either `lt_uid` (preferred) OR `id` (the entrust row id) — at least one required.
    #[serde(default)]
    #[validate(range(min = 1, max = 99_999_999))]
    pub lt_uid: u64,
    #[serde(default)]
    #[validate(range(min = 0, max = 99_999_999))]
    pub id: u64,
}

/// Unbind a headhunter by `lt_uid` or by row `id`
#[utoipa::path(
    post,
    path = "/v1/mcenter/entrust/delete",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = UnbindForm,
    responses((status = 200, description = "ok"))
)]
pub async fn unbind(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<UnbindForm>,
) -> AppResult<ApiMsg> {
    if f.lt_uid > 0 {
        entrust_service::unbind(&state, &user, f.lt_uid).await?;
    } else if f.id > 0 {
        entrust_service::unbind_by_id(&state, &user, f.id).await?;
    } else {
        return Err(AppError::param_invalid("lt_uid_or_id_required"));
    }
    Ok(ApiMsg("entrust_unbound"))
}
