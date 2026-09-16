//! News group subscriptions. Not `/v1/wap/subscribe` (job-alert email).

use axum::{extract::State, routing::post, Router};
use phpyun_core::{ApiResponse, AppResult, AppState, AuthenticatedUser, ValidatedJson};
use phpyun_services::article_channel_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/article-channels", post(list))
        .route("/article-channels/save", post(save))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ChannelItem {
    pub id: u64,
    pub name: String,
    pub keyid: i32,
    pub subscribed: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ChannelList {
    pub list: Vec<ChannelItem>,
    pub ids: Vec<u64>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SaveForm {
    #[serde(default)]
    #[validate(length(max = 100))]
    pub ids: Vec<u64>,
}

/// All `phpyun_news_group` rows plus this user's subscription marks.
/// **Not** the job-email subscribe at `/v1/wap/subscribe`.
#[utoipa::path(
    post,
    path = "/v1/mcenter/article-channels",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = ChannelList))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<ChannelList>> {
    let r = article_channel_service::list(&state, &user).await?;
    Ok(ApiResponse::data(ChannelList {
        ids: r.ids,
        list: r
            .list
            .into_iter()
            .map(|c| ChannelItem {
                id: c.id,
                name: c.name,
                keyid: c.keyid,
                subscribed: c.subscribed,
            })
            .collect(),
    }))
}

/// Replace the subscription set (`ids` empty = unsubscribe all).
#[utoipa::path(
    post,
    path = "/v1/mcenter/article-channels/save",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = SaveForm,
    responses((status = 200, description = "ok", body = ChannelList))
)]
pub async fn save(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SaveForm>,
) -> AppResult<ApiResponse<ChannelList>> {
    let r = article_channel_service::save(&state, &user, &f.ids).await?;
    Ok(ApiResponse::data(ChannelList {
        ids: r.ids,
        list: r
            .list
            .into_iter()
            .map(|c| ChannelItem {
                id: c.id,
                name: c.name,
                keyid: c.keyid,
                subscribed: c.subscribed,
            })
            .collect(),
    }))
}
