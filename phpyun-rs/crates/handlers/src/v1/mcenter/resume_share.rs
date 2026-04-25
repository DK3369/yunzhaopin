//! Resume share link management (jobseeker side).

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{
    ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_services::resume_share_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/resume-share-tokens",
            get(list_mine).post(create),
        )
        .route("/resume-share-tokens/{token}", post(revoke))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateForm {
    /// Validity in seconds (1..=30 days = 2,592,000)
    #[validate(range(min = 60, max = 2_592_000))]
    pub ttl_secs: i64,
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ShareTokenView {
    pub token: String,
    pub uid: u64,
    pub view_count: u32,
    pub expires_at: i64,
    pub expires_at_n: String,
    pub revoked_at: i64,
    pub revoked_at_n: String,
    pub created_at: i64,
    pub created_at_n: String,
    /// Derived: not revoked and not expired
    pub active: bool,
}

impl From<phpyun_models::resume_share::entity::ShareToken> for ShareTokenView {
    fn from(t: phpyun_models::resume_share::entity::ShareToken) -> Self {
        let now = phpyun_core::clock::now_ts();
        let active = t.revoked_at == 0 && (t.expires_at == 0 || t.expires_at > now);
        Self {
            token: t.token,
            uid: t.uid,
            view_count: t.view_count,
            expires_at_n: fmt_dt(t.expires_at),
            expires_at: t.expires_at,
            revoked_at_n: fmt_dt(t.revoked_at),
            revoked_at: t.revoked_at,
            created_at_n: fmt_dt(t.created_at),
            created_at: t.created_at,
            active,
        }
    }
}

/// Create a share link
#[utoipa::path(
    post,
    path = "/v1/mcenter/resume-share-tokens",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = CreateForm,
    responses((status = 200, description = "ok", body = ShareTokenView))
)]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<CreateForm>,
) -> AppResult<ApiJson<ShareTokenView>> {
    let t = resume_share_service::create(&state, &user, f.ttl_secs).await?;
    Ok(ApiJson(ShareTokenView::from(t)))
}

/// My share list
#[utoipa::path(
    get,
    path = "/v1/mcenter/resume-share-tokens",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<ShareTokenView>>> {
    let r = resume_share_service::list_mine(&state, &user, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(ShareTokenView::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Revoke share
#[utoipa::path(
    post,
    path = "/v1/mcenter/resume-share-tokens/{token}",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("token" = String, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn revoke(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(token): Path<String>,
) -> AppResult<ApiOk> {
    resume_share_service::revoke(&state, &user, &token).await?;
    Ok(ApiOk("revoked"))
}
