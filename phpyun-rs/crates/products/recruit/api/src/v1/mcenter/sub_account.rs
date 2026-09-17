//! Classic company sub-accounts (`phpyun_member.pid`).

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::CreatedId;
use phpyun_core::utils::fmt_dt;
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, ValidatedJson,
};
use phpyun_services::sub_account_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/company/sub-accounts/list", post(list))
        .route("/company/sub-accounts/create", post(create))
        .route("/company/sub-accounts/update", post(update))
        .route("/company/sub-accounts/delete", post(delete))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SubAccountView {
    pub uid: u64,
    pub username: String,
    pub status: i32,
    pub login_date: i64,
    pub login_date_n: String,
}

impl From<phpyun_models::sub_account::entity::SubAccountRow> for SubAccountView {
    fn from(r: phpyun_models::sub_account::entity::SubAccountRow) -> Self {
        Self {
            uid: r.uid,
            username: r.username,
            status: r.status,
            login_date_n: fmt_dt(r.login_date),
            login_date: r.login_date,
        }
    }
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/company/sub-accounts/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = [SubAccountView]))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<Vec<SubAccountView>>> {
    let rows = sub_account_service::list(&state, &user).await?;
    Ok(ApiResponse::data(rows.into_iter().map(SubAccountView::from).collect()))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateForm {
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    #[validate(length(min = 6, max = 128))]
    pub password: String,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/company/sub-accounts/create",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = CreateForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<CreateForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    let id = sub_account_service::create(&state, &user, &f.username, &f.password).await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub uid: u64,
    #[serde(default)]
    #[validate(length(min = 6, max = 128))]
    pub password: Option<String>,
    #[serde(default)]
    #[validate(range(min = 0, max = 2))]
    pub status: Option<i32>,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/company/sub-accounts/update",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = UpdateForm,
    responses((status = 200, description = "ok"))
)]
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<UpdateForm>,
) -> AppResult<ApiResponse> {
    sub_account_service::update(
        &state,
        &user,
        f.uid,
        f.password.as_deref(),
        f.status,
    )
    .await?;
    Ok(ApiResponse::message("ok"))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct DeleteForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub uid: u64,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/company/sub-accounts/delete",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = DeleteForm,
    responses((status = 200, description = "ok"))
)]
pub async fn delete(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<DeleteForm>,
) -> AppResult<ApiResponse> {
    sub_account_service::delete(&state, &user, f.uid).await?;
    Ok(ApiResponse::message("ok"))
}
