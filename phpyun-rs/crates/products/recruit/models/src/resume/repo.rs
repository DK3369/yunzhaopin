use super::entity::Resume;
use sqlx::{MySqlPool, QueryBuilder};

// PHPYun `phpyun_resume` real column name `edu` -> Rust `education`; other column names match.
// Covers all fields used by the PHPYun WAP resume detail page.
const FIELDS: &str = "\
    uid, name, COALESCE(nametype, 0) AS nametype, COALESCE(sex, 0) AS sex, \
    birthday, COALESCE(marriage, 0) AS marriage, \
    COALESCE(edu, 0) AS education, \
    telphone, telhome, email, photo, COALESCE(phototype, 0) AS phototype, \
    COALESCE(status, 0) AS status, COALESCE(r_status, 0) AS r_status, \
    COALESCE(def_job, 0) AS def_job, COALESCE(lastupdate, 0) AS lastupdate, \
    height, weight, nationality, living, domicile, homepage, address, \
    description, idcard, idcard_pic, \
    COALESCE(idcard_status, 0) AS idcard_status, \
    COALESCE(moblie_status, 0) AS moblie_status, \
    COALESCE(email_status, 0) AS email_status, \
    COALESCE(exp, 0) AS exp, \
    resume_photo, qq, wxewm, tag, label, retire, \
    COALESCE(resumetime, 0) AS resumetime, \
    COALESCE(login_date, 0) AS login_date, \
    COALESCE(did, 0) AS did";

/// Cheap existence check — `SELECT 1`. Counterpart of
/// [`crate::company::repo::exists_by_uid`].
pub async fn exists_by_uid(pool: &MySqlPool, uid: u64) -> Result<bool, sqlx::Error> {
    let row: Option<(i64,)> = sqlx::query_as("SELECT 1 FROM phpyun_resume WHERE uid = ? LIMIT 1")
        .bind(uid)
        .fetch_optional(pool)
        .await?;
    Ok(row.is_some())
}

pub async fn find_by_uid(pool: &MySqlPool, uid: u64) -> Result<Option<Resume>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_resume WHERE uid = ? LIMIT 1");
    sqlx::query_as::<_, Resume>(&sql)
        .bind(uid)
        .fetch_optional(pool)
        .await
}

/// Publicly visible: status=1 (public) + r_status=1. `status=3` (visible only to applied companies) does not go through here.
pub async fn find_public(pool: &MySqlPool, uid: u64) -> Result<Option<Resume>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_resume
         WHERE uid = ? AND status = 1 AND r_status = 1 LIMIT 1"
    );
    sqlx::query_as::<_, Resume>(&sql)
        .bind(uid)
        .fetch_optional(pool)
        .await
}

// ==================== Public search (company perspective) ====================

#[derive(Debug, Default, Clone)]
pub struct ResumeFilter<'a> {
    /// LIKE match against `name`. Resumes with nametype=2 (hidden real name) also participate in matching, but their names are masked when returned.
    pub keyword: Option<&'a str>,
    pub education: Option<i32>,
    pub exp: Option<i32>,
    pub job1: Option<i32>,
    pub province_id: Option<i32>,
    pub city_id: Option<i32>,
    pub sex: Option<i32>,
    pub marriage: Option<i32>,
    pub did: u32,
}

