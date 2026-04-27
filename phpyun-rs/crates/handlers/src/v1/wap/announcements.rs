//! Site announcements (aligned with PHPYun `wap/announcement`).

use axum::{
    extract::{Path, State},
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, AppError, AppResult, AppState, Paged, Pagination, ValidatedJson};
use phpyun_core::error::InfraError;
use phpyun_services::announcement_service;
use serde::Serialize;
use utoipa::ToSchema;
use phpyun_core::dto::{IdBody};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/announcements", post(list))
        .route("/announcements/detail", post(detail))
}

fn fmt_date(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_default()
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

/// Announcement list item — aligned with all fields of phpyun_announcement.
#[derive(Debug, Serialize, ToSchema)]
pub struct AnnouncementSummary {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub view_num: u32,
    pub datetime: i64,
    /// PHP `date('Y-m-d', $datetime)`
    pub datetime_n: String,
    pub startime: i64,
    pub startime_n: String,
    pub endtime: i64,
    pub endtime_n: String,
    pub did: u64,
    pub status: i32,
    pub created_at: i64,
}

impl From<phpyun_models::announcement::entity::Announcement> for AnnouncementSummary {
    fn from(a: phpyun_models::announcement::entity::Announcement) -> Self {
        Self {
            datetime_n: fmt_date(a.datetime),
            startime_n: fmt_date(a.startime),
            endtime_n: fmt_date(a.endtime),
            id: a.id,
            title: a.title,
            description: a.description,
            view_num: a.view_num,
            datetime: a.datetime,
            startime: a.startime,
            endtime: a.endtime,
            did: a.did,
            status: a.status,
            created_at: a.created_at,
        }
    }
}

/// Announcement detail — Summary + content + formatted-time extensions.
#[derive(Debug, Serialize, ToSchema)]
pub struct AnnouncementDetail {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub content: String,
    pub view_num: u32,
    pub datetime: i64,
    pub datetime_n: String,
    /// Full PHP `date('Y-m-d H:i', $datetime)`
    pub datetime_full: String,
    pub startime: i64,
    pub startime_n: String,
    pub endtime: i64,
    pub endtime_n: String,
    pub did: u64,
    pub status: i32,
    pub created_at: i64,
}

impl From<phpyun_models::announcement::entity::Announcement> for AnnouncementDetail {
    fn from(a: phpyun_models::announcement::entity::Announcement) -> Self {
        Self {
            datetime_n: fmt_date(a.datetime),
            datetime_full: fmt_dt(a.datetime),
            startime_n: fmt_date(a.startime),
            endtime_n: fmt_date(a.endtime),
            id: a.id,
            title: a.title,
            description: a.description,
            content: a.content,
            view_num: a.view_num,
            datetime: a.datetime,
            startime: a.startime,
            endtime: a.endtime,
            did: a.did,
            status: a.status,
            created_at: a.created_at,
        }
    }
}

/// Announcement list
#[utoipa::path(
    post,
    path = "/v1/wap/announcements/detail",
    tag = "wap",
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    page: Pagination,
) -> AppResult<ApiJson<Paged<AnnouncementSummary>>> {
    let r = announcement_service::list(&state, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(AnnouncementSummary::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Announcement detail (`upViewNum` semantics: async +1)
#[utoipa::path(post,
    path = "/v1/wap/announcements",
    tag = "wap",
    request_body = IdBody,
    responses((status = 200, description = "ok", body = AnnouncementDetail), (status = 404))
)]
pub async fn detail(State(state): State<AppState>,
    ValidatedJson(b): ValidatedJson<IdBody>) -> AppResult<ApiJson<AnnouncementDetail>> {
    let id = b.id;
    let row = announcement_service::get_detail(&state, id)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("announcement_not_found".into())))?;
    Ok(ApiJson(AnnouncementDetail::from(row)))
}

