//! Recommendations (matching PHPYun `finder.model.php`).

use axum::{
    extract::{Query, State},
    routing::get,
    Router,
};
use phpyun_core::i18n::{current_lang, t};
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser};
use phpyun_services::recommend_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/recommend/jobs", get(jobs))
        .route("/recommend/resumes", get(resumes))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct RecQuery {
    #[serde(default = "recommend_service::default_limit")]
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
    get,
    path = "/v1/mcenter/recommend/jobs",
    tag = "mcenter",
    security(("bearer" = [])),
    params(RecQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn jobs(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Query(q): Query<RecQuery>,
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
    get,
    path = "/v1/mcenter/recommend/resumes",
    tag = "mcenter",
    security(("bearer" = [])),
    params(RecQuery),
    responses((status = 200, description = "ok"))
)]
pub async fn resumes(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Query(q): Query<RecQuery>,
) -> AppResult<ApiJson<Vec<RecResume>>> {
    let list = recommend_service::recommend_resumes_for_me(&state, &user, q.limit).await?;
    Ok(ApiJson(list.into_iter().map(RecResume::from).collect()))
}
