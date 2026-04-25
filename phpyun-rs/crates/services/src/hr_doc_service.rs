//! HR toolbox documents (aligned with PHPYun `hr.model.php` / `toolbox_doc`).

use phpyun_core::error::InfraError;
use phpyun_core::{background, AppError, AppResult, AppState, Paged, Pagination};
use phpyun_models::hr_doc::{entity::HrDoc, repo as hr_repo};

pub async fn list(
    state: &AppState,
    cid: Option<u64>,
    page: Pagination,
) -> AppResult<Paged<HrDoc>> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        hr_repo::list_public(db, cid, page.offset, page.limit),
        hr_repo::count_public(db, cid),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

pub async fn get(state: &AppState, id: u64) -> AppResult<HrDoc> {
    let d = hr_repo::find(state.db.reader(), id)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("doc_not_found".into())))?;
    let pool = state.db.pool().clone();
    background::spawn_best_effort("hr_doc.hit", async move {
        let _ = hr_repo::incr_hit(&pool, id).await;
    });
    Ok(d)
}
