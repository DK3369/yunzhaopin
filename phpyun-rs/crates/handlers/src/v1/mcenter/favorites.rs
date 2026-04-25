//! Job favorites — toggle semantics.
//!
//! - POST   /v1/mcenter/favorites                  — **Toggle** { kind, target_id }; favorited ↔ unfavorited
//! - POST   /v1/mcenter/favorites/{kind}/{target}  — Explicit unfavorite (kept for completeness)
//! - GET    /v1/mcenter/favorites?kind=1           — My favorites list
//! - GET    /v1/mcenter/favorites/exists/{kind}/{target} — Whether already favorited (cheap probe)
//!
//! The toggle design eliminates the "already favorited" 409 third-state — the
//! frontend just calls POST and reads `data.favorited` from the response.

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{
    ApiJson, ApiMsg, ApiMsgData, AppResult, AppState, AuthenticatedUser, ClientIp, Paged,
    Pagination, ValidatedJson,
};
use phpyun_services::collect_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/favorites", post(add).get(list))
        .route("/favorites/{kind}/{target_id}", post(remove))
        .route("/favorites/exists/{kind}/{target_id}", get(exists))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AddFavoriteForm {
    /// 1=job / 2=company / 3=resume
    #[validate(range(min = 1, max = 3))]
    pub kind: i32,
    pub target_id: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ToggleResp {
    /// New state after toggle: `true` = now favorited, `false` = now removed.
    pub favorited: bool,
}

/// Toggle favorite — favorited ↔ unfavorited. The response's `data.favorited`
/// is the authoritative new state; the `msg` is the localized confirmation
/// ("添加收藏成功" / "已取消收藏" / etc.).
#[utoipa::path(
    post,
    path = "/v1/mcenter/favorites",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = AddFavoriteForm,
    responses(
        (status = 200, description = "ok", body = ToggleResp),
        (status = 400, description = "Invalid kind"),
        (status = 404, description = "Target not found"),
    )
)]
pub async fn add(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<AddFavoriteForm>,
) -> AppResult<ApiMsgData<ToggleResp>> {
    let now_favorited = collect_service::toggle(&state, &user, f.kind, f.target_id, &ip).await?;
    Ok(ApiMsgData {
        msg_key: if now_favorited { "collect_added" } else { "collect_removed" },
        data: ToggleResp { favorited: now_favorited },
    })
}

/// Remove favorite
#[utoipa::path(
    post,
    path = "/v1/mcenter/favorites/{kind}/{target_id}",
    tag = "mcenter",
    security(("bearer" = [])),
    params(
        ("kind" = i32, Path, description = "1/2/3"),
        ("target_id" = u64, Path),
    ),
    responses((status = 200, description = "ok"))
)]
pub async fn remove(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    Path((kind, target_id)): Path<(i32, u64)>,
) -> AppResult<ApiMsg> {
    collect_service::remove(&state, &user, kind, target_id, &ip).await?;
    Ok(ApiMsg("collect_removed"))
}

// Favorites list reuses `JobSummary` — exact same shape as `GET /v1/wap/jobs`
// so the front-end can render the favorites page with the same card component
// (salary / city / edu / exp / com_logo / is_favorited / ... all included).
use super::super::wap::jobs::JobSummary;

#[derive(Debug, Deserialize, IntoParams)]
pub struct ListQuery {
    /// 1=job / 2=company / 3=resume
    pub kind: i32,
}

/// My favorites list (filtered by kind). Item shape matches `GET /v1/wap/jobs`
/// — identical `JobSummary` so the frontend reuses the same card component.
/// `is_favorited` is always `true` here.
#[utoipa::path(
    get,
    path = "/v1/mcenter/favorites",
    tag = "mcenter",
    security(("bearer" = [])),
    params(ListQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    Query(q): Query<ListQuery>,
) -> AppResult<ApiJson<Paged<JobSummary>>> {
    let r = collect_service::list(&state, &user, q.kind, page).await?;

    // Extract job_ids in the favorited order; filter out any rows where the
    // snapshot's `job_id` is NULL (shouldn't happen but be defensive).
    let ordered_ids: Vec<u64> = r.list.iter().filter_map(|c| c.job_id).collect();

    // Batch-load live job rows; missing ones (job deleted / off-shelf) just
    // get omitted from the result. Front-end handles "list shorter than total".
    let live_jobs =
        phpyun_models::job::repo::list_by_ids(state.db.reader(), &ordered_ids)
            .await
            .unwrap_or_default();
    let mut by_id: std::collections::HashMap<u64, phpyun_models::job::entity::Job> =
        live_jobs.into_iter().map(|j| (j.id, j)).collect();

    let dicts = phpyun_services::dict_service::get(&state).await?;
    let now = phpyun_core::clock::now_ts();

    // Map favorited job_ids → JobSummary in the favorited (DESC) order, with
    // is_favorited stamped true (they're all favorited by definition).
    let items: Vec<JobSummary> = ordered_ids
        .into_iter()
        .filter_map(|id| by_id.remove(&id))
        .map(|j| JobSummary::from_with_dict_fav(j, &dicts, now, true))
        .collect();

    Ok(ApiJson(Paged::new(items, r.total, page.page, page.page_size)))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExistsResp {
    pub exists: bool,
}

/// Whether the current user has already favorited a given target (front-end button state)
#[utoipa::path(
    get,
    path = "/v1/mcenter/favorites/exists/{kind}/{target_id}",
    tag = "mcenter",
    security(("bearer" = [])),
    params(
        ("kind" = i32, Path),
        ("target_id" = u64, Path),
    ),
    responses((status = 200, description = "ok"))
)]
pub async fn exists(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path((kind, target_id)): Path<(i32, u64)>,
) -> AppResult<ApiJson<ExistsResp>> {
    let ok = collect_service::exists(&state, &user, kind, target_id).await?;
    Ok(ApiJson(ExistsResp { exists: ok }))
}
