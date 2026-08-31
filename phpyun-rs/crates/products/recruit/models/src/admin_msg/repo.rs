//! PHP `msgNum_model::msgNum()` pending-review counts for the admin home.

use serde::Serialize;
use sqlx::{MySqlPool, QueryBuilder};

#[derive(Debug, Default, Clone, Serialize)]
pub struct AdminMsgNum {
    pub msg_num: u64,
    pub company: u64,
    pub company_job: u64,
    pub partjob: u64,
    pub company_cert: u64,
    pub comlogo: u64,
    pub comshow: u64,
    pub combanner: u64,
    pub company_product: u64,
    pub company_news: u64,
    pub resume_expect: u64,
    pub usercert_num: u64,
    pub appealnum: u64,
    pub once_job: u64,
    pub tiny: u64,
    pub zphcom: u64,
    pub ask: u64,
    pub order: u64,
    pub reportjob: u64,
    pub reportresume: u64,
    pub reportask: u64,
    pub userpic: u64,
    pub link_num: u64,
    pub redeem: u64,
    pub specialcom: u64,
    pub userchangenum: u64,
    pub handlenum: u64,
    pub logout: u64,
    pub warning: u64,
}

async fn count(pool: &MySqlPool, sql: &str) -> u64 {
    let row: Result<(i64,), _> = sqlx::query_as(sql).fetch_one(pool).await;
    row.map(|(n,)| phpyun_core::numeric::nonnegative_count(n))
        .unwrap_or(0)
}

async fn count_ts(pool: &MySqlPool, sql: &str, v: i64) -> u64 {
    let row: Result<(i64,), _> = sqlx::query_as(sql).bind(v).fetch_one(pool).await;
    row.map(|(n,)| phpyun_core::numeric::nonnegative_count(n))
        .unwrap_or(0)
}

