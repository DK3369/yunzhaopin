//! PHP-shaped admin RBAC / admin_navigation / version queries.

use serde::Serialize;
use sqlx::{FromRow, MySqlPool, QueryBuilder};

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct PhpAdminUserRow {
    pub uid: u64,
    pub username: String,
    pub name: String,
    pub group_name: String,
    pub mobile: String,
    pub weixin: String,
    pub qq: String,
    pub num: String,
    pub call_num: String,
    pub tuoxin_num: String,
    pub follow_num: String,
    pub deal_num: String,
    pub month_deal_num: String,
    pub jobtai_ranking: i32,
    pub is_crm: i32,
    pub isdid: i32,
    pub m_id: i32,
    pub crm_city: String,
    pub crm_duty: String,
    pub control_login: String,
    pub index_lookstatistc: i32,
    pub photo: String,
    pub ewm: String,
    pub status: i32,
    pub did: i32,
    pub wxid: String,
}

const USER_FIELDS: &str = "\
    CAST(u.uid AS UNSIGNED) AS uid, \
    COALESCE(u.username,'') AS username, \
    COALESCE(u.name,'') AS name, \
    COALESCE(g.group_name,'') AS group_name, \
    COALESCE(u.moblie,'') AS mobile, \
    COALESCE(u.weixin,'') AS weixin, \
    COALESCE(u.qq,'') AS qq, \
    '' AS num, \
    '' AS call_num, \
    '' AS tuoxin_num, \
    '' AS follow_num, \
    '' AS deal_num, \
    '' AS month_deal_num, \
    CAST(0 AS SIGNED) AS jobtai_ranking, \
    CAST(COALESCE(u.is_crm,0) AS SIGNED) AS is_crm, \
    CAST(COALESCE(u.isdid,0) AS SIGNED) AS isdid, \
    CAST(COALESCE(u.m_id,0) AS SIGNED) AS m_id, \
    COALESCE(u.crm_city,'') AS crm_city, \
    COALESCE(u.crm_duty,'') AS crm_duty, \
    COALESCE(u.control_login,'') AS control_login, \
    CAST(COALESCE(u.index_lookstatistc,2) AS SIGNED) AS index_lookstatistc, \
    COALESCE(u.photo,'') AS photo, \
    COALESCE(u.ewm,'') AS ewm, \
    CAST(COALESCE(u.status,0) AS SIGNED) AS status, \
    CAST(COALESCE(u.did,0) AS SIGNED) AS did, \
    COALESCE(u.wxid,'') AS wxid";

fn push_user_where(qb: &mut QueryBuilder<'_, sqlx::MySql>, keyword: Option<&str>, m_id: Option<i32>) {
    qb.push(" FROM phpyun_admin_user u LEFT JOIN phpyun_admin_user_group g ON g.id = u.m_id WHERE 1=1");
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        let like = format!("%{kw}%");
        qb.push(" AND (u.username LIKE ");
        qb.push_bind(like.clone());
        qb.push(" OR u.name LIKE ");
        qb.push_bind(like);
        qb.push(")");
    }
    if let Some(mid) = m_id.filter(|n| *n > 0) {
        qb.push(" AND u.m_id = ");
        qb.push_bind(mid);
    }
}

pub async fn php_list_users(
    pool: &MySqlPool,
    keyword: Option<&str>,
    m_id: Option<i32>,
    offset: u64,
    limit: u64,
) -> Result<Vec<PhpAdminUserRow>, sqlx::Error> {
    let mut qb = QueryBuilder::new("SELECT ");
    qb.push(USER_FIELDS);
    push_user_where(&mut qb, keyword, m_id);
    qb.push(" ORDER BY u.uid ASC LIMIT ");
    qb.push_bind(limit as i64);
    qb.push(" OFFSET ");
    qb.push_bind(offset as i64);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn php_count_users(
    pool: &MySqlPool,
    keyword: Option<&str>,
    m_id: Option<i32>,
) -> Result<u64, sqlx::Error> {
    let mut qb = QueryBuilder::new("SELECT COUNT(*)");
    push_user_where(&mut qb, keyword, m_id);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn php_get_user(pool: &MySqlPool, uid: u64) -> Result<Option<PhpAdminUserRow>, sqlx::Error> {
    let sql = format!("SELECT {USER_FIELDS} FROM phpyun_admin_user u \
         LEFT JOIN phpyun_admin_user_group g ON g.id = u.m_id WHERE u.uid = ? LIMIT 1");
    sqlx::query_as::<_, PhpAdminUserRow>(&sql)
        .bind(uid)
        .fetch_optional(pool)
        .await
}

pub async fn php_username_taken(pool: &MySqlPool, username: &str, except_uid: u64) -> Result<bool, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_admin_user WHERE username = ? AND uid <> ?",
    )
    .bind(username)
    .bind(except_uid)
    .fetch_one(pool)
    .await?;
    Ok(n > 0)
}

