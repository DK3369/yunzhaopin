//! Console recycle bin — PHP `tool/dataRecycle` + `recycle.model.php`.
//!
//! PHP fills `phpyun_recycle` from inside `delete_all`: before the `DELETE`
//! runs it selects the doomed rows and stores each one `serialize()`d, tagged
//! with the source table and an `ident` md5 shared by everything removed in the
//! same operation. This page reads that log, puts rows back, or clears it.
//!
//! The Rust port writes the same snapshots from the admin log queues it has
//! taken over (`admin_archive_service::snapshot`), so rows keep arriving here
//! and restore keeps working across both stacks. Content tables are the
//! exception: those carry a `deleted` flag (`models::soft_delete`) and are
//! recovered from their own pages, never through the bin.
//!
//! One difference from PHP, which kept snapshots until an admin cleared the
//! bin: `maintenance::purge_recycle_bin` drops anything older than 30 days, so
//! the window this page can restore from is that long rather than unbounded.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::i18n;
use phpyun_core::utils::fmt_ts;
use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::php_ser;
use phpyun_models::recycle_bin::entity::PhpRecycleRow;
use phpyun_models::recycle_bin::php_repo::{self as repo, PhpFilter};
use serde_json::{json, Map, Value};

fn tr(key: &str) -> String {
    i18n::t(&format!("messages.{key}"), i18n::current_lang())
}

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

// ---------- 列表 ----------

/// PHP `dataRecycle::index_action` search box.
#[derive(Debug, Default)]
pub struct ListQuery {
    pub username: Option<String>,
    pub keyword: Option<String>,
    pub table: Option<String>,
    pub ident: Option<String>,
    /// The date picker's `[start, end]` in milliseconds.
    pub time: Option<(i64, i64)>,
}

/// PHP takes `date('Y-m-d', ms/1000)` and re-parses it with an explicit
/// `00:00:00` / `23:59:59`, so a picked pair always covers whole local days.
fn day_range(ms_from: i64, ms_to: i64) -> (Option<i64>, Option<i64>) {
    (day_edge(ms_from, false), day_edge(ms_to, true))
}

fn day_edge(ms: i64, end_of_day: bool) -> Option<i64> {
    use chrono::TimeZone;
    let secs = ms.checked_div(1000).filter(|s| *s > 0)?;
    let local = chrono::DateTime::from_timestamp(secs, 0)?.with_timezone(&clock::tz());
    let date = local.date_naive();
    let naive = if end_of_day {
        date.and_hms_opt(23, 59, 59)?
    } else {
        date.and_hms_opt(0, 0, 0)?
    };
    clock::tz()
        .from_local_datetime(&naive)
        .single()
        .map(|dt| dt.timestamp())
}

fn filter_of(q: &ListQuery) -> PhpFilter {
    let (from, to) = q
        .time
        .map(|(a, b)| day_range(a, b))
        .unwrap_or((None, None));
    PhpFilter {
        username: q.username.clone(),
        keyword: q.keyword.clone(),
        table: q.table.clone(),
        ident: q.ident.clone(),
        ctime_from: from,
        ctime_to: to,
    }
}

/// The grid binds `time_n`, and the detail drawer iterates `body_n` as a plain
/// object, so both are precomputed here the way PHP's `getList` does.
fn decorate(row: PhpRecycleRow) -> Value {
    let mut body = Map::new();
    for (col, val) in php_ser::unserialize_pairs(&row.body) {
        body.insert(
            col,
            match val {
                Some(s) => Value::String(s),
                None => Value::Null,
            },
        );
    }
    json!({
        "id": row.id,
        "uid": row.uid,
        "username": row.username,
        "tablename": row.tablename,
        "ctime": row.ctime,
        "ident": row.ident,
        "uri": row.uri,
        "time_n": fmt_ts(row.ctime, "%Y-%m-%d %H:%M:%S"),
        "body_n": Value::Object(body),
    })
}

pub async fn list(
    state: &AppState,
    user: &AuthenticatedUser,
    q: &ListQuery,
    page: Pagination,
) -> AppResult<Paged<Value>> {
    user.require_admin()?;
    let db = state.db.reader();
    let f = filter_of(q);
    let total = repo::count(db, &f).await?;
    let list = if total > 0 {
        repo::list(db, &f, page.offset, page.limit)
            .await?
            .into_iter()
            .map(decorate)
            .collect()
    } else {
        Vec::new()
    };
    Ok(Paged::new(list, total, page.page, page.page_size))
}

