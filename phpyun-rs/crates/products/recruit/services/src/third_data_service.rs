//! Admin CRUD for crawler source URLs (`phpyun_rs_third_data`).

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::third_data::{self, entity::ThirdData, repo as third_data_repo};

async fn audit_write(
    state: &AppState,
    actor: &AuthenticatedUser,
    action: &'static str,
    target: String,
) {
    let _ = audit::emit(
        state,
        AuditEvent::new(action, Actor::uid(actor.uid)).target(target),
    )
    .await;
}

fn is_http_url(url: &str) -> bool {
    let u = url.trim();
    (u.starts_with("http://") || u.starts_with("https://")) && !u.contains(' ') && u.len() < 512
}

fn trim_url(url: &str) -> String {
    url.trim().trim_end_matches('/').to_string()
}

pub async fn list(state: &AppState, page: Pagination) -> AppResult<Paged<ThirdData>> {
    let db = state.db.reader();
    let list = third_data_repo::list(db, page.offset, page.limit).await?;
    let total = third_data_repo::count_all(db).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub struct UpsertIn<'a> {
    pub id: Option<u64>,
    pub name: &'a str,
    pub url: &'a str,
    pub api_url: &'a str,
    pub provider: &'a str,
    pub sort: i32,
    pub enabled: i32,
}

pub async fn upsert(state: &AppState, actor: &AuthenticatedUser, a: UpsertIn<'_>) -> AppResult<u64> {
    let name = a.name.trim();
    let url = trim_url(a.url);
    let api_url = trim_url(a.api_url);
    if name.is_empty() || !is_http_url(&url) {
        return Err(ApiError::param_invalid("third_data_url"));
    }
    if !api_url.is_empty() && !is_http_url(&api_url) {
        return Err(ApiError::param_invalid("third_data_api_url"));
    }
    if let Some(exist) = third_data_repo::find_by_url(state.db.reader(), &url).await? {
        if a.id.filter(|i| *i > 0) != Some(exist.id) {
            return Err(ApiError::param_invalid("third_data_url"));
        }
    }
    let provider = third_data::normalize_provider(a.provider, &url, &api_url);
    let mut sort = a.sort;
    if a.id.filter(|i| *i > 0).is_none() && sort == 0 {
        sort = third_data_repo::max_sort(state.db.reader()).await?.saturating_add(10);
    }
    let enabled = if a.enabled == 0 { 0 } else { 1 };
    let now = clock::now_ts();
    let id = third_data_repo::upsert(
        state.db.pool(),
        third_data_repo::ThirdDataUpsert {
            id: a.id,
            name,
            url: &url,
            api_url: &api_url,
            provider: &provider,
            sort,
            enabled,
            now,
        },
    )
    .await?;
    audit_write(state, actor, "admin.third_data.upsert", format!("third_data:{id}")).await;
    Ok(id)
}

pub async fn delete(state: &AppState, actor: &AuthenticatedUser, ids: &[u64]) -> AppResult<()> {
    if ids.is_empty() {
        return Err(ApiError::param_invalid("ids"));
    }
    let n = third_data_repo::delete_ids(state.db.pool(), ids).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("third_data_not_found"));
    }
    audit_write(
        state,
        actor,
        "admin.third_data.delete",
        format!("third_data:{}", ids.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",")),
    )
    .await;
    Ok(())
}
