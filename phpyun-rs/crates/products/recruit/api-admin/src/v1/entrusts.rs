//! Admin 个人委托简历 (`phpyun_user_entrust`).

use axum::{extract::OriginalUri, extract::State, routing::post, Router};
use phpyun_core::dto::IdsBody;
use phpyun_core::{
    ApiMessage, ApiResponse, AppResult, AppState, AuthenticatedUser, Pagination, ValidatedJson,
};
use phpyun_models::user_entrust::entity::{TrustStat, UserEntrustRow};
use phpyun_services::admin_entrust_service::{self, ListFilter};
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::dto::AdminPaged;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/user-entrusts", post(list))
        .route("/user-entrusts/statist", post(statist))
        .route("/user-entrusts/delete", post(delete))
        .route("/user-entrusts/status", post(set_status))
}

#[derive(Debug, Default, Deserialize, Validate, ToSchema)]
pub struct ListQuery {
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub status: Option<i32>,
    #[validate(length(max = 80))]
    pub keyword: Option<String>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub r#type: Option<i32>,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_i32_opt")]
    pub end: Option<i32>,
    #[validate(length(max = 20))]
    pub t: Option<String>,
    #[validate(length(max = 10))]
    pub order: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SetStatusForm {
    #[validate(range(min = 1))]
    pub id: u64,
    #[serde(deserialize_with = "phpyun_core::date_parse::de_loose_i32")]
    pub status: i32,
}

#[utoipa::path(post, path = "/v1/admin/user-entrusts", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<ListQuery>,
) -> AppResult<ApiResponse<AdminPaged<UserEntrustRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_entrust_service::list(
            &state,
            ListFilter {
                status: q.status,
                keyword: q.keyword.as_deref(),
                name_kind: q.r#type.unwrap_or(1),
                end: q.end,
                sort: q.t.as_deref().unwrap_or("id"),
                dir: q.order.as_deref().unwrap_or("desc"),
            },
            page,
        )
        .await?,
    )))
}

#[utoipa::path(post, path = "/v1/admin/user-entrusts/statist", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn statist(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<TrustStat>> {
    user.require_admin()?;
    Ok(ApiResponse::data(admin_entrust_service::stat(&state).await?))
}

#[utoipa::path(post, path = "/v1/admin/user-entrusts/delete", tag = "admin", security(("bearer" = [])), request_body = IdsBody, responses((status = 200, description = "ok")))]
pub async fn delete(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    OriginalUri(uri): OriginalUri,
    ValidatedJson(f): ValidatedJson<IdsBody>,
) -> AppResult<ApiMessage> {
    user.require_admin()?;
    let msg = admin_entrust_service::delete(&state, &user, &f.ids, uri.path()).await?;
    Ok(ApiMessage::new("admin_user_00187", msg))
}

#[utoipa::path(post, path = "/v1/admin/user-entrusts/status", tag = "admin", security(("bearer" = [])), request_body = SetStatusForm, responses((status = 200, description = "ok")))]
pub async fn set_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SetStatusForm>,
) -> AppResult<ApiMessage> {
    user.require_admin()?;
    let msg = admin_entrust_service::set_status(&state, &user, f.id, f.status).await?;
    Ok(ApiMessage::new("wap_user_00264", msg))
}
