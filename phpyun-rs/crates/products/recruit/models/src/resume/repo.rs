use super::entity::Resume;
use sqlx::{MySqlPool, QueryBuilder};

// PHPYun `phpyun_resume` real column name `edu` -> Rust `education`; other column names match.
// Covers all fields used by the PHPYun WAP resume detail page.
const FIELDS: &str = "\
    uid, name, COALESCE(nametype, 0) AS nametype, COALESCE(sex, 0) AS sex, \
    birthday, COALESCE(marriage, 0) AS marriage, \
    COALESCE(edu, 0) AS education, \
    telphone, telhome, email, photo, COALESCE(phototype, 0) AS phototype, \
    COALESCE(photo_status, 0) AS photo_status, COALESCE(defphoto, 1) AS defphoto, \
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

/// Employer view: public resumes plus `status=3` when this company has a `userid_job` row.
pub async fn find_visible_for_employer(
    pool: &MySqlPool,
    uid: u64,
    com_id: u64,
) -> Result<Option<Resume>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_resume
         WHERE uid = ? AND r_status = 1 AND (
            status = 1
            OR (
                status = 3 AND EXISTS (
                    SELECT 1 FROM phpyun_userid_job
                    WHERE uid = ? AND com_id = ? AND isdel = 9 LIMIT 1
                )
            )
         ) LIMIT 1"
    );
    sqlx::query_as::<_, Resume>(&sql)
        .bind(uid)
        .bind(uid)
        .bind(com_id)
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
    pub job1_son: Option<i32>,
    pub job_post: Option<i32>,
    pub province_id: Option<i32>,
    pub city_id: Option<i32>,
    pub three_city_id: Option<i32>,
    pub sex: Option<i32>,
    pub marriage: Option<i32>,
    pub hy: Option<i32>,
    pub report: Option<i32>,
    pub r#type: Option<i32>,
    pub tag: Option<i32>,
    /// PHP FIND_IN_SET(标签名称, resume.tag)
    pub tag_name: Option<&'a str>,
    pub min_salary: Option<i32>,
    pub max_salary: Option<i32>,
    pub min_age: Option<i32>,
    pub max_age: Option<i32>,
    /// PHP `uptime`: 1 = today; otherwise last N days.
    pub uptime: Option<i32>,
    /// PHP integrity keys 1/2/3/4 → 55/65/75/85 on `phpyun_resume_expect.integrity`.
    pub integrity: Option<i32>,
    /// `lastdate` (default) or `ctime`.
    pub order: Option<&'a str>,
    pub photo: bool,
    pub idcard: bool,
    pub work: bool,
    pub did: u32,
    /// PHP `userlist recg=1` → `phpyun_resume_expect.rec_resume = 1`
    pub recg: bool,
    /// PHP `userlist topdate=1`: default expect `top=1 AND topdate>now`.
    pub top: bool,
    pub education_ids: Option<&'a [i32]>,
    pub exp_ids: Option<&'a [i32]>,
    pub exclude_uids: Option<&'a [u64]>,
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
    qb.push(" FROM phpyun_resume WHERE status = 1 AND r_status = 1 AND (");
    qb.push_bind(f.did);
    qb.push(" = 0 OR COALESCE(did, 0) = ");
    qb.push_bind(f.did);
    qb.push(")");
    push_filters(&mut qb, f);
    qb.push(
        " AND (uid, lastupdate) IN (SELECT uid, MAX(lastupdate) FROM phpyun_resume WHERE status = 1 AND r_status = 1 AND (",
    );
    qb.push_bind(f.did);
    qb.push(" = 0 OR COALESCE(did, 0) = ");
    qb.push_bind(f.did);
    qb.push(")");
    push_filters(&mut qb, f);
    qb.push(" GROUP BY uid) ");
    push_order(&mut qb, f);
    qb.push(" LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as::<Resume>().fetch_all(pool).await
}

