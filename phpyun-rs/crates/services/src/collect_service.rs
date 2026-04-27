//! Job favorites — aligned with PHPYun `job.model.php::collectJob` (the only
//! favorites table PHPYun ships, `phpyun_fav_job`).
//!
//! API surface keeps a forward-compatible `kind` parameter so callers stay
//! source-compatible, but only `KIND_JOB` is accepted at the data layer; passing
//! `KIND_COMPANY` / `KIND_RESUME` returns `collect_bad_kind` 400 because PHP has
//! no backing table for those.
//!
//! ## Side effects (kept in sync with PHP `collectJob` / `cancelFavJob`)
//!
//! On **add**:
//!   1. INSERT into `phpyun_fav_job` (denormalized snapshot — same columns)
//!   2. `member_statis.fav_jobnum += 1`     (UPSERT-safe; row is created if missing)
//!   3. `member_log` operation row "收藏了职位 xxx"
//!   4. `phpyun_sysmsg` notify the company "用户 xx 收藏了您的职位 xxx"
//!
//! On **remove**:
//!   1. DELETE from `phpyun_fav_job`
//!   2. `member_statis.fav_jobnum -= 1`
//!   3. `member_log` operation row "取消收藏职位 xxx"
//!   (No company notification on un-favorite — PHP doesn't either.)
//!
//! All side effects are **best-effort**: a failure on the counter / log / sysmsg
//! does NOT undo the primary INSERT/DELETE. PHP behaves the same (no transaction).