// ---------- 恢复 ----------

/// Put snapshots back and drop the ones that made it.
///
/// PHP inserts and deletes per row with no transaction and ignores insert
/// failures, which silently loses a snapshot whose table has since changed
/// shape. Here a row leaves the bin only once its `INSERT` reports a row, and a
/// snapshot the schema now rejects is logged and left behind rather than
/// failing the whole batch — the operator can still see it and retry.
async fn restore(
    state: &AppState,
    user: &AuthenticatedUser,
    rows: Vec<PhpRecycleRow>,
) -> AppResult<String> {
    if rows.is_empty() {
        return Err(ApiError::param_invalid("common_00818"));
    }
    let pool = state.db.pool();
    let mut done: Vec<u64> = Vec::new();
    for row in &rows {
        let cols = php_ser::unserialize_pairs(&row.body);
        if cols.is_empty() {
            continue;
        }
        match repo::insert_row(pool, &row.tablename, &cols).await {
            Ok(n) if n > 0 => done.push(row.id),
            Ok(_) => {}
            Err(e) => tracing::warn!(id = row.id, table = %row.tablename, error = %e,
                                     "recycle restore rejected by schema"),
        }
    }
    if done.is_empty() {
        return Err(ApiError::param_invalid("common_00818"));
    }
    repo::delete_by_ids(pool, &done).await?;
    audit_write(state, user, "admin.recycle.restore", format!("{done:?}")).await;
    Ok(tr("common_06572"))
}

/// PHP `recover_action` → `recycle.model::recoverTb`.
pub async fn recover(
    state: &AppState,
    user: &AuthenticatedUser,
    ids: &[u64],
) -> AppResult<String> {
    user.require_admin()?;
    let ids: Vec<u64> = ids.iter().copied().filter(|v| *v > 0).collect();
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_00203"));
    }
    let rows = repo::find_by_ids(state.db.reader(), &ids).await?;
    restore(state, user, rows).await
}

/// PHP `recoverAll_action` → `recoverByIdent`: restores everything removed in
/// the same original operation.
pub async fn recover_by_ident(
    state: &AppState,
    user: &AuthenticatedUser,
    ident: &str,
) -> AppResult<String> {
    user.require_admin()?;
    let ident = ident.trim();
    if ident.is_empty() {
        return Err(ApiError::param_invalid("wap_00203"));
    }
    let rows = repo::find_by_ident(state.db.reader(), ident).await?;
    restore(state, user, rows).await
}

// ---------- 删除 / 清空 ----------

/// PHP `delRecycle_action`. Its message is `model_00226` + ids + `model_00130`
/// with `admin_user_00187` / `admin_user_00186` appended.
///
/// PHP then tests `$return['error']`, but the model returns `errcode`, so the
/// undefined key compares equal to 0 and every outcome — including a failed
/// delete — is reported as success with a "delete failed" message. Nothing
/// deleted is returned as a failure here, which costs the `回收站数据` prefix on
/// that one path because the error envelope carries a key, not free text.
pub async fn delete(state: &AppState, user: &AuthenticatedUser, ids: &[u64]) -> AppResult<String> {
    user.require_admin()?;
    let ids: Vec<u64> = ids.iter().copied().filter(|v| *v > 0).collect();
    if ids.is_empty() {
        return Err(ApiError::param_invalid("wap_00203"));
    }
    if repo::delete_by_ids(state.db.pool(), &ids).await? == 0 {
        return Err(ApiError::param_invalid("admin_user_00186"));
    }
    audit_write(state, user, "admin.recycle.delete", format!("{ids:?}")).await;
    let joined = ids
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join(",");
    Ok(format!(
        "{}{joined}{}{}",
        tr("model_00226"),
        tr("model_00130"),
        tr("admin_user_00187")
    ))
}

/// PHP `tuncateRecycle_action`: the button posts a literal `tuncate` (their
/// spelling) and anything else is refused.
pub async fn truncate(
    state: &AppState,
    user: &AuthenticatedUser,
    confirm: &str,
) -> AppResult<String> {
    user.require_admin()?;
    if confirm != "tuncate" {
        return Err(ApiError::param_invalid("admin_tool_00019"));
    }
    repo::truncate(state.db.pool()).await?;
    audit_write(state, user, "admin.recycle.truncate", "all".into()).await;
    Ok(tr("admin_01458"))
}
