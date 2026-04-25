//! Member center - resume (usertype=1 job seeker only).

use axum::{
    extract::State,
    routing::{get, post},
    Router,
};
use phpyun_core::json;
use phpyun_core::{
    ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson,
};
use phpyun_services::resume_service::{self, ResumeUpdateInput};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/resume", get(get_mine).post(update_mine))
        .route("/resume/status", post(update_status))
        .route("/resume/refresh", post(refresh))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ResumeData {
    pub uid: u64,
    pub name: Option<String>,
    pub nametype: i32,
    pub sex: i32,
    pub birthday: Option<String>,
    pub marriage: i32,
    pub education: i32,
    pub telphone: Option<String>,
    pub email: Option<String>,
    pub photo: Option<String>,
    pub phototype: i32,
    pub status: i32,
    pub r_status: i32,
    pub def_job: i32,
    pub lastupdate: i64,
}

/// Get the current job seeker's resume
#[utoipa::path(
    get,
    path = "/v1/mcenter/resume",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = ResumeData))
)]
pub async fn get_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<ResumeData>> {
    let r = resume_service::get_mine(&state, &user).await?;
    Ok(ApiJson(ResumeData {
        uid: r.uid,
        name: r.name,
        nametype: r.nametype,
        sex: r.sex,
        birthday: r.birthday,
        marriage: r.marriage,
        education: r.education,
        telphone: r.telphone,
        email: r.email,
        photo: r.photo,
        phototype: r.phototype,
        status: r.status,
        r_status: r.r_status,
        def_job: r.def_job,
        lastupdate: r.lastupdate,
    }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateResumeForm {
    #[validate(length(min = 2, max = 25))]
    pub name: Option<String>,
    #[validate(range(min = 1, max = 2))]
    pub nametype: Option<i32>,
    #[validate(range(min = 0, max = 2))]
    pub sex: Option<i32>,
    #[validate(length(min = 8, max = 10))] // YYYY-MM-DD
    pub birthday: Option<String>,
    #[validate(range(min = 0, max = 2))]
    pub marriage: Option<i32>,
    #[validate(range(min = 0))]
    pub education: Option<i32>,
    #[validate(length(min = 5, max = 20))]
    pub telphone: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(max = 255))]
    pub photo: Option<String>,
}

/// Update the resume main table
#[utoipa::path(
    post,
    path = "/v1/mcenter/resume",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = UpdateResumeForm,
    responses((status = 200, description = "ok"))
)]
pub async fn update_mine(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<UpdateResumeForm>,
) -> AppResult<ApiJson<json::Value>> {
    resume_service::update_mine(
        &state,
        &user,
        ResumeUpdateInput {
            name: f.name.as_deref(),
            nametype: f.nametype,
            sex: f.sex,
            birthday: f.birthday.as_deref(),
            marriage: f.marriage,
            education: f.education,
            telphone: f.telphone.as_deref(),
            email: f.email.as_deref(),
            photo: f.photo.as_deref(),
        },
        &ip,
    )
    .await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateStatusForm {
    /// 1=public, 2=hidden, 3=visible only to applied companies
    #[validate(range(min = 1, max = 3))]
    pub status: i32,
}

/// Change resume visibility status
#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/status",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = UpdateStatusForm,
    responses((status = 200, description = "ok"))
)]
pub async fn update_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<UpdateStatusForm>,
) -> AppResult<ApiJson<json::Value>> {
    resume_service::set_status(&state, &user, f.status, &ip).await?;
    Ok(ApiJson(json::json!({ "ok": true, "status": f.status })))
}

/// Refresh my resume (bump lastupdate to rank higher in public search).
/// **Rate limit**: once every 5 minutes.
#[utoipa::path(
    post,
    path = "/v1/mcenter/resume/refresh",
    tag = "mcenter",
    security(("bearer" = [])),
    responses(
        (status = 200, description = "ok"),
        (status = 429, description = "Refreshed too frequently"),
    )
)]
pub async fn refresh(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
) -> AppResult<ApiJson<json::Value>> {
    let ts = resume_service::refresh_mine(&state, &user, &ip).await?;
    Ok(ApiJson(json::json!({ "ok": true, "lastupdate": ts })))
}
