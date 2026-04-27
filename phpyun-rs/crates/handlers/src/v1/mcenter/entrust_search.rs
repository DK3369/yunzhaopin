//! Headhunter-side: list jobseekers who entrusted this headhunter.
//!
//! Mirrors `phpyun_entrust` reverse direction (`WHERE lt_uid = current_user.uid`).
//! The richer "search by city / salary / edu" capability the previous Rust
//! version offered has no backing schema in PHP and was therefore removed.

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use serde::Serialize;
use utoipa::ToSchema;
use phpyun_core::utils::{fmt_dt};

pub fn routes() -> Router<AppState> {
    Router::new().route("/entrusts", post(list_for_headhunter))
}


#[derive(Debug, Serialize, ToSchema)]
pub struct EntrustedSeekerItem {
    pub id: u64,
    /// Jobseeker uid that bound to me.
    pub uid: u64,
    pub lt_uid: u64,
    pub datetime: i64,
    pub datetime_n: String,
    pub remind_status: i32,
}

/// List jobseekers who have entrusted me (call as headhunter, usertype=4)
#[utoipa::path(
    post,
    path = "/v1/mcenter/entrusts",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_for_headhunter(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<EntrustedSeekerItem>>> {
    // No formal `require_headhunter` helper yet — accept any authenticated
    // user and just return their inbound bindings. Callers who aren't lt
    // members will simply see an empty list.
    let db = state.db.reader();
    let (total, rows) = tokio::join!(
        phpyun_models::entrust::repo::count_by_lt_uid(db, user.uid),
        phpyun_models::entrust::repo::list_by_lt_uid(db, user.uid, page.offset, page.limit),
    );

    let items: Vec<EntrustedSeekerItem> = rows
        .unwrap_or_default()
        .into_iter()
        .map(|e| EntrustedSeekerItem {
            id: e.id,
            uid: e.uid,
            lt_uid: e.lt_uid,
            datetime_n: fmt_dt(e.datetime),
            datetime: e.datetime,
            remind_status: e.remind_status,
        })
        .collect();

    Ok(ApiJson(Paged::new(
        items,
        total.unwrap_or(0),
        page.page,
        page.page_size,
    )))
}
