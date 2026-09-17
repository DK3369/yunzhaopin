//! Public browsing for articles / news posts.

use phpyun_core::cache::TieredCache;
use phpyun_core::ApiError;
use phpyun_core::{background, AppResult, AppState, Pagination};
use phpyun_models::article::repo::ArticleFilter;
use phpyun_models::article::{entity::Article, repo as article_repo};
use std::sync::OnceLock;
use std::time::Duration;

pub struct ArticlePage {
    pub list: Vec<Article>,
    pub total: u64,
}

pub async fn list_public(
    state: &AppState,
    filter: &ArticleFilter<'_>,
    page: Pagination,
) -> AppResult<ArticlePage> {
    let (total, list) = tokio::join!(
        article_repo::count_public(state.db.reader(), filter),
        article_repo::list_public(state.db.reader(), filter, page.offset, page.limit),
    );
    Ok(ArticlePage {
        total: total?,
        list: list?,
    })
}

pub async fn get_public(state: &AppState, id: u64) -> AppResult<Article> {
    let a = article_repo::find_by_id(state.db.reader(), id)
        .await?
        .ok_or(ApiError::business("resume_not_found"))?;
    if a.status != 1 {
        return Err(ApiError::business("resume_not_found"));
    }
    // hits +1 written in the background
    let pool = state.db.pool().clone();
    background::spawn_best_effort("article.hits", async move {
        let _ = article_repo::incr_hits(&pool, id).await;
    });
    let mut a = a;
    a.content = phpyun_core::html::sanitize_opt(a.content);
    Ok(a)
}

const GROUPS_TTL: Duration = Duration::from_secs(300);
const GROUPS_KEY: &str = "article:groups";

static GROUPS_CACHE: OnceLock<TieredCache<Vec<phpyun_models::article::entity::NewsGroup>>> =
    OnceLock::new();

fn groups_cache() -> &'static TieredCache<Vec<phpyun_models::article::entity::NewsGroup>> {
    GROUPS_CACHE.get_or_init(|| TieredCache::new(4, GROUPS_TTL))
}

pub async fn invalidate_groups(state: &AppState) {
    groups_cache().invalidate(&state.redis, GROUPS_KEY).await;
}

pub async fn list_groups(state: &AppState) -> AppResult<Vec<phpyun_models::article::entity::NewsGroup>> {
    let st = state.clone();
    let arc = groups_cache()
        .get_or_load(
            &state.redis,
            GROUPS_KEY.to_string(),
            GROUPS_TTL,
            "article.groups",
            move || async move { Ok(article_repo::list_groups(st.db.reader()).await?) },
        )
        .await?;
    Ok((*arc).clone())
}

pub async fn neighbors_and_related(
    state: &AppState,
    a: &Article,
) -> AppResult<(
    Option<phpyun_models::article::repo::Neighbor>,
    Option<phpyun_models::article::repo::Neighbor>,
    Vec<phpyun_models::article::repo::Neighbor>,
)> {
    let db = state.db.reader();
    let (prev, next) = article_repo::neighbors(db, a.id, a.nid, a.published_at).await?;
    let related = article_repo::related(db, a.id, a.nid, &a.keyword, 6).await?;
    Ok((prev, next, related))
}
