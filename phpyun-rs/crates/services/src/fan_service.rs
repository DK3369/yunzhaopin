//! "Fans" — users who have favorited this company's jobs.
//!
//! Counterpart of PHPYun `wap/member/com.class.php::attention_me_action`,
//! the company-side "对我感兴趣" page. Selects from `phpyun_fav_job` grouped
//! by `uid` where `com_id = my-uid`.
//!
//! Job-seekers calling this endpoint receive an empty list (the relationship
//! direction is jobseeker → company; the inverse for jobseekers is "who has
//! viewed my resume", which lives on the existing `views` endpoint).

use phpyun_core::{AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::collect::repo as collect_repo;

#[derive(Debug, Clone)]
pub struct FanRow {
    pub uid: u64,
    pub username: String,
    pub fav_count: u64,
    pub last_datetime: i64,
}

pub struct FanPage {
    pub total: u64,
    pub list: Vec<FanRow>,
}

pub async fn list_fans(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<FanPage> {
    // Only company users have a meaningful "fans" set; for other usertypes
    // return an empty page rather than 403 — the frontend already gates the
    // page by usertype, this is just defensive.
    if user.usertype != 2 {
        return Ok(FanPage { total: 0, list: vec![] });
    }

    let pool = state.db.reader();
    let (total, rows) = tokio::join!(
        collect_repo::count_fans_by_com_uid(pool, user.uid),
        collect_repo::list_fans_by_com_uid(pool, user.uid, page.offset, page.limit),
    );
    let total = total?;
    let rows = rows?;

    let uids: Vec<u64> = rows.iter().map(|(uid, _, _)| *uid).collect();
    let names = lookup_usernames(pool, &uids).await;

    let list = rows
        .into_iter()
        .map(|(uid, fav_count, last_datetime)| FanRow {
            username: names.get(&uid).cloned().unwrap_or_default(),
            uid,
            fav_count,
            last_datetime,
        })
        .collect();

    Ok(FanPage { total, list })
}

async fn lookup_usernames(
    pool: &sqlx::MySqlPool,
    uids: &[u64],
) -> std::collections::HashMap<u64, String> {
    use std::collections::HashMap;
    if uids.is_empty() {
        return HashMap::new();
    }
    let placeholders = vec!["?"; uids.len()].join(",");
    let sql = format!(
        "SELECT CAST(uid AS UNSIGNED) AS uid, COALESCE(username, '') AS username \
           FROM phpyun_member WHERE uid IN ({placeholders})"
    );
    let mut q = sqlx::query_as::<_, (u64, String)>(&sql); // TODO(arch): inline sqlx pending repo lift
    for id in uids {
        q = q.bind(*id);
    }
    match q.fetch_all(pool).await {
        Ok(rows) => rows.into_iter().collect(),
        Err(_) => HashMap::new(),
    }
}
