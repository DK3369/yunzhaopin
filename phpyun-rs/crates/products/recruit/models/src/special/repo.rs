use super::entity::{Special, SpecialCompany};
use crate::soft_delete::{self, PREDICATE};
use sqlx::MySqlPool;

/// Aligned with PHPYun `phpyun_special` (special recruitment topics).
/// Column mapping: banner→pic, body→intro, description→intro, start_at→ctime, end_at→etime,
/// status→display, view_count→num, created_at→ctime
const FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    COALESCE(title, '') AS title, \
    COALESCE(pic, '') AS banner, \
    COALESCE(intro, '') AS description, \
    COALESCE(intro, '') AS body, \
    CAST(COALESCE(ctime, 0) AS SIGNED) AS start_at, \
    CAST(COALESCE(etime, 0) AS SIGNED) AS end_at, \
    CAST(COALESCE(display, 0) AS SIGNED) AS status, \
    CAST(COALESCE(num, 0) AS SIGNED) AS view_count, \
    CAST(COALESCE(ctime, 0) AS SIGNED) AS created_at, \
    COALESCE(tpl, '') AS tpl, \
    COALESCE(background, '') AS background, \
    CAST(COALESCE(`limit`, 0) AS SIGNED) AS max_count, \
    COALESCE(rating, '') AS rating, \
    CAST(COALESCE(com_bm, 0) AS SIGNED) AS com_bm, \
    CAST(COALESCE(integral, 0) AS SIGNED) AS integral, \
    CAST(COALESCE(sort, 0) AS SIGNED) AS sort, \
    COALESCE(intro, '') AS intro, \
    COALESCE(wappic, '') AS wappic, \
    COALESCE(wapback, '') AS wapback";

pub async fn list(pool: &MySqlPool, offset: u64, limit: u64) -> Result<Vec<Special>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_special \
         WHERE display = 1 AND {PREDICATE} ORDER BY sort DESC, ctime DESC, id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Special>(&sql)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let sql = format!("SELECT COUNT(*) FROM phpyun_special WHERE display = 1 AND {PREDICATE}");
    let (n,): (i64,) = sqlx::query_as(&sql).fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn find(pool: &MySqlPool, id: u64) -> Result<Option<Special>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_special WHERE id = ? AND {PREDICATE}");
    sqlx::query_as::<_, Special>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn incr_view(pool: &MySqlPool, id: u64) -> Result<(), sqlx::Error> {
    // PHPYun alignment: the view count column is `num`
    sqlx::query("UPDATE phpyun_special SET num = num + 1 WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn list_admin(
    pool: &MySqlPool,
    offset: u64,
    limit: u64,
) -> Result<Vec<Special>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_special WHERE {PREDICATE} ORDER BY sort DESC, ctime DESC, id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Special>(&sql)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_admin(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let sql = format!("SELECT COUNT(*) FROM phpyun_special WHERE {PREDICATE}");
    let (n,): (i64,) = sqlx::query_as(&sql).fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn set_display(pool: &MySqlPool, id: u64, display: i32) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_special SET display = ? WHERE id = ?")
        .bind(display)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

// ---------- companies ----------

pub async fn list_company_uids(
    pool: &MySqlPool,
    sid: u64,
    hy: i32,
    offset: u64,
    limit: u64,
) -> Result<Vec<SpecialCompany>, sqlx::Error> {
    // phpyun_special_com columns: id/sid/uid/integral/status/time/statusbody/sort/famous
    // Rust field created_at ← time
    sqlx::query_as::<_, SpecialCompany>(
        r#"SELECT
             CAST(sc.id AS UNSIGNED) AS id,
             CAST(COALESCE(sc.sid, 0) AS UNSIGNED) AS sid,
             CAST(COALESCE(sc.uid, 0) AS UNSIGNED) AS uid,
             CAST(COALESCE(sc.sort, 0) AS SIGNED) AS sort,
             CAST(COALESCE(sc.status, 0) AS SIGNED) AS status,
             CAST(COALESCE(sc.`time`, 0) AS SIGNED) AS created_at
           FROM phpyun_special_com sc
           LEFT JOIN phpyun_company c ON c.uid = sc.uid
           WHERE sc.sid = ? AND sc.status = 1 AND COALESCE(sc.deleted,0)=0
             AND (? = 0 OR COALESCE(c.hy, 0) = ?)
           ORDER BY COALESCE(sc.famous, 0) DESC, sc.sort DESC, sc.`time` ASC
           LIMIT ? OFFSET ?"#,
    )
    .bind(sid)
    .bind(hy)
    .bind(hy)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn count_companies(pool: &MySqlPool, sid: u64, hy: i32) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as(
            "SELECT COUNT(*) FROM phpyun_special_com sc
             LEFT JOIN phpyun_company c ON c.uid = sc.uid
             WHERE sc.sid = ? AND sc.status = 1 AND COALESCE(sc.deleted,0)=0
               AND (? = 0 OR COALESCE(c.hy, 0) = ?)",
        )
            .bind(sid)
            .bind(hy)
            .bind(hy)
            .fetch_one(pool)
            .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn list_industries(pool: &MySqlPool, sid: u64) -> Result<Vec<i32>, sqlx::Error> {
    let rows: Vec<(i32,)> = sqlx::query_as(
        "SELECT DISTINCT CAST(COALESCE(c.hy, 0) AS SIGNED)
         FROM phpyun_special_com sc
         INNER JOIN phpyun_company c ON c.uid = sc.uid
         WHERE sc.sid = ? AND sc.status = 1 AND COALESCE(sc.deleted,0)=0 AND c.hy > 0
         ORDER BY c.hy ASC",
    )
    .bind(sid)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|(h,)| h).collect())
}

/// List of company uids in the special event (flattened to Vec<u64>, used for job queries).
pub async fn list_company_uid_ids(
    pool: &MySqlPool,
    sid: u64,
    limit: u64,
) -> Result<Vec<u64>, sqlx::Error> {
    let rows: Vec<(u64,)> = sqlx::query_as(
        r#"SELECT uid FROM phpyun_special_com
           WHERE sid = ? AND status = 1 AND COALESCE(deleted,0)=0
           ORDER BY sort DESC LIMIT ?"#,
    )
    .bind(sid)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|(u,)| u).collect())
}

