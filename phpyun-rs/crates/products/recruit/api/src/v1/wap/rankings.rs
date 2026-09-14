//! PC ranking page (PHP `index/top.htm`) — one public aggregation.

use axum::{extract::State, routing::get, Router};
use phpyun_core::i18n::{current_lang, t};
use phpyun_core::{ApiResponse, AppResult, AppState, ValidatedJsonOrQuery};
use phpyun_services::ranking_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub const GET_ALLOWED_PATHS: &[&str] = &["/v1/wap/rankings"];

pub fn routes() -> Router<AppState> {
    Router::new().route("/rankings", get(rankings).post(rankings))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct RankingsQuery {
    #[serde(default = "default_did")]
    #[validate(range(max = 9_999_999))]
    pub did: u32,
}
fn default_did() -> u32 {
    0
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RankKeyword {
    pub keyword: String,
    pub hits: i32,
    /// Same as `hits` (PHP `num`).
    pub num: i32,
    /// PHP `phpyun_hot_key.type` as string.
    #[serde(rename = "type")]
    pub type_id: String,
    pub type_name: String,
    /// Module path (`/jobs`, `/once`, …); client adds `?keyword=`.
    pub to: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RankingsData {
    pub rec_jobs: Vec<super::jobs::JobSummary>,
    pub companies: Vec<super::companies::CompanySummary>,
    pub latest_jobs: Vec<super::jobs::JobSummary>,
    pub resumes: Vec<super::resumes::ResumeSummary>,
    pub keywords: Vec<RankKeyword>,
    pub articles: Vec<super::articles::ArticleSummary>,
    pub urgent_jobs: Vec<super::jobs::JobSummary>,
}

fn keyword_meta(scope: &str) -> (&'static str, &'static str) {
    match scope {
        "1" => ("/once", "errors.wap_js_00130"),
        "2" => ("/parts", "errors.wap_user_00220"),
        "3" => ("/jobs", "errors.default_00246"),
        "4" => ("/companies", "errors.default_00114"),
        "5" => ("/resumes", "errors.default_00312"),
        "12" => ("/questions", "errors.wap_user_00223"),
        "13" => ("/tiny", "errors.wap_js_00066"),
        _ => ("/jobs", "errors.default_00246"),
    }
}

/// PHP `{yun:}key{/yun}` ranking board.
#[utoipa::path(
    post,
    path = "/v1/wap/rankings",
    tag = "wap",
    params(RankingsQuery),
    responses((status = 200, description = "ok", body = RankingsData))
)]
pub async fn rankings(
    State(state): State<AppState>,
    ValidatedJsonOrQuery(q): ValidatedJsonOrQuery<RankingsQuery>,
) -> AppResult<ApiResponse<RankingsData>> {
    let p = ranking_service::rankings(&state, q.did).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    let now = phpyun_core::clock::now_ts();
    let lang = current_lang();
    let p = (*p).clone();

    let rec_jobs = p
        .rec_jobs
        .into_iter()
        .map(|j| super::jobs::job_summary_from_dict(j, &dicts, now))
        .collect();
    let latest_jobs = p
        .latest_jobs
        .into_iter()
        .map(|j| super::jobs::job_summary_from_dict(j, &dicts, now))
        .collect();
    let urgent_jobs = p
        .urgent_jobs
        .into_iter()
        .map(|j| super::jobs::job_summary_from_dict(j, &dicts, now))
        .collect();
    let companies = p
        .companies
        .into_iter()
        .map(|c| super::companies::company_summary_from_dict(c, &dicts))
        .collect();
    let show_cfg = super::resumes::load_resume_show_cfg(&state).await;
    let mut resumes: Vec<super::resumes::ResumeSummary> = p
        .resumes
        .into_iter()
        .map(|r| super::resumes::ResumeSummary::from_with_dict(r, &state, &dicts, &show_cfg))
        .collect();
    super::resumes::attach_expect_fields(&state, &dicts, &mut resumes).await;
    let articles = p
        .articles
        .into_iter()
        .map(|a| super::articles::ArticleSummary::from_with_ctx(a, &state))
        .collect();
    let keywords = p
        .keywords
        .into_iter()
        .map(|h| {
            let (to, name_key) = keyword_meta(&h.scope);
            RankKeyword {
                keyword: h.keyword,
                hits: h.hits,
                num: h.hits,
                type_id: h.scope,
                type_name: t(name_key, lang),
                to: to.to_string(),
            }
        })
        .collect();

    Ok(ApiResponse::data(RankingsData {
        rec_jobs,
        companies,
        latest_jobs,
        resumes,
        keywords,
        articles,
        urgent_jobs,
    }))
}
