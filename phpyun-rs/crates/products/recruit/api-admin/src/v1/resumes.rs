//! PHP `users_resume` 审核树：列表 / `r_status` / CSV。

use axum::{extract::State, routing::post, Json, Router};
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, Pagination, ValidatedJson,
};

use crate::dto::AdminPaged;
use phpyun_models::resume::edu::Edu;
use phpyun_models::resume::repo::AdminResumeRow;
use phpyun_models::resume::training::Training;
use phpyun_models::resume::work::Work;
use phpyun_services::admin_longtail_service::{self, CsvExport};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/resumes", post(list))
        .route("/resumes/status", post(set_status))
        .route("/resumes/export", post(export_csv))
        .route("/resumes/works", post(list_works))
        .route("/resumes/edus", post(list_edus))
        .route("/resumes/trainings", post(list_trainings))
        .route("/resumes/php-add", post(php_add))
        .route("/resumes/php-edit", post(php_edit))
        .route("/resumes/php-save-expect", post(php_save_expect))
        .route("/resumes/php-save-tag", post(php_save_tag))
}

#[derive(Debug, Deserialize, Validate, IntoParams, ToSchema)]
pub struct ListQuery {
    pub r_status: Option<i32>,
    #[validate(length(max = 80))]
    pub keyword: Option<String>,
}

#[utoipa::path(post, path = "/v1/admin/resumes", tag = "admin", security(("bearer" = [])), request_body = ListQuery, responses((status = 200, description = "ok")))]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
    ValidatedJson(q): ValidatedJson<ListQuery>,
) -> AppResult<ApiResponse<AdminPaged<AdminResumeRow>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(AdminPaged::from(
        admin_longtail_service::list_resumes(&state, q.r_status, q.keyword.as_deref(), page).await?,
    )))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SetStatusForm {
    #[validate(range(min = 1))]
    pub uid: u64,
    pub r_status: i32,
}

#[utoipa::path(post, path = "/v1/admin/resumes/status", tag = "admin", security(("bearer" = [])), request_body = SetStatusForm, responses((status = 200, description = "ok")))]
pub async fn set_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<SetStatusForm>,
) -> AppResult<ApiResponse> {
    user.require_admin()?;
    admin_longtail_service::set_resume_r_status(&state, &user, f.uid, f.r_status).await?;
    Ok(ApiResponse::message("ok"))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UidBody {
    #[validate(range(min = 1))]
    pub uid: u64,
}

/// PHP `phpyun_resume_work` columns: id,uid,eid,name,sdate,edate,department,title,content
#[utoipa::path(post, path = "/v1/admin/resumes/works", tag = "admin", security(("bearer" = [])), request_body = UidBody, responses((status = 200, description = "ok")))]
pub async fn list_works(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<UidBody>,
) -> AppResult<ApiResponse<Vec<Work>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_longtail_service::list_resume_works(&state, b.uid).await?,
    ))
}

/// PHP `phpyun_resume_edu` columns: id,uid,eid,name,sdate,edate,specialty,education
#[utoipa::path(post, path = "/v1/admin/resumes/edus", tag = "admin", security(("bearer" = [])), request_body = UidBody, responses((status = 200, description = "ok")))]
pub async fn list_edus(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<UidBody>,
) -> AppResult<ApiResponse<Vec<Edu>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_longtail_service::list_resume_edus(&state, b.uid).await?,
    ))
}

/// PHP `phpyun_resume_training` columns: id,uid,eid,name,sdate,edate,title,content
#[utoipa::path(post, path = "/v1/admin/resumes/trainings", tag = "admin", security(("bearer" = [])), request_body = UidBody, responses((status = 200, description = "ok")))]
pub async fn list_trainings(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<UidBody>,
) -> AppResult<ApiResponse<Vec<Training>>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_longtail_service::list_resume_trainings(&state, b.uid).await?,
    ))
}

#[utoipa::path(post, path = "/v1/admin/resumes/export", tag = "admin", security(("bearer" = [])), request_body = ListQuery, responses((status = 200, description = "ok")))]
pub async fn export_csv(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(q): ValidatedJson<ListQuery>,
) -> AppResult<ApiResponse<CsvExport>> {
    user.require_admin()?;
    Ok(ApiResponse::data(
        admin_longtail_service::export_resumes_csv(&state, q.r_status, q.keyword.as_deref())
            .await?,
    ))
}

/// PHP `users_resume::add_action`。
#[utoipa::path(post, path = "/v1/admin/resumes/php-add", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn php_add(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    let add = match body.get("add") {
        Some(serde_json::Value::Number(n)) => n.as_i64().unwrap_or(0),
        Some(serde_json::Value::String(s)) => s.trim().parse().unwrap_or(0),
        _ => 0,
    };
    let data = admin_longtail_service::resume_php_add(&state, &user, &body).await?;
    if add == 1 || add == 2 {
        return Ok(ApiResponse::data(data));
    }
    Ok(ApiResponse::message_data("admin_01318", data))
}

/// PHP `users_resume::editResume_action`.
#[utoipa::path(post, path = "/v1/admin/resumes/php-edit", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn php_edit(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    let uid = body
        .get("uid")
        .and_then(|v| v.as_u64().or_else(|| v.as_str().and_then(|s| s.parse().ok())))
        .unwrap_or(0);
    let eid = body
        .get("id")
        .and_then(|v| v.as_u64().or_else(|| v.as_str().and_then(|s| s.parse().ok())))
        .unwrap_or(0);
    Ok(ApiResponse::data(
        admin_longtail_service::resume_php_edit(&state, &user, uid, eid).await?,
    ))
}

/// PHP `users_resume::saveExpect_action`.
#[utoipa::path(post, path = "/v1/admin/resumes/php-save-expect", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn php_save_expect(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse<serde_json::Value>> {
    let eid = body
        .get("eid")
        .and_then(|v| v.as_u64().or_else(|| v.as_str().and_then(|s| s.parse().ok())))
        .unwrap_or(0);
    let data = admin_longtail_service::resume_save_expect(&state, &user, &body).await?;
    let key = if eid == 0 {
        "admin_model_00133"
    } else {
        "admin_model_00135"
    };
    Ok(ApiResponse::message_data(key, data))
}

/// PHP `users_resume::saveTag_action`.
#[utoipa::path(post, path = "/v1/admin/resumes/php-save-tag", tag = "admin", security(("bearer" = [])), responses((status = 200, description = "ok")))]
pub async fn php_save_tag(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(body): Json<serde_json::Value>,
) -> AppResult<ApiResponse> {
    admin_longtail_service::resume_save_tag(&state, &user, &body).await?;
    Ok(ApiResponse::message("admin_model_00137"))
}