pub struct PhpAdminUserSave<'a> {
    pub uid: Option<u64>,
    pub username: &'a str,
    pub name: &'a str,
    pub m_id: i32,
    pub mobile: &'a str,
    pub weixin: &'a str,
    pub qq: &'a str,
    pub is_crm: i32,
    pub num: &'a str,
    pub call_num: &'a str,
    pub tuoxin_num: &'a str,
    pub follow_num: &'a str,
    pub deal_num: &'a str,
    pub month_deal_num: &'a str,
    pub jobtai_ranking: i32,
    pub crm_duty: &'a str,
    pub crm_city: &'a str,
    pub photo: Option<&'a str>,
    pub ewm: Option<&'a str>,
    pub control_login: &'a str,
    pub index_lookstatistc: i32,
    pub isdid: i32,
    pub password_hash: Option<&'a str>,
}

pub async fn php_save_user(pool: &MySqlPool, s: PhpAdminUserSave<'_>) -> Result<u64, sqlx::Error> {
    let _ = (s.num, s.call_num, s.tuoxin_num, s.follow_num, s.deal_num, s.month_deal_num, s.jobtai_ranking);
    if let Some(uid) = s.uid.filter(|n| *n > 0) {
        sqlx::query(
            "UPDATE phpyun_admin_user SET username=?, name=?, m_id=?, moblie=?, weixin=?, qq=?, \
             is_crm=?, crm_duty=?, crm_city=?, control_login=?, index_lookstatistc=?, isdid=? \
             WHERE uid=?",
        )
        .bind(s.username)
        .bind(s.name)
        .bind(s.m_id)
        .bind(s.mobile)
        .bind(s.weixin)
        .bind(s.qq)
        .bind(s.is_crm)
        .bind(s.crm_duty)
        .bind(s.crm_city)
        .bind(s.control_login)
        .bind(s.index_lookstatistc)
        .bind(s.isdid)
        .bind(uid)
        .execute(pool)
        .await?;
        if let Some(pw) = s.password_hash.filter(|p| !p.is_empty()) {
            sqlx::query("UPDATE phpyun_admin_user SET password=? WHERE uid=?")
                .bind(pw)
                .bind(uid)
                .execute(pool)
                .await?;
        }
        if let Some(p) = s.photo.filter(|p| !p.is_empty()) {
            sqlx::query("UPDATE phpyun_admin_user SET photo=? WHERE uid=?")
                .bind(p)
                .bind(uid)
                .execute(pool)
                .await?;
        }
        if let Some(p) = s.ewm.filter(|p| !p.is_empty()) {
            sqlx::query("UPDATE phpyun_admin_user SET ewm=? WHERE uid=?")
                .bind(p)
                .bind(uid)
                .execute(pool)
                .await?;
        }
        return Ok(uid);
    }
    let pw = s.password_hash.unwrap_or("");
    let photo = s.photo.unwrap_or("");
    let ewm = s.ewm.unwrap_or("");
    Ok(sqlx::query(
        "INSERT INTO phpyun_admin_user \
         (username, name, password, m_id, moblie, weixin, qq, is_crm, crm_duty, crm_city, photo, ewm, \
          control_login, index_lookstatistc, isdid, status, did) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 1, 0)",
    )
    .bind(s.username)
    .bind(s.name)
    .bind(pw)
    .bind(s.m_id)
    .bind(s.mobile)
    .bind(s.weixin)
    .bind(s.qq)
    .bind(s.is_crm)
    .bind(s.crm_duty)
    .bind(s.crm_city)
    .bind(photo)
    .bind(ewm)
    .bind(s.control_login)
    .bind(s.index_lookstatistc)
    .bind(s.isdid)
    .execute(pool)
    .await?
    .last_insert_id())
}

