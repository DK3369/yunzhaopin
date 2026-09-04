//! Home-page aggregation (aligned with PHPYun `wap/index::index`): returns the sections the frontend needs above the fold in a single call.
//!
//! - Latest announcements (5)
//! - Hot jobs (8, sorted by latest update)
//! - Famous companies from `phpyun_hotjob` (12; not ordinary `list_public`)
//! - Latest articles (14), featured with cover (2), homepage-recommend `indextj` (10)
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
use phpyun_models::company::{entity::Company, repo as company_repo};
use phpyun_models::hot_search::{entity::HotSearch, repo as hot_search_repo};
use phpyun_models::job::{entity::Job, repo as job_repo, repo::JobFilter};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Default, Clone)]
pub struct HomePayload {
    pub announcements: Vec<Announcement>,
    pub hot_jobs: Vec<Job>,
    pub rec_jobs: Vec<Job>,
    pub latest_jobs: Vec<Job>,
    pub urgent_jobs: Vec<Job>,
    pub bid_jobs: Vec<Job>,
    pub rec_companies: Vec<Company>,
    pub rec_hot_pics: HashMap<u64, String>,
    pub new_articles: Vec<Article>,
    pub featured_articles: Vec<Article>,
    pub hot_articles: Vec<Article>,
    pub hot_keywords: Vec<HotSearch>,
}

const HOME_TTL_SECS: u64 = 60;

static HOME_CACHE: std::sync::OnceLock<SimpleCache<u32, HomePayload>> = std::sync::OnceLock::new();

fn home_cache() -> &'static SimpleCache<u32, HomePayload> {
    HOME_CACHE.get_or_init(|| SimpleCache::new(32, std::time::Duration::from_secs(HOME_TTL_SECS)))
}

/// Manual invalidation hook — call after a writer publishes an announcement / article
/// or wants to force-refresh the home page early.
///
/// `did=0` is a real sub-site id (same as `/v1/wap/jobs`), not an alias for `1`.
pub async fn invalidate(did: u32) {
    home_cache().invalidate(&did).await;
}

pub async fn invalidate_all() {
    home_cache().invalidate_all();
}

pub async fn home(state: &AppState, did: u32) -> AppResult<Arc<HomePayload>> {
    let cache = home_cache();
    let st = state.clone();
    cache
        .get_or_load(did, move || async move {
            let db = st.db.reader();
            let now = phpyun_core::clock::now_ts();
            let job_filter = JobFilter {
                did,
                ..Default::default()
            };
            let art_filter = ArticleFilter {
                did,
                ..Default::default()
            };
            let feat_filter = ArticleFilter {
                did,
                cover_only: true,
                describe_tag: Some("t"),
                ..Default::default()
            };
            let hot_art_filter = ArticleFilter {
                did,
                describe_tag: Some("indextj"),
                ..Default::default()
            };

            let sort_mode = phpyun_models::site_setting::repo::find(st.db.reader(), "hotcom_top")
                .await
                .ok()
                .flatten()
                .and_then(|s| s.value.trim().parse().ok())
                .unwrap_or(0);
            let site = if did > 0 {
                phpyun_models::domain::repo::find_by_id(db, u64::from(did))
                    .await
                    .ok()
                    .flatten()
            } else {
                None
            };
            let rec_filter = JobFilter {
                did,
                rec: true,
                ..Default::default()
            };
            let latest_filter = JobFilter {
                did,
                ..Default::default()
            };
            let urgent_filter = JobFilter {
                did,
                urgent: true,
                ..Default::default()
            };
            let bid_filter = JobFilter {
                did,
                bid: true,
                ..Default::default()
            };
            let (ann_r, jobs_r, rec_jobs_r, latest_jobs_r, urgent_jobs_r, bid_jobs_r, hot_com_r, art_r, feat_r, hot_art_r, hot_r) = tokio::join!(
                ann_repo::list_published(db, did, 0, 5),
                job_repo::list_public(db, &job_filter, 0, 8, now),
                job_repo::list_public(db, &rec_filter, 0, 32, now),
                job_repo::list_public(db, &latest_filter, 0, 32, now),
                job_repo::list_public(db, &urgent_filter, 0, 32, now),
                job_repo::list_public(db, &bid_filter, 0, 32, now),
                company_repo::list_hot(db, sort_mode, 12, now, site.as_ref()),
                article_repo::list_public(db, &art_filter, 0, 14),
                article_repo::list_public(db, &feat_filter, 0, 2),
                article_repo::list_public(db, &hot_art_filter, 0, 10),
                hot_search_repo::top(db, "job", 10),
            );

            let hot_coms = hot_com_r.unwrap_or_default();
            let rec_hot_pics: HashMap<u64, String> = hot_coms
                .iter()
                .filter_map(|h| {
                    let pic = h.hot_pic.as_deref().unwrap_or("").trim();
                    if pic.is_empty() {
                        None
                    } else {
                        Some((h.uid, pic.to_string()))
                    }
                })
                .collect();
            let uids: Vec<u64> = hot_coms.iter().map(|h| h.uid).collect();
            let rec_companies = if uids.is_empty() {
                Vec::new()
            } else {
                company_repo::list_by_uids(st.db.reader(), &uids)
                    .await
                    .unwrap_or_default()
            };

            Ok(HomePayload {
                announcements: ann_r.unwrap_or_default(),
                hot_jobs: jobs_r.unwrap_or_default(),
                rec_jobs: rec_jobs_r.unwrap_or_default(),
                latest_jobs: latest_jobs_r.unwrap_or_default(),
                urgent_jobs: urgent_jobs_r.unwrap_or_default(),
                bid_jobs: bid_jobs_r.unwrap_or_default(),
                rec_companies,
                rec_hot_pics,
                new_articles: art_r.unwrap_or_default(),
                featured_articles: feat_r.unwrap_or_default(),
                hot_articles: hot_art_r.unwrap_or_default(),
                hot_keywords: hot_r.unwrap_or_default(),
            })
        })
        .await
}
