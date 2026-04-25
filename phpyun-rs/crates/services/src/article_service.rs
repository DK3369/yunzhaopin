//! Public browsing for articles / news posts.

use phpyun_core::{background, AppResult, AppState, Pagination};
use phpyun_models::article::{entity::Article, repo as article_repo};
use phpyun_models::article::repo::ArticleFilter;

use crate::domain_errors::ResumeError; // Reused: article-not-found also returns not_found; a dedicated ArticleError can be added later

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
        .ok_or(ResumeError::NotFound)?;
    if a.status != 1 {
        return Err(ResumeError::NotFound.into());
    }
    // hits +1 written in the background
    let pool = state.db.pool().clone();
    background::spawn_best_effort("article.hits", async move {
        let _ = article_repo::incr_hits(&pool, id).await;
    });
    Ok(a)
}