pub async fn count_public(pool: &MySqlPool, f: &ResumeFilter<'_>) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT COUNT(DISTINCT uid) FROM phpyun_resume WHERE status = 1 AND r_status = 1 AND (",
    );
    qb.push_bind(f.did);
    qb.push(" = 0 OR COALESCE(did, 0) = ");
    qb.push_bind(f.did);
    qb.push(")");
    push_filters(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

fn push_order<'a>(qb: &mut QueryBuilder<'a, sqlx::MySql>, f: &ResumeFilter<'a>) {
    if f.order == Some("ctime") {
        qb.push("ORDER BY resumetime DESC");
    } else if f.order == Some("topdate") {
        qb.push(
            "ORDER BY IF(COALESCE((SELECT e.topdate FROM phpyun_resume_expect e \
             WHERE e.uid = phpyun_resume.uid AND e.defaults = 1 LIMIT 1), 0) > UNIX_TIMESTAMP(), \
             (SELECT e.topdate FROM phpyun_resume_expect e WHERE e.uid = phpyun_resume.uid AND e.defaults = 1 LIMIT 1), \
             lastupdate) DESC",
        );
    } else {
        qb.push("ORDER BY lastupdate DESC");
    }
}

fn integrity_floor(v: i32) -> i32 {
    match v {
        1 => 55,
        2 => 65,
        3 => 75,
        4 => 85,
        n if n >= 55 => n,
        _ => 0,
    }
}

fn has_expect_filters(f: &ResumeFilter<'_>) -> bool {
    f.hy.is_some()
        || f.report.is_some()
        || f.r#type.is_some()
        || f.min_salary.is_some()
        || f.max_salary.is_some()
        || f.integrity.is_some()
        || f.recg
}