pub async fn list_public(
    pool: &MySqlPool,
    f: &ResumeFilter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<Resume>, sqlx::Error> {
    // `phpyun_resume.uid` is a non-unique KEY; this database has duplicate
    // rows per person. Public list is one card per uid (PHP lists people,
    // not raw table copies).
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(FIELDS);
    qb.push(" FROM phpyun_resume WHERE status = 1 AND r_status = 1 AND did = ");
    qb.push_bind(f.did);
    push_filters(&mut qb, f);
    qb.push(
        " AND (uid, lastupdate) IN (SELECT uid, MAX(lastupdate) FROM phpyun_resume WHERE status = 1 AND r_status = 1 AND did = ",
    );
    qb.push_bind(f.did);
    push_filters(&mut qb, f);
    qb.push(" GROUP BY uid) ORDER BY lastupdate DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as::<Resume>().fetch_all(pool).await
}

pub async fn count_public(pool: &MySqlPool, f: &ResumeFilter<'_>) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT COUNT(DISTINCT uid) FROM phpyun_resume WHERE status = 1 AND r_status = 1 AND did = ",
    );
    qb.push_bind(f.did);
    push_filters(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

fn push_filters<'a>(qb: &mut QueryBuilder<'a, sqlx::MySql>, f: &ResumeFilter<'a>) {
    if let Some(kw) = f.keyword {
        if !kw.is_empty() {
            qb.push(" AND name LIKE ");
            qb.push_bind(format!("%{kw}%"));
        }
    }
    if let Some(v) = f.education {
        qb.push(" AND edu = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.exp {
        qb.push(" AND exp = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.job1 {
        qb.push(
            " AND uid IN (SELECT uid FROM phpyun_resume_expect WHERE FIND_IN_SET(",
        );
        qb.push_bind(v);
        qb.push(", job_classid) OR job_classid = ");
        qb.push_bind(v.to_string());
        qb.push(")");
    }
    if let Some(v) = f.province_id {
        qb.push(" AND uid IN (SELECT uid FROM phpyun_resume_expect WHERE provinceid = ");
        qb.push_bind(v);
        qb.push(")");
    }
    if let Some(v) = f.city_id {
        qb.push(" AND uid IN (SELECT uid FROM phpyun_resume_expect WHERE cityid = ");
        qb.push_bind(v);
        qb.push(")");
    }
    if let Some(v) = f.sex {
        qb.push(" AND sex = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.marriage {
        qb.push(" AND marriage = ");
        qb.push_bind(v);
    }
}

/// SELECT-then-INSERT guard for `phpyun_resume`. PHPYun's schema only
/// has `KEY uid` (non-unique) on this table — `INSERT IGNORE` would
/// happily duplicate on every wizard step. Used by `get_mine` /
/// `update_mine` paths where we want at-most-one row per uid.
///
/// Race window between SELECT and INSERT exists; same race as PHPYun's
/// PHP code, mitigated by the fact the only races come from concurrent
/// wizard saves which all carry the same uid (worst case: an extra row).
pub async fn ensure_row(
    pool: &sqlx::MySqlPool,
    uid: u64,
    did: u32,
    now: i64,
) -> Result<(), sqlx::Error> {
    let exists: Option<(i64,)> =
        sqlx::query_as("SELECT 1 FROM phpyun_resume WHERE uid = ? LIMIT 1")
            .bind(uid)
            .fetch_optional(pool)
            .await?;
    if exists.is_some() {
        return Ok(());
    }
    sqlx::query(
        r#"INSERT INTO phpyun_resume (uid, did, status, r_status, nametype, sex,
           marriage, edu, phototype, def_job, lastupdate)
           VALUES (?, ?, 2, 1, 1, 0, 0, 0, 0, 0, ?)"#,
    )
    .bind(uid)
    .bind(did)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(())
}

/// Insert the row inside a registration transaction. The caller already
/// guarantees this is a freshly-created uid (member INSERT just succeeded
/// in the same tx), so a SELECT first would be redundant. Kept generic on
/// `Executor` so the registration service can pass `&mut **tx` directly.
pub async fn ensure_row_in_tx<'e, E>(
    exec: E,
    uid: u64,
    did: u32,
    now: i64,
) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::MySql>,
{
    sqlx::query(
        r#"INSERT INTO phpyun_resume (uid, did, status, r_status, nametype, sex,
           marriage, edu, phototype, def_job, lastupdate)
           VALUES (?, ?, 2, 1, 1, 0, 0, 0, 0, 0, ?)"#,
    )
    .bind(uid)
    .bind(did)
    .bind(now)
    .execute(exec)
    .await?;
    Ok(())
}

/// Bare ensure_row — only sets `uid`; every other column relies on the
/// MySQL default. Used by `seed_role_rows` when a member's usertype is set
/// post-registration. SELECT-then-INSERT discipline.
pub async fn ensure_uid_only(pool: &sqlx::MySqlPool, uid: u64) -> Result<(), sqlx::Error> {
    let exists: Option<(i64,)> =
        sqlx::query_as("SELECT 1 FROM phpyun_resume WHERE uid = ? LIMIT 1")
            .bind(uid)
            .fetch_optional(pool)
            .await?;
    if exists.is_some() {
        return Ok(());
    }
    sqlx::query("INSERT INTO phpyun_resume (uid) VALUES (?)")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub struct ResumeUpdate<'a> {
    pub name: Option<&'a str>,
    pub nametype: Option<i32>,
    pub sex: Option<i32>,
    pub birthday: Option<&'a str>,
    pub marriage: Option<i32>,
    pub education: Option<i32>,
    pub telphone: Option<&'a str>,
    pub email: Option<&'a str>,
    pub photo: Option<&'a str>,
}

/// Update the resume main table — only non-None fields are changed.
/// To keep the SQL static (faster for sqlx), uses COALESCE rather than dynamically building SQL.
pub async fn update(
    pool: &MySqlPool,
    uid: u64,
    u: ResumeUpdate<'_>,
    now: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"UPDATE phpyun_resume SET
            name       = COALESCE(?, name),
            nametype   = COALESCE(?, nametype),
            sex        = COALESCE(?, sex),
            birthday   = COALESCE(?, birthday),
            marriage   = COALESCE(?, marriage),
            edu        = COALESCE(?, edu),
            telphone   = COALESCE(?, telphone),
            email      = COALESCE(?, email),
            photo      = COALESCE(?, photo),
            lastupdate = ?
           WHERE uid = ?"#,
    )
    .bind(u.name)
    .bind(u.nametype)
    .bind(u.sex)
    .bind(u.birthday)
    .bind(u.marriage)
    .bind(u.education)
    .bind(u.telphone)
    .bind(u.email)
    .bind(u.photo)
    .bind(now)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(())
}

/// Refresh the resume — bump `lastupdate` to the current time. The public list is sorted by `lastupdate` DESC,
/// so after refreshing the resume will move to the front of search results.
pub async fn touch_lastupdate(pool: &MySqlPool, uid: u64, now: i64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_resume SET lastupdate = ? WHERE uid = ?")
        .bind(now)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

/// Change resume display status: 1 = public, 2 = hidden, 3 = visible only to applied companies
pub async fn update_status(pool: &MySqlPool, uid: u64, status: i32) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE phpyun_resume SET status = ? WHERE uid = ?")
        .bind(status)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

/// PHP `users_resume` 审核列 `r_status`：0 待审 / 1 通过 / 2 未通过。
pub async fn update_r_status(pool: &MySqlPool, uid: u64, r_status: i32) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_resume SET r_status = ? WHERE uid = ?")
        .bind(r_status)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct AdminResumeRow {
    pub uid: u64,
    pub name: String,
    pub r_status: i32,
    pub status: i32,
    pub lastupdate: i64,
}

pub async fn list_admin(
    pool: &MySqlPool,
    r_status: Option<i32>,
    keyword: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<AdminResumeRow>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        r#"SELECT CAST(uid AS UNSIGNED) AS uid,
                  COALESCE(name, '') AS name,
                  CAST(COALESCE(r_status, 0) AS SIGNED) AS r_status,
                  CAST(COALESCE(status, 0) AS SIGNED) AS status,
                  CAST(COALESCE(lastupdate, 0) AS SIGNED) AS lastupdate
           FROM phpyun_resume WHERE 1=1"#,
    );
    push_admin_resume_filters(&mut qb, r_status, keyword);
    qb.push(" ORDER BY lastupdate DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as::<AdminResumeRow>().fetch_all(pool).await
}

pub async fn count_admin(
    pool: &MySqlPool,
    r_status: Option<i32>,
    keyword: Option<&str>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_resume WHERE 1=1");
    push_admin_resume_filters(&mut qb, r_status, keyword);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

fn push_admin_resume_filters<'a>(
    qb: &mut QueryBuilder<'a, sqlx::MySql>,
    r_status: Option<i32>,
    keyword: Option<&'a str>,
) {
    if let Some(st) = r_status {
        qb.push(" AND r_status = ");
        qb.push_bind(st);
    }
    if let Some(kw) = keyword {
        if !kw.is_empty() {
            qb.push(" AND name LIKE ");
            qb.push_bind(format!("%{kw}%"));
        }
    }
}

/// Cheap getter for the avatar/photo column only — used by features that
/// render a user card (asker/answerer/viewer) and don't need the full Resume
/// entity.
pub async fn photo_for_uid(pool: &MySqlPool, uid: u64) -> Result<Option<String>, sqlx::Error> {
    let row: Option<(Option<String>,)> =
        sqlx::query_as("SELECT photo FROM phpyun_resume WHERE uid = ? LIMIT 1")
            .bind(uid)
            .fetch_optional(pool)
            .await?;
    Ok(row.and_then(|(p,)| p))
}

/// Resolve a jobseeker's default `phpyun_resume_expect.id` (`def_job`).
/// Returns `0` when the resume is hidden / unreviewed, used by PHP's legacy
/// `wap/resume/index::showuid_action` short-URL redirect.
pub async fn default_eid(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let row: Option<(u64,)> = sqlx::query_as(
        "SELECT CAST(COALESCE(def_job, 0) AS UNSIGNED) FROM phpyun_resume \
         WHERE uid = ? AND COALESCE(r_status, 0) = 1 LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(n,)| n).unwrap_or(0))
}

pub async fn find_uid_by_name(pool: &MySqlPool, name: &str) -> Result<Option<u64>, sqlx::Error> {
    let row: Option<(u64,)> = sqlx::query_as(
        "SELECT CAST(uid AS UNSIGNED) FROM phpyun_resume WHERE name = ? LIMIT 1",
    )
    .bind(name)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(uid,)| uid))
}
