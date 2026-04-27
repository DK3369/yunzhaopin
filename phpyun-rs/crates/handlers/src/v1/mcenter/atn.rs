//! Follow / unfollow — aligned with PHP `wap/ajax::atn_action` &
//! `wap/ajax::atncompany_action`.
//!
//! Toggle semantics: `POST /v1/mcenter/follows` flips between followed and
//! unfollowed; `data.following` is the authoritative new state.

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{
    ApiJson, ApiMsgData, AppResult, AppState, AuthenticatedUser, Paged, Pagination,
    ValidatedJson, ValidatedQuery
};
use phpyun_services::atn_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/follows", post(toggle).get(list_following))
        .route("/follows/exists/{kind}/{target_uid}", get(exists))
        .route("/followers", get(list_followers))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct FollowToggleForm {
    /// 1 = user, 2 = company
    #[validate(range(min = 1, max = 2))]
    pub target_kind: i32,
    pub target_uid: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ToggleResp {
    /// New state after toggle: true = now following, false = now unfollowed.
    pub following: bool,
}

/// Toggle follow — followed ↔ unfollowed.
#[utoipa::path(
    post,
    path = "/v1/mcenter/follows",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = FollowToggleForm,
    responses(
        (status = 200, description = "ok", body = ToggleResp),
        (status = 400, description = "Invalid kind / cannot follow yourself"),
        (status = 403, description = "Only jobseekers may follow"),
    )
)]
pub async fn toggle(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<FollowToggleForm>,
) -> AppResult<ApiMsgData<ToggleResp>> {
    let r = atn_service::toggle(&state, &user, f.target_kind, f.target_uid).await?;
    Ok(ApiMsgData {
        msg_key: if r.following { "follow_added" } else { "follow_removed" },
        data: ToggleResp { following: r.following },
    })
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FollowItem {
    pub id: u64,
    pub uid: u64,
    pub sc_uid: u64,
    pub time: i64,
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
            usertype: a.usertype,
            sc_usertype: a.sc_usertype,
        }
    }
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ListQuery {
    /// 1 = user, 2 = company
    pub kind: i32,
}

/// Targets I am following (filtered by kind).
#[utoipa::path(
    get,
    path = "/v1/mcenter/follows",
    tag = "mcenter",
    security(("bearer" = [])),
    params(ListQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn list_following(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedQuery(q): ValidatedQuery<ListQuery>,
) -> AppResult<ApiJson<Paged<FollowItem>>> {
    let r = atn_service::list_following(&state, &user, q.kind, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(FollowItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

/// Followers of the current user (employers see who follows their company,
/// jobseekers see who follows them as a teacher/contact).
#[utoipa::path(
    get,
    path = "/v1/mcenter/followers",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_followers(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<FollowItem>>> {
    let r = atn_service::list_followers(&state, &user, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(FollowItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExistsResp {
    pub exists: bool,
}

/// Cheap probe used by frontend to render the follow-button state.
#[utoipa::path(
    get,
    path = "/v1/mcenter/follows/exists/{kind}/{target_uid}",
    tag = "mcenter",
    security(("bearer" = [])),
    params(
        ("kind" = i32, Path),
        ("target_uid" = u64, Path),
    ),
    responses((status = 200, description = "ok"))
)]
pub async fn exists(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path((kind, target_uid)): Path<(i32, u64)>,
) -> AppResult<ApiJson<ExistsResp>> {
    let ok = atn_service::exists(&state, &user, kind, target_uid).await?;
    Ok(ApiJson(ExistsResp { exists: ok }))
}