/// Best-effort: a missing optional table yields 0 rather than failing the home page.
pub async fn load(pool: &MySqlPool, now: i64) -> AdminMsgNum {
    let mut m = AdminMsgNum {
        company: count(pool, "SELECT COUNT(*) FROM phpyun_company WHERE r_status = 0").await,
        company_job: count(pool, "SELECT COUNT(*) FROM phpyun_company_job WHERE state = 0").await,
        partjob: count(pool, "SELECT COUNT(*) FROM phpyun_partjob WHERE state = 0").await,
        company_cert: count(
            pool,
            "SELECT COUNT(*) FROM phpyun_company_cert WHERE status = 0 AND type = 3",
        )
        .await,
        comlogo: count(
            pool,
            "SELECT COUNT(*) FROM phpyun_company WHERE logo <> '' AND logo_status = 1",
        )
        .await,
        comshow: count(
            pool,
            "SELECT COUNT(*) FROM phpyun_company_show WHERE picurl <> '' AND status = 1 AND COALESCE(deleted,0)=0",
        )
        .await,
        combanner: count(
            pool,
            "SELECT COUNT(*) FROM phpyun_banner WHERE pic <> '' AND status = 1 AND COALESCE(deleted,0)=0",
        )
        .await,
        company_product: count(
            pool,
            "SELECT COUNT(*) FROM phpyun_company_product WHERE status = 0",
        )
        .await,
        company_news: count(pool, "SELECT COUNT(*) FROM phpyun_company_news WHERE status = 0")
            .await,
        resume_expect: count(pool, "SELECT COUNT(*) FROM phpyun_resume_expect WHERE state = 0")
            .await,
        usercert_num: count(
            pool,
            "SELECT COUNT(*) FROM phpyun_resume WHERE idcard_pic <> '' AND idcard_status = 0",
        )
        .await,
        appealnum: count(
            pool,
            "SELECT COUNT(*) FROM phpyun_member WHERE appealtime > 0 AND appealstate = 1",
        )
        .await,
        once_job: count_ts(
            pool,
            "SELECT COUNT(*) FROM phpyun_once_job WHERE status = 0 AND edate > ?",
            now,
        )
        .await,
        tiny: count(pool, "SELECT COUNT(*) FROM phpyun_resume_tiny WHERE status = 0").await,
        zphcom: count(pool, "SELECT COUNT(*) FROM phpyun_zhaopinhui_com WHERE status = 0").await,
        ask: count(pool, "SELECT COUNT(*) FROM phpyun_question WHERE state = 0 AND COALESCE(deleted,0)=0").await,
        order: count(
            pool,
            "SELECT COUNT(*) FROM phpyun_company_order WHERE order_state = 1",
        )
        .await,
        reportjob: count(
            pool,
            "SELECT COUNT(*) FROM phpyun_report WHERE usertype = 1 AND type = 0 AND status = 0",
        )
        .await,
        reportresume: count(
            pool,
            "SELECT COUNT(*) FROM phpyun_report WHERE usertype = 2 AND type = 0 AND status = 0",
        )
        .await,
        reportask: count(
            pool,
            "SELECT COUNT(*) FROM phpyun_report WHERE type = 1 AND status = 0",
        )
        .await,
        userpic: count(
            pool,
            "SELECT COUNT(*) FROM phpyun_resume WHERE photo <> '' AND defphoto = 1 AND photo_status = 1",
        )
        .await,
        link_num: count(pool, "SELECT COUNT(*) FROM phpyun_admin_link WHERE link_state = 0 AND COALESCE(deleted,0)=0")
            .await,
        redeem: count(pool, "SELECT COUNT(*) FROM phpyun_change WHERE status = 0").await,
        specialcom: count(pool, "SELECT COUNT(*) FROM phpyun_special_com WHERE status = 0 AND COALESCE(deleted,0)=0").await,
        userchangenum: count(pool, "SELECT COUNT(*) FROM phpyun_user_change WHERE status = 0")
            .await,
        handlenum: count(pool, "SELECT COUNT(*) FROM phpyun_advice_question WHERE status = 1")
            .await,
        logout: count(pool, "SELECT COUNT(*) FROM phpyun_member_logout WHERE status = 1").await,
        warning: count(pool, "SELECT COUNT(*) FROM phpyun_warning WHERE status = 2").await,
        msg_num: 0,
    };
    m.msg_num = m.company
        + m.company_job
        + m.partjob
        + m.company_cert
        + m.comlogo
        + m.comshow
        + m.combanner
        + m.company_product
        + m.company_news
        + m.resume_expect
        + m.usercert_num
        + m.appealnum
        + m.once_job
        + m.tiny
        + m.zphcom
        + m.ask
        + m.order
        + m.reportjob
        + m.reportresume
        + m.reportask
        + m.userpic
        + m.link_num
        + m.redeem
        + m.specialcom
        + m.userchangenum
        + m.handlenum
        + m.logout
        + m.warning;
    m
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct LoginLogRow {
    pub id: u64,
    pub uid: i64,
    pub usertype: i32,
    pub content: String,
    pub ip: String,
    pub ctime: i64,
}

pub async fn list_login_logs(
    pool: &MySqlPool,
    usertype: Option<i32>,
    uid: Option<u64>,
    offset: u64,
    limit: u64,
) -> Result<Vec<LoginLogRow>, sqlx::Error> {
    let mut qb = sqlx::QueryBuilder::<sqlx::MySql>::new(
        r#"SELECT CAST(id AS UNSIGNED) AS id,
                  CAST(COALESCE(uid, 0) AS SIGNED) AS uid,
                  CAST(COALESCE(usertype, 0) AS SIGNED) AS usertype,
                  COALESCE(content, '') AS content,
                  COALESCE(ip, '') AS ip,
                  CAST(COALESCE(ctime, 0) AS SIGNED) AS ctime
           FROM phpyun_login_log WHERE 1=1"#,
    );
    if let Some(t) = usertype {
        qb.push(" AND usertype = ");
        qb.push_bind(t);
    }
    if let Some(u) = uid {
        qb.push(" AND uid = ");
        qb.push_bind(u);
    }
    qb.push(" ORDER BY id DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as::<LoginLogRow>().fetch_all(pool).await
}

pub async fn count_login_logs(
    pool: &MySqlPool,
    usertype: Option<i32>,
    uid: Option<u64>,
) -> Result<u64, sqlx::Error> {
    let mut qb =
        sqlx::QueryBuilder::<sqlx::MySql>::new("SELECT COUNT(*) FROM phpyun_login_log WHERE 1=1");
    if let Some(t) = usertype {
        qb.push(" AND usertype = ");
        qb.push_bind(t);
    }
    if let Some(u) = uid {
        qb.push(" AND uid = ");
        qb.push_bind(u);
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

/// PHP `admin_loginlog::index` row after `getLoginlogList(..., utype=admin)`.
#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct PhpLoginLogListRow {
    pub id: u64,
    pub uid: u64,
    pub usertype: i32,
    pub content: String,
    pub ip: String,
    pub ctime: i64,
    pub remoteport: i32,
    pub username: String,
    pub rname: String,
    pub eid: u64,
    pub comname: String,
    pub pid: u64,
}

pub struct PhpLoginLogFilter<'a> {
    pub usertype: i32,
    pub uid: Option<u64>,
    pub username_like: Option<&'a str>,
    pub content_like: Option<&'a str>,
    pub time_from: Option<i64>,
    pub time_to: Option<i64>,
    pub order_t: &'a str,
    pub order_dir: &'a str,
}

fn push_php_login_filters<'a>(qb: &mut QueryBuilder<'a, sqlx::MySql>, f: &PhpLoginLogFilter<'a>) {
    qb.push(" AND l.usertype = ");
    qb.push_bind(f.usertype);
    if let Some(u) = f.uid.filter(|v| *v > 0) {
        qb.push(" AND l.uid = ");
        qb.push_bind(u);
    }
    if let Some(kw) = f.username_like.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND l.uid IN (SELECT uid FROM phpyun_member WHERE username LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(")");
    }
    if let Some(kw) = f.content_like.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND l.content LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    if let Some(t0) = f.time_from {
        qb.push(" AND l.ctime >= ");
        qb.push_bind(t0);
    }
    if let Some(t1) = f.time_to {
        qb.push(" AND l.ctime <= ");
        qb.push_bind(t1);
    }
}

fn push_php_login_order<'a>(qb: &mut QueryBuilder<'a, sqlx::MySql>, t: &str, order: &str) {
    let asc = order.eq_ignore_ascii_case("asc");
    qb.push(" ORDER BY ");
    qb.push(match (t, asc) {
        ("uid", true) => "l.uid ASC",
        ("uid", false) => "l.uid DESC",
        ("ctime", true) => "l.ctime ASC",
        ("ctime", false) => "l.ctime DESC",
        ("id", true) => "l.id ASC",
        _ => "l.id DESC",
    });
}