fn push_filters<'a>(qb: &mut QueryBuilder<'a, sqlx::MySql>, f: &ResumeFilter<'a>) {
    qb.push(
        " AND EXISTS (SELECT 1 FROM phpyun_resume_expect e \
         WHERE e.uid = phpyun_resume.uid AND e.defaults = 1 \
           AND e.state = 1 AND e.status = 1 AND e.r_status = 1)",
    );
    if let Some(ids) = f.exclude_uids.filter(|s| !s.is_empty()) {
        qb.push(" AND uid NOT IN (");
        let mut sep = qb.separated(",");
        for id in ids {
            sep.push_bind(*id);
        }
        qb.push(")");
    }
    if let Some(kw) = f.keyword {
        if !kw.is_empty() {
            let pat = format!("%{kw}%");
            qb.push(" AND (name LIKE ");
            qb.push_bind(pat.clone());
            qb.push(
                " OR EXISTS (SELECT 1 FROM phpyun_resume_expect e \
                 WHERE e.uid = phpyun_resume.uid AND e.uname LIKE ",
            );
            qb.push_bind(pat.clone());
            qb.push(
                ") OR uid IN (SELECT uid FROM phpyun_resume_work \
                 WHERE name LIKE ",
            );
            qb.push_bind(pat.clone());
            qb.push(" OR title LIKE ");
            qb.push_bind(pat.clone());
            qb.push(" OR content LIKE ");
            qb.push_bind(pat);
            qb.push("))");
        }
    }
    if let Some(ids) = f.education_ids.filter(|s| !s.is_empty()) {
        qb.push(" AND edu IN (");
        let mut sep = qb.separated(",");
        for id in ids {
            sep.push_bind(*id);
        }
        qb.push(")");
    } else if let Some(v) = f.education {
        qb.push(" AND edu = ");
        qb.push_bind(v);
    }
    if let Some(ids) = f.exp_ids.filter(|s| !s.is_empty()) {
        qb.push(" AND exp IN (");
        let mut sep = qb.separated(",");
        for id in ids {
            sep.push_bind(*id);
        }
        qb.push(")");
    } else if let Some(v) = f.exp {
        qb.push(" AND exp = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.sex {
        qb.push(" AND sex = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.marriage {
        qb.push(" AND marriage = ");
        qb.push_bind(v);
    }
    if let Some(name) = f.tag_name.filter(|s| !s.is_empty()) {
        qb.push(" AND FIND_IN_SET(");
        qb.push_bind(name);
        qb.push(", tag)");
    } else if let Some(v) = f.tag {
        qb.push(" AND FIND_IN_SET(");
        qb.push_bind(v.to_string());
        qb.push(", tag)");
    }
    if f.photo {
        qb.push(" AND photo IS NOT NULL AND photo <> '' AND phototype != 1 AND COALESCE(defphoto, 1) = 1");
    }
    if f.idcard {
        qb.push(" AND COALESCE(idcard_status, 0) = 1");
    }
    if f.work {
        qb.push(" AND uid IN (SELECT uid FROM phpyun_resume_work)");
    }
    let year = i32::from(phpyun_core::clock::now_year());
    if let Some(min_age) = f.min_age {
        qb.push(" AND birthday IS NOT NULL AND birthday <> '' AND (");
        qb.push_bind(year);
        qb.push(" - CAST(LEFT(birthday, 4) AS SIGNED)) >= ");
        qb.push_bind(min_age);
    }
    if let Some(max_age) = f.max_age {
        qb.push(" AND birthday IS NOT NULL AND birthday <> '' AND (");
        qb.push_bind(year);
        qb.push(" - CAST(LEFT(birthday, 4) AS SIGNED)) <= ");
        qb.push_bind(max_age);
    }
    if let Some(days) = f.uptime.filter(|d| *d > 0) {
        let now = phpyun_core::clock::now_ts();
        let since = if days == 1 {
            now - (now % 86400)
        } else {
            now - i64::from(days) * 86400
        };
        qb.push(" AND lastupdate > ");
        qb.push_bind(since);
    }
    if has_expect_filters(f) {
        qb.push(" AND uid IN (SELECT uid FROM phpyun_resume_expect WHERE 1=1");
        if let Some(v) = f.hy {
            qb.push(" AND hy = ");
            qb.push_bind(v);
        }
        if let Some(v) = f.report {
            qb.push(" AND report = ");
            qb.push_bind(v);
        }
        if let Some(v) = f.r#type {
            qb.push(" AND `type` = ");
            qb.push_bind(v);
        }
        if let Some(min) = f.min_salary {
            qb.push(" AND (COALESCE(maxsalary, 0) = 0 OR maxsalary >= ");
            qb.push_bind(min);
            qb.push(")");
        }
        if let Some(max) = f.max_salary {
            qb.push(" AND COALESCE(minsalary, 0) <= ");
            qb.push_bind(max);
        }
        if let Some(v) = f.integrity {
            let floor = integrity_floor(v);
            if floor > 0 {
                qb.push(" AND COALESCE(integrity, 0) >= ");
                qb.push_bind(floor);
            }
        }
        if f.recg {
            qb.push(" AND COALESCE(rec_resume, 0) = 1");
        }
        qb.push(")");
    }
    push_city_job_class(qb, f);
    if f.top {
        qb.push(
            " AND COALESCE(def_job, 0) > 0 AND def_job IN (\
             SELECT id FROM phpyun_resume_expect \
             WHERE COALESCE(top, 0) = 1 AND COALESCE(topdate, 0) > UNIX_TIMESTAMP())",
        );
    }
}

