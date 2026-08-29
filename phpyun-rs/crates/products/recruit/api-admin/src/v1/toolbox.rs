//! PHP `neirong/toolbox_doc` / `toolbox_class`.

use axum::{extract::State, routing::post, Router};
use phpyun_core::dto::{CreatedId, IdsBody};
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, Pagination, ValidatedJson,
};
use phpyun_models::hr_doc::entity::{AdminHrDoc, ToolboxClass};
use phpyun_services::admin_eval_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::dto::AdminPaged;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/toolbox/docs", post(upsert_doc))
        .route("/toolbox/docs/list", post(list_docs))
        .route("/toolbox/docs/delete", post(delete_docs))
        .route("/toolbox/docs/show", post(set_show))
        .route("/toolbox/docs/meta", post(doc_meta))
        .route("/toolbox/docs/detail", post(doc_detail))
        .route("/toolbox/classes", post(upsert_class))
        .route("/toolbox/classes/list", post(list_classes))
        .route("/toolbox/classes/delete", post(delete_classes))
}

#[derive(Debug, Default, Deserialize, Validate, ToSchema)]
pub struct DocListQuery {
    pub cid: Option<u64>,
    #[validate(length(max = 80))]
    pub keyword: Option<String>,
    pub status: Option<i32>,
}

#[utoipa::path(post, path = "/v1/admin/toolbox/docs/list", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_docs(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<DocListQuery>,
) -> AppResult<ApiResponse<AdminPaged<AdminHrDoc>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_eval_service::list_docs(&state, q.cid, q.keyword.as_deref(), q.status, page).await?,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct DocForm {
    pub id: Option<u64>,
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    #[validate(range(min = 1))]
    pub cid: u64,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub is_show: i32,
}

#[utoipa::path(post, path = "/v1/admin/toolbox/docs", tag = "admin", security(("bearer" = [])), request_body = DocForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn upsert_doc(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<DocForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    user.require_admin()?;
    let id = admin_eval_service::upsert_doc(
        &state,
        &user,
        f.id,
        &f.name,
        f.cid,
        &f.url,
        f.is_show,
    )
    .await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct DocShowForm {
    #[validate(range(min = 1))]
    pub id: u64,
    pub show: i32,
}

#[utoipa::path(post, path = "/v1/admin/toolbox/docs/show", tag = "admin", security(("bearer" = [])), request_body = DocShowForm, responses((status = 200, description = "ok")))]
pub async fn set_show(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<DocShowForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_eval_service::set_doc_show(&state, &user, f.id, f.show).await?;
    Ok(ApiResponse::message("ok"))
}

#[utoipa::path(post, path = "/v1/admin/toolbox/docs/delete", tag = "admin", security(("bearer" = [])), request_body = IdsBody, responses((status = 200, description = "ok")))]
pub async fn delete_docs(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdsBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_eval_service::delete_docs(&state, &user, &f.ids).await?;
    Ok(ApiResponse::message("ok"))
}

#[derive(Debug, Serialize)]
pub struct ToolboxClassList {
    pub list: Vec<ToolboxClass>,
}

#[utoipa::path(post, path = "/v1/admin/toolbox/classes/list", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list_classes(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<ToolboxClassList>> {
    user.require_admin()?;
    Ok(ApiResponse::data(ToolboxClassList {
        list: admin_eval_service::list_classes(&state).await?,
    }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct DocIdForm {
    pub id: Option<u64>,
}

#[utoipa::path(post, path = "/v1/admin/toolbox/docs/meta", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn doc_meta(user: AuthenticatedUser) -> AppResult<ApiResponse<serde_json::Value>> {
    user.require_admin()?;
    Ok(ApiResponse::data(serde_json::json!({
        "search_list": [
            {"param": "status", "name": "admin_00271", "value": {"1": "member_com_00023", "0": "admin_user_00340"}},
            {"param": "end", "name": "admin_00269", "value": {"1": "common_01940", "3": "admin_user_00179", "7": "admin_user_00178", "15": "admin_user_00180", "30": "admin_user_00175"}}
        ]
    })))
}

#[utoipa::path(post, path = "/v1/admin/toolbox/docs/detail", tag = "admin", security(("bearer" = [])), request_body = DocIdForm, responses((status = 200, description = "ok")))]
pub async fn doc_detail(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<DocIdForm>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_eval_service::doc_editor(&state, f.id).await?,
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ClassForm {
    pub id: Option<u64>,
    #[validate(length(min = 1, max = 80))]
    pub name: String,
    #[validate(length(min = 1, max = 4000))]
    pub content: String,
    pub pic: Option<String>,
}

#[utoipa::path(post, path = "/v1/admin/toolbox/classes", tag = "admin", security(("bearer" = [])), request_body = ClassForm, responses((status = 200, description = "ok", body = CreatedId)))]
pub async fn upsert_class(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<ClassForm>,
) -> AppResult<ApiResponse<CreatedId>> {
    user.require_admin()?;
    let id =
        admin_eval_service::upsert_class(&state, &user, f.id, &f.name, &f.content, f.pic.as_deref())
            .await?;
    Ok(ApiResponse::data(CreatedId { id }))
}

#[utoipa::path(post, path = "/v1/admin/toolbox/classes/delete", tag = "admin", security(("bearer" = [])), request_body = IdsBody, responses((status = 200, description = "ok")))]
pub async fn delete_classes(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<IdsBody>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_eval_service::delete_classes(&state, &user, &f.ids).await?;
    Ok(ApiResponse::message("ok"))
}
