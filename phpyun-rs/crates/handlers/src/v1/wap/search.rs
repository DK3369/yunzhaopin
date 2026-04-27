//! Global search (aligned with PHPYun `wap/search`).
//!
//! Reuses the rich Summary types from each domain: JobSummary (34) / CompanySummary (18) / ArticleSummary (29) / QuestionSummary (19),
//! so the search page and list pages keep the same field shapes.

use axum::{
    extract::State,
    Router,
    routing::{get, post},
};
use phpyun_core::{ApiJson, AppResult, AppState, ValidatedJson};
use phpyun_services::search_service;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new().route("/search", post(search))
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
pub struct SearchQuery {
    /// Free-text search; capped at 100 chars and required (empty `kw`
    /// returns 400 instead of running an unbounded LIKE).
    #[validate(length(min = 1, max = 100))]
    pub kw: String,
    /// all / job / company / article / question
    #[serde(default = "default_scope")]
    #[validate(length(min = 1, max = 16))]
    pub scope: String,
    #[serde(default = "default_did")]
    #[validate(range(max = 9_999_999))]
    pub did: u32,
}
fn default_scope() -> String {
    "all".to_string()
}
fn default_did() -> u32 {
    1
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SearchData {
    /// Reuses wap::jobs::JobSummary (34 fields, with dictionary translations)
    pub jobs: Vec<super::jobs::JobSummary>,
    /// Reuses wap::companies::CompanySummary (18 fields, with dictionary translations)
    pub companies: Vec<super::companies::CompanySummary>,
    /// Reuses wap::articles::ArticleSummary (29 fields, with class names / CDN / formatted timestamps)
    pub articles: Vec<super::articles::ArticleSummary>,
    /// Reuses wap::qna::QuestionSummary (19 fields)
    pub questions: Vec<super::qna::QuestionSummary>,
}

/// Global search
#[utoipa::path(post, path = "/v1/wap/search", tag = "wap", params(SearchQuery), responses((status = 200, description = "ok", body = SearchData)))]
pub async fn search(
    State(state): State<AppState>,
    phpyun_core::MaybeUser(user): phpyun_core::MaybeUser,
    ValidatedJson(q): ValidatedJson<SearchQuery>,
) -> AppResult<ApiJson<SearchData>> {
    let r = search_service::global_search(&state, &q.kw, &q.scope, q.did).await?;
    let dicts = phpyun_services::dict_service::get(&state).await?;
    let now = phpyun_core::clock::now_ts();
    let job_ids: Vec<u64> = r.jobs.iter().map(|j| j.id).collect();
    let fav_set =
        phpyun_services::collect_service::favorited_set(&state, user.as_ref().map(|u| u.uid), &job_ids).await;
    Ok(ApiJson(SearchData {
        jobs: r
            .jobs
            .into_iter()
            .map(|j| {
                let fav = fav_set.contains(&j.id);
                crate::v1::wap::jobs::job_summary_from_dict_fav(j, &dicts, now, fav)
            })
            .collect(),
        companies: r
            .companies
            .into_iter()
            .map(|c| super::companies::company_summary_from_dict(c, &dicts))
            .collect(),
        articles: r
            .articles
            .into_iter()
            .map(|a| super::articles::ArticleSummary::from_with_ctx(a, &state))
            .collect(),
        questions: r
            .questions
            .into_iter()
            .map(|q| super::qna::QuestionSummary::from_with_ctx(q, &state, &dicts))
            .collect(),
    }))
}
