//! Public rating reads (aligned with PHPYun `rating.model.php` detail page comment block).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, AppResult, AppState, Paged, Pagination, ValidatedJson};
use phpyun_services::rating_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RatingTargetBody {
    /// 1=company / 2=resume / 3=job
    #[validate(range(min = 1, max = 99))]
    pub kind: i32,
    /// Target uid (the entity being rated).
    #[validate(range(min = 1, max = 999_999_999))]
    pub uid: u64,
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/ratings/list", post(list))
        .route("/ratings/summary", post(summary))
}

/// Rating item — all 9 columns of phpyun_rating + formatted timestamps.
#[derive(Debug, Serialize, ToSchema)]
pub struct RatingItem {
    pub id: u64,
    pub rater_uid: u64,
    pub target_uid: u64,
    /// 1=company / 2=resume / 3=job
    pub target_kind: i32,
    pub stars: i32,
    pub comment: String,
    /// 1=normal / 0=pending review / 2=deleted
    pub status: i32,
    pub created_at: i64,
    pub created_at_n: String,
    pub updated_at: i64,
    pub updated_at_n: String,
}

impl From<phpyun_models::rating::entity::Rating> for RatingItem {
    fn from(r: phpyun_models::rating::entity::Rating) -> Self {
        Self {
            id: r.id,
            rater_uid: r.rater_uid,
            target_uid: r.target_uid,
            target_kind: r.target_kind,
            stars: r.stars,
            comment: r.comment,
            status: r.status,
            created_at_n: fmt_dt(r.created_at),
            created_at: r.created_at,
            updated_at_n: fmt_dt(r.updated_at),
            updated_at: r.updated_at,
        }
    }
}

/// Rating aggregate — all 6 columns of phpyun_rating_aggregate + derived `avg` (avg_x100 / 100 as float).
#[derive(Debug, Serialize, ToSchema)]
pub struct RatingSummary {
    pub target_uid: u64,
    pub target_kind: i32,
    pub count: u32,
    pub sum_stars: u32,
    pub avg_x100: u32,
    /// Float average (avg_x100 / 100, convenient for directly rendering 4.5★)
    pub avg: f32,
    pub updated_at: i64,
    pub updated_at_n: String,
}

impl From<phpyun_models::rating::entity::RatingAggregate> for RatingSummary {
    fn from(a: phpyun_models::rating::entity::RatingAggregate) -> Self {
        Self {
            target_uid: a.target_uid,
            target_kind: a.target_kind,
            count: a.count,
            sum_stars: a.sum_stars,
            avg: (a.avg_x100 as f32) / 100.0,
            avg_x100: a.avg_x100,
            updated_at_n: fmt_dt(a.updated_at),
            updated_at: a.updated_at,
        }
    }
}

/// Rating list (newest first)
#[utoipa::path(
    post,
    path = "/v1/wap/ratings/list",
    tag = "wap",
    request_body = RatingTargetBody,
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    page: Pagination,
    ValidatedJson(b): ValidatedJson<RatingTargetBody>,
) -> AppResult<ApiJson<Paged<RatingItem>>> {
    let r = rating_service::list(&state, b.uid, b.kind, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(RatingItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Rating summary (count + avg)
#[utoipa::path(
    post,
    path = "/v1/wap/ratings/summary",
    tag = "wap",
    request_body = RatingTargetBody,
    responses((status = 200, description = "ok", body = RatingSummary))
)]
pub async fn summary(
    State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<RatingTargetBody>,
) -> AppResult<ApiJson<RatingSummary>> {
    let a = rating_service::aggregate(&state, b.uid, b.kind).await?;
    Ok(ApiJson(RatingSummary::from(a)))
}
