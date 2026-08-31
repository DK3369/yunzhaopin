use super::entity::InterviewTemplate;
use sqlx::{FromRow, MySqlPool, QueryBuilder};

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
    let sql = format!("SELECT {FIELDS} FROM phpyun_yqmb WHERE uid = ? ORDER BY id DESC");
    sqlx::query_as::<_, InterviewTemplate>(&sql)
        .bind(uid)
        .fetch_all(pool)
        .await
}

pub async fn count_by_uid(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_yqmb WHERE uid = ?")
        .bind(uid)
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn find_by_id(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
) -> Result<Option<InterviewTemplate>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_yqmb WHERE id = ? AND uid = ?");
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

pub async fn create(pool: &MySqlPool, c: TplCreate<'_>, now: i64) -> Result<u64, sqlx::Error> {
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

/// PHP `company_interview::index_action` 行（`yqmb` + 企业名）。
#[derive(Debug, Clone, FromRow)]
pub struct AdminYqmbRow {
    pub id: u64,
    pub uid: u64,
    pub name: String,
    pub linkman: String,
    pub linktel: String,
    pub address: String,
    pub intertime: i64,
    pub content: String,
    pub addtime: i64,
    pub status: i32,
    pub statusbody: String,
    pub comname: String,
}

const ADMIN_FIELDS: &str = "\
    CAST(y.id AS UNSIGNED) AS id, \
    CAST(COALESCE(y.uid, 0) AS UNSIGNED) AS uid, \
    COALESCE(y.name, '') AS name, \
    COALESCE(y.linkman, '') AS linkman, \
    COALESCE(y.linktel, '') AS linktel, \
    COALESCE(y.address, '') AS address, \
    COALESCE(y.intertime, 0) AS intertime, \
    COALESCE(y.content, '') AS content, \
    COALESCE(y.addtime, 0) AS addtime, \
    COALESCE(y.status, 0) AS status, \
    COALESCE(y.statusbody, '') AS statusbody, \
    COALESCE(c.name, '') AS comname";

#[derive(Debug, Default, Clone)]
pub struct AdminYqmbFilter<'a> {
    pub keyword: Option<&'a str>,
    /// PHP `type`: 1 公司名 / 2 uid
    pub keyword_type: i32,
    pub status: Option<i32>,
}

fn push_admin_yqmb_filters<'a>(qb: &mut QueryBuilder<'a, sqlx::MySql>, f: &AdminYqmbFilter<'a>) {
    if let Some(kw) = f.keyword.map(str::trim).filter(|s| !s.is_empty()) {
        match f.keyword_type {
            2 => {
                if let Ok(uid) = kw.parse::<u64>() {
                    qb.push(" AND y.uid = ");
                    qb.push_bind(uid);
                } else {
                    qb.push(" AND 1=0");
                }
            }
            _ => {
                qb.push(" AND c.name LIKE ");
                qb.push_bind(format!("%{kw}%"));
            }
        }
    }
    if let Some(st) = f.status {
        qb.push(" AND y.status = ");
        qb.push_bind(st);
    }
}

pub async fn admin_php_list(
    pool: &MySqlPool,
    f: &AdminYqmbFilter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<AdminYqmbRow>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(ADMIN_FIELDS);
    qb.push(" FROM phpyun_yqmb y LEFT JOIN phpyun_company c ON c.uid = y.uid WHERE 1=1");
    push_admin_yqmb_filters(&mut qb, f);
    qb.push(" ORDER BY y.status ASC, y.id DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as::<AdminYqmbRow>().fetch_all(pool).await
}

pub async fn admin_php_count(pool: &MySqlPool, f: &AdminYqmbFilter<'_>) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT COUNT(*) FROM phpyun_yqmb y LEFT JOIN phpyun_company c ON c.uid = y.uid WHERE 1=1",
    );
    push_admin_yqmb_filters(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn admin_insert(
    pool: &MySqlPool,
    uid: u64,
    name: &str,
    linkman: &str,
    linktel: &str,
    content: &str,
    address: &str,
    intertime: i64,
    status: i32,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_yqmb
           (uid, name, linkman, linktel, content, address, intertime, status, addtime)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
    )
    .bind(uid)
    .bind(name)
    .bind(linkman)
    .bind(linktel)
    .bind(content)
    .bind(address)
    .bind(intertime)
    .bind(status)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn admin_update(
    pool: &MySqlPool,
    id: u64,
    uid: u64,
    name: &str,
    linkman: &str,
    linktel: &str,
    content: &str,
    address: &str,
    intertime: i64,
    status: i32,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"UPDATE phpyun_yqmb SET
              uid = ?, name = ?, linkman = ?, linktel = ?, content = ?,
              address = ?, intertime = ?, status = ?
           WHERE id = ?"#,
    )
    .bind(uid)
    .bind(name)
    .bind(linkman)
    .bind(linktel)
    .bind(content)
    .bind(address)
    .bind(intertime)
    .bind(status)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn admin_set_status(
    pool: &MySqlPool,
    ids: &[u64],
    status: i32,
    statusbody: &str,
) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("UPDATE phpyun_yqmb SET status = ");
    qb.push_bind(status);
    qb.push(", statusbody = ");
    qb.push_bind(statusbody);
    qb.push(" WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn admin_delete_ids(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("DELETE FROM phpyun_yqmb WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}
