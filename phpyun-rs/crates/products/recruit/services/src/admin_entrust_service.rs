//! Admin 个人委托简历. PHP 后台无对应控制器，契约来自 `resume_trust.vue`.

use md5::{Digest, Md5};
use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::recycle_bin::php_repo as recycle;
use phpyun_models::user_entrust::entity::{TrustStat, UserEntrustRow};
use phpyun_models::user_entrust::repo::{self, Filter};

async fn audit_write(state: &AppState, actor: &AuthenticatedUser, action: &'static str, target: String) {
    let _ = audit::emit(
        state,
        AuditEvent::new(action, Actor::uid(actor.uid)).target(target),
    )
    .await;
}

fn tr(key: &str) -> String {
    phpyun_core::i18n::t(
        &format!("messages.{key}"),
        phpyun_core::i18n::current_lang(),
    )
}

fn php_msg(prefix: &str, ids: &[u64], suffixes: &[&str]) -> String {
    let mut out = tr(prefix);
    out.push_str(
        &ids.iter()
            .map(u64::to_string)
            .collect::<Vec<_>>()
            .join(","),
    );
    for key in suffixes {
        out.push_str(&tr(key));
    }
    out
}

fn clean_ids(ids: &[u64]) -> AppResult<Vec<u64>> {
    let out: Vec<u64> = ids.iter().copied().filter(|v| *v > 0).collect();
    if out.is_empty() {
        return Err(ApiError::param_invalid("common_00921"));
    }
    Ok(out)
}

pub struct ListFilter<'a> {
    pub status: Option<i32>,
    pub keyword: Option<&'a str>,
    pub name_kind: i32,
    pub end: Option<i32>,
    pub sort: &'a str,
    pub dir: &'a str,
}

fn since_from_end(end: Option<i32>) -> Option<i64> {
    let end = end.filter(|d| *d > 0)?;
    let now = clock::now_ts();
    Some(if end == 1 {
        clock::start_of_day(now)
    } else {
        now.saturating_sub(i64::from(end) * 86_400)
    })
}

/// Vue 待审角标点的是 `statusSearch('3')`，表里待审是 `0`.
fn map_status(status: Option<i32>) -> Option<i32> {
    match status {
        Some(3) => Some(0),
        other => other,
    }
}

pub async fn list(
    state: &AppState,
    f: ListFilter<'_>,
    page: Pagination,
) -> AppResult<Paged<UserEntrustRow>> {
    let db = state.db.reader();
    let filter = Filter {
        status: map_status(f.status),
        keyword: f.keyword,
        name_kind: f.name_kind,
        since: since_from_end(f.end),
        sort: f.sort,
        dir: f.dir,
    };
    let mut list = repo::list(db, &filter, page.offset, page.limit).await?;
    for row in &mut list {
        row.add_time_n = phpyun_core::utils::fmt_dt(row.add_time);
    }
    let total = repo::count(db, &filter).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn stat(state: &AppState) -> AppResult<TrustStat> {
    Ok(repo::stat(state.db.reader()).await?)
}

pub async fn delete(
    state: &AppState,
    actor: &AuthenticatedUser,
    ids: &[u64],
    uri: &str,
) -> AppResult<String> {
    let ids = clean_ids(ids)?;
    let pool = state.db.pool();
    let username = recycle::admin_username(pool, actor.uid)
        .await
        .unwrap_or_default();
    let ident = {
        let joined = ids.iter().map(u64::to_string).collect::<Vec<_>>().join(",");
        format!(
            "{:x}",
            Md5::digest(format!("user_entrust{joined}").as_bytes())
        )
    };
    if let Err(e) = recycle::archive(
        pool,
        "user_entrust",
        &ids,
        actor.uid,
        &username,
        &ident,
        uri,
    )
    .await
    {
        tracing::warn!(error = %e, "recycle snapshot skipped");
    }
    if repo::delete_by_ids(pool, &ids).await? == 0 {
        return Err(ApiError::param_invalid("admin_user_00186"));
    }
    let msg = php_msg("model_00229", &ids, &["model_00112"]);
    audit_write(state, actor, "admin.user.entrust.delete", msg.clone()).await;
    Ok(msg)
}

pub async fn set_status(
    state: &AppState,
    actor: &AuthenticatedUser,
    id: u64,
    status: i32,
) -> AppResult<String> {
    if id == 0 || (status != 1 && status != 2) {
        return Err(ApiError::param_invalid("param_invalid"));
    }
    let n = repo::set_status(state.db.pool(), id, status, actor.uid, clock::now_ts()).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("not_found"));
    }
    audit_write(
        state,
        actor,
        "admin.user.entrust.status",
        format!("id:{id} status:{status}"),
    )
    .await;
    Ok(tr("wap_user_00264"))
}