pub async fn php_delete_user(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    sqlx::query("UPDATE phpyun_company SET crm_uid=0, crm_status=0 WHERE crm_uid=?")
        .bind(uid)
        .execute(pool)
        .await?;
    sqlx::query("UPDATE phpyun_company_order SET crm_uid=0 WHERE crm_uid=?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(sqlx::query("DELETE FROM phpyun_admin_user WHERE uid=?")
        .bind(uid)
        .execute(pool)
        .await?
        .rows_affected())
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct PhpAdminGroupRow {
    pub id: u64,
    pub group_name: String,
    pub group_type: i32,
    pub group_power: String,
    pub did: i32,
    pub num: i64,
}

pub async fn php_list_groups(
    pool: &MySqlPool,
    offset: u64,
    limit: u64,
) -> Result<Vec<PhpAdminGroupRow>, sqlx::Error> {
    sqlx::query_as::<_, PhpAdminGroupRow>(
        "SELECT CAST(g.id AS UNSIGNED) AS id, COALESCE(g.group_name,'') AS group_name, \
         CAST(COALESCE(g.group_type,0) AS SIGNED) AS group_type, \
         COALESCE(g.group_power,'') AS group_power, \
         CAST(COALESCE(g.did,0) AS SIGNED) AS did, \
         CAST((SELECT COUNT(*) FROM phpyun_admin_user u WHERE u.m_id = g.id) AS SIGNED) AS num \
         FROM phpyun_admin_user_group g ORDER BY g.id ASC LIMIT ? OFFSET ?",
    )
    .bind(limit as i64)
    .bind(offset as i64)
    .fetch_all(pool)
    .await
}

pub async fn php_count_groups(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_admin_user_group")
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn php_get_group(pool: &MySqlPool, id: u64) -> Result<Option<PhpAdminGroupRow>, sqlx::Error> {
    sqlx::query_as::<_, PhpAdminGroupRow>(
        "SELECT CAST(g.id AS UNSIGNED) AS id, COALESCE(g.group_name,'') AS group_name, \
         CAST(COALESCE(g.group_type,0) AS SIGNED) AS group_type, \
         COALESCE(g.group_power,'') AS group_power, \
         CAST(COALESCE(g.did,0) AS SIGNED) AS did, \
         CAST((SELECT COUNT(*) FROM phpyun_admin_user u WHERE u.m_id = g.id) AS SIGNED) AS num \
         FROM phpyun_admin_user_group g WHERE g.id = ? LIMIT 1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn php_group_name_taken(
    pool: &MySqlPool,
    name: &str,
    except_id: u64,
) -> Result<bool, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_admin_user_group WHERE group_name = ? AND id <> ?",
    )
    .bind(name)
    .bind(except_id)
    .fetch_one(pool)
    .await?;
    Ok(n > 0)
}

pub async fn php_save_group(
    pool: &MySqlPool,
    id: Option<u64>,
    name: &str,
    power: &str,
    did: i32,
) -> Result<u64, sqlx::Error> {
    if let Some(id) = id.filter(|n| *n > 0) {
        sqlx::query("UPDATE phpyun_admin_user_group SET group_name=?, group_power=? WHERE id=?")
            .bind(name)
            .bind(power)
            .bind(id)
            .execute(pool)
            .await?;
        return Ok(id);
    }
    Ok(sqlx::query(
        "INSERT INTO phpyun_admin_user_group (group_name, group_power, group_type, did) VALUES (?, ?, 1, ?)",
    )
    .bind(name)
    .bind(power)
    .bind(did)
    .execute(pool)
    .await?
    .last_insert_id())
}

pub async fn php_count_group_users(pool: &MySqlPool, group_id: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_admin_user WHERE m_id = ?")
        .bind(group_id as i64)
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn php_delete_group(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    Ok(sqlx::query("DELETE FROM phpyun_admin_user_group WHERE id=?")
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected())
}

pub async fn php_clear_qy(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let _ = (pool, uid);
    Ok(0)
}

pub async fn php_qy_userid(pool: &MySqlPool, uid: u64) -> Result<String, sqlx::Error> {
    let _ = (pool, uid);
    Ok(String::new())
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct PhpAdminNavRow {
    pub id: i64,
    pub keyid: i64,
    pub name: String,
    pub url: String,
    pub path: String,
    pub classname: String,
    pub menu: i32,
    pub sort: i32,
    pub display: i32,
    pub dids: i32,
}

const ADMIN_NAV_FIELDS: &str = "\
    CAST(id AS SIGNED) AS id, \
    CAST(COALESCE(keyid,0) AS SIGNED) AS keyid, \
    COALESCE(name,'') AS name, \
    COALESCE(url,'') AS url, \
    COALESCE(path,'') AS path, \
    COALESCE(classname,'') AS classname, \
    CAST(COALESCE(menu,0) AS SIGNED) AS menu, \
    CAST(COALESCE(sort,0) AS SIGNED) AS sort, \
    CAST(COALESCE(display,0) AS SIGNED) AS display, \
    CAST(COALESCE(dids,0) AS SIGNED) AS dids";

pub async fn php_list_admin_nav(
    pool: &MySqlPool,
    keyid: Option<i64>,
) -> Result<Vec<PhpAdminNavRow>, sqlx::Error> {
    let sql = match keyid {
        Some(_) => format!(
            "SELECT {ADMIN_NAV_FIELDS} FROM phpyun_admin_navigation WHERE keyid = ? ORDER BY sort ASC, id ASC"
        ),
        None => format!(
            "SELECT {ADMIN_NAV_FIELDS} FROM phpyun_admin_navigation ORDER BY sort ASC, id ASC"
        ),
    };
    let q = sqlx::query_as::<_, PhpAdminNavRow>(&sql);
    match keyid {
        Some(k) => q.bind(k).fetch_all(pool).await,
        None => q.fetch_all(pool).await,
    }
}

pub async fn php_admin_nav_child_keyids(pool: &MySqlPool, ids: &[i64]) -> Result<Vec<i64>, sqlx::Error> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let mut qb = QueryBuilder::new(
        "SELECT CAST(keyid AS SIGNED) FROM phpyun_admin_navigation WHERE keyid IN (",
    );
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(") GROUP BY keyid");
    let rows: Vec<(i64,)> = qb.build_query_as().fetch_all(pool).await?;
    Ok(rows.into_iter().map(|(k,)| k).collect())
}

pub async fn php_get_admin_nav(pool: &MySqlPool, id: i64) -> Result<Option<PhpAdminNavRow>, sqlx::Error> {
    let sql = format!("SELECT {ADMIN_NAV_FIELDS} FROM phpyun_admin_navigation WHERE id = ? LIMIT 1");
    sqlx::query_as::<_, PhpAdminNavRow>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn php_count_admin_nav_children(pool: &MySqlPool, id: i64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_admin_navigation WHERE keyid = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub struct PhpAdminNavSave<'a> {
    pub id: Option<i64>,
    pub keyid: i64,
    pub name: &'a str,
    pub url: &'a str,
    pub path: &'a str,
    pub classname: &'a str,
    pub display: i32,
    pub dids: i32,
    pub sort: i32,
}

pub async fn php_save_admin_nav(pool: &MySqlPool, s: PhpAdminNavSave<'_>) -> Result<u64, sqlx::Error> {
    if let Some(id) = s.id.filter(|n| *n > 0) {
        sqlx::query(
            "UPDATE phpyun_admin_navigation SET keyid=?, name=?, url=?, path=?, classname=?, display=?, dids=?, sort=? WHERE id=?",
        )
        .bind(s.keyid)
        .bind(s.name)
        .bind(s.url)
        .bind(s.path)
        .bind(s.classname)
        .bind(s.display)
        .bind(s.dids)
        .bind(s.sort)
        .bind(id)
        .execute(pool)
        .await?;
        return Ok(id as u64);
    }
    Ok(sqlx::query(
        "INSERT INTO phpyun_admin_navigation (keyid, name, url, path, classname, display, dids, sort, menu) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, 1)",
    )
    .bind(s.keyid)
    .bind(s.name)
    .bind(s.url)
    .bind(s.path)
    .bind(s.classname)
    .bind(s.display)
    .bind(s.dids)
    .bind(s.sort)
    .execute(pool)
    .await?
    .last_insert_id())
}

pub async fn php_set_admin_nav_field(
    pool: &MySqlPool,
    id: i64,
    field: &str,
    status: i32,
) -> Result<u64, sqlx::Error> {
    let col = match field {
        "display" => "display",
        "menu" => "menu",
        "dids" => "dids",
        "sort" => "sort",
        _ => return Ok(0),
    };
    let sql = format!("UPDATE phpyun_admin_navigation SET `{col}` = ? WHERE id = ?");
    Ok(sqlx::query(&sql)
        .bind(status)
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected())
}

pub async fn php_delete_admin_nav(pool: &MySqlPool, id: i64) -> Result<u64, sqlx::Error> {
    Ok(sqlx::query("DELETE FROM phpyun_admin_navigation WHERE id=?")
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected())
}

pub async fn php_mark_admin_nav_dids(pool: &MySqlPool, ids: &[i64], dids: i32) -> Result<u64, sqlx::Error> {
    sqlx::query("UPDATE phpyun_admin_navigation SET dids = 0 WHERE display <> 1")
        .execute(pool)
        .await?;
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("UPDATE phpyun_admin_navigation SET dids = ");
    qb.push_bind(dids);
    qb.push(" WHERE display <> 1 AND id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct PhpVersionRow {
    pub id: u64,
    pub version: String,
    pub code: String,
    pub ctime: i64,
}

pub async fn php_list_versions(pool: &MySqlPool) -> Result<Vec<PhpVersionRow>, sqlx::Error> {
    sqlx::query_as::<_, PhpVersionRow>(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(version,'') AS version, \
         COALESCE(code,'') AS code, CAST(COALESCE(ctime,0) AS SIGNED) AS ctime \
         FROM phpyun_version ORDER BY id ASC",
    )
    .fetch_all(pool)
    .await
}
