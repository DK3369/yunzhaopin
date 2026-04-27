//! My referral rewards.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_services::referral_service;
use serde::Serialize;
use utoipa::ToSchema;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/referrals", post(list))
        .route("/referrals/summary", post(summary))
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReferralItem {
    pub id: u64,
    pub inviter_uid: u64,
    pub invitee_uid: u64,
    pub points: i32,
    pub status: i32,
    pub created_at: i64,
    pub created_at_n: String,
}

impl From<phpyun_models::referral::entity::Referral> for ReferralItem {
    fn from(r: phpyun_models::referral::entity::Referral) -> Self {
        Self {
            id: r.id,
            inviter_uid: r.inviter_uid,
            invitee_uid: r.invitee_uid,
            points: r.points,
            status: r.status,
            created_at_n: fmt_dt(r.created_at),
            created_at: r.created_at,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SummaryView {
    pub count: u64,
    pub total_points: i64,
}

/// My referral list
#[utoipa::path(
    post,
    path = "/v1/mcenter/referrals",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<ReferralItem>>> {
    let r = referral_service::list_mine(&state, &user, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(ReferralItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Summary: number of invitees + accumulated points
#[utoipa::path(
    post,
    path = "/v1/mcenter/referrals/summary",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = SummaryView))
)]
pub async fn summary(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<SummaryView>> {
    let s = referral_service::summary(&state, &user).await?;
    Ok(ApiJson(SummaryView {
        count: s.count,
        total_points: s.total_points,
    }))
}
