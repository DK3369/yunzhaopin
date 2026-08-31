use super::entity::{AdminLogoutListRow, MemberLogout};
use sqlx::{MySqlPool, QueryBuilder};

// `phpyun_member_logout` marks uid/status/ctime nullable; `MemberLogout`
// uses plain `u64 / i32 / i64`. COALESCE NULLs to defaults.
const FIELDS: &str = "id, \
    COALESCE(uid, 0) AS uid, \
    username, tel, \
    COALESCE(status, 0) AS status, \
    COALESCE(ctime, 0) AS ctime";

pub async fn find_by_uid(pool: &MySqlPool, uid: u64) -> Result<Option<MemberLogout>, sqlx::Error> {
    let sql =
        format!("SELECT {FIELDS} FROM phpyun_member_logout WHERE uid = ? ORDER BY id DESC LIMIT 1");
    sqlx::query_as::<_, MemberLogout>(&sql)
        .bind(uid)
        .fetch_optional(pool)
        .await
}

pub async fn create(
    pool: &MySqlPool,
    uid: u64,
    username: &str,
    tel: Option<&str>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_member_logout (uid, username, tel, status, ctime) VALUES (?, ?, ?, 1, ?)",
    )
    .bind(uid)
    .bind(username)
    .bind(tel.unwrap_or(""))
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

/// Admin action: approve deletion (status=2 = completed).
pub async fn approve(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_member_logout SET status = 2 WHERE id = ? AND status = 1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

/// Admin action: reject deletion (status=3).
pub async fn reject(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_member_logout SET status = 3 WHERE id = ? AND status = 1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub async fn list_admin(
    pool: &MySqlPool,
    status: Option<i32>,
    keyword: Option<&str>,
    kw_type: i32,
    offset: u64,
    limit: u64,
) -> Result<Vec<AdminLogoutListRow>, sqlx::Error> {
    let limit = phpyun_core::numeric::checked_db_i64(limit, "pagination.limit")?;
    let offset = phpyun_core::numeric::checked_db_i64(offset, "pagination.offset")?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(l.id AS UNSIGNED) AS id, CAST(COALESCE(l.uid,0) AS UNSIGNED) AS uid, \
         COALESCE(l.username,'') AS username, COALESCE(l.tel,'') AS tel, \
         CAST(COALESCE(l.status,0) AS SIGNED) AS status, CAST(COALESCE(l.ctime,0) AS SIGNED) AS ctime, \
         CAST(COALESCE(m.usertype,0) AS SIGNED) AS usertype \
         FROM phpyun_member_logout l LEFT JOIN phpyun_member m ON m.uid = l.uid WHERE 1=1",
    );
    push_logout_filters(&mut qb, status, keyword, kw_type);
    qb.push(" ORDER BY l.id DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_admin(
    pool: &MySqlPool,
    status: Option<i32>,
    keyword: Option<&str>,
    kw_type: i32,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_member_logout l WHERE 1=1");
    push_logout_filters(&mut qb, status, keyword, kw_type);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

fn push_logout_filters<'a>(
    qb: &mut QueryBuilder<'a, sqlx::MySql>,
    status: Option<i32>,
    keyword: Option<&'a str>,
    kw_type: i32,
) {
    if let Some(s) = status {
        qb.push(" AND l.status = ");
        qb.push_bind(s);
    }
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        match kw_type {
            2 => {
                qb.push(" AND l.tel LIKE ");
                qb.push_bind(format!("%{kw}%"));
            }
            3 => {
                let uid: u64 = kw.parse().unwrap_or(0);
                qb.push(" AND l.uid = ");
                qb.push_bind(uid);
            }
            _ => {
                qb.push(" AND l.username LIKE ");
                qb.push_bind(format!("%{kw}%"));
            }
        }
    }
}
