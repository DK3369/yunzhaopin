//! Joint recruitment (aligned with PHPYun `wap/gongzhao`).

use phpyun_core::error::InfraError;
use phpyun_core::{background, AppError, AppResult, AppState, Paged, Pagination};
use phpyun_models::gongzhao::{entity::Gongzhao, repo as gz_repo};

pub async fn list(
    state: &AppState,
    tag: Option<&str>,
    page: Pagination,
) -> AppResult<Paged<Gongzhao>> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        gz_repo::list(db, tag, page.offset, page.limit),
        gz_repo::count(db, tag),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

pub async fn get(state: &AppState, id: u64) -> AppResult<Gongzhao> {
    let g = gz_repo::find(state.db.reader(), id)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("gongzhao_not_found".into())))?;
    if g.status != 1 {
        return Err(AppError::new(InfraError::InvalidParam("gongzhao_unavailable".into())));
    }
    let pool = state.db.pool().clone();
    background::spawn_best_effort("gongzhao.view", async move {
        let _ = gz_repo::incr_view(&pool, id).await;
    });
    Ok(g)
}
