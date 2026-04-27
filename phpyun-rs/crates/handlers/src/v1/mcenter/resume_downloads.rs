//! Resume download record APIs.
//! - `POST /v1/mcenter/resume-downloads` — company downloads a job seeker's resume
//! - `GET /v1/mcenter/resume-downloads/outbox` — company views resumes it has downloaded
//! - `GET /v1/mcenter/resume-downloads/inbox` — job seeker views who has downloaded their resume

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::json;
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, Paged, Pagination, ValidatedJson};
use phpyun_services::resume_download_service;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use phpyun_core::utils::{fmt_dt};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/resume-downloads", post(download))
        .route("/resume-downloads/outbox", post(list_outbox))
        .route("/resume-downloads/inbox", post(list_inbox))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct DownloadForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub uid: u64,
}

/// Company downloads a resume
#[utoipa::path(
    post,
    path = "/v1/mcenter/resume-downloads",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = DownloadForm,
    responses((status = 200, description = "ok"))
)]
pub async fn download(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<DownloadForm>,
) -> AppResult<ApiJson<json::Value>> {
    resume_download_service::download(&state, &user, f.uid, &ip).await?;
    Ok(ApiJson(json::json!({ "ok": true })))
}


#[derive(Debug, Serialize, ToSchema)]
pub struct DownloadItem {
    pub id: u64,
    pub com_id: u64,
    pub uid: u64,
    pub eid: u64,
    pub datetime: i64,
    pub datetime_n: String,
}

impl From<phpyun_models::resume_download::entity::ResumeDownload> for DownloadItem {
    fn from(d: phpyun_models::resume_download::entity::ResumeDownload) -> Self {
        Self {
            id: d.id,
            com_id: d.com_id,
            uid: d.uid,
            eid: d.eid,
            datetime_n: fmt_dt(d.datetime),
            datetime: d.datetime,
        }
    }
}

/// Company view: resumes I have downloaded
#[utoipa::path(
    post,
    path = "/v1/mcenter/resume-downloads/outbox",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_outbox(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<DownloadItem>>> {
    let r = resume_download_service::list_mine_as_company(&state, &user, page).await?;
    Ok(ApiJson(Paged::from_listing(r.list, r.total, page)))
}

/// Job seeker view: who has downloaded me
#[utoipa::path(
    post,
    path = "/v1/mcenter/resume-downloads/inbox",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_inbox(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<DownloadItem>>> {
    let r = resume_download_service::list_mine_as_user(&state, &user, page).await?;
    Ok(ApiJson(Paged::from_listing(r.list, r.total, page)))
}
