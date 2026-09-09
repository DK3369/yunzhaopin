//! Company special-topic sign-ups (PHP `member/com/model/special.class.php`).

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::IdBody;
use phpyun_core::utils::fmt_dt;
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_services::special_service;
use serde::Serialize;
use utoipa::ToSchema;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/specials/mine", post(mine))
        .route("/specials/delete", post(delete))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MineSpecialView {
    pub id: u64,
    pub sid: u64,
    pub status: i32,
    pub created_at: i64,
    pub datetime_n: String,
    pub title: String,
}

/// My special-topic applications
#[utoipa::path(
    post,
    path = "/v1/mcenter/specials/mine",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<MineSpecialView>>> {
    let r = special_service::list_mine(&state, &user, page).await?;
    Ok(ApiResponse::data(Paged::from_listing(
        r.list
            .into_iter()
            .map(|row| MineSpecialView {
                id: row.id,
                sid: row.sid,
                status: row.status,
                datetime_n: fmt_dt(row.created_at),
                created_at: row.created_at,
                title: row.title,
            })
            .collect(),
        r.total,
        page,
    )))
}

/// Delete my special-topic application
#[utoipa::path(
    post,
    path = "/v1/mcenter/specials/delete",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdBody,
    responses((status = 200, description = "ok"))
)]
pub async fn delete(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse> {
    special_service::delete_mine(&state, &user, b.id).await?;
    Ok(ApiResponse::message("deleted"))
}