fn push_city_job_class<'a>(qb: &mut QueryBuilder<'a, sqlx::MySql>, f: &ResumeFilter<'a>) {
    let city_col = if f.three_city_id.is_some() {
        Some("three_cityid")
    } else if f.city_id.is_some() {
        Some("cityid")
    } else if f.province_id.is_some() {
        Some("provinceid")
    } else {
        None
    };
    let city_val = f.three_city_id.or(f.city_id).or(f.province_id);
    let job_col = if f.job_post.is_some() {
        Some("job_post")
    } else if f.job1_son.is_some() {
        Some("job1_son")
    } else if f.job1.is_some() {
        Some("job1")
    } else {
        None
    };
    let job_val = f.job_post.or(f.job1_son).or(f.job1);
    if city_col.is_none() && job_col.is_none() {
        return;
    }
    qb.push(" AND uid IN (SELECT uid FROM phpyun_resume_city_job_class WHERE 1=1");
    if let (Some(col), Some(v)) = (city_col, city_val) {
        qb.push(" AND `");
        qb.push(col);
        qb.push("` = ");
        qb.push_bind(v);
    }
    if let (Some(col), Some(v)) = (job_col, job_val) {
        qb.push(" AND `");
        qb.push(col);
        qb.push("` = ");
        qb.push_bind(v);
    }
    let num_col = match (city_col, job_col) {
        (Some(c), Some(j)) => format!("{c}_{j}_num"),
        (Some(c), None) => format!("{c}_num"),
        (None, Some(j)) => format!("{j}_num"),
        (None, None) => unreachable!(),
    };
    qb.push(" AND `");
    qb.push(num_col);
    qb.push("` = 1)");
}

/// PHP first-page `ORDER BY rand() limit 5` for currently topped expects.
pub async fn list_top_random(
    pool: &MySqlPool,
    f: &ResumeFilter<'_>,
    limit: u64,
) -> Result<Vec<Resume>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(FIELDS);
    qb.push(" FROM phpyun_resume WHERE status = 1 AND r_status = 1 AND (");
    qb.push_bind(f.did);
    qb.push(" = 0 OR COALESCE(did, 0) = ");
    qb.push_bind(f.did);
    qb.push(")");
    push_filters(&mut qb, f);
    qb.push(
        " AND COALESCE(def_job, 0) > 0 AND def_job IN (\
         SELECT id FROM phpyun_resume_expect \
         WHERE COALESCE(top, 0) = 1 AND COALESCE(topdate, 0) > UNIX_TIMESTAMP())",
    );
    qb.push(" ORDER BY RAND() LIMIT ");
    qb.push_bind(limit);
    qb.build_query_as::<Resume>().fetch_all(pool).await
}

/// PHP `lookresume.model.php::browseResume`.
pub async fn find_look_resume(
    pool: &MySqlPool,
    com_id: u64,
    eid: u64,
    usertype: i32,
) -> Result<Option<u64>, sqlx::Error> {
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT id FROM phpyun_look_resume \
         WHERE com_id = ? AND resume_id = ? AND usertype = ? LIMIT 1",
    )
    .bind(com_id)
    .bind(eid)
    .bind(usertype)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(id,)| phpyun_core::numeric::nonnegative_count(id)))
}

