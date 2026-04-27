//! Feedback queue (admin).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{dto::{BatchResult, StatusFilterBody}, ApiJson, ApiOk, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson};
use phpyun_services::admin_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use phpyun_core::utils::{fmt_dt};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/feedback", post(list))
        .route("/feedback/status", post(set_status))
        .route("/feedback/batch/status", post(batch_set_status))
}


fn fb_status_name(s: i32) -> &'static str {
    match s { 0 => "pending", 1 => "processing", 2 => "resolved", 3 => "closed", _ => "unknown" }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AdminFeedbackItem {
    pub id: u64,
    pub uid: Option<u64>,
    pub category: String,
    pub content: String,
    pub contact: String,
    pub client_ip: String,
    pub status: i32,
    pub status_n: String,
    pub created_at: i64,
    pub created_at_n: String,
}

impl From<phpyun_models::feedback::entity::Feedback> for AdminFeedbackItem {
    fn from(f: phpyun_models::feedback::entity::Feedback) -> Self {
        Self {
            id: f.id,
            uid: f.uid,
            category: f.category,
            content: f.content,
            contact: f.contact,
            client_ip: f.client_ip,
            status_n: fb_status_name(f.status).to_string(),
            status: f.status,
            created_at_n: fmt_dt(f.created_at),
            created_at: f.created_at,
        }
    }
}

/// Feedback list
#[utoipa::path(
    post,
    path = "/v1/admin/feedback",
    tag = "admin",
    security(("bearer" = [])),
    request_body = StatusFilterBody,
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<StatusFilterBody>,
) -> AppResult<ApiJson<Paged<AdminFeedbackItem>>> {
    user.require_admin()?;
    let r = admin_service::list_feedback(&state, q.status, page).await?;
    Ok(ApiJson(Paged::from_listing(r.list, r.total, page)))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SetFeedbackStatusForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,
    /// 1=resolved
    #[validate(range(min = 1, max = 1))]
    pub status: i32,
}

/// Mark feedback as resolved
#[utoipa::path(post,
    path = "/v1/admin/feedback/status",
    tag = "admin",
    security(("bearer" = [])),
    request_body = SetFeedbackStatusForm,
    responses((status = 200, description = "ok"))
)]
pub async fn set_status(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SetFeedbackStatusForm>) -> AppResult<ApiOk> {
    let id = f.id;
    user.require_admin()?;
    admin_service::set_feedback_status(&state, &user, id, f.status).await?;
    Ok(ApiOk("ok"))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct BatchStatusForm {
    #[validate(length(min = 1, max = 200))]
    pub ids: Vec<u64>,
    #[validate(range(min = 1, max = 1))]
    pub status: i32,
}

/// Batch mark as resolved
#[utoipa::path(
    post,
    path = "/v1/admin/feedback/batch/status",
    tag = "admin",
    security(("bearer" = [])),
    request_body = BatchStatusForm,
    responses((status = 200, description = "ok", body = BatchResult))
)]
pub async fn batch_set_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<BatchStatusForm>,
) -> AppResult<ApiJson<BatchResult>> {
    user.require_admin()?;
    let r = admin_service::batch_set_feedback_status(&state, &user, &f.ids, f.status).await?;
    Ok(ApiJson(BatchResult {
        requested: r.requested,
        affected: r.affected,
    }))
}
