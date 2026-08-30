//! Phase-2 admin gap SQL: photo stats, banners, biz logs, rating services, cron/domain extras.

use super::entity::*;
use super::repo::{delete_in, lim};
use crate::soft_delete::{self, PREDICATE};
use sqlx::{MySqlPool, QueryBuilder};

pub fn parse_id_csv(raw: &str) -> Vec<u64> {
    raw.split(|c: char| c == ',' || c == ';' || c.is_whitespace())
        .filter_map(|p| p.trim().parse::<u64>().ok())
        .filter(|n| *n > 0)
        .collect()
}

async fn count_sql(pool: &MySqlPool, sql: &str) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(sql).fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn photo_stat(pool: &MySqlPool) -> Result<PhotoStat, sqlx::Error> {
    Ok(PhotoStat {
        num_all: count_sql(
            pool,
            "SELECT COUNT(*) FROM phpyun_resume WHERE photo <> '' AND COALESCE(defphoto,1)=1",
        )
        .await?,
        num_audited: count_sql(
            pool,
            "SELECT COUNT(*) FROM phpyun_resume WHERE photo <> '' AND COALESCE(defphoto,1)=1 AND photo_status=1",
        )
        .await?,
        num_unaudited: count_sql(
            pool,
            "SELECT COUNT(*) FROM phpyun_resume WHERE photo <> '' AND COALESCE(defphoto,1)=1 AND photo_status=0",
        )
        .await?,
        num_failed: None,
    })
}

pub async fn cert_stat(pool: &MySqlPool) -> Result<PhotoStat, sqlx::Error> {
    Ok(PhotoStat {
        num_all: count_sql(pool, "SELECT COUNT(*) FROM phpyun_resume WHERE idcard_pic <> ''").await?,
        num_audited: count_sql(
            pool,
            "SELECT COUNT(*) FROM phpyun_resume WHERE idcard_pic <> '' AND idcard_status=1",
        )
        .await?,
        num_unaudited: count_sql(
            pool,
            "SELECT COUNT(*) FROM phpyun_resume WHERE idcard_pic <> '' AND idcard_status=0",
        )
        .await?,
        num_failed: Some(
            count_sql(
                pool,
                "SELECT COUNT(*) FROM phpyun_resume WHERE idcard_pic <> '' AND idcard_status=2",
            )
            .await?,
        ),
    })
}

pub async fn msg_stat(pool: &MySqlPool) -> Result<PhotoStat, sqlx::Error> {
    Ok(PhotoStat {
        num_all: count_sql(pool, "SELECT COUNT(*) FROM phpyun_msg WHERE COALESCE(del_status,0)=0")
            .await?,
        num_audited: count_sql(
            pool,
            "SELECT COUNT(*) FROM phpyun_msg WHERE COALESCE(del_status,0)=0 AND status=1",
        )
        .await?,
        num_unaudited: count_sql(
            pool,
            "SELECT COUNT(*) FROM phpyun_msg WHERE COALESCE(del_status,0)=0 AND status=0",
        )
        .await?,
        num_failed: Some(
            count_sql(
                pool,
                "SELECT COUNT(*) FROM phpyun_msg WHERE COALESCE(del_status,0)=0 AND status=2",
            )
            .await?,
        ),
    })
}

pub async fn company_logo_stat(pool: &MySqlPool) -> Result<PhotoStat, sqlx::Error> {
    Ok(PhotoStat {
        num_all: count_sql(pool, "SELECT COUNT(*) FROM phpyun_company WHERE logo <> ''").await?,
        num_audited: count_sql(
            pool,
            "SELECT COUNT(*) FROM phpyun_company WHERE logo <> '' AND logo_status=0",
        )
        .await?,
        num_unaudited: count_sql(
            pool,
            "SELECT COUNT(*) FROM phpyun_company WHERE logo <> '' AND logo_status=1",
        )
        .await?,
        num_failed: None,
    })
}

pub async fn scalar_str(pool: &MySqlPool, sql: &str, id: u64) -> Result<String, sqlx::Error> {
    let v: Option<String> = sqlx::query_scalar(sql).bind(id).fetch_optional(pool).await?;
    Ok(v.unwrap_or_default().trim().to_string())
}

pub async fn photo_statusbody(pool: &MySqlPool, uid: u64) -> Result<String, sqlx::Error> {
    scalar_str(
        pool,
        "SELECT COALESCE(photo_statusbody,'') FROM phpyun_resume WHERE uid=? LIMIT 1",
        uid,
    )
    .await
}

pub async fn cert_statusbody(pool: &MySqlPool, uid: u64) -> Result<String, sqlx::Error> {
    scalar_str(
        pool,
        "SELECT COALESCE(statusbody,'') FROM phpyun_resume WHERE uid=? LIMIT 1",
        uid,
    )
    .await
}

pub async fn logo_statusbody(pool: &MySqlPool, uid: u64) -> Result<String, sqlx::Error> {
    scalar_str(
        pool,
        "SELECT COALESCE(logo_statusbody,'') FROM phpyun_company WHERE uid=? LIMIT 1",
        uid,
    )
    .await
}

pub async fn gallery_statusbody(pool: &MySqlPool, kind: &str, id: u64) -> Result<String, sqlx::Error> {
    let table = if kind == "resume" {
        "phpyun_resume_show"
    } else {
        "phpyun_company_show"
    };
    let sql = format!(
        "SELECT COALESCE(statusbody,'') FROM {table} WHERE id=? AND COALESCE(deleted,0)=0 LIMIT 1"
    );
    scalar_str(pool, &sql, id).await
}

