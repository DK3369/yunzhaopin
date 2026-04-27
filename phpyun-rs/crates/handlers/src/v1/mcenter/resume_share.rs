//! Resume share link management (jobseeker side).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson};
use phpyun_services::resume_share_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use phpyun_core::dto::{TokenBody};
use phpyun_core::utils::{fmt_dt};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/resume-share-tokens", post(create))
        .route("/resume-share-tokens/list", post(list_mine))
        .route("/resume-share-tokens/revoke", post(revoke))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateForm {
    /// Validity in seconds (1..=30 days = 2,592,000)
    #[validate(range(min = 60, max = 2_592_000))]
    pub ttl_secs: i64,
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
    post,
    path = "/v1/mcenter/resume-share-tokens/list",
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
    Ok(ApiJson(Paged::from_listing(r.list, r.total, page)))
}

/// Revoke share
#[utoipa::path(
    post,
    path = "/v1/mcenter/resume-share-tokens/revoke",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = TokenBody,
    responses((status = 200, description = "ok"))
)]
pub async fn revoke(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<TokenBody>,
) -> AppResult<ApiOk> {
    phpyun_core::validators::ensure_path_hex_token(&b.token)?;
    resume_share_service::revoke(&state, &user, &b.token).await?;
    Ok(ApiOk("revoked"))
}
