//! Global search (aligned with PHPYun `wap/search`).
//!
//! scope="all"  -> top N from each section; scope=job|company|resume|article|question -> only that section.
//!
//! Side effect: also bumps a hot-search keyword for the job/company/resume scopes (fire-and-forget).

use phpyun_core::{AppResult, AppState};
use phpyun_models::article::{entity::Article, repo as article_repo, repo::ArticleFilter};
use phpyun_models::company::{entity::Company, repo as company_repo, repo::CompanyFilter};
use phpyun_models::job::{entity::Job, repo as job_repo, repo::JobFilter};
use phpyun_models::qna::{entity::Question, repo as qna_repo};

use crate::hot_search_service;

const DEFAULT_LIMIT: u64 = 10;

#[derive(Debug, Default)]
pub struct SearchResult {
    pub jobs: Vec<Job>,
    pub companies: Vec<Company>,
    pub articles: Vec<Article>,
    pub questions: Vec<Question>,
}

pub async fn global_search(
    state: &AppState,
    keyword: &str,
    scope: &str,
    did: u32,
) -> AppResult<SearchResult> {
    let db = state.db.reader();
    let now = phpyun_core::clock::now_ts();
    let did = if did == 0 { 1 } else { did };
    let kw_trim = keyword.trim();
    if kw_trim.is_empty() {
        return Ok(SearchResult::default());
    }

    // Bump hot-search keyword (matched against the scope)
    if scope == "all" || scope == "job" {
        hot_search_service::bump_async(state, "job", kw_trim.to_string());
    }
    if scope == "all" || scope == "company" {
        hot_search_service::bump_async(state, "company", kw_trim.to_string());
    }

    let want_job = scope == "all" || scope == "job";
    let want_com = scope == "all" || scope == "company";
    let want_art = scope == "all" || scope == "article";
    let want_q = scope == "all" || scope == "question";

    let job_f = JobFilter {
        did,
        keyword: if want_job { Some(kw_trim) } else { None },
        ..Default::default()
    };
    let com_f = CompanyFilter {
        did,
        keyword: if want_com { Some(kw_trim) } else { None },
        ..Default::default()
    };
    let art_f = ArticleFilter {
        did,
        keyword: if want_art { Some(kw_trim) } else { None },
        ..Default::default()
    };

    let q_filter = qna_repo::QuestionFilter {
        keyword: if want_q { Some(kw_trim) } else { None },
        category_id: None,
        order: qna_repo::QuestionOrder::Hot,
    };

    let jobs_fut = async {
        if want_job {
            job_repo::list_public(db, &job_f, 0, DEFAULT_LIMIT, now).await
        } else {
            Ok(vec![])
        }
    };
    let coms_fut = async {
        if want_com {
            company_repo::list_public(db, &com_f, 0, DEFAULT_LIMIT, now).await
        } else {
            Ok(vec![])
        }
    };
    let arts_fut = async {
        if want_art {
            article_repo::list_public(db, &art_f, 0, DEFAULT_LIMIT).await
        } else {
            Ok(vec![])
        }
    };
    let qs_fut = async {
        if want_q {
            qna_repo::list_questions(db, &q_filter, 0, DEFAULT_LIMIT).await
        } else {
            Ok(vec![])
        }
    };

    let (jobs_r, coms_r, arts_r, qs_r) = tokio::join!(jobs_fut, coms_fut, arts_fut, qs_fut);
    Ok(SearchResult {
        jobs: jobs_r.unwrap_or_default(),
        companies: coms_r.unwrap_or_default(),
        articles: arts_r.unwrap_or_default(),
        questions: qs_r.unwrap_or_default(),
    })
}
