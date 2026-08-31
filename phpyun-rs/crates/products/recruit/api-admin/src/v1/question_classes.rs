//! PHP `neirong/question_class`.

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::{CreatedId, IdBody, IdsBody};
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, Pagination, ValidatedJson,
};
use phpyun_models::qna::entity::QClass;
use phpyun_services::admin_eval_service::{self, QClassAdminRow};
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::dto::AdminPaged;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/question-classes", post(upsert))
        .route("/question-classes/list", post(list))
        .route("/question-classes/delete", post(delete))
        .route("/question-classes/detail", post(detail))
}

#[derive(Debug, Default, Deserialize, Validate, ToSchema)]
pub struct ListQuery {
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub pid: Option<i32>,
    #[validate(length(max = 80))]
    pub keyword: Option<String>,
}

#[utoipa::path(post, path = "/v1/admin/question-classes/list", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<ListQuery>,
) -> AppResult<ApiResponse<AdminPaged<QClassAdminRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_eval_service::list_qclasses(&state, q.pid, q.keyword.as_deref(), page).await?,
    )))
}

#[utoipa::path(post, path = "/v1/admin/question-classes/detail", tag = "admin", security(("bearer" = [])), request_body = IdBody, responses((status = 200, description = "ok")))]
pub async fn detail(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdBody>,
) -> AppResult<ApiResponse<QClass>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_eval_service::qclass_detail(&state, f.id).await?,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct QClassForm {
    pub id: Option<u64>,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[serde(default)]
    pub pid: i32,
    #[serde(default)]
    pub intro: String,
    #[serde(default)]
    pub sort: i32,
    pub pic: Option<String>,
}

#[utoipa::path(post, path = "/v1/admin/question-classes", tag = "admin", security(("bearer" = [])), request_body = QClassForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn upsert(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<QClassForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    user.require_admin()?;
    let id = admin_eval_service::upsert_qclass(
        &state,
        &user,
        f.id,
        &f.name,
        f.pid,
        &f.intro,
        f.sort,
        f.pic.as_deref(),
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

#[utoipa::path(post, path = "/v1/admin/question-classes/delete", tag = "admin", security(("bearer" = [])), request_body = IdsBody, responses((status = 200, description = "ok")))]
pub async fn delete(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdsBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_eval_service::delete_qclasses(&state, &user, &f.ids).await?;
    Ok(ApiResponse::message("ok"))
}
