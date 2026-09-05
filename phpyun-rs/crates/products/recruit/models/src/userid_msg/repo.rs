//! `phpyun_userid_msg` — PHP `job.model.php::addYqms` interview invitations.

use super::entity::UseridMsg;
use sqlx::MySqlPool;

const FIELDS: &str = "id, COALESCE(uid,0) AS uid, COALESCE(title,'') AS title, \
    COALESCE(content,'') AS content, COALESCE(fid,0) AS fid, COALESCE(fname,'') AS fname, \
    COALESCE(`type`,0) AS `type`, CAST(COALESCE(datetime,0) AS SIGNED) AS datetime, \
    COALESCE(is_browse,0) AS is_browse, COALESCE(address,'') AS address, \
    COALESCE(intertime,'') AS intertime, COALESCE(linkman,'') AS linkman, \
    COALESCE(linktel,'') AS linktel, COALESCE(jobid,0) AS jobid, \
    COALESCE(jobname,'') AS jobname, COALESCE(did,0) AS did, \
    COALESCE(x,'') AS x, COALESCE(y,'') AS y, COALESCE(mappic,'') AS mappic, \
    COALESCE(isdel,9) AS isdel, COALESCE(remark,'') AS remark";

pub struct UseridMsgCreate<'a> {
    pub uid: u64,
    pub title: &'a str,
    pub content: &'a str,
    pub fid: u64,
    pub fname: &'a str,
    pub datetime: i64,
    pub address: &'a str,
    pub intertime: &'a str,
    pub linkman: &'a str,
    pub linktel: &'a str,
    pub jobid: u64,
    pub jobname: &'a str,
    pub did: u64,
    pub x: &'a str,
    pub y: &'a str,
    pub mappic: Option<&'a str>,
}

pub async fn insert(pool: &MySqlPool, c: UseridMsgCreate<'_>) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_userid_msg
           (uid, title, content, fid, fname, type, datetime, `default`, is_browse,
            address, intertime, linkman, linktel, jobid, jobname, did, x, y, mappic, isdel)
           VALUES (?, ?, ?, ?, ?, 0, ?, 0, 1, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 9)"#,
    )
    .bind(c.uid)
    .bind(c.title)
    .bind(c.content)
    .bind(c.fid)
    .bind(c.fname)
    .bind(c.datetime)
    .bind(c.address)
    .bind(c.intertime)
    .bind(c.linkman)
    .bind(c.linktel)
    .bind(c.jobid)
    .bind(c.jobname)
    .bind(c.did)
    .bind(c.x)
    .bind(c.y)
    .bind(c.mappic.unwrap_or(""))
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

/// PHP `member/user/invite`: `uid = me AND type <> 1 AND isdel = 9`.
pub async fn list_by_uid(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<UseridMsg>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_userid_msg \
         WHERE uid = ? AND COALESCE(`type`,0) <> 1 AND COALESCE(isdel,9) = 9 \
         ORDER BY id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, UseridMsg>(&sql)
        .bind(uid)
        .bind(phpyun_core::numeric::checked_db_i64(
            limit,
            "pagination.limit",
        )?)
        .bind(phpyun_core::numeric::checked_db_i64(
            offset,
            "pagination.offset",
        )?)
        .fetch_all(pool)
        .await
}

pub async fn count_by_uid(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_userid_msg \
         WHERE uid = ? AND COALESCE(`type`,0) <> 1 AND COALESCE(isdel,9) = 9",
    )
    .bind(uid)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn find_by_id_uid(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
) -> Result<Option<UseridMsg>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_userid_msg \
         WHERE id = ? AND uid = ? AND COALESCE(isdel,9) = 9 LIMIT 1"
    );
    sqlx::query_as::<_, UseridMsg>(&sql)
        .bind(id)
        .bind(uid)
        .fetch_optional(pool)
        .await
}

/// PHP `job.model.php::setYqms` — `is_browse` 3=agree / 4=reject.
pub async fn set_browse(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
    browse: i32,
    remark: &str,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_userid_msg SET is_browse = ?, remark = ? \
         WHERE id = ? AND uid = ? AND COALESCE(isdel,9) = 9",
    )
    .bind(browse)
    .bind(remark)
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

/// PHP `delYqms` for usertype=1: `isdel = 1`.
pub async fn hide_by_uid(pool: &MySqlPool, id: u64, uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_userid_msg SET isdel = 1 \
         WHERE id = ? AND uid = ? AND COALESCE(isdel,9) = 9",
    )
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

/// PHP `member/com/invite`: `fid = me AND isdel = 9`.
pub async fn list_by_fid(
    pool: &MySqlPool,
    fid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<UseridMsg>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_userid_msg \
         WHERE fid = ? AND COALESCE(isdel,9) = 9 \
         ORDER BY id DESC LIMIT ? OFFSET ?"
    );
    sqlx::query_as::<_, UseridMsg>(&sql)
        .bind(fid)
        .bind(phpyun_core::numeric::checked_db_i64(
            limit,
            "pagination.limit",
        )?)
        .bind(phpyun_core::numeric::checked_db_i64(
            offset,
            "pagination.offset",
        )?)
        .fetch_all(pool)
        .await
}

pub async fn count_by_fid(pool: &MySqlPool, fid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_userid_msg \
         WHERE fid = ? AND COALESCE(isdel,9) = 9",
    )
    .bind(fid)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

/// PHP `delYqms` for usertype=2: `isdel = 2`.
pub async fn hide_by_fid(pool: &MySqlPool, id: u64, fid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_userid_msg SET isdel = 2 \
         WHERE id = ? AND fid = ? AND COALESCE(isdel,9) = 9",
    )
    .bind(id)
    .bind(fid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

/// PHP shield-company: hide remaining invites from that company.
pub async fn hide_by_uid_fid(pool: &MySqlPool, uid: u64, fid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_userid_msg SET isdel = 1 \
         WHERE uid = ? AND fid = ? AND COALESCE(isdel,9) = 9",
    )
    .bind(uid)
    .bind(fid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}
