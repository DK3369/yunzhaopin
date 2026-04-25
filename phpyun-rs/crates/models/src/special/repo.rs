use super::entity::{Special, SpecialCompany};
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

pub async fn list(
    pool: &MySqlPool,
    offset: u64,
    limit: u64,
) -> Result<Vec<Special>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_special \
         WHERE display = 1 ORDER BY sort DESC, ctime DESC, id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, Special>(&sql)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_special WHERE display = 1")
            .fetch_one(pool)
            .await?;
    Ok(n.max(0) as u64)
}

pub async fn find(pool: &MySqlPool, id: u64) -> Result<Option<Special>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_special WHERE id = ?");
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

// ---------- companies ----------

pub async fn list_company_uids(
    pool: &MySqlPool,
    sid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<SpecialCompany>, sqlx::Error> {
    // phpyun_special_com columns: id/sid/uid/integral/status/time/statusbody/sort/famous
    // Rust field created_at ← time
    sqlx::query_as::<_, SpecialCompany>(
        r#"SELECT
             CAST(id AS UNSIGNED) AS id,
             CAST(COALESCE(sid, 0) AS UNSIGNED) AS sid,
             CAST(COALESCE(uid, 0) AS UNSIGNED) AS uid,
             CAST(COALESCE(sort, 0) AS SIGNED) AS sort,
             CAST(COALESCE(status, 0) AS SIGNED) AS status,
             CAST(COALESCE(`time`, 0) AS SIGNED) AS created_at
           FROM phpyun_special_com
           WHERE sid = ? AND status = 1
           ORDER BY sort DESC, `time` ASC
           LIMIT ? OFFSET ?"#,
    )
    .bind(sid)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn count_companies(pool: &MySqlPool, sid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_special_com WHERE sid = ? AND status = 1",
    )
    .bind(sid)
    .fetch_one(pool)
    .await?;
    Ok(n.max(0) as u64)
}

/// List of company uids in the special event (flattened to Vec<u64>, used for job queries).
pub async fn list_company_uid_ids(
    pool: &MySqlPool,
    sid: u64,
    limit: u64,
) -> Result<Vec<u64>, sqlx::Error> {
    let rows: Vec<(u64,)> = sqlx::query_as(
        r#"SELECT uid FROM phpyun_special_com
           WHERE sid = ? AND status = 1
           ORDER BY sort DESC LIMIT ?"#,
    )
    .bind(sid)
    .bind(limit)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|(u,)| u).collect())
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
