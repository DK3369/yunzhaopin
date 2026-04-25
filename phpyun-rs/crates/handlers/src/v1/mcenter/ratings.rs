//! Ratings I have given to others.

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Router,
};
use phpyun_core::{
    ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, ValidatedJson,
};
use phpyun_services::rating_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/ratings", post(rate))
        .route("/ratings/{kind}/{target_uid}", get(get_mine).post(unrate))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RateForm {
    pub target_uid: u64,
    /// 1=company 2=resume 3=job
    #[validate(range(min = 1, max = 3))]
    pub target_kind: i32,
    #[validate(range(min = 1, max = 5))]
    pub stars: i32,
    #[validate(length(max = 1000))]
    #[serde(default)]
    pub comment: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MyRating {
    pub target_uid: u64,
    pub target_kind: i32,
    pub stars: i32,
    pub comment: String,
    pub updated_at: i64,
}

impl From<phpyun_models::rating::entity::Rating> for MyRating {
    fn from(r: phpyun_models::rating::entity::Rating) -> Self {
        Self {
            target_uid: r.target_uid,
            target_kind: r.target_kind,
            stars: r.stars,
            comment: r.comment,
            updated_at: r.updated_at,
        }
    }
}

/// Rate / update rating
#[utoipa::path(
    post,
    path = "/v1/mcenter/ratings",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = RateForm,
    responses((status = 200, description = "ok"))
)]
pub async fn rate(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<RateForm>,
) -> AppResult<ApiOk> {
    rating_service::rate(
        &state,
        &user,
        f.target_uid,
        f.target_kind,
        f.stars,
        &f.comment,
    )
    .await?;
    Ok(ApiOk("ok"))
}

/// Get my rating for a target
#[utoipa::path(
    get,
    path = "/v1/mcenter/ratings/{kind}/{target_uid}",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("kind" = i32, Path), ("target_uid" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn get_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path((kind, target_uid)): Path<(i32, u64)>,
) -> AppResult<ApiJson<Option<MyRating>>> {
    let r = rating_service::get_mine(&state, &user, target_uid, kind).await?;
    Ok(ApiJson(r.map(MyRating::from)))
}

/// Withdraw rating
#[utoipa::path(
    post,
    path = "/v1/mcenter/ratings/{kind}/{target_uid}",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("kind" = i32, Path), ("target_uid" = u64, Path)),
    responses((status = 200, description = "ok"))
)]
pub async fn unrate(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path((kind, target_uid)): Path<(i32, u64)>,
) -> AppResult<ApiOk> {
    rating_service::unrate(&state, &user, target_uid, kind).await?;
    Ok(ApiOk("removed"))
}
