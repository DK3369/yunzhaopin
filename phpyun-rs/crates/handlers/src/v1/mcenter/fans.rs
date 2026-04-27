//! "对我感兴趣" — list jobseekers who have favorited my company's jobs.
//!
//! Counterpart of PHP `wap/member/com.class.php::attention_me_action`. Only
//! the company side returns rows — jobseekers receive an empty list (the
//! reciprocal "who has viewed my resume" lives on `/v1/mcenter/profile-views`).

use axum::{extract::State, routing::get, Router};
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_services::fan_service;
use serde::Serialize;
use utoipa::ToSchema;

pub fn routes() -> Router<AppState> {
    Router::new().route("/fans", get(list_mine))
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 {
        return String::new();
    }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FanItem {
    /// Jobseeker uid
    pub uid: u64,
    /// Username (PHP `phpyun_member.username`); blank if the row was deleted.
    pub username: String,
    /// How many of my jobs this user has favorited.
    pub fav_count: u64,
    /// Most-recent favorite timestamp (unix seconds).
    pub last_datetime: i64,
    /// Most-recent favorite formatted (`YYYY-MM-DD HH:MM`).
    pub last_datetime_n: String,
}

impl From<fan_service::FanRow> for FanItem {
    fn from(r: fan_service::FanRow) -> Self {
        Self {
            uid: r.uid,
            username: r.username,
            fav_count: r.fav_count,
            last_datetime_n: fmt_dt(r.last_datetime),
            last_datetime: r.last_datetime,
        }
    }
}

/// Paginated list of jobseekers who have favorited my company's jobs.
/// Job-seekers receive an empty list (only `usertype=2` has fans).
#[utoipa::path(
    get,
    path = "/v1/mcenter/fans",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<FanItem>>> {
    let r = fan_service::list_fans(&state, &user, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(FanItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}
