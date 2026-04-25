use super::entity::InterviewTemplate;
use sqlx::MySqlPool;

const FIELDS: &str =
    "id, uid, name, content, address, linkman, linktel, intertime, status, created_at, updated_at";

pub async fn list_by_uid(
    pool: &MySqlPool,
    uid: u64,
) -> Result<Vec<InterviewTemplate>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_yqmb WHERE uid = ? ORDER BY id DESC"
    );
    sqlx::query_as::<_, InterviewTemplate>(&sql)
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn count_by_uid(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_yqmb WHERE uid = ?")
            .bind(uid)
            .fetch_one(pool)
            .await?;
    Ok(n.max(0) as u64)
}

pub async fn find_by_id(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
) -> Result<Option<InterviewTemplate>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_yqmb WHERE id = ? AND uid = ?"
    );
    sqlx::query_as::<_, InterviewTemplate>(&sql)
        .bind(id)
        .bind(uid)
        .fetch_optional(pool)
        .await
}

pub struct TplCreate<'a> {
    pub uid: u64,
    pub name: &'a str,
    pub content: &'a str,
    pub address: &'a str,
    pub linkman: &'a str,
    pub linktel: &'a str,
    pub intertime: i64,
}

pub async fn create(
    pool: &MySqlPool,
    c: TplCreate<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_yqmb
           (uid, name, content, address, linkman, linktel, intertime, status, created_at, updated_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, 1, ?, ?)"#,
    )
    .bind(c.uid)
    .bind(c.name)
    .bind(c.content)
    .bind(c.address)
    .bind(c.linkman)
    .bind(c.linktel)
    .bind(c.intertime)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub struct TplUpdate<'a> {
    pub name: Option<&'a str>,
    pub content: Option<&'a str>,
    pub address: Option<&'a str>,
    pub linkman: Option<&'a str>,
    pub linktel: Option<&'a str>,
    pub intertime: Option<i64>,
    pub status: Option<i32>,
}

pub async fn update(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
    u: TplUpdate<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"UPDATE phpyun_yqmb SET
              name       = COALESCE(?, name),
              content    = COALESCE(?, content),
              address    = COALESCE(?, address),
              linkman    = COALESCE(?, linkman),
              linktel    = COALESCE(?, linktel),
              intertime  = COALESCE(?, intertime),
              status     = COALESCE(?, status),
              updated_at = ?
           WHERE id = ? AND uid = ?"#,
    )
    .bind(u.name)
    .bind(u.content)
    .bind(u.address)
    .bind(u.linkman)
    .bind(u.linktel)
    .bind(u.intertime)
    .bind(u.status)
    .bind(now)
    .bind(id)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn delete(pool: &MySqlPool, id: u64, uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_yqmb WHERE id = ? AND uid = ?")
        .bind(id)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
