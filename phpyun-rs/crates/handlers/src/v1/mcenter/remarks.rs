//! Remarks: companies note remarks on job seekers / resumes / applications.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{dto::KindTargetUidBody, ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson};
use phpyun_services::remark_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
use phpyun_core::utils::{fmt_dt};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/remarks", post(upsert))
        .route("/remarks/list", post(list))
        .route("/remarks/get-one", post(get_one))
        .route("/remarks/delete", post(remove))
}


fn remark_kind_name(k: i32) -> &'static str {
    match k { 1 => "resume", 2 => "company", 3 => "apply", _ => "unknown" }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RemarkView {
    pub uid: u64,
    pub target_uid: u64,
    pub target_kind: i32,
    pub target_kind_n: String,
    pub note: String,
    pub updated_at: i64,
    pub updated_at_n: String,
}

impl From<phpyun_models::remark::entity::Remark> for RemarkView {
    fn from(r: phpyun_models::remark::entity::Remark) -> Self {
        Self {
            uid: r.uid,
            target_uid: r.target_uid,
            target_kind_n: remark_kind_name(r.target_kind).to_string(),
            target_kind: r.target_kind,
            note: r.note,
            updated_at_n: fmt_dt(r.updated_at),
            updated_at: r.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ListQuery {
    /// 1=resume 2=company 3=apply; omit = all
    #[validate(range(min = 0, max = 99))]
    pub kind: Option<i32>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpsertForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub target_uid: u64,
    /// 1=resume 2=company 3=apply
    #[validate(range(min = 1, max = 3))]
    pub target_kind: i32,
    #[validate(length(min = 0, max = 5000))]
    pub note: String,
}

/// My remarks list
#[utoipa::path(
    post,
    path = "/v1/mcenter/remarks/list",
    tag = "mcenter",
    security(("bearer" = [])),
    params(ListQuery),
    responses((status = 200, description = "ok"))
)]pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<ListQuery>,
) -> AppResult<ApiJson<Paged<RemarkView>>> {
    let r = remark_service::list(&state, &user, q.kind, page).await?;
    Ok(ApiJson(Paged::from_listing(r.list, r.total, page)))
}

/// Create / update a remark
#[utoipa::path(
    post,
    path = "/v1/mcenter/remarks",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = UpsertForm,
    responses((status = 200, description = "ok"))
)]
pub async fn upsert(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<UpsertForm>,
) -> AppResult<ApiOk> {
    remark_service::upsert(&state, &user, f.target_uid, f.target_kind, &f.note).await?;
    Ok(ApiOk("ok"))
}

/// Get a specific remark
#[utoipa::path(
    post,
    path = "/v1/mcenter/remarks/get-one",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = KindTargetUidBody,
    responses((status = 200, description = "ok"))
)]
pub async fn get_one(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<KindTargetUidBody>,
) -> AppResult<ApiJson<Option<RemarkView>>> {
    let r = remark_service::get(&state, &user, b.target_uid, b.kind).await?;
    Ok(ApiJson(r.map(RemarkView::from)))
}

/// Delete a remark
#[utoipa::path(
    post,
    path = "/v1/mcenter/remarks/delete",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = KindTargetUidBody,
    responses((status = 200, description = "ok"))
)]
pub async fn remove(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<KindTargetUidBody>,
) -> AppResult<ApiOk> {
    remark_service::delete(&state, &user, b.target_uid, b.kind).await?;
    Ok(ApiOk("deleted"))
}