pub async fn list_php_login_logs(
    pool: &MySqlPool,
    f: &PhpLoginLogFilter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<PhpLoginLogListRow>, sqlx::Error> {
    let limit = phpyun_core::numeric::checked_db_i64(limit, "pagination.limit")?;
    let offset = phpyun_core::numeric::checked_db_i64(offset, "pagination.offset")?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(l.id AS UNSIGNED) AS id, CAST(COALESCE(l.uid,0) AS UNSIGNED) AS uid, \
         CAST(COALESCE(l.usertype,0) AS SIGNED) AS usertype, COALESCE(l.content,'') AS content, \
         COALESCE(l.ip,'') AS ip, CAST(COALESCE(l.ctime,0) AS SIGNED) AS ctime, \
         CAST(COALESCE(l.remoteport,0) AS SIGNED) AS remoteport, \
         COALESCE(m.username,'') AS username, COALESCE(r.name,'') AS rname, \
         CAST(COALESCE(r.def_job,0) AS UNSIGNED) AS eid, COALESCE(c.name,'') AS comname, \
         CAST(COALESCE(m.pid,0) AS UNSIGNED) AS pid \
         FROM phpyun_login_log l \
         LEFT JOIN phpyun_member m ON m.uid = l.uid \
         LEFT JOIN (SELECT uid, MIN(name) AS name, MAX(def_job) AS def_job FROM phpyun_resume GROUP BY uid) r \
           ON r.uid = l.uid \
         LEFT JOIN phpyun_company c ON c.uid = l.uid \
         WHERE 1=1",
    );
    push_php_login_filters(&mut qb, f);
    push_php_login_order(&mut qb, f.order_t, f.order_dir);
    qb.push(" LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_php_login_logs(
    pool: &MySqlPool,
    f: &PhpLoginLogFilter<'_>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_login_log l WHERE 1=1");
    push_php_login_filters(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn delete_php_login_logs(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("DELETE FROM phpyun_login_log WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn delete_php_login_logs_by_usertype(
    pool: &MySqlPool,
    usertype: i32,
) -> Result<u64, sqlx::Error> {
    let r = sqlx::query("DELETE FROM phpyun_login_log WHERE usertype = ?")
        .bind(usertype)
        .execute(pool)
        .await?;
    Ok(r.rows_affected())
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct AdminLogRow {
    pub id: u64,
    pub uid: i64,
    pub username: String,
    pub content: String,
    pub ctime: i64,
    pub ip: String,
}

pub async fn list_admin_logs(
    pool: &MySqlPool,
    offset: u64,
    limit: u64,
) -> Result<Vec<AdminLogRow>, sqlx::Error> {
    sqlx::query_as::<_, AdminLogRow>(
        r#"SELECT CAST(id AS UNSIGNED) AS id,
                  CAST(COALESCE(uid, 0) AS SIGNED) AS uid,
                  COALESCE(username, '') AS username,
                  COALESCE(content, '') AS content,
                  CAST(COALESCE(ctime, 0) AS SIGNED) AS ctime,
                  COALESCE(ip, '') AS ip
           FROM phpyun_admin_log
           ORDER BY id DESC LIMIT ? OFFSET ?"#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn count_admin_logs(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_admin_log")
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}
