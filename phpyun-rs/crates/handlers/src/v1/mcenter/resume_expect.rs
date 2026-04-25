//! Job expectation CRUD (usertype=1).

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Router,
};
use phpyun_core::json;
use phpyun_core::{
    ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson,
};
use phpyun_models::resume::expect::ExpectInput;
use phpyun_services::resume_children_service::expect_svc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/resume/expects", get(list).post(create))
        .route("/resume/expects/{id}", post(update))
}

/// Job expectation item — **reuses** `wap::resumes::ResumeExpectItem` (14 fields, including 3 dictionary translations + time formatting).
pub type ExpectItem = crate::v1::wap::resumes::ResumeExpectItem;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ExpectForm {
    #[validate(length(max = 50))]
    pub name: Option<String>,
    pub job_classid: i64,
    pub city_classid: i64,
    pub salary: i32,
    /// Soft delete: pass `2` to delete. Other values or None will trigger an update.
    #[serde(default)]
    pub status: Option<i32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreatedId {
    pub id: u64,
}

/// List job expectations
#[utoipa::path(
    get,
    path = "/v1/mcenter/resume/expects",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<Vec<ExpectItem>>> {
    let list = expect_svc::list(&state, &user).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    Ok(ApiJson(
        list.into_iter()
            .map(|e| ExpectItem::from_with_dict(e, &dicts))
            .collect(),
    ))
}

/// Create a new job expectation
#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/expects",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = ExpectForm,
    responses((status = 200, description = "ok", body = CreatedId))
)]
pub async fn create(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<ExpectForm>,
) -> AppResult<ApiJson<CreatedId>> {
    let id = expect_svc::create(
        &state,
        &user,
        ExpectInput {
            name: f.name.as_deref(),
            job_classid: f.job_classid,
            city_classid: f.city_classid,
            salary: f.salary,
        },
        &ip,
    )
    .await?;
    Ok(ApiJson(CreatedId { id }))
}

/// Update or soft-delete a job expectation (body with `"status":2` means delete).
#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/expects/{id}",
    tag = "mcenter",
    security(("bearer" = [])),
    params(("id" = u64, Path)),
    request_body = ExpectForm,
    responses((status = 200, description = "ok"))
)]
pub async fn update(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    Path(id): Path<u64>,
    ValidatedJson(f): ValidatedJson<ExpectForm>,
) -> AppResult<ApiJson<json::Value>> {
    if f.status == Some(2) {
        expect_svc::delete(&state, &user, id, &ip).await?;
        return Ok(ApiJson(json::json!({ "ok": true, "deleted": true })));
    }
    expect_svc::update(
        &state,
        &user,
        id,
        ExpectInput {
            name: f.name.as_deref(),
            job_classid: f.job_classid,
            city_classid: f.city_classid,
            salary: f.salary,
        },
        &ip,
    )
    .await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}
