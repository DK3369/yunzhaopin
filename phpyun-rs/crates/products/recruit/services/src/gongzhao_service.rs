//! Joint recruitment (aligned with PHPYun `wap/gongzhao`).

use phpyun_core::cache::TieredCache;
use phpyun_core::{background, ApiError, AppResult, AppState, Paged, Pagination};
use phpyun_models::gongzhao::{entity::Gongzhao, repo as gz_repo};
use std::sync::OnceLock;
use std::time::Duration;

const LIST_TTL: Duration = Duration::from_secs(20);
static LIST_CACHE: OnceLock<TieredCache<Paged<Gongzhao>>> = OnceLock::new();

fn list_cache() -> &'static TieredCache<Paged<Gongzhao>> {
    LIST_CACHE.get_or_init(|| TieredCache::new(64, LIST_TTL))
}

pub fn invalidate_list() {
    list_cache().invalidate_prefix_local();
}

pub async fn list(
    state: &AppState,
    tag: Option<&str>,
    did: u32,
    page: Pagination,
) -> AppResult<Paged<Gongzhao>> {
    let key = format!(
        "gongzhao:list:t={}:d={}:pg={}:ps={}",
        tag.unwrap_or(""),
        did,
        page.page,
        page.page_size
    );
    let st = state.clone();
    let tag = tag.map(str::to_string);
    let arc = list_cache()
        .get_or_load(
            &state.redis,
            key,
            LIST_TTL,
            "gongzhao.list",
            move || async move { load_list(&st, tag.as_deref(), did, page).await },
        )
        .await?;
    Ok((*arc).clone())
}

async fn load_list(
    state: &AppState,
    tag: Option<&str>,
    did: u32,
    page: Pagination,
) -> AppResult<Paged<Gongzhao>> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        gz_repo::list(db, tag, did, page.offset, page.limit),
        gz_repo::count(db, tag, did),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

pub async fn get(state: &AppState, id: u64) -> AppResult<Gongzhao> {
    let g = gz_repo::find(state.db.reader(), id)
        .await?
        .ok_or_else(|| ApiError::param_invalid("gongzhao_not_found"))?;
    if g.status != 1 {
        return Err(ApiError::param_invalid("gongzhao_unavailable"));
    }
    let pool = state.db.pool().clone();
    background::spawn_best_effort("gongzhao.view", async move {
        let _ = gz_repo::incr_view(&pool, id).await;
    });
    let mut g = g;
    g.body = phpyun_core::html::sanitize_html(&g.body);
    Ok(g)
}

pub async fn neighbors(
    state: &AppState,
    g: &Gongzhao,
) -> AppResult<(
    Option<phpyun_models::gongzhao::repo::Neighbor>,
    Option<phpyun_models::gongzhao::repo::Neighbor>,
)> {
    Ok(gz_repo::neighbors(state.db.reader(), g.id).await?)
}
