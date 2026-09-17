//! Recruiter member-center charts (`member/com/zhaopin` + `tongji` + WAP week).

use sqlx::{FromRow, MySqlPool, QueryBuilder};

#[derive(Debug, Clone, Copy)]
pub enum BucketFmt {
    Hour,
    Day,
    MonthDay,
    MonthNum,
    Ymd,
    MdCompact,
}

impl BucketFmt {
    fn sql(self) -> &'static str {
        match self {
            Self::Hour => "%H:00",
            Self::Day => "%Y-%m-%d",
            Self::MonthDay => "%m-%d",
            Self::MonthNum => "%c",
            Self::Ymd => "%Y%m%d",
            Self::MdCompact => "%m%d",
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct BucketCount {
    pub bucket: String,
    pub cnt: i64,
}

#[derive(Debug, Clone, FromRow)]
pub struct ClassCount {
    pub class_id: i32,
    pub cnt: i64,
}

#[derive(Debug, Clone, FromRow)]
pub struct NameRow {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct RatingCaps {
    pub job_num: i32,
    pub breakjob_num: i32,
    pub resume: i32,
    pub interview: i32,
    pub zph_num: i32,
    pub top_num: i32,
    pub urgent_num: i32,
    pub rec_num: i32,
}

#[derive(Debug, Clone, FromRow, serde::Serialize)]
pub struct JobBrief {
    pub id: u64,
    pub name: String,
}

fn ncount(n: i64) -> u64 {
    phpyun_core::numeric::nonnegative_count(n)
}

async fn group_simple(
    pool: &MySqlPool,
    sql: String,
    uid: u64,
    extra: Option<i32>,
    start: i64,
    end: i64,
) -> Result<Vec<BucketCount>, sqlx::Error> {
    let mut q = sqlx::query_as::<_, BucketCount>(&sql).bind(uid);
    if let Some(x) = extra {
        q = q.bind(x);
    }
    q.bind(start).bind(end).fetch_all(pool).await
}

pub async fn group_look_resume(
    pool: &MySqlPool,
    com_id: u64,
    usertype: i32,
    start: i64,
    end: i64,
    fmt: BucketFmt,
) -> Result<Vec<BucketCount>, sqlx::Error> {
    let f = fmt.sql();
    let sql = format!(
        "SELECT FROM_UNIXTIME(datetime,'{f}') AS bucket, COUNT(*) AS cnt \
         FROM phpyun_look_resume \
         WHERE com_id = ? AND usertype = ? AND COALESCE(com_status,0)=0 \
           AND datetime >= ? AND datetime <= ? GROUP BY bucket"
    );
    sqlx::query_as::<_, BucketCount>(&sql)
        .bind(com_id)
        .bind(usertype)
        .bind(start)
        .bind(end)
        .fetch_all(pool)
        .await
}

pub async fn group_look_job(
    pool: &MySqlPool,
    com_id: u64,
    start: i64,
    end: i64,
    fmt: BucketFmt,
    job_id: Option<u64>,
) -> Result<Vec<BucketCount>, sqlx::Error> {
    let f = fmt.sql();
    let mut sql = format!(
        "SELECT FROM_UNIXTIME(datetime,'{f}') AS bucket, COUNT(*) AS cnt \
         FROM phpyun_look_job \
         WHERE com_id = ? AND COALESCE(com_status,0)=0 \
           AND datetime >= ? AND datetime <= ?"
    );
    if job_id.is_some() {
        sql.push_str(" AND jobid = ?");
    }
    sql.push_str(" GROUP BY bucket");
    let mut q = sqlx::query_as::<_, BucketCount>(&sql)
        .bind(com_id)
        .bind(start)
        .bind(end);
    if let Some(jid) = job_id {
        q = q.bind(jid);
    }
    q.fetch_all(pool).await
}

pub async fn group_down_resume(
    pool: &MySqlPool,
    table: &'static str,
    com_id: u64,
    usertype: i32,
    start: i64,
    end: i64,
    fmt: BucketFmt,
) -> Result<Vec<BucketCount>, sqlx::Error> {
    let table = match table {
        "down_resume" => "phpyun_down_resume",
        "freedown_resume" => "phpyun_freedown_resume",
        _ => return Ok(Vec::new()),
    };
    let f = fmt.sql();
    let sql = format!(
        "SELECT FROM_UNIXTIME(downtime,'{f}') AS bucket, COUNT(*) AS cnt \
         FROM {table} WHERE comid = ? AND usertype = ? \
           AND downtime >= ? AND downtime <= ? GROUP BY bucket"
    );
    sqlx::query_as::<_, BucketCount>(&sql)
        .bind(com_id)
        .bind(usertype)
        .bind(start)
        .bind(end)
        .fetch_all(pool)
        .await
}

pub async fn group_apply(
    pool: &MySqlPool,
    com_id: u64,
    start: i64,
    end: i64,
    fmt: BucketFmt,
    job_id: Option<u64>,
) -> Result<Vec<BucketCount>, sqlx::Error> {
    let f = fmt.sql();
    let mut sql = format!(
        "SELECT FROM_UNIXTIME(datetime,'{f}') AS bucket, COUNT(*) AS cnt \
         FROM phpyun_userid_job \
         WHERE com_id = ? AND COALESCE(`type`,0) <> 3 AND COALESCE(isdel,9)=9 \
           AND datetime >= ? AND datetime <= ?"
    );
    if job_id.is_some() {
        sql.push_str(" AND job_id = ?");
    }
    sql.push_str(" GROUP BY bucket");
    let mut q = sqlx::query_as::<_, BucketCount>(&sql)
        .bind(com_id)
        .bind(start)
        .bind(end);
    if let Some(jid) = job_id {
        q = q.bind(jid);
    }
    q.fetch_all(pool).await
}

pub async fn group_invite(
    pool: &MySqlPool,
    fid: u64,
    start: i64,
    end: i64,
    fmt: BucketFmt,
    job_id: Option<u64>,
) -> Result<Vec<BucketCount>, sqlx::Error> {
    let f = fmt.sql();
    let mut sql = format!(
        "SELECT FROM_UNIXTIME(datetime,'{f}') AS bucket, COUNT(*) AS cnt \
         FROM phpyun_userid_msg \
         WHERE fid = ? AND COALESCE(isdel,9)=9 \
           AND datetime >= ? AND datetime <= ?"
    );
    if job_id.is_some() {
        sql.push_str(" AND jobid = ?");
    }
    sql.push_str(" GROUP BY bucket");
    let mut q = sqlx::query_as::<_, BucketCount>(&sql)
        .bind(fid)
        .bind(start)
        .bind(end);
    if let Some(jid) = job_id {
        q = q.bind(jid);
    }
    q.fetch_all(pool).await
}

pub async fn group_login(
    pool: &MySqlPool,
    uid: u64,
    start: i64,
    end: i64,
    fmt: BucketFmt,
) -> Result<Vec<BucketCount>, sqlx::Error> {
    let f = fmt.sql();
    let sql = format!(
        "SELECT FROM_UNIXTIME(ctime,'{f}') AS bucket, COUNT(*) AS cnt \
         FROM phpyun_login_log \
         WHERE uid = ? AND usertype = 2 AND ctime >= ? AND ctime <= ? \
         GROUP BY bucket"
    );
    group_simple(pool, sql, uid, None, start, end).await
}

pub async fn count_login_range(
    pool: &MySqlPool,
    uid: u64,
    start: i64,
    end: i64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM phpyun_login_log \
         WHERE uid = ? AND usertype = 2 AND ctime >= ? AND ctime <= ?",
    )
    .bind(uid)
    .bind(start)
    .bind(end)
    .fetch_one(pool)
    .await?;
    Ok(ncount(n))
}

pub async fn count_apply_range(
    pool: &MySqlPool,
    com_id: u64,
    start: i64,
    end: i64,
    job_id: Option<u64>,
) -> Result<u64, sqlx::Error> {
    let mut sql = String::from(
        "SELECT COUNT(*) FROM phpyun_userid_job \
         WHERE com_id = ? AND COALESCE(`type`,0) <> 3 AND COALESCE(isdel,9)=9 \
           AND datetime >= ? AND datetime <= ?",
    );
    if job_id.is_some() {
        sql.push_str(" AND job_id = ?");
    }
    let mut q = sqlx::query_as::<_, (i64,)>(&sql)
        .bind(com_id)
        .bind(start)
        .bind(end);
    if let Some(jid) = job_id {
        q = q.bind(jid);
    }
    let (n,) = q.fetch_one(pool).await?;
    Ok(ncount(n))
}

pub async fn count_look_job_range(
    pool: &MySqlPool,
    com_id: u64,
    start: i64,
    end: i64,
    job_id: Option<u64>,
) -> Result<u64, sqlx::Error> {
    let mut sql = String::from(
        "SELECT COUNT(*) FROM phpyun_look_job \
         WHERE com_id = ? AND COALESCE(com_status,0)=0 \
           AND datetime >= ? AND datetime <= ?",
    );
    if job_id.is_some() {
        sql.push_str(" AND jobid = ?");
    }
    let mut q = sqlx::query_as::<_, (i64,)>(&sql)
        .bind(com_id)
        .bind(start)
        .bind(end);
    if let Some(jid) = job_id {
        q = q.bind(jid);
    }
    let (n,) = q.fetch_one(pool).await?;
    Ok(ncount(n))
}

pub async fn count_invite_range(
    pool: &MySqlPool,
    fid: u64,
    start: i64,
    end: i64,
    job_id: Option<u64>,
    is_browse: Option<i32>,
) -> Result<u64, sqlx::Error> {
    let mut sql = String::from(
        "SELECT COUNT(*) FROM phpyun_userid_msg \
         WHERE fid = ? AND COALESCE(isdel,9)=9 AND datetime >= ? AND datetime <= ?",
    );
    if job_id.is_some() {
        sql.push_str(" AND jobid = ?");
    }
    if is_browse.is_some() {
        sql.push_str(" AND is_browse = ?");
    }
    let mut q = sqlx::query_as::<_, (i64,)>(&sql)
        .bind(fid)
        .bind(start)
        .bind(end);
    if let Some(jid) = job_id {
        q = q.bind(jid);
    }
    if let Some(b) = is_browse {
        q = q.bind(b);
    }
    let (n,) = q.fetch_one(pool).await?;
    Ok(ncount(n))
}

async fn distinct_uids(
    pool: &MySqlPool,
    sql: &str,
    uid: u64,
    extra: Option<i32>,
    start: i64,
    end: i64,
) -> Result<Vec<u64>, sqlx::Error> {
    let mut q = sqlx::query_as::<_, (u64,)>(sql).bind(uid);
    if let Some(x) = extra {
        q = q.bind(x);
    }
    let rows = q.bind(start).bind(end).fetch_all(pool).await?;
    Ok(rows.into_iter().map(|(id,)| id).filter(|n| *n > 0).collect())
}

pub async fn uids_look_resume(
    pool: &MySqlPool,
    com_id: u64,
    usertype: i32,
    start: i64,
    end: i64,
) -> Result<Vec<u64>, sqlx::Error> {
    distinct_uids(
        pool,
        "SELECT CAST(uid AS UNSIGNED) FROM phpyun_look_resume \
         WHERE com_id = ? AND usertype = ? AND COALESCE(com_status,0)=0 \
           AND datetime >= ? AND datetime <= ? GROUP BY uid",
        com_id,
        Some(usertype),
        start,
        end,
    )
    .await
}

pub async fn uids_look_job(
    pool: &MySqlPool,
    com_id: u64,
    start: i64,
    end: i64,
) -> Result<Vec<u64>, sqlx::Error> {
    distinct_uids(
        pool,
        "SELECT CAST(uid AS UNSIGNED) FROM phpyun_look_job \
         WHERE com_id = ? AND COALESCE(com_status,0)=0 \
           AND datetime >= ? AND datetime <= ? GROUP BY uid",
        com_id,
        None,
        start,
        end,
    )
    .await
}

pub async fn uids_down(
    pool: &MySqlPool,
    table: &'static str,
    com_id: u64,
    usertype: i32,
    start: i64,
    end: i64,
) -> Result<Vec<u64>, sqlx::Error> {
    let table = match table {
        "down_resume" => "phpyun_down_resume",
        "freedown_resume" => "phpyun_freedown_resume",
        _ => return Ok(Vec::new()),
    };
    let sql = format!(
        "SELECT CAST(uid AS UNSIGNED) FROM {table} \
         WHERE comid = ? AND usertype = ? AND downtime >= ? AND downtime <= ? GROUP BY uid"
    );
    distinct_uids(pool, &sql, com_id, Some(usertype), start, end).await
}

pub async fn uids_apply(
    pool: &MySqlPool,
    com_id: u64,
    start: i64,
    end: i64,
) -> Result<Vec<u64>, sqlx::Error> {
    distinct_uids(
        pool,
        "SELECT CAST(uid AS UNSIGNED) FROM phpyun_userid_job \
         WHERE com_id = ? AND COALESCE(`type`,0) <> 3 AND COALESCE(isdel,9)=9 \
           AND datetime >= ? AND datetime <= ? GROUP BY uid",
        com_id,
        None,
        start,
        end,
    )
    .await
}

pub async fn uids_invite(
    pool: &MySqlPool,
    fid: u64,
    start: i64,
    end: i64,
) -> Result<Vec<u64>, sqlx::Error> {
    distinct_uids(
        pool,
        "SELECT CAST(uid AS UNSIGNED) FROM phpyun_userid_msg \
         WHERE fid = ? AND COALESCE(isdel,9)=9 AND datetime >= ? AND datetime <= ? GROUP BY uid",
        fid,
        None,
        start,
        end,
    )
    .await
}

pub async fn apply_eids(
    pool: &MySqlPool,
    com_id: u64,
    start: i64,
    end: i64,
    job_id: Option<u64>,
) -> Result<Vec<u64>, sqlx::Error> {
    let mut sql = String::from(
        "SELECT CAST(eid AS UNSIGNED) FROM phpyun_userid_job \
         WHERE com_id = ? AND COALESCE(`type`,0) <> 3 AND COALESCE(isdel,9)=9 \
           AND datetime >= ? AND datetime <= ?",
    );
    if job_id.is_some() {
        sql.push_str(" AND job_id = ?");
    }
    let mut q = sqlx::query_as::<_, (u64,)>(&sql)
        .bind(com_id)
        .bind(start)
        .bind(end);
    if let Some(jid) = job_id {
        q = q.bind(jid);
    }
    let rows = q.fetch_all(pool).await?;
    Ok(rows.into_iter().map(|(id,)| id).filter(|n| *n > 0).collect())
}

pub async fn group_expect_exp(
    pool: &MySqlPool,
    uids: &[u64],
) -> Result<Vec<ClassCount>, sqlx::Error> {
    if uids.is_empty() {
        return Ok(Vec::new());
    }
    let mut qb = QueryBuilder::new(
        "SELECT CAST(exp AS SIGNED) AS class_id, COUNT(*) AS cnt \
         FROM phpyun_resume_expect WHERE defaults = 1 AND uid IN (",
    );
    let mut first = true;
    for id in uids {
        if !first {
            qb.push(",");
        }
        qb.push_bind(*id);
        first = false;
    }
    qb.push(") AND exp > 0 GROUP BY exp");
    qb.build_query_as().fetch_all(pool).await
}

pub async fn group_resume_edu(
    pool: &MySqlPool,
    uids: &[u64],
) -> Result<Vec<ClassCount>, sqlx::Error> {
    if uids.is_empty() {
        return Ok(Vec::new());
    }
    let mut qb = QueryBuilder::new(
        "SELECT CAST(edu AS SIGNED) AS class_id, COUNT(*) AS cnt \
         FROM phpyun_resume WHERE uid IN (",
    );
    let mut first = true;
    for id in uids {
        if !first {
            qb.push(",");
        }
        qb.push_bind(*id);
        first = false;
    }
    qb.push(") AND edu > 0 GROUP BY edu");
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_expect_salary(
    pool: &MySqlPool,
    uids: &[u64],
    min: i32,
    max: i32,
) -> Result<u64, sqlx::Error> {
    if uids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new(
        "SELECT COUNT(*) FROM phpyun_resume_expect WHERE defaults = 1 AND uid IN (",
    );
    let mut first = true;
    for id in uids {
        if !first {
            qb.push(",");
        }
        qb.push_bind(*id);
        first = false;
    }
    qb.push(") AND minsalary >= ");
    qb.push_bind(min);
    if max > 0 {
        qb.push(" AND minsalary < ");
        qb.push_bind(max);
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(ncount(n))
}

pub async fn group_expect_edu_by_ids(
    pool: &MySqlPool,
    eids: &[u64],
) -> Result<Vec<ClassCount>, sqlx::Error> {
    if eids.is_empty() {
        return Ok(Vec::new());
    }
    let mut qb = QueryBuilder::new(
        "SELECT CAST(edu AS SIGNED) AS class_id, COUNT(*) AS cnt \
         FROM phpyun_resume_expect WHERE id IN (",
    );
    let mut first = true;
    for id in eids {
        if !first {
            qb.push(",");
        }
        qb.push_bind(*id);
        first = false;
    }
    qb.push(") AND edu > 0 GROUP BY edu");
    qb.build_query_as().fetch_all(pool).await
}

pub async fn group_expect_exp_by_ids(
    pool: &MySqlPool,
    eids: &[u64],
) -> Result<Vec<ClassCount>, sqlx::Error> {
    if eids.is_empty() {
        return Ok(Vec::new());
    }
    let mut qb = QueryBuilder::new(
        "SELECT CAST(exp AS SIGNED) AS class_id, COUNT(*) AS cnt \
         FROM phpyun_resume_expect WHERE id IN (",
    );
    let mut first = true;
    for id in eids {
        if !first {
            qb.push(",");
        }
        qb.push_bind(*id);
        first = false;
    }
    qb.push(") AND exp > 0 GROUP BY exp");
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_expect_salary_by_ids(
    pool: &MySqlPool,
    eids: &[u64],
    min: i32,
    max: i32,
) -> Result<u64, sqlx::Error> {
    if eids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("SELECT COUNT(*) FROM phpyun_resume_expect WHERE id IN (");
    let mut first = true;
    for id in eids {
        if !first {
            qb.push(",");
        }
        qb.push_bind(*id);
        first = false;
    }
    qb.push(") AND minsalary >= ");
    qb.push_bind(min);
    if max > 0 {
        qb.push(" AND minsalary < ");
        qb.push_bind(max);
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(ncount(n))
}

pub async fn group_apply_province(
    pool: &MySqlPool,
    com_id: u64,
    start: i64,
    end: i64,
    job_id: Option<u64>,
) -> Result<Vec<ClassCount>, sqlx::Error> {
    let mut sql = String::from(
        "SELECT CAST(j.provinceid AS SIGNED) AS class_id, COUNT(*) AS cnt \
         FROM phpyun_userid_job a \
         INNER JOIN phpyun_company_job j ON j.id = a.job_id \
         WHERE a.com_id = ? AND COALESCE(a.`type`,0) <> 3 AND COALESCE(a.isdel,9)=9 \
           AND a.datetime >= ? AND a.datetime <= ?",
    );
    if job_id.is_some() {
        sql.push_str(" AND a.job_id = ?");
    }
    sql.push_str(" AND j.provinceid > 0 GROUP BY j.provinceid");
    let mut q = sqlx::query_as::<_, ClassCount>(&sql)
        .bind(com_id)
        .bind(start)
        .bind(end);
    if let Some(jid) = job_id {
        q = q.bind(jid);
    }
    q.fetch_all(pool).await
}

pub async fn names_userclass(
    pool: &MySqlPool,
    ids: &[i32],
) -> Result<Vec<NameRow>, sqlx::Error> {
    names_from(pool, "phpyun_userclass", ids).await
}

pub async fn names_city(pool: &MySqlPool, ids: &[i32]) -> Result<Vec<NameRow>, sqlx::Error> {
    names_from(pool, "phpyun_city_class", ids).await
}

async fn names_from(
    pool: &MySqlPool,
    table: &'static str,
    ids: &[i32],
) -> Result<Vec<NameRow>, sqlx::Error> {
    let ids: Vec<i32> = ids.iter().copied().filter(|n| *n > 0).collect();
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let table = match table {
        "phpyun_userclass" | "phpyun_city_class" => table,
        _ => return Ok(Vec::new()),
    };
    let mut qb = QueryBuilder::new(format!(
        "SELECT CAST(id AS SIGNED) AS id, COALESCE(name,'') AS name FROM {table} WHERE id IN ("
    ));
    let mut first = true;
    for id in ids {
        if !first {
            qb.push(",");
        }
        qb.push_bind(id);
        first = false;
    }
    qb.push(")");
    qb.build_query_as().fetch_all(pool).await
}

pub async fn find_rating_caps(pool: &MySqlPool, rating: i32) -> Result<Option<RatingCaps>, sqlx::Error> {
    if rating <= 0 {
        return Ok(None);
    }
    sqlx::query_as::<_, RatingCaps>(
        "SELECT CAST(COALESCE(job_num,0) AS SIGNED) AS job_num, \
                CAST(COALESCE(breakjob_num,0) AS SIGNED) AS breakjob_num, \
                CAST(COALESCE(resume,0) AS SIGNED) AS resume, \
                CAST(COALESCE(interview,0) AS SIGNED) AS interview, \
                CAST(COALESCE(zph_num,0) AS SIGNED) AS zph_num, \
                CAST(COALESCE(top_num,0) AS SIGNED) AS top_num, \
                CAST(COALESCE(urgent_num,0) AS SIGNED) AS urgent_num, \
                CAST(COALESCE(rec_num,0) AS SIGNED) AS rec_num \
         FROM phpyun_company_rating WHERE id = ? LIMIT 1",
    )
    .bind(rating)
    .fetch_optional(pool)
    .await
}

pub async fn list_jobs_brief(pool: &MySqlPool, uid: u64) -> Result<Vec<JobBrief>, sqlx::Error> {
    sqlx::query_as::<_, JobBrief>(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(name,'') AS name \
         FROM phpyun_company_job WHERE uid = ? ORDER BY id DESC LIMIT 200",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}
