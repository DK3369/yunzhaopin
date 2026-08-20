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