pub async fn touch_look_resume(pool: &MySqlPool, id: u64, now: i64) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE phpyun_look_resume SET datetime = ? WHERE id = ?")
        .bind(now)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn insert_look_resume(
    pool: &MySqlPool,
    resume_uid: u64,
    eid: u64,
    com_id: u64,
    did: u32,
    usertype: i32,
    now: i64,
    ip: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO phpyun_look_resume \
         (uid, resume_id, com_id, did, usertype, datetime, ip, status, com_status) \
         VALUES (?, ?, ?, ?, ?, ?, ?, 0, 0)",
    )
    .bind(resume_uid)
    .bind(eid)
    .bind(com_id)
    .bind(did)
    .bind(usertype)
    .bind(now)
    .bind(ip)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn mark_userid_job_browsed(
    pool: &MySqlPool,
    com_id: u64,
    eid: u64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE phpyun_userid_job SET is_browse = 2 \
         WHERE com_id = ? AND eid = ? AND isdel = 9 AND is_browse = 1",
    )
    .bind(com_id)
    .bind(eid)
    .execute(pool)
    .await?;
    Ok(())
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
    pub exp: Option<i32>,
    pub living: Option<&'a str>,
    pub domicile: Option<&'a str>,
    pub height: Option<&'a str>,
    pub weight: Option<&'a str>,
    pub address: Option<&'a str>,
    pub description: Option<&'a str>,
    pub qq: Option<&'a str>,
    pub idcard: Option<&'a str>,
    pub idcard_pic: Option<&'a str>,
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
            exp        = COALESCE(?, exp),
            living     = COALESCE(?, living),
            domicile   = COALESCE(?, domicile),
            height     = COALESCE(?, height),
            weight     = COALESCE(?, weight),
            address    = COALESCE(?, address),
            description = COALESCE(?, description),
            qq         = COALESCE(?, qq),
            idcard     = COALESCE(?, idcard),
            idcard_pic = COALESCE(?, idcard_pic),
            idcard_status = CASE WHEN ? IS NOT NULL OR ? IS NOT NULL THEN 0 ELSE idcard_status END,
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
    .bind(u.exp)
    .bind(u.living)
    .bind(u.domicile)
    .bind(u.height)
    .bind(u.weight)
    .bind(u.address)
    .bind(u.description)
    .bind(u.qq)
    .bind(u.idcard)
    .bind(u.idcard_pic)
    .bind(u.idcard)
    .bind(u.idcard_pic)
    .bind(now)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn update_tag_desc(
    pool: &MySqlPool,
    uid: u64,
    tag: &str,
    description: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_resume SET tag = ?, description = ?, lastupdate = ? WHERE uid = ?",
    )
    .bind(tag)
    .bind(description)
    .bind(now)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn update_admin_basic(
    pool: &MySqlPool,
    uid: u64,
    name: &str,
    sex: i32,
    birthday: &str,
    living: &str,
    edu: i32,
    exp: i32,
    telphone: &str,
    email: &str,
    description: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_resume SET name=?, sex=?, birthday=?, living=?, edu=?, exp=?, \
         telphone=?, email=?, description=?, lastupdate=? WHERE uid=?",
    )
    .bind(name)
    .bind(sex)
    .bind(birthday)
    .bind(living)
    .bind(edu)
    .bind(exp)
    .bind(telphone)
    .bind(email)
    .bind(description)
    .bind(now)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn update_admin_profile(
    pool: &MySqlPool,
    uid: u64,
    name: &str,
    sex: i32,
    birthday: &str,
    exp: i32,
    edu: i32,
    telphone: &str,
    email: &str,
    domicile: &str,
    living: &str,
    marriage: i32,
    height: &str,
    nationality: &str,
    weight: &str,
    idcard: &str,
    address: &str,
    homepage: &str,
    qq: &str,
    description: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_resume SET name=?, sex=?, birthday=?, exp=?, edu=?, telphone=?, email=?, \
         domicile=?, living=?, marriage=?, height=?, nationality=?, weight=?, idcard=?, address=?, \
         homepage=?, qq=?, description=?, lastupdate=? WHERE uid=?",
    )
    .bind(name)
    .bind(sex)
    .bind(birthday)
    .bind(exp)
    .bind(edu)
    .bind(telphone)
    .bind(email)
    .bind(domicile)
    .bind(living)
    .bind(marriage)
    .bind(height)
    .bind(nationality)
    .bind(weight)
    .bind(idcard)
    .bind(address)
    .bind(homepage)
    .bind(qq)
    .bind(description)
    .bind(now)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn set_def_job(pool: &MySqlPool, uid: u64, eid: u64) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE phpyun_resume SET def_job = ? WHERE uid = ?")
        .bind(eid)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_by_uid<'e, E>(exec: E, uid: u64) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::MySql>,
{
    sqlx::query("DELETE FROM phpyun_resume WHERE uid = ?")
        .bind(uid)
        .execute(exec)
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
    pub sex: i32,
    pub edu: i32,
    pub exp: i32,
    pub telphone: String,
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
                  CAST(COALESCE(lastupdate, 0) AS SIGNED) AS lastupdate,
                  CAST(COALESCE(sex, 0) AS SIGNED) AS sex,
                  CAST(COALESCE(edu, 0) AS SIGNED) AS edu,
                  CAST(COALESCE(exp, 0) AS SIGNED) AS exp,
                  COALESCE(telphone, '') AS telphone
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
