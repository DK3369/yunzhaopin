//! Friend links (public).

use axum::{extract::State, routing::{get, post}, Router};
use phpyun_core::{ApiResponse, AppResult, AppState, ClientIp, ValidatedJson, ValidatedJsonOrQuery};
use phpyun_services::friend_link_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub const GET_ALLOWED_PATHS: &[&str] = &["/v1/wap/friend-links"];

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/friend-links", get(list).post(list))
        .route("/friend-links/apply", post(apply))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct LinkQuery {
    #[validate(length(max = 100))]
    pub category: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LinkItem {
    pub id: u64,
    pub name: String,
    pub url: String,
    pub logo: String,
    pub category: String,
    pub sort: i32,
}

impl From<phpyun_models::friend_link::entity::FriendLink> for LinkItem {
    fn from(l: phpyun_models::friend_link::entity::FriendLink) -> Self {
        Self {
            id: l.id,
            name: l.name,
            url: l.url,
            logo: l.logo,
            category: l.category,
            sort: l.sort,
        }
    }
}

/// List friend links
#[utoipa::path(post, path = "/v1/wap/friend-links", tag = "wap", params(LinkQuery), responses((status = 200, description = "ok")))]
pub async fn list(
    State(state): State<AppState>,
    ValidatedJsonOrQuery(q): ValidatedJsonOrQuery<LinkQuery>,
) -> AppResult<ApiResponse<Vec<LinkItem>>> {
    let list = friend_link_service::list(&state, q.category.as_deref()).await?;
    Ok(ApiResponse::data(
        list.iter().cloned().map(LinkItem::from).collect(),
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LinkApplyForm {
    #[validate(length(min = 1, max = 64))]
    pub name: String,
    #[validate(length(min = 1, max = 255))]
    pub url: String,
    #[validate(length(min = 1, max = 64))]
    pub captcha_cid: String,
    #[validate(length(min = 1, max = 16))]
    pub captcha_input: String,
}

pub async fn apply(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<LinkApplyForm>,
) -> AppResult<ApiResponse<phpyun_core::dto::CreatedId>> {
    phpyun_core::verify::verify(
        &state.redis,
        phpyun_core::verify::VerifyKind::ImageCaptcha,
        &f.captcha_cid,
        &f.captcha_input.to_uppercase(),
    )
    .await?
    .then_some(())
    .ok_or_else(phpyun_core::ApiError::captcha)?;
    let id = friend_link_service::apply(&state, &f.name, &f.url, &ip).await?;
    Ok(ApiResponse::data(phpyun_core::dto::CreatedId { id }))
}
