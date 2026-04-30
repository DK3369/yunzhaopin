//! Home-page aggregation (aligned with PHPYun `wap/index::index`): returns the sections the frontend needs above the fold in a single call.
//!
//! - Latest announcements (5)
//! - Hot jobs (8, sorted by latest update)
//! - Featured companies (8)
//! - Latest articles (5)
//! - Hot search keywords (10)
//!
//! Every section reads from a read replica and is fired concurrently; a failure in one section does not break the whole endpoint (the failed section returns an empty array).
//!
//! The whole payload is wrapped in a 60-second TTL cache keyed by `did` to keep the
//! home page cheap under load — fresh content still appears within a minute.

use phpyun_core::cache::SimpleCache;
use phpyun_core::{AppResult, AppState};
use phpyun_models::announcement::{entity::Announcement, repo as ann_repo};
use phpyun_models::article::{entity::Article, repo as article_repo, repo::ArticleFilter};
use phpyun_models::company::{entity::Company, repo as company_repo, repo::CompanyFilter};
use phpyun_models::hot_search::{entity::HotSearch, repo as hot_search_repo};
use phpyun_models::job::{entity::Job, repo as job_repo, repo::JobFilter};
use std::sync::Arc;

#[derive(Debug, Default, Clone)]
pub struct HomePayload {
    pub announcements: Vec<Announcement>,
    pub hot_jobs: Vec<Job>,
    pub rec_companies: Vec<Company>,
    pub new_articles: Vec<Article>,
    pub hot_keywords: Vec<HotSearch>,
}

const HOME_TTL_SECS: u64 = 60;

static HOME_CACHE: std::sync::OnceLock<SimpleCache<u32, HomePayload>> = std::sync::OnceLock::new();

fn home_cache() -> &'static SimpleCache<u32, HomePayload> {
    HOME_CACHE.get_or_init(|| SimpleCache::new(32, std::time::Duration::from_secs(HOME_TTL_SECS)))
}

/// Manual invalidation hook — call after a writer publishes an announcement / article
/// or wants to force-refresh the home page early.
pub async fn invalidate(did: u32) {
    let key = if did == 0 { 1 } else { did };
    home_cache().invalidate(&key).await;
}

pub async fn invalidate_all() {
    home_cache().invalidate_all();
}

pub async fn home(state: &AppState, did: u32) -> AppResult<Arc<HomePayload>> {
    let did = if did == 0 { 1 } else { did };
    let cache = home_cache();
    let st = state.clone();
    cache
        .get_or_load(did, move || async move {
            let db = st.db.reader();
            let now = phpyun_core::clock::now_ts();
            let job_filter = JobFilter { did, ..Default::default() };
            let com_filter = CompanyFilter { did, ..Default::default() };
            let art_filter = ArticleFilter { did, ..Default::default() };

            let (ann_r, jobs_r, coms_r, art_r, hot_r) = tokio::join!(
                ann_repo::list_published(db, 0, 5),
                job_repo::list_public(db, &job_filter, 0, 8, now),
                company_repo::list_public(db, &com_filter, 0, 8, now),
                article_repo::list_public(db, &art_filter, 0, 5),
                hot_search_repo::top(db, "job", 10),
            );

            Ok(HomePayload {
                announcements: ann_r.unwrap_or_default(),
                hot_jobs: jobs_r.unwrap_or_default(),
                rec_companies: coms_r.unwrap_or_default(),
                new_articles: art_r.unwrap_or_default(),
                hot_keywords: hot_r.unwrap_or_default(),
            })
        })
        .await
}