pub async fn banner_statusbody(pool: &MySqlPool, id: u64) -> Result<String, sqlx::Error> {
    scalar_str(
        pool,
        "SELECT COALESCE(statusbody,'') FROM phpyun_banner WHERE id=? AND COALESCE(deleted,0)=0 LIMIT 1",
        id,
    )
    .await
}

pub async fn set_photo_review(
    pool: &MySqlPool,
    uid: u64,
    status: i32,
    body: &str,
) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_resume SET photo_status=?, photo_statusbody=? WHERE uid=?")
            .bind(status)
            .bind(body)
            .bind(uid)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

pub async fn save_user_photo(pool: &MySqlPool, uid: u64, photo: &str) -> Result<u64, sqlx::Error> {
    Ok(sqlx::query("UPDATE phpyun_resume SET photo=? WHERE uid=?")
        .bind(photo)
        .bind(uid)
        .execute(pool)
        .await?
        .rows_affected())
}

pub async fn clear_user_photos(pool: &MySqlPool, uids: &[u64]) -> Result<u64, sqlx::Error> {
    delete_in(
        pool,
        "UPDATE phpyun_resume SET photo='', photo_status=0 WHERE uid IN (",
        uids,
    )
    .await
}

pub async fn set_idcard_review(
    pool: &MySqlPool,
    uid: u64,
    status: i32,
    body: &str,
) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_resume SET idcard_status=?, statusbody=? WHERE uid=?")
            .bind(status)
            .bind(body)
            .bind(uid)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

pub async fn set_logo_review(
    pool: &MySqlPool,
    uid: u64,
    status: i32,
    body: &str,
) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_company SET logo_status=?, logo_statusbody=? WHERE uid=?")
            .bind(status)
            .bind(body)
            .bind(uid)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

pub async fn save_company_logo(pool: &MySqlPool, uid: u64, logo: &str) -> Result<u64, sqlx::Error> {
    Ok(sqlx::query("UPDATE phpyun_company SET logo=? WHERE uid=?")
        .bind(logo)
        .bind(uid)
        .execute(pool)
        .await?
        .rows_affected())
}

pub async fn clear_company_logos(pool: &MySqlPool, uids: &[u64]) -> Result<u64, sqlx::Error> {
    delete_in(
        pool,
        "UPDATE phpyun_company SET logo='', logo_status=0 WHERE uid IN (",
        uids,
    )
    .await
}

pub async fn save_gallery_pic(
    pool: &MySqlPool,
    kind: &str,
    id: u64,
    picurl: &str,
    title: &str,
) -> Result<u64, sqlx::Error> {
    let table = if kind == "resume" {
        "phpyun_resume_show"
    } else {
        "phpyun_company_show"
    };
    let sql = format!("UPDATE {table} SET picurl=?, title=? WHERE id=? AND COALESCE(deleted,0)=0");
    Ok(sqlx::query(&sql)
        .bind(picurl)
        .bind(title)
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected())
}

pub async fn delete_gallery(pool: &MySqlPool, kind: &str, ids: &[u64]) -> Result<u64, sqlx::Error> {
    let table = if kind == "resume" {
        "phpyun_resume_show"
    } else {
        "phpyun_company_show"
    };
    soft_delete::mark_ids(pool, table, ids).await
}

