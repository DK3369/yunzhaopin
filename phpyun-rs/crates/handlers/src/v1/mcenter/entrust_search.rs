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

pub fn routes() -> Router<AppState> {
    Router::new().route("/entrusts", post(list_for_headhunter))
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
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
    let (total, rows): (Result<u64, sqlx::Error>, Result<Vec<phpyun_models::entrust::entity::Entrust>, sqlx::Error>) = tokio::join!(
        async {
            let (n,): (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM phpyun_entrust WHERE lt_uid = ?",
            )
            .bind(user.uid)
            .fetch_one(db)
            .await?;
            Ok(n.max(0) as u64)
        },
        async {
            sqlx::query_as::<_, phpyun_models::entrust::entity::Entrust>(
                "SELECT
                    CAST(id AS UNSIGNED) AS id,
                    CAST(uid AS UNSIGNED) AS uid,
                    CAST(COALESCE(lt_uid, 0) AS UNSIGNED) AS lt_uid,
                    COALESCE(datetime, 0) AS datetime,
                    COALESCE(remind_status, 0) AS remind_status
                 FROM phpyun_entrust
                 WHERE lt_uid = ?
                 ORDER BY datetime DESC, id DESC
                 LIMIT ? OFFSET ?",
            )
            .bind(user.uid)
            .bind(page.limit)
            .bind(page.offset)
            .fetch_all(db)
            .await
        }
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
