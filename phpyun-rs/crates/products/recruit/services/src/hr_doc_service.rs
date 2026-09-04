//! HR toolbox documents (aligned with PHPYun `hr.model.php` / `toolbox_doc`).

use phpyun_core::{background, ApiError, AppResult, AppState, Paged, Pagination};
use phpyun_models::hr_doc::{entity::HrDoc, repo as hr_repo};

pub async fn list(
    state: &AppState,
    cid: Option<u64>,
    keyword: Option<&str>,
    order: Option<&str>,
    page: Pagination,
) -> AppResult<Paged<HrDoc>> {
    let db = state.db.reader();
    let order_hits = matches!(order, Some("hits") | Some("downnum"));
    let (list, total) = tokio::join!(
        hr_repo::list_public(db, cid, keyword, order_hits, page.offset, page.limit),
        hr_repo::count_public(db, cid, keyword),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

pub async fn get(state: &AppState, id: u64) -> AppResult<HrDoc> {
    let d = hr_repo::find(state.db.reader(), id)
        .await?
        .ok_or_else(|| ApiError::param_invalid("doc_not_found"))?;
    let pool = state.db.pool().clone();
    background::spawn_best_effort("hr_doc.hit", async move {
        let _ = hr_repo::incr_hit(&pool, id).await;
    });
    Ok(d)
}
