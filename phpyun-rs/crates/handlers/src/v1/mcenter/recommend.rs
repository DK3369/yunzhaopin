//! Recommendations (matching PHPYun `finder.model.php`).

use axum::{
    extract::{Path, State},
    Router,
    routing::post,
};
use phpyun_core::i18n::{current_lang, t};
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, ValidatedJson};
use phpyun_services::{recommend_email_service, recommend_service};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/recommend/jobs", post(jobs))
        .route("/recommend/resumes", post(resumes))
        // Email-recommendation endpoint (PHP `wap/resume/resumeshare::index`).
        // `kind` is `job` or `resume`.
        .route("/recommend/email", post(send_email))
        .route("/recommend/email/quota", post(quota))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct RecQuery {
    #[serde(default = "recommend_service::default_limit")]
    #[validate(range(min = 1, max = 200))]
    pub limit: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RecJob {
    pub id: u64,
    pub uid: u64,
    pub name: String,
    pub com_name: Option<String>,
    pub city_id: i32,
    pub min_salary: i32,
    pub max_salary: i32,
    pub lastupdate: i64,
}

impl From<phpyun_models::job::entity::Job> for RecJob {
    fn from(j: phpyun_models::job::entity::Job) -> Self {
        Self {
            id: j.id,
            uid: j.uid,
            name: j.name,
            com_name: j.com_name,
            city_id: j.cityid,
            min_salary: j.minsalary,
            max_salary: j.maxsalary,
            lastupdate: j.lastupdate,
        }
    }
}

/// Recommend jobs based on my expectations + resume education
#[utoipa::path(
    post,
    path = "/v1/mcenter/recommend/jobs",
    tag = "mcenter",
    security(("bearer" = [])),
    params(RecQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn jobs(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(q): ValidatedJson<RecQuery>,
) -> AppResult<ApiJson<Vec<RecJob>>> {
    let list = recommend_service::recommend_jobs_for_me(&state, &user, q.limit).await?;
    Ok(ApiJson(list.into_iter().map(RecJob::from).collect()))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RecResume {
    pub uid: u64,
    pub display_name: String,
    pub sex: i32,
    pub education: i32,
    pub lastupdate: i64,
}

impl From<phpyun_models::resume::entity::Resume> for RecResume {
    fn from(r: phpyun_models::resume::entity::Resume) -> Self {
        // nametype=1 show; nametype=2 mask
        let display_name = match r.name.as_deref() {
            Some(n) if !n.is_empty() && r.nametype == 1 => n.to_string(),
            Some(n) if !n.is_empty() => {
                let mut s = String::new();
                for (i, ch) in n.chars().enumerate() {
                    if i == 0 { s.push(ch); } else { s.push('*'); }
                }
                s
            }
            _ => t("ui.resume.anonymous", current_lang()),
        };
        Self {
            uid: r.uid,
            display_name,
            sex: r.sex,
            education: r.education,
            lastupdate: r.lastupdate,
        }
    }
}

/// Company: recommend resumes based on the edu of the first active job under this company
#[utoipa::path(
    post,
    path = "/v1/mcenter/recommend/resumes",
    tag = "mcenter",
    security(("bearer" = [])),
    params(RecQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn resumes(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(q): ValidatedJson<RecQuery>,
) -> AppResult<ApiJson<Vec<RecResume>>> {
    let list = recommend_service::recommend_resumes_for_me(&state, &user, q.limit).await?;
    Ok(ApiJson(list.into_iter().map(RecResume::from).collect()))
}

// ==================== Email recommend ====================

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct EmailRecommendForm {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,
    #[validate(length(min = 1, max = 64), custom(function = "phpyun_core::validators::path_token"))]
    pub kind: String,
    #[validate(email)]
    pub email: String,
    /// Optional personal note from the sender; max 500 chars. PHP composes
    /// this into the email body via the `sendresume.htm` template.
    #[serde(default)]
    #[validate(length(max = 500))]
    pub message: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EmailRecommendResp {
    pub log_id: u64,
}

/// Recommend a job (`kind=job`) or resume (`kind=resume`) to a friend via
/// email. Counterpart of PHP `wap/resume/resumeshare::index_action`.
///
/// Throttling (matches PHP):
///   - Per-user-per-day cap (default 5)
///   - Min interval between sends (default 60s)
///
/// Captcha is enforced via the standard `captcha_cid + authcode` pair on the
/// caller — handled at the front-end gate, not duplicated here.
#[utoipa::path(post,
    path = "/v1/mcenter/recommend/email",
    tag = "mcenter",
    security(("bearer" = [])),
    params(
        ("kind" = String, Path, description = "job / resume"),
        ("id"   = u64,    Path, description = "job_id when kind=job; eid when kind=resume"),
    ),
    request_body = EmailRecommendForm,
    responses(
        (status = 200, description = "ok", body = EmailRecommendResp),
        (status = 400, description = "Invalid kind / email / target"),
        (status = 403, description = "Only employers can recommend"),
        (status = 429, description = "Per-day cap reached or interval too short"),
    )
)]
pub async fn send_email(State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<EmailRecommendForm>) -> AppResult<ApiJson<EmailRecommendResp>> {
    let kind = f.kind;
    let id = f.id;
    phpyun_core::validators::ensure_path_token(&kind)?;
    let input = recommend_email_service::RecommendInput {
        target_email: &f.email,
        message: f.message.as_deref(),
    };
    let r = match kind.as_str() {
        "job" => recommend_email_service::recommend_job(&state, &user, id, input).await?,
        "resume" => recommend_email_service::recommend_resume(&state, &user, id, input).await?,
        _ => {
            return Err(phpyun_core::AppError::param_invalid(format!(
                "kind: {kind}"
            )))
        }
    };
    Ok(ApiJson(EmailRecommendResp { log_id: r.log_id }))
}

// ==================== Quota preflight ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct QuotaView {
    /// 0 = ok, 1 = daily cap reached, 2 = interval too short
    pub status: i32,
    pub msg: String,
    pub used_today: u64,
    pub day_cap: u64,
    pub interval_secs: i64,
    pub seconds_remaining: i64,
}

impl From<phpyun_services::recommend_email_service::QuotaStatus> for QuotaView {
    fn from(q: phpyun_services::recommend_email_service::QuotaStatus) -> Self {
        Self {
            status: q.status,
            msg: q.msg,
            used_today: q.used_today,
            day_cap: q.day_cap,
            interval_secs: q.interval_secs,
            seconds_remaining: q.seconds_remaining,
        }
    }
}

/// Pre-flight check for email-recommendation quota. Counterpart of PHP
/// `ajax::ajax_recommend_interval_action`. Lets the front-end disable the
/// "send" button proactively (or display the cooldown timer) without
/// trying-and-being-rejected.
#[utoipa::path(
    post,
    path = "/v1/mcenter/recommend/email/quota",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = QuotaView))
)]
pub async fn quota(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiJson<QuotaView>> {
    let q = recommend_email_service::check_quota(&state, &user).await?;
    Ok(ApiJson(QuotaView::from(q)))
}
