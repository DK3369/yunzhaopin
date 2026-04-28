use super::entity::InterviewTemplate;
use sqlx::MySqlPool;

// `phpyun_yqmb` real columns: id, uid, name, linkman, linktel, address,
// intertime, content, addtime, did, status, statusbody. The Rust entity
// expects `created_at` / `updated_at` — there's no `updated_at` on the PHP
// table, so we project `addtime` for both. Nullable ints get COALESCE.
const FIELDS: &str = "id, \
    COALESCE(uid, 0) AS uid, \
    name, content, address, linkman, linktel, intertime, \
    COALESCE(status, 0) AS status, \
    COALESCE(addtime, 0) AS created_at, \
    COALESCE(addtime, 0) AS updated_at";

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
    // `phpyun_yqmb` has no `created_at` / `updated_at` columns — only
    // `addtime` (and `did` / `statusbody`). Map both timestamps onto
    // `addtime`; the Rust caller's `updated_at` semantics is lost on the
    // PHP table but that's a property of the legacy schema we share with.
    let res = sqlx::query(
        r#"INSERT INTO phpyun_yqmb
           (uid, name, content, address, linkman, linktel, intertime, status, addtime)
           VALUES (?, ?, ?, ?, ?, ?, ?, 1, ?)"#,
    )
    .bind(c.uid)
    .bind(c.name)
    .bind(c.content)
    .bind(c.address)
    .bind(c.linkman)
    .bind(c.linktel)
    .bind(c.intertime)
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
    // No `updated_at` column on `phpyun_yqmb` — the Rust API exposes one
    // but the underlying table only has `addtime`, which we don't bump on
    // updates (matches PHP behaviour: addtime is creation time only).
    let _ = now; // kept on the signature for source compatibility
    let res = sqlx::query(
        r#"UPDATE phpyun_yqmb SET
              name      = COALESCE(?, name),
              content   = COALESCE(?, content),
              address   = COALESCE(?, address),
              linkman   = COALESCE(?, linkman),
              linktel   = COALESCE(?, linktel),
              intertime = COALESCE(?, intertime),
              status    = COALESCE(?, status)
           WHERE id = ? AND uid = ?"#,
    )
    .bind(u.name)
    .bind(u.content)
    .bind(u.address)
    .bind(u.linkman)
    .bind(u.linktel)
    .bind(u.intertime)
    .bind(u.status)
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