// ==================== Company sign-up to a special ====================

pub async fn already_applied(pool: &MySqlPool, sid: u64, uid: u64) -> Result<bool, sqlx::Error> {
    let row: Option<(i64,)> =
        sqlx::query_as(
            "SELECT 1 FROM phpyun_special_com WHERE sid = ? AND uid = ? AND COALESCE(deleted,0)=0 LIMIT 1",
        )
            .bind(sid)
            .bind(uid)
            .fetch_optional(pool)
            .await?;
    Ok(row.is_some())
}

pub async fn count_signups(pool: &MySqlPool, sid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(&format!(
        "SELECT COUNT(*) FROM phpyun_special_com WHERE sid = ? AND {PREDICATE}"
    ))
    .bind(sid)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn count_active_jobs_by_company(
    pool: &MySqlPool,
    uid: u64,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_company_job \
         WHERE uid = ? AND state = 1 AND sdate < ?",
    )
    .bind(uid)
    .bind(now)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

/// Read the company's stored rating tier (1..n) — used to gate `info.rating`.
/// Re-exported from the canonical `company_statis::repo`.
pub async fn get_company_rating(pool: &MySqlPool, uid: u64) -> Result<i32, sqlx::Error> {
    crate::company_statis::repo::read_rating(pool, uid).await
}

/// Read the company's integral balance.
/// Re-exported from the canonical `company_statis::repo`.
pub async fn get_company_integral(pool: &MySqlPool, uid: u64) -> Result<i64, sqlx::Error> {
    crate::company_statis::repo::read_integral(pool, uid).await
}

/// Atomic deduction. Returns `1` on success, `0` when balance is insufficient.
/// Re-exported from the canonical `company_statis::repo`.
pub async fn try_deduct_company_integral(
    pool: &MySqlPool,
    uid: u64,
    points: i64,
) -> Result<u64, sqlx::Error> {
    crate::company_statis::repo::try_deduct_integral(pool, uid, points).await
}

pub async fn insert_special_com(
    pool: &MySqlPool,
    sid: u64,
    uid: u64,
    integral: i32,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_special_com (sid, uid, integral, status, time) \
         VALUES (?, ?, ?, 0, ?)",
    )
    .bind(sid)
    .bind(uid)
    .bind(integral)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

/// Active job postings for multiple companies in a special event (batched via `IN(...)`).
pub async fn list_jobs_for_uids(
    pool: &MySqlPool,
    uids: &[u64],
    now: i64,
    limit: u64,
) -> Result<Vec<crate::job::entity::Job>, sqlx::Error> {
    use sqlx::QueryBuilder;
    if uids.is_empty() {
        return Ok(vec![]);
    }
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT id, uid, name, com_name, job1, job1_son, job_post,
         provinceid, cityid, three_cityid, salary, minsalary, maxsalary,
         `type`, number, exp, edu, state, status, r_status, rec, urgent,
         rec_time, sdate, edate, lastupdate, did, content, wel, hits
         FROM phpyun_company_job WHERE uid IN (",
    );
    let mut first = true;
    for u in uids {
        if !first {
            qb.push(",");
        }
        qb.push_bind(u);
        first = false;
    }
    qb.push(") AND state = 1 AND status = 0 AND r_status = 1 AND edate > ");
    qb.push_bind(now);
    qb.push(" ORDER BY lastupdate DESC LIMIT ");
    qb.push_bind(limit.min(200));
    qb.build_query_as().fetch_all(pool).await
}

pub async fn list_admin_kw(
    pool: &MySqlPool,
    keyword: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<Special>, sqlx::Error> {
    let mut qb = sqlx::QueryBuilder::new(format!(
        "SELECT {FIELDS} FROM phpyun_special WHERE {PREDICATE}"
    ));
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND title LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    qb.push(" ORDER BY sort DESC, ctime DESC, id DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_admin_kw(pool: &MySqlPool, keyword: Option<&str>) -> Result<u64, sqlx::Error> {
    let mut qb = sqlx::QueryBuilder::new(format!(
        "SELECT COUNT(*) FROM phpyun_special WHERE {PREDICATE}"
    ));
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND title LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn count_coms_by_sid(pool: &MySqlPool, sid: u64) -> Result<(i64, i64), sqlx::Error> {
    let (okn,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_special_com WHERE sid = ? AND status = 1 AND COALESCE(deleted,0)=0",
    )
    .bind(sid)
    .fetch_one(pool)
    .await?;
    let (pend,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_special_com WHERE sid = ? AND status = 0 AND COALESCE(deleted,0)=0",
    )
    .bind(sid)
    .fetch_one(pool)
    .await?;
    Ok((okn, pend))
}

pub struct SpecialWrite<'a> {
    pub id: Option<u64>,
    pub title: &'a str,
    pub tpl: &'a str,
    pub display: i32,
    pub integral: i32,
    pub com_bm: i32,
    pub sort: i32,
    pub limit: i32,
    pub etime: i64,
    pub intro: &'a str,
    pub rating: &'a str,
    pub now: i64,
}

pub async fn upsert_special(pool: &MySqlPool, a: SpecialWrite<'_>) -> Result<u64, sqlx::Error> {
    if let Some(id) = a.id.filter(|i| *i > 0) {
        sqlx::query(
            "UPDATE phpyun_special SET title=?, tpl=?, display=?, integral=?, com_bm=?, sort=?, \
             `limit`=?, etime=?, intro=?, rating=? WHERE id=?",
        )
        .bind(a.title)
        .bind(a.tpl)
        .bind(a.display)
        .bind(a.integral)
        .bind(a.com_bm)
        .bind(a.sort)
        .bind(a.limit)
        .bind(a.etime)
        .bind(a.intro)
        .bind(a.rating)
        .bind(id)
        .execute(pool)
        .await?;
        Ok(id)
    } else {
        let res = sqlx::query(
            "INSERT INTO phpyun_special (title, tpl, display, integral, com_bm, sort, `limit`, etime, intro, rating, ctime) \
             VALUES (?,?,?,?,?,?,?,?,?,?,?)",
        )
        .bind(a.title)
        .bind(a.tpl)
        .bind(a.display)
        .bind(a.integral)
        .bind(a.com_bm)
        .bind(a.sort)
        .bind(a.limit)
        .bind(a.etime)
        .bind(a.intro)
        .bind(a.rating)
        .bind(a.now)
        .execute(pool)
        .await?;
        Ok(res.last_insert_id())
    }
}

pub async fn delete_specials(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    let n = soft_delete::mark_ids(pool, "phpyun_special", ids).await?;
    let _ = soft_delete::mark_col_in(pool, "phpyun_special_com", "sid", ids).await?;
    Ok(n)
}

pub async fn set_sort(pool: &MySqlPool, id: u64, sort: i32) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_special SET sort = ? WHERE id = ?")
            .bind(sort)
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

pub async fn set_com_sort(pool: &MySqlPool, id: u64, sort: i32) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_special_com SET sort = ? WHERE id = ?")
            .bind(sort)
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

pub async fn set_famous(pool: &MySqlPool, sid: u64, uid: u64, famous: i32) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_special_com SET famous = ? WHERE sid = ? AND uid = ?")
            .bind(famous)
            .bind(sid)
            .bind(uid)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

pub async fn find_com_one(pool: &MySqlPool, id: u64) -> Result<Option<SpecialCompany>, sqlx::Error> {
    sqlx::query_as::<_, SpecialCompany>(
        r#"SELECT
             CAST(id AS UNSIGNED) AS id,
             CAST(COALESCE(sid, 0) AS UNSIGNED) AS sid,
             CAST(COALESCE(uid, 0) AS UNSIGNED) AS uid,
             CAST(COALESCE(sort, 0) AS SIGNED) AS sort,
             CAST(COALESCE(status, 0) AS SIGNED) AS status,
             CAST(COALESCE(`time`, 0) AS SIGNED) AS created_at
           FROM phpyun_special_com WHERE id = ? LIMIT 1"#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct SpecialAddCompany {
    pub uid: u64,
    pub name: String,
    pub linkman: String,
    pub linktel: String,
}

pub async fn list_add_companies(
    pool: &MySqlPool,
    sid: u64,
    keyword: Option<&str>,
    kw_type: i32,
    offset: u64,
    limit: u64,
) -> Result<Vec<SpecialAddCompany>, sqlx::Error> {
    let mut qb = sqlx::QueryBuilder::new(
        "SELECT CAST(c.uid AS UNSIGNED) AS uid, COALESCE(c.name,'') AS name, \
         COALESCE(c.linkman,'') AS linkman, COALESCE(c.linktel,'') AS linktel \
         FROM phpyun_company c WHERE c.r_status = 1 \
         AND c.uid NOT IN (SELECT uid FROM phpyun_special_com WHERE sid = ",
    );
    qb.push_bind(sid);
    qb.push(" AND COALESCE(deleted,0)=0)");
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        let like = format!("%{kw}%");
        match kw_type {
            2 => {
                qb.push(" AND c.uid IN (SELECT uid FROM phpyun_member WHERE username LIKE ");
                qb.push_bind(like);
                qb.push(")");
            }
            3 => {
                qb.push(" AND c.linkman LIKE ");
                qb.push_bind(like);
            }
            4 => {
                qb.push(" AND c.linktel LIKE ");
                qb.push_bind(like);
            }
            5 => {
                qb.push(" AND c.linkmail LIKE ");
                qb.push_bind(like);
            }
            6 => {
                if let Ok(uid) = kw.parse::<u64>() {
                    qb.push(" AND c.uid = ");
                    qb.push_bind(uid);
                }
            }
            _ => {
                qb.push(" AND (c.name LIKE ");
                qb.push_bind(like.clone());
                qb.push(" OR c.shortname LIKE ");
                qb.push_bind(like);
                qb.push(")");
            }
        }
    }
    qb.push(" ORDER BY c.uid DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_add_companies(
    pool: &MySqlPool,
    sid: u64,
    keyword: Option<&str>,
    kw_type: i32,
) -> Result<u64, sqlx::Error> {
    let mut qb = sqlx::QueryBuilder::new(
        "SELECT COUNT(*) FROM phpyun_company c WHERE c.r_status = 1 \
         AND c.uid NOT IN (SELECT uid FROM phpyun_special_com WHERE sid = ",
    );
    qb.push_bind(sid);
    qb.push(" AND COALESCE(deleted,0)=0)");
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        let like = format!("%{kw}%");
        match kw_type {
            2 => {
                qb.push(" AND c.uid IN (SELECT uid FROM phpyun_member WHERE username LIKE ");
                qb.push_bind(like);
                qb.push(")");
            }
            3 => {
                qb.push(" AND c.linkman LIKE ");
                qb.push_bind(like);
            }
            4 => {
                qb.push(" AND c.linktel LIKE ");
                qb.push_bind(like);
            }
            5 => {
                qb.push(" AND c.linkmail LIKE ");
                qb.push_bind(like);
            }
            6 => {
                if let Ok(uid) = kw.parse::<u64>() {
                    qb.push(" AND c.uid = ");
                    qb.push_bind(uid);
                }
            }
            _ => {
                qb.push(" AND (c.name LIKE ");
                qb.push_bind(like.clone());
                qb.push(" OR c.shortname LIKE ");
                qb.push_bind(like);
                qb.push(")");
            }
        }
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}
