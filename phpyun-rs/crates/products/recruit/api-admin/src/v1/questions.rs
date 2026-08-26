//! Q&A review. PHP POST `status` maps to column `state`.

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::IdBody;
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, Paged, Pagination, ValidatedJson,
};
use phpyun_models::qna::entity::Question;
use phpyun_services::admin_cms_service;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/questions", post(list))
        .route("/questions/state", post(set_state))
        .route("/questions/delete", post(delete))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct ListQuery {
    /// PHP `status` → `phpyun_question.state`
    pub status: Option<i32>,
    #[validate(length(max = 80))]
    pub keyword: Option<String>,
    pub is_recom: Option<i32>,
}

#[utoipa::path(post, path = "/v1/admin/questions", tag = "admin", security(("bearer" = [])), params(ListQuery), responses((status = 200, description = "ok")))]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<ListQuery>,
) -> AppResult<ApiResponse<Paged<Question>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_cms_service::list_questions(
            &state,
            q.status,
            q.keyword.as_deref(),
            q.is_recom,
            page,
        )
        .await?,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SetStateForm {
    #[validate(range(min = 1))]
    pub id: u64,
    #[serde(alias = "status")]
    pub state: i32,
}

#[utoipa::path(post, path = "/v1/admin/questions/state", tag = "admin", security(("bearer" = [])), request_body = SetStateForm, responses((status = 200, description = "ok")))]
pub async fn set_state(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SetStateForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_cms_service::set_question_state(&state, &user, f.id, f.state).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/questions/delete", tag = "admin", security(("bearer" = [])), request_body = IdBody, responses((status = 200, description = "ok")))]
pub async fn delete(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_cms_service::delete_question(&state, &user, f.id).await?;
    Ok(ApiResponse::message("ok"))
}
