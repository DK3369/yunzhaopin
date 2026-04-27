//! Friend links (public).

use axum::{
    extract::{State},
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, AppResult, AppState, ValidatedJson};
use phpyun_services::friend_link_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new().route("/friend-links", post(list))
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
    ValidatedJson(q): ValidatedJson<LinkQuery>,
) -> AppResult<ApiJson<Vec<LinkItem>>> {
    let list = friend_link_service::list(&state, q.category.as_deref()).await?;
    Ok(ApiJson(list.iter().cloned().map(LinkItem::from).collect()))
}
