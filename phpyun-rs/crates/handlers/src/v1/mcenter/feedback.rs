//! User feedback endpoints.

use axum::{extract::State, routing::post, Router};
use phpyun_core::{
    ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, MaybeUser, Paged, Pagination,
    ValidatedJson,
};
use phpyun_services::feedback_service::{self, FeedbackInput};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/feedback", post(submit).get(list_mine))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct FeedbackForm {
    #[validate(length(min = 1, max = 32))]
    pub category: String,
    #[validate(length(min = 5, max = 5000))]
    pub content: String,
    #[validate(length(max = 100))]
    pub contact: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreatedId {
    pub id: u64,
}

/// Submit feedback (anonymous allowed)
#[utoipa::path(
    post,
    path = "/v1/mcenter/feedback",
    tag = "mcenter",
    request_body = FeedbackForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn submit(
    State(state): State<AppState>,
    MaybeUser(user): MaybeUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<FeedbackForm>,
) -> AppResult<ApiJson<CreatedId>> {
    let id = feedback_service::submit(
        &state,
        user.as_ref(),
        FeedbackInput {
            category: &f.category,
            content: &f.content,
            contact: f.contact.as_deref().unwrap_or(""),
        },
        &ip,
    )
    .await?;
    Ok(ApiJson(CreatedId { id }))
}

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}

#[derive(Debug, Serialize, ToSchema)]
pub struct FeedbackItem {
    pub id: u64,
    pub uid: Option<u64>,
    pub category: String,
    pub content: String,
    pub contact: String,
    pub client_ip: String,
    pub status: i32,
    pub created_at: i64,
    pub created_at_n: String,
}

impl From<phpyun_models::feedback::entity::Feedback> for FeedbackItem {
    fn from(f: phpyun_models::feedback::entity::Feedback) -> Self {
        Self {
            id: f.id,
            uid: f.uid,
            category: f.category,
            content: f.content,
            contact: f.contact,
            client_ip: f.client_ip,
            status: f.status,
            created_at_n: fmt_dt(f.created_at),
            created_at: f.created_at,
        }
    }
}

/// Feedback I have submitted (login required)
#[utoipa::path(
    get,
    path = "/v1/mcenter/feedback",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<FeedbackItem>>> {
    let r = feedback_service::list_mine(&state, &user, page).await?;
    Ok(ApiJson(Paged::new(
        r.list.into_iter().map(FeedbackItem::from).collect(),
        r.total,
        page.page,
        page.page_size,
    )))
}
