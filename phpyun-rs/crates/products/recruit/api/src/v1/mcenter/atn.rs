//! Followers of the current member (`POST /v1/mcenter/followers`).
//! Follow / unfollow of companies and users is `/v1/mcenter/favorites*`.

use axum::{extract::State, routing::post, Router};
use phpyun_core::utils::fmt_dt;
use phpyun_core::{ApiResponse, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::apply::repo as apply_repo;
use phpyun_services::atn_service;
use serde::Serialize;
use utoipa::ToSchema;

pub fn routes() -> Router<AppState> {
    Router::new().route("/followers", post(list_followers))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FollowItem {
    pub id: u64,
    pub uid: u64,
    pub sc_uid: u64,
    pub time: i64,
    pub datetime_n: String,
    pub uname: String,
    pub usertype: Option<i32>,
    pub sc_usertype: Option<i32>,
}

impl From<phpyun_models::atn::entity::Atn> for FollowItem {
    fn from(a: phpyun_models::atn::entity::Atn) -> Self {
        Self {
            id: a.id,
            uid: a.uid,
            sc_uid: a.sc_uid,
            time: a.time,
            datetime_n: fmt_dt(a.time),
            uname: String::new(),
            usertype: a.usertype,
            sc_usertype: a.sc_usertype,
        }
    }
}

/// Followers of the current user (employers see who follows their company,
/// jobseekers see who follows them as a teacher/contact).
#[utoipa::path(
    post,
    path = "/v1/mcenter/followers",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_followers(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<FollowItem>>> {
    let r = atn_service::list_followers(&state, &user, page).await?;
    let mut list: Vec<FollowItem> = r.list.into_iter().map(FollowItem::from).collect();
    let uids: Vec<u64> = list.iter().map(|i| i.uid).collect();
    let names = apply_repo::resume_names_by_uids(state.db.reader(), &uids).await?;
    for it in &mut list {
        if let Some(n) = names.get(&it.uid) {
            it.uname = n.clone();
        }
    }
    Ok(ApiResponse::data(Paged::from_listing(list, r.total, page)))
}