pub async fn set_gallery_review(
    pool: &MySqlPool,
    kind: &str,
    ids: &[u64],
    status: i32,
    body: &str,
) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let table = if kind == "resume" {
        "phpyun_resume_show"
    } else {
        "phpyun_company_show"
    };
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new(format!("UPDATE {table} SET status="));
    qb.push_bind(status);
    qb.push(", statusbody=");
    qb.push_bind(body);
    qb.push(" WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

const BANNER_FIELDS: &str = "CAST(b.id AS UNSIGNED) AS id, CAST(COALESCE(b.uid,0) AS UNSIGNED) AS uid, \
    COALESCE(b.pic,'') AS pic, CAST(COALESCE(b.status,0) AS SIGNED) AS status, \
    COALESCE(b.statusbody,'') AS statusbody, COALESCE(c.name,'') AS name";

pub async fn list_banners(
    pool: &MySqlPool,
    status: Option<i32>,
    keyword: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<BannerAdminRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(format!(
        "SELECT {BANNER_FIELDS} FROM phpyun_banner b \
         LEFT JOIN phpyun_company c ON c.uid=b.uid WHERE COALESCE(b.deleted,0)=0"
    ));
    if let Some(s) = status {
        qb.push(" AND b.status=");
        qb.push_bind(s);
    }
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND (c.name LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR b.uid=");
        qb.push_bind(kw.parse::<u64>().unwrap_or(0));
        qb.push(")");
    }
    qb.push(" ORDER BY b.status DESC, b.id DESC LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_banners(
    pool: &MySqlPool,
    status: Option<i32>,
    keyword: Option<&str>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT COUNT(*) FROM phpyun_banner b LEFT JOIN phpyun_company c ON c.uid=b.uid WHERE COALESCE(b.deleted,0)=0",
    );
    if let Some(s) = status {
        qb.push(" AND b.status=");
        qb.push_bind(s);
    }
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND (c.name LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR b.uid=");
        qb.push_bind(kw.parse::<u64>().unwrap_or(0));
        qb.push(")");
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn set_banner_status(
    pool: &MySqlPool,
    ids: &[u64],
    status: i32,
    body: &str,
) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("UPDATE phpyun_banner SET status=");
    qb.push_bind(status);
    qb.push(", statusbody=");
    qb.push_bind(body);
    qb.push(" WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn save_banner(
    pool: &MySqlPool,
    id: Option<u64>,
    uid: u64,
    pic: &str,
) -> Result<u64, sqlx::Error> {
    if let Some(id) = id.filter(|v| *v > 0) {
        sqlx::query("UPDATE phpyun_banner SET pic=? WHERE id=?")
            .bind(pic)
            .bind(id)
            .execute(pool)
            .await?;
        return Ok(id);
    }
    Ok(
        sqlx::query("INSERT INTO phpyun_banner (uid, pic, status) VALUES (?, ?, 1)")
            .bind(uid)
            .bind(pic)
            .execute(pool)
            .await?
            .last_insert_id(),
    )
}

pub async fn delete_banners(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    soft_delete::mark_ids(pool, "phpyun_banner", ids).await
}

const BIZ_SELECT: &str = "CAST(t.id AS UNSIGNED) AS id, CAST(COALESCE(t.uid,0) AS UNSIGNED) AS uid, \
    CAST(COALESCE(t.comid,0) AS UNSIGNED) AS comid, CAST(COALESCE(t.eid,0) AS UNSIGNED) AS eid, \
    CAST(COALESCE(t.jobid,0) AS UNSIGNED) AS jobid, COALESCE(t.username,'') AS username, \
    COALESCE(t.com_name,'') AS com_name, COALESCE(t.com_username,'') AS com_username, \
    COALESCE(t.job_name,'') AS job_name, COALESCE(t.telphone,'') AS telphone, \
    CAST(COALESCE(t.datetime,0) AS SIGNED) AS datetime, CAST(COALESCE(t.is_browse,0) AS SIGNED) AS is_browse, \
    COALESCE(t.isdel_n,'') AS isdel_n, CAST(COALESCE(t.status,0) AS SIGNED) AS status, \
    COALESCE(t.title,'') AS title, COALESCE(t.ip,'') AS ip, COALESCE(t.remark,'') AS remark, \
    COALESCE(t.pic,'') AS pic";

async fn list_biz(
    pool: &MySqlPool,
    inner: &str,
    keyword: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<BizLogRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new(format!("SELECT {BIZ_SELECT} FROM ({inner}) t WHERE 1=1"));
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND (t.username LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR t.com_name LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR t.job_name LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(")");
    }
    qb.push(" ORDER BY t.id DESC LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

async fn count_biz(pool: &MySqlPool, inner: &str, keyword: Option<&str>) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new(format!("SELECT COUNT(*) FROM ({inner}) t WHERE 1=1"));
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND (t.username LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR t.com_name LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR t.job_name LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(")");
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

const DOWN_INNER: &str = "SELECT d.id, d.uid, d.comid, d.eid, 0 AS jobid, \
    COALESCE(r.name,'') AS username, COALESCE(c.name,'') AS com_name, \
    COALESCE(m.username,'') AS com_username, '' AS job_name, COALESCE(r.telphone,'') AS telphone, \
    COALESCE(d.downtime,0) AS datetime, 0 AS is_browse, \
    CASE WHEN COALESCE(d.isdel,9)=1 THEN '1' ELSE '' END AS isdel_n, \
    COALESCE(d.status,0) AS status, '' AS title, '' AS ip, COALESCE(d.remark,'') AS remark, \
    COALESCE(r.photo,'') AS pic \
    FROM phpyun_down_resume d \
    LEFT JOIN phpyun_resume r ON r.uid=d.uid \
    LEFT JOIN phpyun_company c ON c.uid=d.comid \
    LEFT JOIN phpyun_member m ON m.uid=d.comid";

const FREEDOWN_INNER: &str = "SELECT d.id, d.uid, d.comid, d.eid, 0 AS jobid, \
    COALESCE(r.name,'') AS username, COALESCE(c.name,'') AS com_name, \
    COALESCE(m.username,'') AS com_username, '' AS job_name, COALESCE(r.telphone,'') AS telphone, \
    COALESCE(d.downtime,0) AS datetime, 0 AS is_browse, '' AS isdel_n, \
    COALESCE(d.status,0) AS status, '' AS title, '' AS ip, '' AS remark, COALESCE(r.photo,'') AS pic \
    FROM phpyun_freedown_resume d \
    LEFT JOIN phpyun_resume r ON r.uid=d.uid \
    LEFT JOIN phpyun_company c ON c.uid=d.comid \
    LEFT JOIN phpyun_member m ON m.uid=d.comid";

const LOOK_RESUME_INNER: &str = "SELECT d.id, d.uid, d.com_id AS comid, COALESCE(d.resume_id,0) AS eid, 0 AS jobid, \
    COALESCE(r.name,'') AS username, COALESCE(c.name,'') AS com_name, \
    COALESCE(m.username,'') AS com_username, '' AS job_name, COALESCE(r.telphone,'') AS telphone, \
    COALESCE(d.datetime,0) AS datetime, 0 AS is_browse, '' AS isdel_n, \
    COALESCE(d.status,0) AS status, '' AS title, COALESCE(d.ip,'') AS ip, '' AS remark, COALESCE(r.photo,'') AS pic \
    FROM phpyun_look_resume d \
    LEFT JOIN phpyun_resume r ON r.uid=d.uid \
    LEFT JOIN phpyun_company c ON c.uid=d.com_id \
    LEFT JOIN phpyun_member m ON m.uid=d.com_id";

const TALENT_INNER: &str = "SELECT d.id, d.uid, COALESCE(d.cuid,0) AS comid, COALESCE(d.eid,0) AS eid, 0 AS jobid, \
    COALESCE(r.name,'') AS username, COALESCE(c.name,'') AS com_name, \
    COALESCE(m.username,'') AS com_username, '' AS job_name, COALESCE(r.telphone,'') AS telphone, \
    COALESCE(d.ctime,0) AS datetime, 0 AS is_browse, '' AS isdel_n, 0 AS status, \
    '' AS title, '' AS ip, COALESCE(d.remark,'') AS remark, COALESCE(r.photo,'') AS pic \
    FROM phpyun_talent_pool d \
    LEFT JOIN phpyun_resume r ON r.uid=d.uid \
    LEFT JOIN phpyun_company c ON c.uid=d.cuid \
    LEFT JOIN phpyun_member m ON m.uid=d.cuid";

const TRUST_INNER: &str = "SELECT d.id, d.uid, COALESCE(d.comid,0) AS comid, COALESCE(d.eid,0) AS eid, COALESCE(d.jobid,0) AS jobid, \
    COALESCE(r.name,'') AS username, COALESCE(c.name,'') AS com_name, \
    COALESCE(m.username,'') AS com_username, COALESCE(j.name,'') AS job_name, COALESCE(r.telphone,'') AS telphone, \
    COALESCE(d.ctime,0) AS datetime, 0 AS is_browse, '' AS isdel_n, 0 AS status, \
    '' AS title, '' AS ip, '' AS remark, COALESCE(r.photo,'') AS pic \
    FROM phpyun_user_entrust_record d \
    LEFT JOIN phpyun_resume r ON r.uid=d.uid \
    LEFT JOIN phpyun_company c ON c.uid=d.comid \
    LEFT JOIN phpyun_member m ON m.uid=d.comid \
    LEFT JOIN phpyun_company_job j ON j.id=d.jobid";

const REFRESH_INNER: &str = "SELECT d.id, d.uid, 0 AS comid, COALESCE(d.resume_id,0) AS eid, 0 AS jobid, \
    COALESCE(r.name,'') AS username, '' AS com_name, COALESCE(m.username,'') AS com_username, \
    '' AS job_name, COALESCE(r.telphone,'') AS telphone, \
    CAST(UNIX_TIMESTAMP(STR_TO_DATE(NULLIF(d.r_time,''),'%Y%m%d')) AS SIGNED) AS datetime, \
    0 AS is_browse, '' AS isdel_n, 0 AS status, COALESCE(d.r_time,'') AS title, \
    COALESCE(d.ip,'') AS ip, '' AS remark, COALESCE(r.photo,'') AS pic \
    FROM phpyun_resume_refresh_log d \
    LEFT JOIN phpyun_resume r ON r.uid=d.uid \
    LEFT JOIN phpyun_member m ON m.uid=d.uid";

const USERID_JOB_INNER: &str = "SELECT d.id, d.uid, d.com_id AS comid, COALESCE(d.eid,0) AS eid, COALESCE(d.job_id,0) AS jobid, \
    COALESCE(r.name,'') AS username, COALESCE(d.com_name,'') AS com_name, \
    COALESCE(m.username,'') AS com_username, COALESCE(d.job_name,'') AS job_name, COALESCE(r.telphone,'') AS telphone, \
    COALESCE(d.datetime,0) AS datetime, COALESCE(d.is_browse,0) AS is_browse, \
    CASE WHEN COALESCE(d.isdel,9)=1 THEN '1' ELSE '' END AS isdel_n, \
    0 AS status, '' AS title, '' AS ip, COALESCE(d.remark,'') AS remark, COALESCE(r.photo,'') AS pic \
    FROM phpyun_userid_job d \
    LEFT JOIN phpyun_resume r ON r.uid=d.uid \
    LEFT JOIN phpyun_member m ON m.uid=d.com_id";

const USERID_MSG_INNER: &str = "SELECT d.id, d.uid, COALESCE(d.fid,0) AS comid, 0 AS eid, COALESCE(d.jobid,0) AS jobid, \
    COALESCE(r.name,'') AS username, COALESCE(d.fname,'') AS com_name, \
    COALESCE(m.username,'') AS com_username, COALESCE(d.jobname,'') AS job_name, COALESCE(d.linktel,'') AS telphone, \
    COALESCE(d.datetime,0) AS datetime, COALESCE(d.is_browse,0) AS is_browse, \
    CASE WHEN COALESCE(d.isdel,9)=1 THEN '1' ELSE '' END AS isdel_n, \
    0 AS status, COALESCE(d.title,'') AS title, '' AS ip, COALESCE(d.remark,'') AS remark, COALESCE(r.photo,'') AS pic \
    FROM phpyun_userid_msg d \
    LEFT JOIN phpyun_resume r ON r.uid=d.uid \
    LEFT JOIN phpyun_member m ON m.uid=d.fid";

const LOOK_JOB_INNER: &str = "SELECT d.id, d.uid, d.com_id AS comid, 0 AS eid, COALESCE(d.jobid,0) AS jobid, \
    COALESCE(r.name,'') AS username, COALESCE(c.name,'') AS com_name, \
    COALESCE(m.username,'') AS com_username, COALESCE(j.name,'') AS job_name, COALESCE(r.telphone,'') AS telphone, \
    COALESCE(d.datetime,0) AS datetime, 0 AS is_browse, '' AS isdel_n, \
    COALESCE(d.status,0) AS status, '' AS title, COALESCE(d.ip,'') AS ip, '' AS remark, COALESCE(r.photo,'') AS pic \
    FROM phpyun_look_job d \
    LEFT JOIN phpyun_resume r ON r.uid=d.uid \
    LEFT JOIN phpyun_company c ON c.uid=d.com_id \
    LEFT JOIN phpyun_member m ON m.uid=d.com_id \
    LEFT JOIN phpyun_company_job j ON j.id=d.jobid";

const PART_APPLY_INNER: &str = "SELECT d.id, d.uid, COALESCE(d.comid,0) AS comid, 0 AS eid, COALESCE(d.jobid,0) AS jobid, \
    COALESCE(r.name,'') AS username, COALESCE(c.name,'') AS com_name, \
    COALESCE(m.username,'') AS com_username, COALESCE(j.name,'') AS job_name, COALESCE(r.telphone,'') AS telphone, \
    COALESCE(d.ctime,0) AS datetime, 0 AS is_browse, '' AS isdel_n, \
    COALESCE(d.status,0) AS status, '' AS title, '' AS ip, '' AS remark, COALESCE(r.photo,'') AS pic \
    FROM phpyun_part_apply d \
    LEFT JOIN phpyun_resume r ON r.uid=d.uid \
    LEFT JOIN phpyun_company c ON c.uid=d.comid \
    LEFT JOIN phpyun_member m ON m.uid=d.comid \
    LEFT JOIN phpyun_partjob j ON j.id=d.jobid";

const FAV_JOB_INNER: &str = "SELECT d.id, d.uid, d.com_id AS comid, 0 AS eid, COALESCE(d.job_id,0) AS jobid, \
    COALESCE(r.name,'') AS username, COALESCE(d.com_name,'') AS com_name, \
    COALESCE(m.username,'') AS com_username, COALESCE(d.job_name,'') AS job_name, COALESCE(r.telphone,'') AS telphone, \
    COALESCE(d.datetime,0) AS datetime, 0 AS is_browse, '' AS isdel_n, 0 AS status, \
    '' AS title, '' AS ip, '' AS remark, COALESCE(r.photo,'') AS pic \
    FROM phpyun_fav_job d \
    LEFT JOIN phpyun_resume r ON r.uid=d.uid \
    LEFT JOIN phpyun_member m ON m.uid=d.com_id";

const JOB_TELLOG_INNER: &str = "SELECT d.id, d.uid, COALESCE(d.comid,0) AS comid, 0 AS eid, COALESCE(d.jobid,0) AS jobid, \
    COALESCE(r.name,'') AS username, COALESCE(c.name,'') AS com_name, \
    COALESCE(m.username,'') AS com_username, COALESCE(j.name,'') AS job_name, COALESCE(r.telphone,'') AS telphone, \
    COALESCE(d.ctime,0) AS datetime, 0 AS is_browse, '' AS isdel_n, 0 AS status, \
    '' AS title, COALESCE(d.ip,'') AS ip, '' AS remark, COALESCE(r.photo,'') AS pic \
    FROM phpyun_job_tellog d \
    LEFT JOIN phpyun_resume r ON r.uid=d.uid \
    LEFT JOIN phpyun_company c ON c.uid=d.comid \
    LEFT JOIN phpyun_member m ON m.uid=d.comid \
    LEFT JOIN phpyun_company_job j ON j.id=d.jobid";

macro_rules! biz_pair {
    ($list:ident, $count:ident, $inner:expr) => {
        pub async fn $list(
            pool: &MySqlPool,
            keyword: Option<&str>,
            offset: u64,
            limit: u64,
        ) -> Result<Vec<BizLogRow>, sqlx::Error> {
            list_biz(pool, $inner, keyword, offset, limit).await
        }
        pub async fn $count(pool: &MySqlPool, keyword: Option<&str>) -> Result<u64, sqlx::Error> {
            count_biz(pool, $inner, keyword).await
        }
    };
}

biz_pair!(list_down, count_down, DOWN_INNER);
biz_pair!(list_freedown, count_freedown, FREEDOWN_INNER);
biz_pair!(list_look_resume, count_look_resume, LOOK_RESUME_INNER);
biz_pair!(list_talent, count_talent, TALENT_INNER);
biz_pair!(list_trust, count_trust, TRUST_INNER);
biz_pair!(list_refresh_resume, count_refresh_resume, REFRESH_INNER);
biz_pair!(list_userid_job, count_userid_job, USERID_JOB_INNER);
biz_pair!(list_userid_msg, count_userid_msg, USERID_MSG_INNER);
biz_pair!(list_look_job, count_look_job, LOOK_JOB_INNER);
biz_pair!(list_part_apply, count_part_apply, PART_APPLY_INNER);
biz_pair!(list_fav_job, count_fav_job, FAV_JOB_INNER);
biz_pair!(list_job_tellog, count_job_tellog, JOB_TELLOG_INNER);

const SVC_FIELDS: &str = "CAST(id AS UNSIGNED) AS id, COALESCE(name,'') AS name, \
    CAST(COALESCE(display,1) AS SIGNED) AS display, CAST(COALESCE(sort,0) AS SIGNED) AS sort";

pub async fn list_rating_services(pool: &MySqlPool) -> Result<Vec<RatingServiceRow>, sqlx::Error> {
    let sql = format!("SELECT {SVC_FIELDS} FROM phpyun_company_service WHERE {PREDICATE} ORDER BY sort DESC, id DESC");
    sqlx::query_as::<_, RatingServiceRow>(&sql).fetch_all(pool).await
}

pub async fn upsert_rating_service(
    pool: &MySqlPool,
    id: Option<u64>,
    name: &str,
    display: i32,
    sort: i32,
) -> Result<u64, sqlx::Error> {
    if let Some(id) = id.filter(|v| *v > 0) {
        sqlx::query("UPDATE phpyun_company_service SET name=?, display=?, sort=? WHERE id=?")
            .bind(name)
            .bind(display)
            .bind(sort)
            .bind(id)
            .execute(pool)
            .await?;
        return Ok(id);
    }
    Ok(
        sqlx::query("INSERT INTO phpyun_company_service (name, display, sort) VALUES (?, ?, ?)")
            .bind(name)
            .bind(display)
            .bind(sort)
            .execute(pool)
            .await?
            .last_insert_id(),
    )
}

pub async fn set_rating_service_display(
    pool: &MySqlPool,
    id: u64,
    display: i32,
) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_company_service SET display=? WHERE id=?")
            .bind(display)
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

pub async fn delete_rating_services(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    soft_delete::mark_col_in(pool, "phpyun_company_service_detail", "type", ids).await?;
    soft_delete::mark_ids(pool, "phpyun_company_service", ids).await
}

pub async fn find_rating_service(
    pool: &MySqlPool,
    id: u64,
) -> Result<Option<RatingServiceRow>, sqlx::Error> {
    let sql = format!("SELECT {SVC_FIELDS} FROM phpyun_company_service WHERE id=? AND {PREDICATE} LIMIT 1");
    sqlx::query_as::<_, RatingServiceRow>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

const DETAIL_FIELDS: &str = "CAST(id AS UNSIGNED) AS id, COALESCE(service_price,'') AS service_price, \
    CAST(COALESCE(resume,0) AS SIGNED) AS resume, CAST(COALESCE(interview,0) AS SIGNED) AS interview, \
    CAST(COALESCE(job_num,0) AS SIGNED) AS job_num, CAST(COALESCE(breakjob_num,0) AS SIGNED) AS breakjob_num, \
    CAST(COALESCE(part_num,0) AS SIGNED) AS part_num, CAST(COALESCE(breakpart_num,0) AS SIGNED) AS breakpart_num, \
    CAST(COALESCE(lt_job_num,0) AS SIGNED) AS lt_job_num, CAST(COALESCE(lt_breakjob_num,0) AS SIGNED) AS lt_breakjob_num, \
    CAST(COALESCE(lt_resume,0) AS SIGNED) AS lt_resume, CAST(COALESCE(`type`,0) AS SIGNED) AS `type`, \
    CAST(COALESCE(sort,0) AS SIGNED) AS sort, CAST(COALESCE(zph_num,0) AS SIGNED) AS zph_num, \
    CAST(COALESCE(top_num,0) AS SIGNED) AS top_num, CAST(COALESCE(rec_num,0) AS SIGNED) AS rec_num, \
    CAST(COALESCE(urgent_num,0) AS SIGNED) AS urgent_num";

pub async fn list_rating_details(
    pool: &MySqlPool,
    type_id: u64,
) -> Result<Vec<RatingServiceDetailRow>, sqlx::Error> {
    let sql = format!(
        "SELECT {DETAIL_FIELDS} FROM phpyun_company_service_detail WHERE `type`=? AND COALESCE(deleted,0)=0 ORDER BY sort DESC, id DESC"
    );
    sqlx::query_as::<_, RatingServiceDetailRow>(&sql)
        .bind(type_id)
        .fetch_all(pool)
        .await
}

pub async fn find_rating_detail(
    pool: &MySqlPool,
    id: u64,
) -> Result<Option<RatingServiceDetailRow>, sqlx::Error> {
    let sql = format!(
        "SELECT {DETAIL_FIELDS} FROM phpyun_company_service_detail WHERE id=? AND COALESCE(deleted,0)=0 LIMIT 1"
    );
    sqlx::query_as::<_, RatingServiceDetailRow>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub struct RatingDetailIn<'a> {
    pub id: Option<u64>,
    pub r#type: i32,
    pub service_price: &'a str,
    pub resume: i32,
    pub interview: i32,
    pub job_num: i32,
    pub breakjob_num: i32,
    pub part_num: i32,
    pub breakpart_num: i32,
    pub lt_job_num: i32,
    pub lt_breakjob_num: i32,
    pub lt_resume: i32,
    pub sort: i32,
    pub zph_num: i32,
    pub top_num: i32,
    pub rec_num: i32,
    pub urgent_num: i32,
}

pub async fn upsert_rating_detail(pool: &MySqlPool, w: RatingDetailIn<'_>) -> Result<u64, sqlx::Error> {
    if let Some(id) = w.id.filter(|v| *v > 0) {
        sqlx::query(
            "UPDATE phpyun_company_service_detail SET service_price=?, resume=?, interview=?, job_num=?, \
             breakjob_num=?, part_num=?, breakpart_num=?, lt_job_num=?, lt_breakjob_num=?, lt_resume=?, \
             `type`=?, sort=?, zph_num=?, top_num=?, rec_num=?, urgent_num=? WHERE id=?",
        )
        .bind(w.service_price)
        .bind(w.resume)
        .bind(w.interview)
        .bind(w.job_num)
        .bind(w.breakjob_num)
        .bind(w.part_num)
        .bind(w.breakpart_num)
        .bind(w.lt_job_num)
        .bind(w.lt_breakjob_num)
        .bind(w.lt_resume)
        .bind(w.r#type)
        .bind(w.sort)
        .bind(w.zph_num)
        .bind(w.top_num)
        .bind(w.rec_num)
        .bind(w.urgent_num)
        .bind(id)
        .execute(pool)
        .await?;
        return Ok(id);
    }
    Ok(sqlx::query(
        "INSERT INTO phpyun_company_service_detail (service_price, resume, interview, job_num, breakjob_num, \
         part_num, breakpart_num, lt_job_num, lt_breakjob_num, lt_resume, `type`, sort, zph_num, top_num, rec_num, urgent_num) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(w.service_price)
    .bind(w.resume)
    .bind(w.interview)
    .bind(w.job_num)
    .bind(w.breakjob_num)
    .bind(w.part_num)
    .bind(w.breakpart_num)
    .bind(w.lt_job_num)
    .bind(w.lt_breakjob_num)
    .bind(w.lt_resume)
    .bind(w.r#type)
    .bind(w.sort)
    .bind(w.zph_num)
    .bind(w.top_num)
    .bind(w.rec_num)
    .bind(w.urgent_num)
    .execute(pool)
    .await?
    .last_insert_id())
}

const DOMAIN_FIELDS: &str = "CAST(id AS UNSIGNED) AS id, COALESCE(title,'') AS title, COALESCE(domain,'') AS domain, \
    CAST(COALESCE(fz_type,0) AS SIGNED) AS fz_type, CAST(COALESCE(mode,0) AS SIGNED) AS mode, \
    COALESCE(webtitle,'') AS web_title, COALESCE(indexdir,'') AS indexdir, \
    COALESCE(style,'') AS style, CAST(COALESCE(hy,0) AS SIGNED) AS hy, \
    CAST(COALESCE(cityid,0) AS SIGNED) AS cityid, CAST(COALESCE(province,0) AS SIGNED) AS province, \
    COALESCE(tpl,'') AS tpl";

pub async fn find_domain(pool: &MySqlPool, id: u64) -> Result<Option<DomainAdminRow>, sqlx::Error> {
    let sql = format!("SELECT {DOMAIN_FIELDS} FROM phpyun_domain WHERE id=? AND {PREDICATE} LIMIT 1");
    sqlx::query_as::<_, DomainAdminRow>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn upsert_domain_full(
    pool: &MySqlPool,
    id: Option<u64>,
    title: &str,
    domain: &str,
    fz_type: i32,
    mode: i32,
    web_title: &str,
    indexdir: &str,
    style: &str,
    hy: i32,
    cityid: i32,
    province: i32,
    tpl: &str,
) -> Result<u64, sqlx::Error> {
    if let Some(id) = id.filter(|v| *v > 0) {
        sqlx::query(
            "UPDATE phpyun_domain SET title=?, domain=?, fz_type=?, mode=?, webtitle=?, indexdir=?, \
             style=?, hy=?, cityid=?, province=?, tpl=? WHERE id=?",
        )
        .bind(title)
        .bind(domain)
        .bind(fz_type)
        .bind(mode)
        .bind(web_title)
        .bind(indexdir)
        .bind(style)
        .bind(hy)
        .bind(cityid)
        .bind(province)
        .bind(tpl)
        .bind(id)
        .execute(pool)
        .await?;
        return Ok(id);
    }
    Ok(sqlx::query(
        "INSERT INTO phpyun_domain (title, domain, fz_type, mode, webtitle, indexdir, style, hy, cityid, province, tpl) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(title)
    .bind(domain)
    .bind(fz_type)
    .bind(mode)
    .bind(web_title)
    .bind(indexdir)
    .bind(style)
    .bind(hy)
    .bind(cityid)
    .bind(province)
    .bind(tpl)
    .execute(pool)
    .await?
    .last_insert_id())
}

pub async fn upsert_domain_admin(
    pool: &MySqlPool,
    uid: Option<u64>,
    username: &str,
    name: &str,
    password: Option<&str>,
    m_id: i32,
    did: u64,
) -> Result<u64, sqlx::Error> {
    if let Some(uid) = uid.filter(|v| *v > 0) {
        if let Some(pw) = password.map(str::trim).filter(|s| !s.is_empty()) {
            sqlx::query(
                "UPDATE phpyun_admin_user SET username=?, name=?, password=?, m_id=?, did=? WHERE uid=?",
            )
            .bind(username)
            .bind(name)
            .bind(pw)
            .bind(m_id)
            .bind(did)
            .bind(uid)
            .execute(pool)
            .await?;
        } else {
            sqlx::query("UPDATE phpyun_admin_user SET username=?, name=?, m_id=?, did=? WHERE uid=?")
                .bind(username)
                .bind(name)
                .bind(m_id)
                .bind(did)
                .bind(uid)
                .execute(pool)
                .await?;
        }
        return Ok(uid);
    }
    let pw = password.unwrap_or("");
    Ok(
        sqlx::query(
            "INSERT INTO phpyun_admin_user (username, name, password, m_id, did, status) VALUES (?, ?, ?, ?, ?, 1)",
        )
        .bind(username)
        .bind(name)
        .bind(pw)
        .bind(m_id)
        .bind(did)
        .execute(pool)
        .await?
        .last_insert_id(),
    )
}

pub async fn delete_domain_admins(pool: &MySqlPool, uids: &[u64]) -> Result<u64, sqlx::Error> {
    delete_in(
        pool,
        "UPDATE phpyun_admin_user SET status=0 WHERE did>0 AND uid IN (",
        uids,
    )
    .await
}

pub async fn recup_hot_key(pool: &MySqlPool, id: u64, col: &str, rec: i32) -> Result<u64, sqlx::Error> {
    let col = match col {
        "bold" => "bold",
        "tuijian" => "tuijian",
        "check" => "`check`",
        _ => return Ok(0),
    };
    let sql = format!("UPDATE phpyun_hot_key SET {col}=? WHERE id=?");
    Ok(sqlx::query(&sql)
        .bind(rec)
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected())
}

pub async fn batch_hot_key_status(
    pool: &MySqlPool,
    ids: &[u64],
    check: i32,
    tuijian: i32,
    bold: i32,
    color: &str,
    size: &str,
    r#type: Option<i32>,
) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("UPDATE phpyun_hot_key SET `check`=");
    qb.push_bind(check);
    qb.push(", tuijian=");
    qb.push_bind(tuijian);
    qb.push(", bold=");
    qb.push_bind(bold);
    qb.push(", color=");
    qb.push_bind(color);
    qb.push(", size=");
    qb.push_bind(size);
    if let Some(t) = r#type.filter(|v| *v > 0) {
        qb.push(", `type`=");
        qb.push_bind(t);
    }
    qb.push(" WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn find_cron(pool: &MySqlPool, id: u64) -> Result<Option<CronRow>, sqlx::Error> {
    sqlx::query_as::<_, CronRow>(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(name,'') AS name, COALESCE(dir,'') AS dir, \
         CAST(COALESCE(`type`,0) AS SIGNED) AS `type`, CAST(COALESCE(week,0) AS SIGNED) AS week, \
         CAST(COALESCE(month,0) AS SIGNED) AS month, CAST(COALESCE(hour,0) AS SIGNED) AS hour, \
         CAST(COALESCE(minute,0) AS SIGNED) AS minute, CAST(COALESCE(display,0) AS SIGNED) AS display, \
         CAST(COALESCE(nowtime,0) AS SIGNED) AS nowtime, CAST(COALESCE(nexttime,0) AS SIGNED) AS nexttime \
         FROM phpyun_cron WHERE id=? AND COALESCE(deleted,0)=0 LIMIT 1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn touch_cron(pool: &MySqlPool, id: u64, now: i64) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_cron SET nowtime=? WHERE id=?")
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

pub async fn insert_cron_log(pool: &MySqlPool, cid: &str, now: i64) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("INSERT INTO phpyun_cron_log (cid, ctime) VALUES (?, ?)")
            .bind(cid)
            .bind(now)
            .execute(pool)
            .await?
            .last_insert_id(),
    )
}

pub async fn list_cron_logs(
    pool: &MySqlPool,
    keyword: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<CronLogRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(l.id AS UNSIGNED) AS id, COALESCE(l.cid,'') AS cid, \
         CAST(COALESCE(l.ctime,0) AS SIGNED) AS ctime, COALESCE(c.name,'') AS name \
         FROM phpyun_cron_log l LEFT JOIN phpyun_cron c ON CAST(c.id AS CHAR)=l.cid AND COALESCE(c.deleted,0)=0 WHERE 1=1",
    );
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND c.name LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    qb.push(" ORDER BY l.id DESC LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_cron_logs(pool: &MySqlPool, keyword: Option<&str>) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT COUNT(*) FROM phpyun_cron_log l LEFT JOIN phpyun_cron c ON CAST(c.id AS CHAR)=l.cid AND COALESCE(c.deleted,0)=0 WHERE 1=1",
    );
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND c.name LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn set_special_com_status_ids(
    pool: &MySqlPool,
    ids: &[u64],
    status: i32,
    statusbody: &str,
) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("UPDATE phpyun_special_com SET status=");
    qb.push_bind(status);
    qb.push(", statusbody=");
    qb.push_bind(statusbody);
    qb.push(" WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn delete_special_coms(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    soft_delete::mark_ids(pool, "phpyun_special_com", ids).await
}

pub async fn list_marketing_export(
    pool: &MySqlPool,
    xls_type: &str,
    usertype: i32,
    limit: i64,
) -> Result<Vec<MarketingExportRow>, sqlx::Error> {
    let sql = if xls_type == "email" {
        "SELECT CAST(uid AS UNSIGNED) AS uid, COALESCE(username,'') AS username, \
         COALESCE(email,'') AS email, COALESCE(moblie,'') AS moblie \
         FROM phpyun_member WHERE email <> '' AND status=1 AND usertype=? LIMIT ?"
    } else {
        "SELECT CAST(uid AS UNSIGNED) AS uid, COALESCE(username,'') AS username, \
         COALESCE(email,'') AS email, COALESCE(moblie,'') AS moblie \
         FROM phpyun_member WHERE moblie <> '' AND status=1 AND usertype=? LIMIT ?"
    };
    sqlx::query_as::<_, MarketingExportRow>(sql)
        .bind(usertype)
        .bind(limit)
        .fetch_all(pool)
        .await
}

pub async fn list_admin_email(pool: &MySqlPool) -> Result<Vec<AdminEmailRow>, sqlx::Error> {
    sqlx::query_as::<_, AdminEmailRow>(
        "SELECT CAST(id AS UNSIGNED) AS id, \
                COALESCE(smtpserver,'') AS smtpserver, \
                COALESCE(smtpuser,'') AS smtpuser, \
                COALESCE(smtppass,'') AS smtppass, \
                COALESCE(smtpport,'') AS smtpport, \
                COALESCE(smtpnick,'') AS smtpnick, \
                CAST(COALESCE(`default`,0) AS SIGNED) AS default_flag \
         FROM phpyun_admin_email ORDER BY id ASC",
    )
    .fetch_all(pool)
    .await
}

pub async fn list_news_property(pool: &MySqlPool) -> Result<Vec<NewsPropertyRow>, sqlx::Error> {
    sqlx::query_as::<_, NewsPropertyRow>(
        "SELECT COALESCE(name,'') AS name, COALESCE(value,'') AS value FROM phpyun_property ORDER BY id ASC",
    )
    .fetch_all(pool)
    .await
}