use phpyun_core::{clock, background, AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::collect::entity::{Collect, KIND_JOB};
use phpyun_models::collect::repo as collect_repo;
use phpyun_models::message::repo as message_repo;

use crate::domain_errors::CollectError;

// ==================== PHP-aligned side effects ====================
// These are intentionally inlined (not in a shared module) because they're
// only used here. If a future feature needs the same building blocks they
// can be extracted then.

const MEMBER_LOG_OPERA_FAV: i32 = 5; // PHP `collectJob` passes opera=5 (collection ops)
const MEMBER_LOG_TYPE_ADD: i32 = 1;  // PHP type=1 = add
const MEMBER_LOG_TYPE_DEL: i32 = 3;  // PHP type=3 = delete

async fn bump_fav_jobnum(state: &AppState, uid: u64, did: u32, delta: i32) {
    // UPSERT pattern: PHP `update_once` silently no-ops when the row doesn't
    // exist. We make it idempotent: insert with the delta, or update by it.
    // `fav_jobnum` is `int(10) NOT NULL`; clamp to >= 0 on the way down so
    // we don't underflow if a stale row had 0 already.
    let q = if delta >= 0 {
        sqlx::query(
            r#"INSERT INTO phpyun_member_statis (uid, integral, fav_jobnum, resume_num, sq_jobnum, message_num, down_num)
               VALUES (?, '', ?, 0, 0, 0, 0)
               ON DUPLICATE KEY UPDATE fav_jobnum = fav_jobnum + ?"#,
        )
        .bind(uid)
        .bind(delta)
        .bind(delta)
    } else {
        let dec = (-delta) as i32;
        sqlx::query(
            "UPDATE phpyun_member_statis
                SET fav_jobnum = GREATEST(fav_jobnum - ?, 0)
              WHERE uid = ?",
        )
        .bind(dec)
        .bind(uid)
    };
    let _ = q.execute(state.db.pool()).await;
    let _ = did; // reserved for future per-site stats (PHP keys `did`+`uid`)
}

async fn add_member_log(
    state: &AppState,
    uid: u64,
    usertype: u8,
    did: u32,
    ip: &str,
    content: &str,
    type_: i32,
) {
    // phpyun_member_log columns: uid, opera, type, usertype, content, ip, ctime, did
    let _ = sqlx::query(
        r#"INSERT INTO phpyun_member_log
              (uid, opera, type, usertype, content, ip, ctime, did)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(uid)
    .bind(MEMBER_LOG_OPERA_FAV)
    .bind(type_)
    .bind(usertype as i32)
    .bind(content)
    .bind(ip)
    .bind(clock::now_ts())
    .bind(did)
    .execute(state.db.pool())
    .await;
}

pub struct CollectPage {
    pub list: Vec<Collect>,
    pub total: u64,
}

fn require_job_kind(kind: i32) -> AppResult<()> {
    if kind != KIND_JOB {
        return Err(CollectError::InvalidKind(kind).into());
    }
    Ok(())
}

/// Toggle a job favorite. If currently favorited → unfavorite; if not → favorite.
/// Returns the new state (`true` = now favorited, `false` = now unfavorited).
///
/// This is the **only** supported "add favorite" path now — the previous
/// strict-add semantics (with `Duplicate` 409) created a stale third-state
/// problem for the frontend (it had to track favorited-or-not separately).
/// Toggle is idempotent in spirit: clicking the heart twice always returns to
/// the starting state.
///
/// Mirrors PHP's full side-effect set (counter sync + member_log + sysmsg);
/// see module docs for details.
pub async fn toggle(
    state: &AppState,
    user: &AuthenticatedUser,
    kind: i32,
    target_id: u64,
    ip: &str,
) -> AppResult<bool> {
    require_job_kind(kind)?;
    // PHP `collectJob` rejects non-jobseeker accounts with state=4 message
    // "您不是个人用户！" — same gate here.
    user.require_jobseeker()?;

    // Read state from Redis cache (warm-on-miss). DB is source of truth, but
    // the cache lookup is O(1) and avoids hitting MySQL on every click.
    // Propagate errors here — silently defaulting to `false` would risk
    // double-inserting since `phpyun_fav_job` has no UNIQUE on (uid, job_id).
    let already_favorited =
        crate::collect_cache::is_favorited(state, user.uid, target_id).await?;

    // Already favorited → cancel branch
    if already_favorited {
        let _ = collect_repo::delete(state.db.pool(), user.uid, target_id).await?;
        crate::collect_cache::record_removed(state, user.uid, target_id).await;

        // Side effects (best-effort, off the critical path)
        let st = state.clone();
        let uid = user.uid;
        let did = user.did;
        let usertype = user.usertype;
        let ip_owned = ip.to_owned();
        background::spawn_best_effort("collect.cancel.side_effects", async move {
            bump_fav_jobnum(&st, uid, did, -1).await;
            let content = format!("收藏管理：取消收藏职位：{target_id}");
            add_member_log(&st, uid, usertype, did, &ip_owned, &content, MEMBER_LOG_TYPE_DEL).await;
        });
        return Ok(false);
    }

    // Add branch — fetch job snapshot for INSERT.
    let Some(job) = phpyun_models::job::repo::find_by_id(state.db.reader(), target_id).await? else {
        return Err(CollectError::TargetNotFound.into());
    };

    collect_repo::insert(
        state.db.pool(),
        collect_repo::InsertJob {
            uid: user.uid,
            com_id: job.uid,
            com_name: job.com_name.as_deref().unwrap_or(""),
            job_id: target_id,
            job_name: &job.name,
            // PHP `collectComJob` defaults to 1 (normal company-posted job).
            r#type: 1,
            datetime: clock::now_ts(),
        },
    )
    .await?;
    crate::collect_cache::record_added(state, user.uid, target_id).await;

    // Side effects (best-effort, off the critical path)
    let st = state.clone();
    let uid = user.uid;
    let did = user.did;
    let usertype = user.usertype;
    let ip_owned = ip.to_owned();
    let job_name = job.name.clone();
    let com_uid = job.uid;
    background::spawn_best_effort("collect.add.side_effects", async move {
        // 1. counter
        bump_fav_jobnum(&st, uid, did, 1).await;
        // 2. member operation log
        let content = format!("职位收藏：收藏了职位：{}", job_name);
        add_member_log(&st, uid, usertype, did, &ip_owned, &content, MEMBER_LOG_TYPE_ADD).await;
        // 3. notify the company that owns the job (PHP `addSystem` to job.uid usertype=2)
        let sysmsg_content = format!("有用户收藏了您的职位：{}", job_name);
        let _ = message_repo::insert_simple(st.db.pool(), com_uid, 2, &sysmsg_content, clock::now_ts()).await;
    });

    Ok(true)
}

pub async fn remove(
    state: &AppState,
    user: &AuthenticatedUser,
    kind: i32,
    target_id: u64,
    ip: &str,
) -> AppResult<()> {
    require_job_kind(kind)?;
    user.require_jobseeker()?;
    let n = collect_repo::delete(state.db.pool(), user.uid, target_id).await?;
    if n > 0 {
        crate::collect_cache::record_removed(state, user.uid, target_id).await;
        let st = state.clone();
        let uid = user.uid;
        let did = user.did;
        let usertype = user.usertype;
        let ip_owned = ip.to_owned();
        background::spawn_best_effort("collect.remove.side_effects", async move {
            bump_fav_jobnum(&st, uid, did, -1).await;
            let content = format!("收藏管理：取消收藏职位：{target_id}");
            add_member_log(&st, uid, usertype, did, &ip_owned, &content, MEMBER_LOG_TYPE_DEL).await;
        });
    }
    Ok(())
}

pub async fn exists(
    state: &AppState,
    user: &AuthenticatedUser,
    kind: i32,
    target_id: u64,
) -> AppResult<bool> {
    require_job_kind(kind)?;
    crate::collect_cache::is_favorited(state, user.uid, target_id).await
}

pub async fn list(
    state: &AppState,
    user: &AuthenticatedUser,
    kind: i32,
    page: Pagination,
) -> AppResult<CollectPage> {
    require_job_kind(kind)?;
    let (total, list) = tokio::join!(
        collect_repo::count_by_user(state.db.reader(), user.uid),
        collect_repo::list_by_user(state.db.reader(), user.uid, page.offset, page.limit),
    );
    Ok(CollectPage { total: total?, list: list? })
}

/// Count how many users have favorited a given job (used by job-detail aggregations).
pub async fn count_collectors_of_job(
    state: &AppState,
    job_id: u64,
) -> AppResult<u64> {
    Ok(collect_repo::count_collectors_of_job(state.db.reader(), job_id).await?)
}

/// Batch lookup for "is favorited" — given a slice of job ids and the current
/// user's uid, return the subset already favorited. Use this to populate the
/// `is_favorited` field on list / detail DTOs without N+1 queries.
///
/// Now Redis-backed: SMISMEMBER in a single RTT after first warm-up.
/// DB fallback if Redis is unhealthy. Empty set if `uid` is None.
pub async fn favorited_set(
    state: &AppState,
    uid: Option<u64>,
    job_ids: &[u64],
) -> std::collections::HashSet<u64> {
    crate::collect_cache::favorited_set(state, uid, job_ids).await
}
