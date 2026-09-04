//! Public browsing for articles / news posts.

use phpyun_core::ApiError;
use phpyun_core::{background, AppResult, AppState, Pagination};
use phpyun_models::article::repo::ArticleFilter;
use phpyun_models::article::{entity::Article, repo as article_repo};

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
    Ok(a)
}

pub async fn list_groups(state: &AppState) -> AppResult<Vec<phpyun_models::article::entity::NewsGroup>> {
    Ok(article_repo::list_groups(state.db.reader()).await?)
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
