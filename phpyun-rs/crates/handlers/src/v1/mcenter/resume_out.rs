//! Resume outbox (`member/user/resumeout`) — jobseekers manually send their resume to external mailboxes.
//!
//! REST mapping:
//! - `GET    /v1/mcenter/resume-outbox`      my outbox records
//! - `POST   /v1/mcenter/resume-outbox`      send once
//! - `DELETE /v1/mcenter/resume-outbox`      batch delete

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{json, ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, Paged, Pagination, ValidatedJson};
use phpyun_services::resume_out_service::{self, Limits, OutInput};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use phpyun_core::dto::{CreatedId, IdsBody};
use phpyun_core::utils::{fmt_dt};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/resume-outbox", post(send))
        .route("/resume-outbox/list", post(list))
        .route("/resume-outbox/delete", post(delete_many))
}


#[derive(Debug, Serialize, ToSchema)]
pub struct OutView {
    pub id: u64,
    pub uid: u64,
    pub resume: u64,
    pub email: String,
    pub com_name: String,
    pub job_name: String,
    pub resume_name: Option<String>,
    pub addtime: i64,
    pub addtime_n: String,
}

impl From<phpyun_models::resume_out::entity::ResumeOut> for OutView {
    fn from(o: phpyun_models::resume_out::entity::ResumeOut) -> Self {
        Self {
            id: o.id,
            uid: o.uid,
            resume: o.resume,
            email: o.email,
            com_name: o.comname,
            job_name: o.jobname,
            resume_name: o.resumename,
            addtime_n: fmt_dt(o.addtime),
            addtime: o.addtime,
        }
    }
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/resume-outbox/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]pub async fn list(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiJson<Paged<OutView>>> {
    let r = resume_out_service::list_mine(&state, &user, page).await?;
    Ok(ApiJson(Paged::from_listing(r.list, r.total, page)))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SendForm {
    #[validate(range(min = 1))]
    pub resume_id: u64,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, max = 128))]
    pub com_name: String,
    #[validate(length(min = 1, max = 128))]
    pub job_name: String,
    #[validate(length(max = 128))]
    pub resume_name: Option<String>,
    /// Aligns with PHP `sy_resumeout_day_num` (0 = disabled)
    #[validate(range(max = 1000))]
    #[serde(default)]
    pub daily_max: u32,
    /// Aligns with PHP `sy_resumeout_interval` (0 = unlimited; seconds)
    #[serde(default)]
    #[validate(range(min = 0i64, max = 4_102_444_800i64))]
    pub interval_secs: i64,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/resume-outbox",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = SendForm,
    responses(
        (status = 200, description = "ok", body = CreatedId),
        (status = 400, description = "daily_max=0 disabled / invalid email"),
        (status = 429, description = "Daily quota exhausted / interval too short"),
    )
)]
pub async fn send(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<SendForm>,
) -> AppResult<ApiJson<CreatedId>> {
    let input = OutInput {
        resume_id: f.resume_id,
        email: &f.email,
        com_name: &f.com_name,
        job_name: &f.job_name,
        resume_name: f.resume_name.as_deref(),
    };
    let limits = Limits {
        daily_max: f.daily_max,
        interval_secs: f.interval_secs,
    };
    let r = resume_out_service::send(&state, &user, &input, &limits, &ip).await?;
    Ok(ApiJson(CreatedId { id: r.id }))
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/resume-outbox/delete",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IdsBody,
    responses((status = 200, description = "ok"))
)]
pub async fn delete_many(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<IdsBody>,
) -> AppResult<ApiJson<json::Value>> {
    let n = resume_out_service::delete_mine(&state, &user, &b.ids).await?;
    Ok(ApiJson(json::json!({ "deleted": n })))
}
