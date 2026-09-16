//! PC ranking page aggregation (PHP `index/top.htm`).
//!
//! Seven public sections, each read from a replica and isolated so one empty
//! table cannot take down the page. Cached ~60s keyed by `did`.

use phpyun_core::cache::TieredCache;
use phpyun_core::{AppResult, AppState};
use phpyun_models::article::{entity::Article, repo as article_repo, repo::ArticleFilter};
use phpyun_models::company::{entity::Company, repo as company_repo, repo::CompanyFilter};
use phpyun_models::hot_search::{entity::HotSearch, repo as hot_search_repo};
use phpyun_models::job::{entity::Job, repo as job_repo, repo::JobFilter};
use phpyun_models::resume::{entity::Resume, repo as resume_repo, repo::ResumeFilter};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock};
use std::time::Duration;

const LIMIT: u64 = 10;
const TTL: Duration = Duration::from_secs(60);

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RankingPayload {
    pub rec_jobs: Vec<Job>,
    pub companies: Vec<Company>,
    pub latest_jobs: Vec<Job>,
    pub resumes: Vec<Resume>,
    pub keywords: Vec<HotSearch>,
    pub articles: Vec<Article>,
    pub urgent_jobs: Vec<Job>,
}

static CACHE: OnceLock<TieredCache<RankingPayload>> = OnceLock::new();

fn cache() -> &'static TieredCache<RankingPayload> {
    CACHE.get_or_init(|| TieredCache::new(32, TTL))
}

pub async fn invalidate(did: u32) {
    cache().invalidate_prefix_local();
    let _ = did;
}

pub async fn invalidate_all() {
    cache().invalidate_prefix_local();
}

pub async fn rankings(state: &AppState, did: u32) -> AppResult<Arc<RankingPayload>> {
    let st = state.clone();
    cache()
        .get_or_load(
            &state.redis,
            format!("ranking:{did}"),
            TTL,
            "ranking",
            move || async move { load_rankings(&st, did).await },
        )
        .await
}

async fn load_rankings(st: &AppState, did: u32) -> AppResult<RankingPayload> {
            let db = st.db.reader();
            let now = phpyun_core::clock::now_ts();
            let rec_filter = JobFilter {
                did,
                rec: true,
                order: Some("lastdate"),
                ..Default::default()
            };
            let latest_filter = JobFilter {
                did,
                order: Some("lastdate"),
                ..Default::default()
            };
            let urgent_filter = JobFilter {
                did,
                urgent: true,
                order: Some("lastdate"),
                ..Default::default()
            };
            let com_filter = CompanyFilter {
                did,
                order: Some("lastupdate"),
                ..Default::default()
            };
            let resume_filter = ResumeFilter {
                did,
                ..Default::default()
            };
            let art_filter = ArticleFilter {
                did,
                order: Some("hits"),
                ..Default::default()
            };

            let (rec_r, com_r, latest_r, resume_r, key_r, art_r, urgent_r) = tokio::join!(
                job_repo::list_public(db, &rec_filter, 0, LIMIT, now),
                company_repo::list_public(db, &com_filter, 0, LIMIT, now),
                job_repo::list_public(db, &latest_filter, 0, LIMIT, now),
                resume_repo::list_public(db, &resume_filter, 0, LIMIT),
                hot_search_repo::top_checked(db, LIMIT),
                article_repo::list_public(db, &art_filter, 0, LIMIT),
                job_repo::list_public(db, &urgent_filter, 0, LIMIT, now),
            );

            Ok(RankingPayload {
                rec_jobs: rec_r.unwrap_or_default(),
                companies: com_r.unwrap_or_default(),
                latest_jobs: latest_r.unwrap_or_default(),
                resumes: resume_r.unwrap_or_default(),
                keywords: key_r.unwrap_or_default(),
                articles: art_r.unwrap_or_default(),
                urgent_jobs: urgent_r.unwrap_or_default(),
            })
}
