//! Phase-2 admin gap SQL: photo stats, banners, biz logs, rating services, cron/domain extras.

use super::entity::*;
use super::repo::{delete_in, lim};
use crate::soft_delete::{self, PREDICATE};
use sqlx::{FromRow, MySqlPool, QueryBuilder};

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
        num_all: count_sql(pool, "SELECT COUNT(*) FROM phpyun_msg").await?,
        num_audited: count_sql(pool, "SELECT COUNT(*) FROM phpyun_msg WHERE status=1").await?,
        num_unaudited: count_sql(pool, "SELECT COUNT(*) FROM phpyun_msg WHERE status=0").await?,
        num_failed: Some(count_sql(pool, "SELECT COUNT(*) FROM phpyun_msg WHERE status=2").await?),
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

pub async fn com_cert_stat(pool: &MySqlPool) -> Result<ComCertStat, sqlx::Error> {
    Ok(ComCertStat {
        com_cert_all: count_sql(pool, "SELECT COUNT(*) FROM phpyun_company_cert WHERE type=3")
            .await?,
        com_cert1: count_sql(
            pool,
            "SELECT COUNT(*) FROM phpyun_company_cert WHERE type=3 AND status=0",
        )
        .await?,
        com_cert2: count_sql(
            pool,
            "SELECT COUNT(*) FROM phpyun_company_cert WHERE type=3 AND status=2",
        )
        .await?,
    })
}

pub async fn com_cert_statusbody(pool: &MySqlPool, uid: u64) -> Result<String, sqlx::Error> {
    scalar_str(
        pool,
        "SELECT COALESCE(statusbody,'') FROM phpyun_company_cert WHERE type=3 AND uid=? LIMIT 1",
        uid,
    )
    .await
}

pub async fn part_stat(pool: &MySqlPool) -> Result<PartStat, sqlx::Error> {
    Ok(PartStat {
        part_all_num: count_sql(pool, "SELECT COUNT(*) FROM phpyun_partjob").await?,
        part_status_num1: count_sql(pool, "SELECT COUNT(*) FROM phpyun_partjob WHERE state=0")
            .await?,
        part_status_num2: count_sql(pool, "SELECT COUNT(*) FROM phpyun_partjob WHERE state=3")
            .await?,
        part_status_num3: count_sql(
            pool,
            "SELECT COUNT(*) FROM phpyun_partjob WHERE edate>0 AND edate<UNIX_TIMESTAMP()",
        )
        .await?,
    })
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

/// PHP `resume::delResumeCert` — clear the id-card fields, do not drop the resume row.
pub async fn clear_idcard_certs(pool: &MySqlPool, uids: &[u64]) -> Result<u64, sqlx::Error> {
    delete_in(
        pool,
        "UPDATE phpyun_resume SET idcard_pic='', idcard_status=0, cert_time=0, statusbody='' WHERE uid IN (",
        uids,
    )
    .await
}

pub async fn cert_ids_by_uids(pool: &MySqlPool, uids: &[u64]) -> Result<Vec<u64>, sqlx::Error> {
    if uids.is_empty() {
        return Ok(Vec::new());
    }
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(id AS UNSIGNED) FROM phpyun_company_cert WHERE type=3 AND uid IN (",
    );
    let mut sep = qb.separated(", ");
    for uid in uids {
        sep.push_bind(*uid);
    }
    qb.push(")");
    let rows: Vec<(u64,)> = qb.build_query_as().fetch_all(pool).await?;
    Ok(rows.into_iter().map(|(id,)| id).collect())
}

pub async fn delete_com_certs_by_uids(pool: &MySqlPool, uids: &[u64]) -> Result<u64, sqlx::Error> {
    delete_in(
        pool,
        "DELETE FROM phpyun_company_cert WHERE type=3 AND uid IN (",
        uids,
    )
    .await
}

pub async fn clear_yyzz_status(pool: &MySqlPool, uids: &[u64]) -> Result<u64, sqlx::Error> {
    delete_in(
        pool,
        "UPDATE phpyun_company SET yyzz_status=0 WHERE uid IN (",
        uids,
    )
    .await
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
    phpyun_core::db::ok_default_if_object_missing(qb.build_query_as().fetch_all(pool).await)
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
    let row: Result<(i64,), _> = qb.build_query_as().fetch_one(pool).await;
    let (n,) = phpyun_core::db::ok_default_if_object_missing(row)?;
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

/// Admin-side removal for the biz-log queues above.
///
/// PHP keeps two branches per log: the member/employer one flips `isdel` /
/// `status` / `com_status` so the row stays visible to the other party, and the
/// admin one calls `delete_all`, which is a real `DELETE`. These queues are only
/// reachable from the console, so only the admin branch is ported.
///
/// `phpyun_user_entrust_record` is absent from this schema (and from the
/// official installer dump), so the trust queue degrades to 0 rows instead of
/// erroring — same outcome as PHP, whose `delete_all` just returns false there.
macro_rules! biz_del {
    ($fn:ident, $table:literal) => {
        pub async fn $fn(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
            phpyun_core::db::ok_default_if_object_missing(
                delete_in(
                    pool,
                    concat!("DELETE FROM ", $table, " WHERE id IN ("),
                    ids,
                )
                .await,
            )
        }
    };
}

biz_del!(delete_down, "phpyun_down_resume");
biz_del!(delete_freedown, "phpyun_freedown_resume");
biz_del!(delete_look_resume, "phpyun_look_resume");
biz_del!(delete_talent, "phpyun_talent_pool");
biz_del!(delete_trust, "phpyun_user_entrust_record");
biz_del!(delete_refresh_resume, "phpyun_resume_refresh_log");
biz_del!(delete_userid_msg, "phpyun_userid_msg");
biz_del!(delete_look_job, "phpyun_look_job");
biz_del!(delete_job_tellog, "phpyun_job_tellog");
biz_del!(delete_fav_job, "phpyun_fav_job");

/// How many of `ids` each member owns, so `member_statis.fav_jobnum` can be
/// corrected after the rows go away.
///
/// PHP means to do this but groups by `zid`, a column `phpyun_fav_job` does not
/// have, so its query errors out and the counter is never touched. Grouping by
/// `uid` is the intended behaviour.
pub async fn fav_job_owner_counts(
    pool: &MySqlPool,
    ids: &[u64],
) -> Result<Vec<(u64, i64)>, sqlx::Error> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(COALESCE(uid,0) AS UNSIGNED), COUNT(*) FROM phpyun_fav_job WHERE id IN (",
    );
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(") GROUP BY uid");
    qb.build_query_as::<(u64, i64)>().fetch_all(pool).await
}

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

pub async fn count_rating_service_name(
    pool: &MySqlPool,
    name: &str,
    except_id: u64,
) -> Result<u64, sqlx::Error> {
    let sql = format!(
        "SELECT COUNT(*) FROM phpyun_company_service WHERE name = ? AND id <> ? AND {PREDICATE}"
    );
    let (n,): (i64,) = sqlx::query_as(&sql)
        .bind(name)
        .bind(except_id)
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn rename_rating_service(
    pool: &MySqlPool,
    id: u64,
    name: &str,
) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_company_service SET name = ? WHERE id = ?")
            .bind(name)
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected(),
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
    CAST(COALESCE(three_cityid,0) AS SIGNED) AS three_cityid, \
    CAST(COALESCE(`type`,0) AS SIGNED) AS type, \
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

pub struct PhpCronLogFilter<'a> {
    pub keyword: Option<&'a str>,
    pub time_min: Option<i64>,
    pub time_max: Option<i64>,
    pub sort: &'a str,
    pub dir: &'a str,
}

fn push_cron_log_where(qb: &mut QueryBuilder<sqlx::MySql>, f: &PhpCronLogFilter<'_>) {
    qb.push(
        " FROM phpyun_cron_log l LEFT JOIN phpyun_cron c \
         ON CAST(c.id AS CHAR)=l.cid AND COALESCE(c.deleted,0)=0 WHERE 1=1",
    );
    if let Some(kw) = f.keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND c.name LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    if let Some(t) = f.time_min {
        qb.push(" AND l.ctime >= ");
        qb.push_bind(t);
    }
    if let Some(t) = f.time_max {
        qb.push(" AND l.ctime <= ");
        qb.push_bind(t);
    }
}

fn cron_log_order(sort: &str, dir: &str) -> &'static str {
    let desc = !dir.eq_ignore_ascii_case("asc");
    match sort {
        "ctime" => {
            if desc {
                " ORDER BY l.ctime DESC, l.id DESC"
            } else {
                " ORDER BY l.ctime ASC, l.id ASC"
            }
        }
        _ => {
            if desc {
                " ORDER BY l.id DESC"
            } else {
                " ORDER BY l.id ASC"
            }
        }
    }
}

pub async fn php_list_cron_logs(
    pool: &MySqlPool,
    f: &PhpCronLogFilter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<CronLogRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(l.id AS UNSIGNED) AS id, COALESCE(l.cid,'') AS cid, \
         CAST(COALESCE(l.ctime,0) AS SIGNED) AS ctime, COALESCE(c.name,'') AS name",
    );
    push_cron_log_where(&mut qb, f);
    qb.push(cron_log_order(f.sort, f.dir));
    qb.push(" LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn php_count_cron_logs(
    pool: &MySqlPool,
    f: &PhpCronLogFilter<'_>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT COUNT(*)");
    push_cron_log_where(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn delete_cron_logs(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    delete_in(pool, "DELETE FROM phpyun_cron_log WHERE id IN (", ids).await
}

/// PHP `userinfo::getUidsByWhere` subset: company name / resume name / username.
pub async fn find_display_uids_like(
    pool: &MySqlPool,
    keyword: &str,
) -> Result<Vec<u64>, sqlx::Error> {
    let like = format!("%{keyword}%");
    let rows: Vec<(u64,)> = sqlx::query_as(
        "SELECT CAST(uid AS UNSIGNED) AS uid FROM ( \
            SELECT uid FROM phpyun_company WHERE name LIKE ? LIMIT 50 \
            UNION \
            SELECT uid FROM phpyun_resume WHERE name LIKE ? LIMIT 50 \
            UNION \
            SELECT uid FROM phpyun_member WHERE username LIKE ? LIMIT 50 \
         ) t LIMIT 50",
    )
    .bind(&like)
    .bind(&like)
    .bind(&like)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|(id,)| id).collect())
}

/// Prefer company name, then resume name, then member username.
pub async fn display_names_by_uids(
    pool: &MySqlPool,
    uids: &[u64],
) -> Result<std::collections::HashMap<u64, String>, sqlx::Error> {
    let mut out = std::collections::HashMap::new();
    if uids.is_empty() {
        return Ok(out);
    }
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(m.uid AS UNSIGNED) AS uid, \
         COALESCE(NULLIF(c.name,''), NULLIF(r.name,''), m.username, '') AS name \
         FROM phpyun_member m \
         LEFT JOIN phpyun_company c ON c.uid = m.uid \
         LEFT JOIN phpyun_resume r ON r.uid = m.uid WHERE m.uid IN (",
    );
    let mut sep = qb.separated(", ");
    for id in uids {
        sep.push_bind(*id);
    }
    qb.push(")");
    let rows: Vec<(u64, String)> = qb.build_query_as().fetch_all(pool).await?;
    for (uid, name) in rows {
        if !name.is_empty() {
            out.insert(uid, name);
        }
    }
    Ok(out)
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

pub async fn count_wx_zdkeyword(pool: &MySqlPool, keyword: Option<&str>) -> Result<u64, sqlx::Error> {
    let kw = keyword.unwrap_or("").trim();
    let n: (i64,) = if kw.is_empty() {
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_wxzdkeyword")
            .fetch_one(pool)
            .await?
    } else {
        let like = format!("%{kw}%");
        sqlx::query_as("SELECT COUNT(*) FROM phpyun_wxzdkeyword WHERE keyword LIKE ?")
            .bind(like)
            .fetch_one(pool)
            .await?
    };
    Ok(phpyun_core::numeric::nonnegative_count(n.0))
}

pub async fn list_wx_zdkeyword(
    pool: &MySqlPool,
    keyword: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<WxZdKeywordRow>, sqlx::Error> {
    let kw = keyword.unwrap_or("").trim();
    if kw.is_empty() {
        sqlx::query_as::<_, WxZdKeywordRow>(
            "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(title,'') AS title, \
                    COALESCE(keyword,'') AS keyword, COALESCE(content,'') AS content, \
                    CAST(COALESCE(time,0) AS SIGNED) AS time \
             FROM phpyun_wxzdkeyword ORDER BY time DESC LIMIT ? OFFSET ?",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    } else {
        let like = format!("%{kw}%");
        sqlx::query_as::<_, WxZdKeywordRow>(
            "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(title,'') AS title, \
                    COALESCE(keyword,'') AS keyword, COALESCE(content,'') AS content, \
                    CAST(COALESCE(time,0) AS SIGNED) AS time \
             FROM phpyun_wxzdkeyword WHERE keyword LIKE ? \
             ORDER BY time DESC LIMIT ? OFFSET ?",
        )
        .bind(like)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
    }
}

pub async fn get_wx_zdkeyword(
    pool: &MySqlPool,
    id: u64,
) -> Result<Option<WxZdKeywordRow>, sqlx::Error> {
    sqlx::query_as::<_, WxZdKeywordRow>(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(title,'') AS title, \
                COALESCE(keyword,'') AS keyword, COALESCE(content,'') AS content, \
                CAST(COALESCE(time,0) AS SIGNED) AS time \
         FROM phpyun_wxzdkeyword WHERE id = ? LIMIT 1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn upsert_wx_zdkeyword(
    pool: &MySqlPool,
    id: u64,
    title: &str,
    keyword: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    if id > 0 {
        sqlx::query(
            "UPDATE phpyun_wxzdkeyword SET title = ?, keyword = ?, time = ? WHERE id = ?",
        )
        .bind(title)
        .bind(keyword)
        .bind(now)
        .bind(id)
        .execute(pool)
        .await?;
        Ok(id)
    } else {
        let res = sqlx::query(
            "INSERT INTO phpyun_wxzdkeyword (title, keyword, content, time) VALUES (?, ?, '', ?)",
        )
        .bind(title)
        .bind(keyword)
        .bind(now)
        .execute(pool)
        .await?;
        Ok(res.last_insert_id())
    }
}

pub async fn delete_wx_zdkeyword(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("DELETE FROM phpyun_wxzdkeyword WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn list_wx_zdcon(pool: &MySqlPool, kid: u64) -> Result<Vec<WxZdConRow>, sqlx::Error> {
    sqlx::query_as::<_, WxZdConRow>(
        "SELECT CAST(id AS UNSIGNED) AS id, CAST(COALESCE(kid,0) AS UNSIGNED) AS kid, \
                COALESCE(msgtype,'') AS msgtype, COALESCE(content,'') AS content, \
                CAST(COALESCE(media_id,0) AS SIGNED) AS media_id, \
                CAST(COALESCE(sort,0) AS SIGNED) AS sort, \
                CAST(COALESCE(time,0) AS SIGNED) AS time \
         FROM phpyun_wxzdcon WHERE kid = ? ORDER BY sort DESC, id ASC",
    )
    .bind(kid)
    .fetch_all(pool)
    .await
}

pub async fn insert_wx_zdcon(
    pool: &MySqlPool,
    kid: u64,
    msgtype: &str,
    content: &str,
    media_id: i32,
    sort: i32,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_wxzdcon (kid, msgtype, content, media_id, sort, time) \
         VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(kid)
    .bind(msgtype)
    .bind(content)
    .bind(media_id)
    .bind(sort)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn update_wx_zdcon(
    pool: &MySqlPool,
    id: u64,
    msgtype: &str,
    content: &str,
    media_id: i32,
    sort: i32,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_wxzdcon SET msgtype = ?, content = ?, media_id = ?, sort = ?, time = ? WHERE id = ?",
    )
    .bind(msgtype)
    .bind(content)
    .bind(media_id)
    .bind(sort)
    .bind(now)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn delete_wx_zdcon_ids(pool: &MySqlPool, ids: &[u64], kid: u64) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("DELETE FROM phpyun_wxzdcon WHERE kid = ");
    qb.push_bind(kid);
    qb.push(" AND id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn delete_wx_zdcon_by_kids(pool: &MySqlPool, kids: &[u64]) -> Result<u64, sqlx::Error> {
    if kids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("DELETE FROM phpyun_wxzdcon WHERE kid IN (");
    let mut sep = qb.separated(", ");
    for id in kids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

/// PHP `company_comlog::index_action` 申请记录筛选项。
#[derive(Debug, Default, Clone)]
pub struct UseridJobPhpFilter<'a> {
    pub keyword: Option<&'a str>,
    /// 1 职位名 / 2 公司名 / 3 个人姓名
    pub keyword_type: i32,
    pub browse: Option<i32>,
    pub datetime_from: Option<i64>,
    pub datetime_to: Option<i64>,
    pub job_id: Option<u64>,
    pub com_id: Option<u64>,
    pub user_id: Option<u64>,
}

fn push_userid_job_php<'a>(qb: &mut QueryBuilder<'a, sqlx::MySql>, f: &UseridJobPhpFilter<'a>) {
    if let Some(kw) = f.keyword.map(str::trim).filter(|s| !s.is_empty()) {
        let like = format!("%{kw}%");
        match f.keyword_type {
            2 => {
                qb.push(" AND t.com_name LIKE ");
                qb.push_bind(like);
            }
            3 => {
                qb.push(" AND t.username LIKE ");
                qb.push_bind(like);
            }
            _ => {
                qb.push(" AND t.job_name LIKE ");
                qb.push_bind(like);
            }
        }
    }
    if let Some(b) = f.browse {
        qb.push(" AND t.is_browse = ");
        qb.push_bind(b);
    }
    if let Some(from) = f.datetime_from {
        qb.push(" AND t.datetime >= ");
        qb.push_bind(from);
    }
    if let Some(to) = f.datetime_to {
        qb.push(" AND t.datetime <= ");
        qb.push_bind(to);
    }
    if let Some(id) = f.job_id {
        qb.push(" AND t.jobid = ");
        qb.push_bind(id);
    }
    if let Some(id) = f.com_id {
        qb.push(" AND t.comid = ");
        qb.push_bind(id);
    }
    if let Some(id) = f.user_id {
        qb.push(" AND t.uid = ");
        qb.push_bind(id);
    }
}

pub async fn list_userid_job_php(
    pool: &MySqlPool,
    f: &UseridJobPhpFilter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<BizLogRow>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new(format!("SELECT {BIZ_SELECT} FROM ({USERID_JOB_INNER}) t WHERE 1=1"));
    push_userid_job_php(&mut qb, f);
    qb.push(" ORDER BY t.id DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_userid_job_php(
    pool: &MySqlPool,
    f: &UseridJobPhpFilter<'_>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new(format!("SELECT COUNT(*) FROM ({USERID_JOB_INNER}) t WHERE 1=1"));
    push_userid_job_php(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

const MEMBER_APPLY_SELECT: &str = "CAST(j.id AS UNSIGNED) AS id, \
    CAST(COALESCE(j.uid,0) AS UNSIGNED) AS uid, \
    CAST(COALESCE(j.com_id,0) AS UNSIGNED) AS com_id, \
    CAST(COALESCE(j.job_id,0) AS UNSIGNED) AS job_id, \
    COALESCE(j.com_name,'') AS com_name, COALESCE(j.job_name,'') AS job_name, \
    CAST(COALESCE(j.datetime,0) AS SIGNED) AS datetime, \
    CAST(COALESCE(j.is_browse,0) AS SIGNED) AS is_browse, \
    CAST(COALESCE(j.isdel,9) AS SIGNED) AS isdel";

/// PHP `users_member::jobSqLog_action` — one seeker's job applications.
pub async fn list_member_applies(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<MemberApplyRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    sqlx::query_as(&format!(
        "SELECT {MEMBER_APPLY_SELECT} FROM phpyun_userid_job j \
         WHERE j.uid = ? ORDER BY j.id DESC LIMIT ? OFFSET ?"
    ))
    .bind(uid)
    .bind(l)
    .bind(o)
    .fetch_all(pool)
    .await
}

pub async fn count_member_applies(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_userid_job WHERE uid = ?")
        .bind(uid)
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

const MEMBER_INVITE_SELECT: &str = "CAST(m.id AS UNSIGNED) AS id, \
    CAST(COALESCE(m.uid,0) AS UNSIGNED) AS uid, \
    CAST(COALESCE(m.fid,0) AS UNSIGNED) AS fid, \
    CAST(COALESCE(m.jobid,0) AS UNSIGNED) AS jobid, \
    COALESCE(m.fname,'') AS fname, COALESCE(m.jobname,'') AS jobname, \
    COALESCE(m.title,'') AS title, COALESCE(m.content,'') AS content, \
    CAST(COALESCE(m.datetime,0) AS SIGNED) AS datetime, \
    CAST(COALESCE(m.is_browse,0) AS SIGNED) AS is_browse, \
    CAST(COALESCE(m.isdel,9) AS SIGNED) AS isdel";

/// PHP `users_member::yqmsLog_action` — interview invitations sent to one seeker.
pub async fn list_member_invites(
    pool: &MySqlPool,
    uid: u64,
    offset: u64,
    limit: u64,
) -> Result<Vec<MemberInviteRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    sqlx::query_as(&format!(
        "SELECT {MEMBER_INVITE_SELECT} FROM phpyun_userid_msg m \
         WHERE m.uid = ? ORDER BY m.id DESC LIMIT ? OFFSET ?"
    ))
    .bind(uid)
    .bind(l)
    .bind(o)
    .fetch_all(pool)
    .await
}

pub async fn count_member_invites(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_userid_msg WHERE uid = ?")
        .bind(uid)
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

/// PHP `resume.model::statusCert` — idcard review for one or more seekers.
/// PHP mirrors the status onto `resume_expect` so the seeker's job-hunting
/// cards stop showing an unverified badge.
pub async fn set_idcard_review_many(
    pool: &MySqlPool,
    uids: &[u64],
    status: i32,
    body: &str,
) -> Result<u64, sqlx::Error> {
    if uids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("UPDATE phpyun_resume SET idcard_status = ");
    qb.push_bind(status);
    qb.push(", statusbody = ");
    qb.push_bind(body.to_string());
    qb.push(" WHERE uid IN (");
    let mut sep = qb.separated(", ");
    for uid in uids {
        sep.push_bind(*uid);
    }
    qb.push(")");
    let n = qb.build().execute(pool).await?.rows_affected();

    let mut qb = QueryBuilder::new("UPDATE phpyun_resume_expect SET idcard_status = ");
    qb.push_bind(status);
    qb.push(" WHERE uid IN (");
    let mut sep = qb.separated(", ");
    for uid in uids {
        sep.push_bind(*uid);
    }
    qb.push(")");
    qb.build().execute(pool).await?;
    Ok(n)
}

/// Which verification flag PHP `users_member::batchfirm` is toggling.
#[derive(Debug, Clone, Copy)]
pub enum CertFlag {
    Email,
    Mobile,
    Idcard,
}

/// PHP `users_member::batchfirm` — flip one verification flag for many seekers.
/// `idcard_status` only lives on `resume`, the other two are mirrored on
/// `member` so login-side checks agree with the resume.
pub async fn set_cert_flag_bulk(
    pool: &MySqlPool,
    uids: &[u64],
    flag: CertFlag,
    status: i32,
) -> Result<u64, sqlx::Error> {
    if uids.is_empty() {
        return Ok(0);
    }
    let (col, also_member) = match flag {
        CertFlag::Email => ("email_status", true),
        CertFlag::Mobile => ("moblie_status", true),
        CertFlag::Idcard => ("idcard_status", false),
    };
    let mut affected = 0;
    let mut tables: Vec<&str> = vec!["resume"];
    if also_member {
        tables.push("member");
    }
    for t in tables {
        let mut qb = QueryBuilder::new(format!("UPDATE phpyun_{t} SET {col} = "));
        qb.push_bind(status);
        qb.push(" WHERE uid IN (");
        let mut sep = qb.separated(", ");
        for uid in uids {
            sep.push_bind(*uid);
        }
        qb.push(")");
        affected += qb.build().execute(pool).await?.rows_affected();
    }
    Ok(affected)
}

/// `resume.email` / `resume.email_status` before an admin rebinds them, so the
/// caller can tell "already verified, nothing to do" from a real change.
pub async fn resume_email_state(
    pool: &MySqlPool,
    uid: u64,
) -> Result<Option<(String, i32)>, sqlx::Error> {
    sqlx::query_as(
        "SELECT COALESCE(email,''), CAST(COALESCE(email_status,0) AS SIGNED) \
         FROM phpyun_resume WHERE uid = ?",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await
}

/// `resume.telphone` / `resume.moblie_status`, counterpart of [`resume_email_state`].
pub async fn resume_mobile_state(
    pool: &MySqlPool,
    uid: u64,
) -> Result<Option<(String, i32)>, sqlx::Error> {
    sqlx::query_as(
        "SELECT COALESCE(telphone,''), CAST(COALESCE(moblie_status,0) AS SIGNED) \
         FROM phpyun_resume WHERE uid = ?",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await
}

/// PHP `users_member::emailstatus` — bind the email to this seeker, then release
/// it from every other account still claiming it (a verified address must be
/// unique). PHP also sweeps `lt_info` / `px_train`; those belong to modules the
/// Rust API does not serve, so they are left alone.
pub async fn rebind_member_email(
    pool: &MySqlPool,
    uid: u64,
    email: &str,
    status: i32,
) -> Result<u64, sqlx::Error> {
    let n = sqlx::query("UPDATE phpyun_resume SET email = ?, email_status = ? WHERE uid = ?")
        .bind(email)
        .bind(status)
        .bind(uid)
        .execute(pool)
        .await?
        .rows_affected();
    if n == 0 {
        return Ok(0);
    }
    sqlx::query("UPDATE phpyun_member SET email = ?, email_status = ? WHERE uid = ?")
        .bind(email)
        .bind(status)
        .bind(uid)
        .execute(pool)
        .await?;
    for sql in [
        "UPDATE phpyun_member SET email = '', email_status = 0 WHERE uid <> ? AND email = ?",
        "UPDATE phpyun_resume SET email = '', email_status = 0 WHERE uid <> ? AND email = ?",
        "UPDATE phpyun_company SET linkmail = '', email_status = 0 WHERE uid <> ? AND linkmail = ?",
    ] {
        sqlx::query(sql)
            .bind(uid)
            .bind(email)
            .execute(pool)
            .await?;
    }
    Ok(n)
}

/// PHP `users_member::mobliestatus` — same as [`rebind_member_email`] for the
/// phone number. When the account's username was its old phone number PHP
/// renames the username too, and any account already squatting the new number
/// gets suffixed `_s`. Returns false when the seeker has no resume row.
pub async fn rebind_member_mobile(
    pool: &MySqlPool,
    uid: u64,
    phone: &str,
    status: i32,
) -> Result<bool, sqlx::Error> {
    let n = sqlx::query("UPDATE phpyun_resume SET telphone = ?, moblie_status = ? WHERE uid = ?")
        .bind(phone)
        .bind(status)
        .bind(uid)
        .execute(pool)
        .await?
        .rows_affected();
    if n == 0 {
        return Ok(false);
    }

    let displaced: Option<(u64,)> =
        sqlx::query_as("SELECT CAST(uid AS UNSIGNED) FROM phpyun_member WHERE username = ? LIMIT 1")
            .bind(phone)
            .fetch_optional(pool)
            .await?;
    if let Some(other) = displaced.map(|(u,)| u).filter(|u| *u != uid) {
        sqlx::query("UPDATE phpyun_member SET username = ? WHERE uid = ?")
            .bind(format!("{phone}_s"))
            .bind(other)
            .execute(pool)
            .await?;
    }

    let self_row: Option<(String, String)> = sqlx::query_as(
        "SELECT COALESCE(username,''), COALESCE(moblie,'') FROM phpyun_member WHERE uid = ?",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    let renamed_self = self_row
        .as_ref()
        .is_some_and(|(username, moblie)| !username.is_empty() && username == moblie);
    if renamed_self {
        sqlx::query("UPDATE phpyun_member SET moblie = ?, moblie_status = ?, username = ? WHERE uid = ?")
            .bind(phone)
            .bind(status)
            .bind(phone)
            .bind(uid)
            .execute(pool)
            .await?;
    } else {
        sqlx::query("UPDATE phpyun_member SET moblie = ?, moblie_status = ? WHERE uid = ?")
            .bind(phone)
            .bind(status)
            .bind(uid)
            .execute(pool)
            .await?;
    }

    for sql in [
        "UPDATE phpyun_member SET moblie = '', moblie_status = 0 WHERE uid <> ? AND moblie = ?",
        "UPDATE phpyun_resume SET telphone = '', moblie_status = 0 WHERE uid <> ? AND telphone = ?",
        "UPDATE phpyun_company SET linktel = '', moblie_status = 0 WHERE uid <> ? AND linktel = ?",
    ] {
        sqlx::query(sql)
            .bind(uid)
            .bind(phone)
            .execute(pool)
            .await?;
    }
    Ok(true)
}

// ---------- 预约刷新 (phpyun_reserve_refresh) ----------

/// PHP `company_job::reserveJob_action` scope: only jobs that are live,
/// approved and still open can sit in the refresh queue.
const RESERVE_JOB_SCOPE: &str =
    " FROM phpyun_company_job j LEFT JOIN phpyun_reserve_refresh r ON r.job_id = j.id \
      WHERE j.is_reserve = 1 AND j.state = 1 AND j.status = 0 AND j.r_status = 1";

pub struct ReserveJobFilter<'a> {
    /// PHP `type`: 1 filters `com_name`, 2 filters the job `name`.
    pub keyword: Option<&'a str>,
    pub keyword_type: i32,
    pub uid: Option<u64>,
    pub order_col: &'a str,
    pub order_desc: bool,
}

fn push_reserve_filters<'a>(
    qb: &mut QueryBuilder<'a, sqlx::MySql>,
    f: &ReserveJobFilter<'a>,
) {
    if let Some(kw) = f.keyword.map(str::trim).filter(|s| !s.is_empty()) {
        let col = if f.keyword_type == 1 {
            "j.com_name"
        } else {
            "j.name"
        };
        qb.push(format!(" AND {col} LIKE "));
        qb.push_bind(format!("%{kw}%"));
    }
    if let Some(uid) = f.uid.filter(|v| *v > 0) {
        qb.push(" AND j.uid = ");
        qb.push_bind(uid);
    }
}

pub async fn list_reserve_jobs(
    pool: &MySqlPool,
    f: &ReserveJobFilter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<ReserveJobRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(j.id AS UNSIGNED) AS id, CAST(COALESCE(j.uid,0) AS UNSIGNED) AS uid, \
         COALESCE(j.name,'') AS name, COALESCE(j.com_name,'') AS com_name, \
         CAST(COALESCE(r.status,0) AS SIGNED) AS reserve_status, \
         CAST(COALESCE(r.`interval`,0) AS SIGNED) AS reserve_interval, \
         CAST(COALESCE(r.start_time,0) AS SIGNED) AS start_time, \
         CAST(COALESCE(r.end_time,0) AS SIGNED) AS end_time, \
         COALESCE(r.s_time,'') AS s_time, COALESCE(r.e_time,'') AS e_time",
    );
    qb.push(RESERVE_JOB_SCOPE);
    push_reserve_filters(&mut qb, f);
    // Sorting is a whitelist: the Element table only offers `id`, and PHP falls
    // back to `lastupdate desc`.
    let col = if f.order_col == "id" {
        "j.id"
    } else {
        "j.lastupdate"
    };
    qb.push(format!(
        " ORDER BY {col} {}",
        if f.order_desc { "DESC" } else { "ASC" }
    ));
    qb.push(" LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_reserve_jobs(
    pool: &MySqlPool,
    f: &ReserveJobFilter<'_>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT COUNT(*)");
    qb.push(RESERVE_JOB_SCOPE);
    push_reserve_filters(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

/// PHP member-side `job.model::reserveInfo` — the schedule the settings dialog
/// pre-fills from. Admin looks up by job only, without binding to a member uid.
pub async fn find_reserve_schedule(
    pool: &MySqlPool,
    job_id: u64,
) -> Result<Option<ReserveScheduleRow>, sqlx::Error> {
    sqlx::query_as(
        "SELECT CAST(COALESCE(status,0) AS SIGNED) AS status, \
         CAST(COALESCE(`interval`,0) AS SIGNED) AS `interval`, \
         COALESCE(s_time,'') AS s_time, COALESCE(e_time,'') AS e_time, \
         CAST(COALESCE(end_time,0) AS SIGNED) AS end_time \
         FROM phpyun_reserve_refresh WHERE job_id = ? ORDER BY id DESC LIMIT 1",
    )
    .bind(job_id)
    .fetch_optional(pool)
    .await
}

/// PHP `job.model::closeReserve` with explicit ids — drop the jobs out of the
/// queue and mark their schedule closed.
pub async fn close_reserve_jobs(pool: &MySqlPool, job_ids: &[u64]) -> Result<u64, sqlx::Error> {
    if job_ids.is_empty() {
        return Ok(0);
    }
    let mut affected = 0;
    for sql in [
        "UPDATE phpyun_company_job SET is_reserve = 0 WHERE id IN (",
        "UPDATE phpyun_reserve_refresh SET status = 2 WHERE job_id IN (",
    ] {
        let mut qb = QueryBuilder::new(sql);
        let mut sep = qb.separated(", ");
        for id in job_ids {
            sep.push_bind(*id);
        }
        qb.push(")");
        affected += qb.build().execute(pool).await?.rows_affected();
    }
    Ok(affected)
}

/// PHP `job.model::closeReserve` with `auto=1` — the housekeeping sweep the
/// refresh page fires on mount: anything still queued but no longer live gets
/// dropped.
pub async fn close_stale_reserve_jobs(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let ids: Vec<(u64,)> = sqlx::query_as(
        "SELECT CAST(id AS UNSIGNED) FROM phpyun_company_job \
         WHERE is_reserve = 1 AND (state <> 1 OR status = 1 OR r_status <> 1)",
    )
    .fetch_all(pool)
    .await?;
    let ids: Vec<u64> = ids.into_iter().map(|(v,)| v).collect();
    close_reserve_jobs(pool, &ids).await
}

/// Of `job_ids`, the ones that belong to `uid` and are still eligible for the
/// refresh queue (PHP `reserveUpJob` re-checks this before writing).
pub async fn eligible_reserve_job_ids(
    pool: &MySqlPool,
    uid: u64,
    job_ids: &[u64],
) -> Result<Vec<u64>, sqlx::Error> {
    if job_ids.is_empty() {
        return Ok(Vec::new());
    }
    let mut qb = QueryBuilder::new(
        "SELECT CAST(id AS UNSIGNED) FROM phpyun_company_job \
         WHERE state = 1 AND r_status = 1 AND status = 0 AND uid = ",
    );
    qb.push_bind(uid);
    qb.push(" AND id IN (");
    let mut sep = qb.separated(", ");
    for id in job_ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    let rows: Vec<(u64,)> = qb.build_query_as().fetch_all(pool).await?;
    Ok(rows.into_iter().map(|(v,)| v).collect())
}

/// Which of `job_ids` already have a `reserve_refresh` row under `uid`, so the
/// caller knows to UPDATE rather than INSERT.
pub async fn existing_reserve_job_ids(
    pool: &MySqlPool,
    uid: u64,
    job_ids: &[u64],
) -> Result<Vec<u64>, sqlx::Error> {
    if job_ids.is_empty() {
        return Ok(Vec::new());
    }
    let mut qb = QueryBuilder::new(
        "SELECT CAST(job_id AS UNSIGNED) FROM phpyun_reserve_refresh WHERE uid = ",
    );
    qb.push_bind(uid);
    qb.push(" AND job_id IN (");
    let mut sep = qb.separated(", ");
    for id in job_ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    let rows: Vec<(u64,)> = qb.build_query_as().fetch_all(pool).await?;
    Ok(rows.into_iter().map(|(v,)| v).collect())
}

/// The schedule an admin just submitted, shared by the INSERT and UPDATE paths.
pub struct ReserveScheduleIn<'a> {
    pub status: i32,
    pub interval: i32,
    pub start_time: i64,
    pub end_time: i64,
    pub next_time: i64,
    pub s_time: &'a str,
    pub e_time: &'a str,
}

pub async fn insert_reserve_schedules(
    pool: &MySqlPool,
    uid: u64,
    job_ids: &[u64],
    v: &ReserveScheduleIn<'_>,
) -> Result<u64, sqlx::Error> {
    if job_ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new(
        "INSERT INTO phpyun_reserve_refresh \
         (job_id, uid, status, `interval`, start_time, end_time, last_time, next_time, s_time, e_time) ",
    );
    qb.push_values(job_ids, |mut b, id| {
        b.push_bind(*id)
            .push_bind(uid)
            .push_bind(v.status)
            .push_bind(v.interval)
            .push_bind(v.start_time)
            .push_bind(v.end_time)
            .push_bind(0_i64)
            .push_bind(v.next_time)
            .push_bind(v.s_time)
            .push_bind(v.e_time);
    });
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn update_reserve_schedules(
    pool: &MySqlPool,
    uid: u64,
    job_ids: &[u64],
    v: &ReserveScheduleIn<'_>,
) -> Result<u64, sqlx::Error> {
    if job_ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("UPDATE phpyun_reserve_refresh SET status = ");
    qb.push_bind(v.status);
    qb.push(", `interval` = ");
    qb.push_bind(v.interval);
    qb.push(", start_time = ");
    qb.push_bind(v.start_time);
    qb.push(", end_time = ");
    qb.push_bind(v.end_time);
    qb.push(", last_time = 0, next_time = ");
    qb.push_bind(v.next_time);
    qb.push(", s_time = ");
    qb.push_bind(v.s_time);
    qb.push(", e_time = ");
    qb.push_bind(v.e_time);
    qb.push(" WHERE uid = ");
    qb.push_bind(uid);
    qb.push(" AND job_id IN (");
    let mut sep = qb.separated(", ");
    for id in job_ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn set_jobs_is_reserve(
    pool: &MySqlPool,
    uid: u64,
    job_ids: &[u64],
    flag: i32,
) -> Result<u64, sqlx::Error> {
    if job_ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("UPDATE phpyun_company_job SET is_reserve = ");
    qb.push_bind(flag);
    qb.push(" WHERE uid = ");
    qb.push_bind(uid);
    qb.push(" AND id IN (");
    let mut sep = qb.separated(", ");
    for id in job_ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

/// The refresh budget PHP `reserveUpJob` checks: paid refreshes left on the
/// company plus today's unused free allowance from its rating tier.
pub async fn reserve_refresh_budget(pool: &MySqlPool, uid: u64) -> Result<i64, sqlx::Error> {
    let statis: Option<(i64, i64)> = sqlx::query_as(
        "SELECT CAST(COALESCE(breakjob_num,0) AS SIGNED), CAST(COALESCE(rating,0) AS SIGNED) \
         FROM phpyun_company_statis WHERE uid = ?",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    let (paid, rating) = statis.unwrap_or((0, 0));

    let free_cap: Option<(i64,)> = sqlx::query_as(
        "SELECT CAST(COALESCE(freerefresh_num,0) AS SIGNED) FROM phpyun_company_rating WHERE id = ?",
    )
    .bind(rating)
    .fetch_optional(pool)
    .await?;
    let free_cap = free_cap.map_or(0, |(v,)| v);
    if free_cap <= 0 {
        return Ok(paid);
    }
    let (used,): (i64,) = sqlx::query_as(
        "SELECT CAST(COALESCE(SUM(free_num),0) AS SIGNED) FROM phpyun_job_refresh_log \
         WHERE uid = ? AND free = 1 AND r_time >= ?",
    )
    .bind(uid)
    .bind(phpyun_core::clock::start_of_today())
    .fetch_one(pool)
    .await?;
    Ok(paid + (free_cap - used).max(0))
}

/// One `updDid` call from PHP `site.model`: which table to touch, which column
/// holds the account id, and whether the table is shared between seeker and
/// company rows (`usertype`).
struct DidTarget {
    table: &'static str,
    col: &'static str,
    usertype: Option<i32>,
}

/// Table/column pairs are compile-time constants, never request data, so the
/// interpolation below cannot be reached by a caller.
async fn apply_did(
    pool: &MySqlPool,
    targets: &[DidTarget],
    uids: &[u64],
    did: i32,
) -> Result<u64, sqlx::Error> {
    if uids.is_empty() {
        return Ok(0);
    }
    let mut affected = 0;
    for t in targets {
        let mut qb = QueryBuilder::new(format!("UPDATE phpyun_{} SET did = ", t.table));
        qb.push_bind(did);
        qb.push(format!(" WHERE {} IN (", t.col));
        let mut sep = qb.separated(", ");
        for uid in uids {
            sep.push_bind(*uid);
        }
        qb.push(")");
        if let Some(ut) = t.usertype {
            qb.push(" AND usertype = ");
            qb.push_bind(ut);
        }
        affected += qb.build().execute(pool).await?.rows_affected();
    }
    Ok(affected)
}

const MEMBER_DID_TARGETS: &[DidTarget] = &[
    DidTarget { table: "report", col: "p_uid", usertype: None },
    DidTarget { table: "company_pay", col: "com_id", usertype: None },
    DidTarget { table: "company_cert", col: "uid", usertype: None },
    DidTarget { table: "company_msg", col: "uid", usertype: None },
    DidTarget { table: "company_order", col: "uid", usertype: None },
    DidTarget { table: "look_job", col: "uid", usertype: None },
    DidTarget { table: "member", col: "uid", usertype: None },
    DidTarget { table: "member_statis", col: "uid", usertype: None },
    DidTarget { table: "resume", col: "uid", usertype: None },
    DidTarget { table: "resume_expect", col: "uid", usertype: None },
    DidTarget { table: "user_entrust", col: "uid", usertype: None },
    DidTarget { table: "userid_job", col: "uid", usertype: None },
];

const COMPANY_DID_TARGETS: &[DidTarget] = &[
    DidTarget { table: "report", col: "p_uid", usertype: Some(2) },
    DidTarget { table: "userid_msg", col: "fid", usertype: None },
    DidTarget { table: "company_pay", col: "com_id", usertype: Some(2) },
    DidTarget { table: "look_resume", col: "com_id", usertype: Some(2) },
    DidTarget { table: "down_resume", col: "comid", usertype: Some(2) },
    DidTarget { table: "ad_order", col: "comid", usertype: None },
    DidTarget { table: "member", col: "uid", usertype: None },
    DidTarget { table: "company", col: "uid", usertype: None },
    DidTarget { table: "company_statis", col: "uid", usertype: None },
    DidTarget { table: "company_job", col: "uid", usertype: None },
    DidTarget { table: "company_cert", col: "uid", usertype: None },
    DidTarget { table: "company_news", col: "uid", usertype: None },
    DidTarget { table: "company_order", col: "uid", usertype: None },
    DidTarget { table: "company_product", col: "uid", usertype: None },
    DidTarget { table: "partjob", col: "uid", usertype: None },
    DidTarget { table: "hotjob", col: "uid", usertype: None },
];

/// PHP `users_member::checksitedid_action` — move seeker accounts to a sub-site.
pub async fn set_member_did(pool: &MySqlPool, uids: &[u64], did: i32) -> Result<u64, sqlx::Error> {
    apply_did(pool, MEMBER_DID_TARGETS, uids, did).await
}

/// PHP `company::checksitedid_action` — move company accounts to a sub-site.
pub async fn set_company_did(pool: &MySqlPool, uids: &[u64], did: i32) -> Result<u64, sqlx::Error> {
    apply_did(pool, COMPANY_DID_TARGETS, uids, did).await
}

pub async fn delete_userid_job_ids(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("DELETE FROM phpyun_userid_job WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

// ---------- 企业管理长尾（PHP `admin/model/user/company.class.php`） ----------

/// PHP `company::bindPackage_action` — the extra `company_rating` ids a company
/// is entitled to, stored as a CSV in `phpyun_company.package`.
pub async fn set_company_package(
    pool: &MySqlPool,
    uid: u64,
    package: &str,
) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_company SET package = ? WHERE uid = ?")
            .bind(package)
            .bind(uid)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

/// PHP `company.model::setLogoByAdmin` — the logo is denormalised onto every
/// job row, and an admin-set logo skips re-review (`logo_status = 0`).
pub async fn set_company_logo_by_admin(
    pool: &MySqlPool,
    uid: u64,
    logo: &str,
) -> Result<u64, sqlx::Error> {
    let affected = sqlx::query("UPDATE phpyun_company SET logo = ?, logo_status = 0 WHERE uid = ?")
        .bind(logo)
        .bind(uid)
        .execute(pool)
        .await?
        .rows_affected();
    if affected > 0 {
        sqlx::query("UPDATE phpyun_company_job SET com_logo = ? WHERE uid = ?")
            .bind(logo)
            .bind(uid)
            .execute(pool)
            .await?;
    }
    Ok(affected)
}

/// PHP `company.model::setComGw` — assign a CRM advisor to one or many
/// companies. PHP also passes `crm_source = 5`, but `phpyun_company` has no such
/// column and `update_once` filters unknown keys, so that write is a no-op.
pub async fn set_company_advisor(
    pool: &MySqlPool,
    uids: &[u64],
    crm_uid: u64,
    now: i64,
) -> Result<u64, sqlx::Error> {
    if uids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("UPDATE phpyun_company SET crm_uid = ");
    qb.push_bind(crm_uid);
    qb.push(", crm_time = ");
    qb.push_bind(now);
    qb.push(" WHERE uid IN (");
    let mut sep = qb.separated(", ");
    for uid in uids {
        sep.push_bind(*uid);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

/// `phpyun_admin_user.name` of an advisor, used to reject unknown ids the way
/// PHP `admin.model::getAdminUser` does.
pub async fn admin_user_name(pool: &MySqlPool, uid: u64) -> Result<Option<String>, sqlx::Error> {
    let row: Option<(String,)> =
        sqlx::query_as("SELECT COALESCE(`name`, '') FROM phpyun_admin_user WHERE uid = ? LIMIT 1")
            .bind(uid)
            .fetch_optional(pool)
            .await?;
    Ok(row.map(|r| r.0))
}

/// PHP `company::statisDetail_action` filter: one company, optional ledger type.
#[derive(Debug, Clone, Default)]
pub struct StatisDetailFilter {
    pub uid: u64,
    pub kind: i32,
}

fn statis_detail_where(qb: &mut QueryBuilder<'_, sqlx::MySql>, f: &StatisDetailFilter) {
    qb.push(" WHERE uid = ");
    qb.push_bind(f.uid);
    if f.kind > 0 {
        qb.push(" AND `type` = ");
        qb.push_bind(f.kind);
    }
}

pub async fn count_company_statis_details(
    pool: &MySqlPool,
    f: &StatisDetailFilter,
) -> Result<u64, sqlx::Error> {
    let mut qb = QueryBuilder::new("SELECT COUNT(*) FROM phpyun_company_statis_detail");
    statis_detail_where(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn list_company_statis_details(
    pool: &MySqlPool,
    f: &StatisDetailFilter,
    limit: u64,
    offset: u64,
) -> Result<Vec<CompanyStatisDetailRow>, sqlx::Error> {
    let (limit, offset) = lim(limit, offset)?;
    // `id` / `uid` are signed INT in MySQL; sqlx refuses to decode those into
    // `u64` without an explicit cast.
    let mut qb = QueryBuilder::new(
        "SELECT CAST(id AS UNSIGNED) AS id, CAST(uid AS UNSIGNED) AS uid, `type`, num, \
         COALESCE(detail, '') AS detail, `time`, \
         COALESCE(uri, '') AS uri, COALESCE(ip, '') AS ip \
         FROM phpyun_company_statis_detail",
    );
    statis_detail_where(&mut qb, f);
    // PHP `orderby = 'id'` with no direction, which its query builder renders ASC.
    qb.push(" ORDER BY id ASC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn delete_company_statis_details(
    pool: &MySqlPool,
    ids: &[u64],
) -> Result<u64, sqlx::Error> {
    delete_in(
        pool,
        "DELETE FROM phpyun_company_statis_detail WHERE id IN (",
        ids,
    )
    .await
}

/// PHP `company::mcomtpl_action` — enabled skins that are either global
/// (`service_uid = 0`) or explicitly granted to this company.
pub async fn list_company_tpls_for(
    pool: &MySqlPool,
    uid: u64,
) -> Result<Vec<CompanyTplRow>, sqlx::Error> {
    sqlx::query_as(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(`name`, '') AS `name`, \
         COALESCE(url, '') AS url, COALESCE(pic, '') AS pic, \
         COALESCE(price, '') AS price, CAST(COALESCE(status, 0) AS SIGNED) AS status \
         FROM phpyun_company_tpl \
         WHERE status = 1 AND (service_uid = '0' OR FIND_IN_SET(?, service_uid)) \
         ORDER BY id DESC",
    )
    .bind(uid)
    .fetch_all(pool)
    .await
}

/// `phpyun_company_tpl.url` of one skin, for `company::msettpl_action`.
pub async fn company_tpl_url(pool: &MySqlPool, id: u64) -> Result<Option<String>, sqlx::Error> {
    let row: Option<(String,)> =
        sqlx::query_as("SELECT COALESCE(url, '') FROM phpyun_company_tpl WHERE id = ? LIMIT 1")
            .bind(id)
            .fetch_optional(pool)
            .await?;
    Ok(row.map(|r| r.0))
}

/// PHP `wxpubtemp.model::addTwTask` `type = 2` reads these three columns before
/// fanning out one task row per company.
pub async fn tuiwen_companies(
    pool: &MySqlPool,
    uids: &[u64],
) -> Result<Vec<TuiWenCompanyRow>, sqlx::Error> {
    if uids.is_empty() {
        return Ok(Vec::new());
    }
    let mut qb = QueryBuilder::new(
        "SELECT CAST(uid AS UNSIGNED) AS uid, COALESCE(`name`, '') AS `name`, \
         COALESCE(lastupdate, '') AS lastupdate \
         FROM phpyun_company WHERE uid IN (",
    );
    let mut sep = qb.separated(", ");
    for uid in uids {
        sep.push_bind(*uid);
    }
    qb.push(")");
    qb.build_query_as().fetch_all(pool).await
}

/// One `phpyun_wxpub_twtask` row to queue. `jobid` / `jobname` stay empty for
/// company-level tasks (`type = 2`).
#[derive(Debug, Clone)]
pub struct TuiWenTaskIn {
    pub cuid: u64,
    pub comname: String,
    pub jobsdate: i64,
    pub auid: u64,
    pub content: String,
    pub urgent: i32,
    pub wcmoments: i32,
    pub gzh: i32,
    pub jobid: u64,
    pub jobname: String,
    pub kind: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct TuiWenJobRow {
    pub id: u64,
    pub uid: u64,
    pub name: String,
    pub com_name: String,
    pub sdate: i64,
}

pub async fn tuiwen_jobs(
    pool: &MySqlPool,
    ids: &[u64],
) -> Result<Vec<TuiWenJobRow>, sqlx::Error> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let mut qb = QueryBuilder::new(
        "SELECT CAST(id AS UNSIGNED) AS id, CAST(uid AS UNSIGNED) AS uid, \
         COALESCE(`name`,'') AS `name`, COALESCE(com_name,'') AS com_name, \
         CAST(COALESCE(sdate,0) AS SIGNED) AS sdate \
         FROM phpyun_company_job WHERE id IN (",
    );
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    qb.build_query_as().fetch_all(pool).await
}

// ---------- 企业暂停 / 恢复配额快照 ----------
//
// PHP `company.model::setComStaticSub` / `restoreComStatic`. Note that PHP's
// `suspend_action` calls `jugdeSuspend()`, a method that is defined nowhere in
// the codebase, so the legacy suspend path fatals before it ever reaches the
// snapshot — which is why `phpyun_company_statis_sub` is empty. There is no
// working legacy behaviour to mirror here, only the intended design.

/// Counters + rating a company holds right before being suspended.
pub async fn company_quota(
    pool: &MySqlPool,
    uid: u64,
) -> Result<Option<CompanyQuotaRow>, sqlx::Error> {
    sqlx::query_as(
        "SELECT CAST(COALESCE(rating, 0) AS SIGNED) AS rating, \
         COALESCE(rating_name, '') AS rating_name, \
         CAST(COALESCE(rating_type, 0) AS SIGNED) AS rating_type, \
         CAST(COALESCE(job_num, 0) AS SIGNED) AS job_num, \
         CAST(COALESCE(breakjob_num, 0) AS SIGNED) AS breakjob_num, \
         CAST(COALESCE(down_resume, 0) AS SIGNED) AS down_resume, \
         CAST(COALESCE(invite_resume, 0) AS SIGNED) AS invite_resume, \
         CAST(COALESCE(zph_num, 0) AS SIGNED) AS zph_num, \
         CAST(COALESCE(top_num, 0) AS SIGNED) AS top_num, \
         CAST(COALESCE(urgent_num, 0) AS SIGNED) AS urgent_num, \
         CAST(COALESCE(rec_num, 0) AS SIGNED) AS rec_num, \
         CAST(COALESCE(vip_stime, 0) AS SIGNED) AS vip_stime, \
         CAST(COALESCE(vip_etime, 0) AS SIGNED) AS vip_etime, \
         CAST(COALESCE(max_time, 0) AS SIGNED) AS max_time \
         FROM phpyun_company_statis WHERE uid = ? LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await
}

/// PHP `setComStaticSub` — snapshot the quota block, then zero it out.
///
/// PHP's zero list also names `chat_num` and `spview_num`, but neither column
/// exists on `phpyun_company_statis`; `update_once` filters unknown keys, so
/// only these nine are really cleared.
pub async fn snapshot_and_clear_company_quota(
    pool: &MySqlPool,
    uid: u64,
    zt_time: i64,
) -> Result<bool, sqlx::Error> {
    let Some(q) = company_quota(pool, uid).await? else {
        return Ok(false);
    };
    sqlx::query(
        "INSERT INTO phpyun_company_statis_sub \
         (uid, rating, rating_name, rating_type, job_num, breakjob_num, down_resume, \
          invite_resume, zph_num, top_num, urgent_num, rec_num, vip_stime, vip_etime, zt_time) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(uid)
    .bind(q.rating)
    .bind(&q.rating_name)
    .bind(q.rating_type)
    .bind(q.job_num)
    .bind(q.breakjob_num)
    .bind(q.down_resume)
    .bind(q.invite_resume)
    .bind(q.zph_num)
    .bind(q.top_num)
    .bind(q.urgent_num)
    .bind(q.rec_num)
    .bind(q.vip_stime)
    .bind(q.vip_etime)
    .bind(zt_time)
    .execute(pool)
    .await?;
    sqlx::query(
        "UPDATE phpyun_company_statis SET down_resume = 0, breakjob_num = 0, invite_resume = 0, \
         zph_num = 0, job_num = 0, top_num = 0, urgent_num = 0, rec_num = 0, sons_num = 0 \
         WHERE uid = ?",
    )
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(true)
}

/// PHP `restoreComStatic` — put the newest snapshot back and stamp the sub row.
///
/// Restores eight counters; `sons_num` stays zeroed because PHP never
/// snapshots it, so a suspend/resume round trip loses it.
pub async fn restore_company_quota(
    pool: &MySqlPool,
    uid: u64,
    zt_type: i32,
    now: i64,
) -> Result<bool, sqlx::Error> {
    let snap: Option<CompanyQuotaSnapshotRow> = sqlx::query_as(
        "SELECT CAST(id AS UNSIGNED) AS id, \
         CAST(COALESCE(job_num, 0) AS SIGNED) AS job_num, \
         CAST(COALESCE(breakjob_num, 0) AS SIGNED) AS breakjob_num, \
         CAST(COALESCE(down_resume, 0) AS SIGNED) AS down_resume, \
         CAST(COALESCE(invite_resume, 0) AS SIGNED) AS invite_resume, \
         CAST(COALESCE(zph_num, 0) AS SIGNED) AS zph_num, \
         CAST(COALESCE(top_num, 0) AS SIGNED) AS top_num, \
         CAST(COALESCE(urgent_num, 0) AS SIGNED) AS urgent_num, \
         CAST(COALESCE(rec_num, 0) AS SIGNED) AS rec_num \
         FROM phpyun_company_statis_sub WHERE uid = ? ORDER BY id DESC LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    let Some(s) = snap else { return Ok(false) };
    sqlx::query(
        "UPDATE phpyun_company_statis SET down_resume = ?, breakjob_num = ?, invite_resume = ?, \
         zph_num = ?, job_num = ?, top_num = ?, urgent_num = ?, rec_num = ? WHERE uid = ?",
    )
    .bind(s.down_resume)
    .bind(s.breakjob_num)
    .bind(s.invite_resume)
    .bind(s.zph_num)
    .bind(s.job_num)
    .bind(s.top_num)
    .bind(s.urgent_num)
    .bind(s.rec_num)
    .bind(uid)
    .execute(pool)
    .await?;
    sqlx::query("UPDATE phpyun_company_statis_sub SET hf_time = ?, zt_type = ? WHERE id = ?")
        .bind(now)
        .bind(zt_type)
        .bind(s.id)
        .execute(pool)
        .await?;
    Ok(true)
}

/// `phpyun_company.package` — CSV of extra `company_rating` ids.
pub async fn company_package(pool: &MySqlPool, uid: u64) -> Result<String, sqlx::Error> {
    let row: Option<(String,)> =
        sqlx::query_as("SELECT COALESCE(package, '') FROM phpyun_company WHERE uid = ? LIMIT 1")
            .bind(uid)
            .fetch_optional(pool)
            .await?;
    Ok(row.map(|r| r.0).unwrap_or_default())
}

/// `phpyun_company.zt_time` — when the current suspension started (0 = active).
pub async fn company_zt_time(pool: &MySqlPool, uid: u64) -> Result<i64, sqlx::Error> {
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT CAST(COALESCE(zt_time, 0) AS SIGNED) FROM phpyun_company WHERE uid = ? LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| r.0).unwrap_or(0))
}

pub async fn set_company_zt_time(pool: &MySqlPool, uid: u64, ts: i64) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_company SET zt_time = ? WHERE uid = ?")
            .bind(ts)
            .bind(uid)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

/// PHP `setupcom` clears the suspension stamp and mirrors the new VIP end date
/// onto `phpyun_company.vipetime`.
pub async fn clear_suspension(
    pool: &MySqlPool,
    uid: u64,
    vip_etime: i64,
) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_company SET zt_time = 0, vipetime = ? WHERE uid = ?")
            .bind(vip_etime)
            .bind(uid)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

pub async fn set_company_vip_etime(
    pool: &MySqlPool,
    uid: u64,
    vip_etime: i64,
) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_company_statis SET vip_etime = ? WHERE uid = ?")
            .bind(vip_etime)
            .bind(uid)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

/// PHP `userinfo.model::status` with `usertype = 2, setup = 1` — reinstate a
/// company and its job rows. Companies sitting at `r_status = 2` are skipped,
/// as PHP filters them out of the update set.
pub async fn reinstate_company(pool: &MySqlPool, uid: u64) -> Result<bool, sqlx::Error> {
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT CAST(COALESCE(r_status, 0) AS SIGNED) FROM phpyun_company WHERE uid = ? LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    match row {
        Some((2,)) | None => return Ok(false),
        _ => {}
    }
    // `logo_status = 0` rides along because PHP sets it whenever a company is
    // approved back to `r_status = 1`.
    sqlx::query("UPDATE phpyun_company SET r_status = 1, logo_status = 0 WHERE uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    sqlx::query("UPDATE phpyun_partjob SET r_status = 1 WHERE uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    sqlx::query("UPDATE phpyun_company_job SET r_status = 1 WHERE uid = ?")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(true)
}

/// `phpyun_company_rating.service_time` / `name` for the company's tier.
pub async fn rating_service_time(
    pool: &MySqlPool,
    rating_id: i64,
) -> Result<Option<(String, i64)>, sqlx::Error> {
    if rating_id <= 0 {
        return Ok(None);
    }
    sqlx::query_as(
        "SELECT COALESCE(`name`, '') AS `name`, \
         CAST(COALESCE(service_time, 0) AS SIGNED) AS service_time \
         FROM phpyun_company_rating WHERE id = ? LIMIT 1",
    )
    .bind(rating_id)
    .fetch_optional(pool)
    .await
}

/// PHP `statis.model::vipOver`, `com_vip_done = '0'` branch — drop the company
/// back to the expired tier and clear its quota.
pub async fn expire_company_rating(
    pool: &MySqlPool,
    uid: u64,
    old_rating_name: &str,
    expired_name: &str,
    unpublish_jobs: bool,
) -> Result<u64, sqlx::Error> {
    let affected = sqlx::query(
        "UPDATE phpyun_company_statis SET job_num = 0, breakjob_num = 0, down_resume = 0, \
         invite_resume = 0, zph_num = 0, top_num = 0, rec_num = 0, urgent_num = 0, \
         oldrating_name = ?, rating_name = ?, rating_type = 0, rating = 0, \
         suspend_num = 0, max_time = 0 WHERE uid = ?",
    )
    .bind(old_rating_name)
    .bind(expired_name)
    .bind(uid)
    .execute(pool)
    .await?
    .rows_affected();
    sqlx::query("UPDATE phpyun_company SET rating = 0, rating_name = ? WHERE uid = ?")
        .bind(expired_name)
        .bind(uid)
        .execute(pool)
        .await?;
    if unpublish_jobs {
        sqlx::query("UPDATE phpyun_company_job SET rating = 0, status = 1 WHERE uid = ?")
            .bind(uid)
            .execute(pool)
            .await?;
    } else {
        sqlx::query("UPDATE phpyun_company_job SET rating = 0 WHERE uid = ?")
            .bind(uid)
            .execute(pool)
            .await?;
    }
    Ok(affected)
}

pub async fn insert_tuiwen_tasks(
    pool: &MySqlPool,
    rows: &[TuiWenTaskIn],
    now: i64,
) -> Result<u64, sqlx::Error> {
    if rows.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new(
        "INSERT INTO phpyun_wxpub_twtask \
         (cuid, comname, jobsdate, auid, content, urgent, wcmoments, gzh, ctime, status, `type`, jobid, jobname) ",
    );
    qb.push_values(rows, |mut b, r| {
        b.push_bind(r.cuid)
            .push_bind(r.comname.clone())
            .push_bind(r.jobsdate)
            .push_bind(r.auid)
            .push_bind(r.content.clone())
            .push_bind(r.urgent)
            .push_bind(r.wcmoments)
            .push_bind(r.gzh)
            .push_bind(now)
            .push_bind(0)
            .push_bind(r.kind)
            .push_bind(r.jobid)
            .push_bind(r.jobname.clone());
    });
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn php_get_admin_email(pool: &MySqlPool, id: u64) -> Result<Option<AdminEmailRow>, sqlx::Error> {
    sqlx::query_as::<_, AdminEmailRow>(
        "SELECT CAST(id AS UNSIGNED) AS id, \
                COALESCE(smtpserver,'') AS smtpserver, \
                COALESCE(smtpuser,'') AS smtpuser, \
                COALESCE(smtppass,'') AS smtppass, \
                COALESCE(smtpport,'') AS smtpport, \
                COALESCE(smtpnick,'') AS smtpnick, \
                CAST(COALESCE(`default`,0) AS SIGNED) AS default_flag \
         FROM phpyun_admin_email WHERE id = ? LIMIT 1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn php_count_default_smtp(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_admin_email WHERE `default` = 1")
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn php_delete_admin_email(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    Ok(sqlx::query("DELETE FROM phpyun_admin_email WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected())
}

#[derive(Debug, Clone, FromRow)]
pub struct PhpWxBoundRow {
    pub uid: u64,
    pub username: String,
    pub wxid: String,
    pub wxbindtime: i64,
}

fn push_wx_bound_where(qb: &mut QueryBuilder<'_, sqlx::MySql>, keyword: Option<&str>) {
    qb.push(" FROM phpyun_member WHERE wxid IS NOT NULL AND wxid <> ''");
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND username LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
}

pub async fn php_list_wx_bound(
    pool: &MySqlPool,
    keyword: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<PhpWxBoundRow>, sqlx::Error> {
    let mut qb = QueryBuilder::new(
        "SELECT CAST(uid AS UNSIGNED) AS uid, COALESCE(username,'') AS username, \
         COALESCE(wxid,'') AS wxid, CAST(COALESCE(wxbindtime,0) AS SIGNED) AS wxbindtime",
    );
    push_wx_bound_where(&mut qb, keyword);
    qb.push(" ORDER BY wxbindtime DESC LIMIT ");
    qb.push_bind(limit as i64);
    qb.push(" OFFSET ");
    qb.push_bind(offset as i64);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn php_count_wx_bound(pool: &MySqlPool, keyword: Option<&str>) -> Result<u64, sqlx::Error> {
    let mut qb = QueryBuilder::new("SELECT COUNT(*)");
    push_wx_bound_where(&mut qb, keyword);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn php_clear_member_wxids(pool: &MySqlPool, uids: &[u64]) -> Result<u64, sqlx::Error> {
    if uids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("UPDATE phpyun_member SET wxid = '', wxbindtime = 0 WHERE uid IN (");
    let mut sep = qb.separated(", ");
    for id in uids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn php_delete_old_wxqrcodes(pool: &MySqlPool, before: i64) -> Result<u64, sqlx::Error> {
    Ok(sqlx::query("DELETE FROM phpyun_wxqrcode WHERE `time` < ?")
        .bind(before)
        .execute(pool)
        .await?
        .rows_affected())
}

#[derive(Debug, Clone, FromRow)]
pub struct PhpWxHotKeyRow {
    pub id: u64,
    pub key_name: String,
    pub num: i32,
    pub wxtime: i64,
}

pub async fn php_list_wx_hot_keys(
    pool: &MySqlPool,
    keyword: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<PhpWxHotKeyRow>, sqlx::Error> {
    let mut qb = QueryBuilder::new(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(key_name,'') AS key_name, \
         CAST(COALESCE(num,0) AS SIGNED) AS num, CAST(COALESCE(wxtime,0) AS SIGNED) AS wxtime \
         FROM phpyun_hot_key WHERE COALESCE(deleted,0)=0 AND `type` = 8",
    );
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND key_name LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    qb.push(" ORDER BY num DESC, id DESC LIMIT ");
    qb.push_bind(limit as i64);
    qb.push(" OFFSET ");
    qb.push_bind(offset as i64);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn php_count_wx_hot_keys(pool: &MySqlPool, keyword: Option<&str>) -> Result<u64, sqlx::Error> {
    let mut qb = QueryBuilder::new(
        "SELECT COUNT(*) FROM phpyun_hot_key WHERE COALESCE(deleted,0)=0 AND `type` = 8",
    );
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND key_name LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

#[derive(Debug, Clone, FromRow)]
pub struct PhpMemberContact {
    pub uid: u64,
    pub email: String,
    pub moblie: String,
    pub usertype: i32,
    pub username: String,
}

pub async fn php_list_members_by_usertype(
    pool: &MySqlPool,
    usertype: i32,
) -> Result<Vec<PhpMemberContact>, sqlx::Error> {
    sqlx::query_as::<_, PhpMemberContact>(
        "SELECT CAST(uid AS UNSIGNED) AS uid, COALESCE(email,'') AS email, COALESCE(moblie,'') AS moblie, \
         CAST(COALESCE(usertype,0) AS SIGNED) AS usertype, COALESCE(username,'') AS username \
         FROM phpyun_member WHERE usertype = ?",
    )
    .bind(usertype)
    .fetch_all(pool)
    .await
}

pub async fn php_list_members_by_emails(
    pool: &MySqlPool,
    emails: &[String],
) -> Result<Vec<PhpMemberContact>, sqlx::Error> {
    if emails.is_empty() {
        return Ok(Vec::new());
    }
    let mut qb = QueryBuilder::new(
        "SELECT CAST(uid AS UNSIGNED) AS uid, COALESCE(email,'') AS email, COALESCE(moblie,'') AS moblie, \
         CAST(COALESCE(usertype,0) AS SIGNED) AS usertype, COALESCE(username,'') AS username \
         FROM phpyun_member WHERE email IN (",
    );
    let mut sep = qb.separated(", ");
    for e in emails {
        sep.push_bind(e);
    }
    qb.push(")");
    qb.build_query_as().fetch_all(pool).await
}

pub async fn php_list_members_by_mobiles(
    pool: &MySqlPool,
    mobiles: &[String],
) -> Result<Vec<PhpMemberContact>, sqlx::Error> {
    if mobiles.is_empty() {
        return Ok(Vec::new());
    }
    let mut qb = QueryBuilder::new(
        "SELECT CAST(uid AS UNSIGNED) AS uid, COALESCE(email,'') AS email, COALESCE(moblie,'') AS moblie, \
         CAST(COALESCE(usertype,0) AS SIGNED) AS usertype, COALESCE(username,'') AS username \
         FROM phpyun_member WHERE moblie IN (",
    );
    let mut sep = qb.separated(", ");
    for m in mobiles {
        sep.push_bind(m);
    }
    qb.push(")");
    qb.build_query_as().fetch_all(pool).await
}

async fn count_in_ids(
    pool: &MySqlPool,
    all_sql: &str,
    in_sql: &str,
    ids: &[u64],
) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        let (n,): (i64,) = sqlx::query_as(all_sql).fetch_one(pool).await?;
        return Ok(phpyun_core::numeric::nonnegative_count(n));
    }
    let mut qb = QueryBuilder::new(in_sql);
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn php_count_jobs(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    count_in_ids(
        pool,
        "SELECT COUNT(*) FROM phpyun_company_job",
        "SELECT COUNT(*) FROM phpyun_company_job WHERE id IN (",
        ids,
    )
    .await
}

pub async fn php_count_expects(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    count_in_ids(
        pool,
        "SELECT COUNT(*) FROM phpyun_resume_expect",
        "SELECT COUNT(*) FROM phpyun_resume_expect WHERE id IN (",
        ids,
    )
    .await
}

pub async fn php_count_companies(pool: &MySqlPool, uids: &[u64]) -> Result<u64, sqlx::Error> {
    count_in_ids(
        pool,
        "SELECT COUNT(*) FROM phpyun_company",
        "SELECT COUNT(*) FROM phpyun_company WHERE uid IN (",
        uids,
    )
    .await
}

pub async fn php_update_job_class(
    pool: &MySqlPool,
    ids: &[u64],
    hy: i32,
    job1: i32,
    job1_son: i32,
    job_post: i32,
) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("UPDATE phpyun_company_job SET hy = ");
    qb.push_bind(hy)
        .push(", job1 = ")
        .push_bind(job1)
        .push(", job1_son = ")
        .push_bind(job1_son)
        .push(", job_post = ")
        .push_bind(job_post)
        .push(" WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

#[derive(Debug, Clone, FromRow)]
pub struct PhpJobExportRow {
    pub id: u64,
    pub uid: u64,
    pub name: String,
    pub com_name: String,
    pub minsalary: i32,
    pub maxsalary: i32,
    pub lastupdate: i64,
}

pub async fn php_list_jobs_export(
    pool: &MySqlPool,
    ids: &[u64],
    limit: u64,
) -> Result<Vec<PhpJobExportRow>, sqlx::Error> {
    let cap = if limit == 0 { 5000 } else { limit.min(5000) };
    let mut qb = QueryBuilder::new(
        "SELECT CAST(id AS UNSIGNED) AS id, CAST(uid AS UNSIGNED) AS uid, \
         COALESCE(`name`,'') AS `name`, COALESCE(com_name,'') AS com_name, \
         CAST(COALESCE(minsalary,0) AS SIGNED) AS minsalary, \
         CAST(COALESCE(maxsalary,0) AS SIGNED) AS maxsalary, \
         CAST(COALESCE(lastupdate,0) AS SIGNED) AS lastupdate \
         FROM phpyun_company_job",
    );
    if !ids.is_empty() {
        qb.push(" WHERE id IN (");
        let mut sep = qb.separated(", ");
        for id in ids {
            sep.push_bind(*id);
        }
        qb.push(")");
    }
    qb.push(" ORDER BY id DESC LIMIT ");
    qb.push_bind(cap as i64);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn php_set_expect_label(
    pool: &MySqlPool,
    id: u64,
    label: &str,
    content: &str,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_resume_expect SET label = ?, content = ? WHERE id = ?",
    )
    .bind(label)
    .bind(content)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn php_delete_expects(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("DELETE FROM phpyun_resume_expect WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn php_delete_expect_children(pool: &MySqlPool, eids: &[u64]) -> Result<(), sqlx::Error> {
    if eids.is_empty() {
        return Ok(());
    }
    for table in [
        "phpyun_resume_work",
        "phpyun_resume_edu",
        "phpyun_resume_training",
        "phpyun_resume_skill",
        "phpyun_resume_project",
        "phpyun_resume_other",
    ] {
        let mut qb = QueryBuilder::new(format!("DELETE FROM {table} WHERE eid IN ("));
        let mut sep = qb.separated(", ");
        for id in eids {
            sep.push_bind(*id);
        }
        qb.push(")");
        qb.build().execute(pool).await?;
    }
    Ok(())
}

pub async fn php_set_fact_status(pool: &MySqlPool, uid: u64, status: i32) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_company SET fact_status = ? WHERE uid = ?")
        .bind(status)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub async fn php_insert_fact_pic(
    pool: &MySqlPool,
    uid: u64,
    picurl: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_company_fact (uid, picurl, ctime) VALUES (?, ?, ?)",
    )
    .bind(uid)
    .bind(picurl)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn php_delete_fact_pics(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("DELETE FROM phpyun_company_fact WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn php_member_wxid(pool: &MySqlPool, uid: u64) -> Result<String, sqlx::Error> {
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT COALESCE(wxid,'') FROM phpyun_member WHERE uid = ? LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| r.0).unwrap_or_default())
}

/// PHP `users_member::index_action` — resume rows joined with member.
#[derive(Debug, Clone, FromRow)]
pub struct PhpUserMemberRow {
    pub uid: u64,
    pub username: String,
    pub email: String,
    pub moblie: String,
    pub login_ip: String,
    pub login_date: i64,
    pub reg_date: i64,
    pub source: i32,
    pub status: i32,
    pub usertype: i32,
    pub username_n: String,
    pub telphone: String,
    pub r_status: i32,
    pub idcard_status: i32,
    pub email_status: i32,
    pub moblie_status: i32,
    pub def_job: i32,
    pub wxid: String,
    pub wxopenid: String,
    pub unionid: String,
    pub login_address: String,
    pub moblie_address: String,
    pub sq_num: i64,
}

pub struct PhpUserMemberFilter<'a> {
    pub keyword: Option<&'a str>,
    pub kw_type: i32,
    pub r_status: Option<i32>,
    pub source: Option<i32>,
    pub def_job: Option<i32>,
    pub login_from: Option<i64>,
    pub login_to: Option<i64>,
    pub reg_from: Option<i64>,
    pub reg_to: Option<i64>,
    pub order_t: &'a str,
    pub order_dir: &'a str,
}

const PHP_USER_MEMBER_FIELDS: &str = "SELECT CAST(r.uid AS UNSIGNED) AS uid, \
 COALESCE(m.username,'') AS username, COALESCE(r.email, m.email, '') AS email, \
 COALESCE(m.moblie,'') AS moblie, COALESCE(m.login_ip,'') AS login_ip, \
 CAST(COALESCE(m.login_date, r.login_date, 0) AS SIGNED) AS login_date, \
 CAST(COALESCE(m.reg_date,0) AS SIGNED) AS reg_date, \
 CAST(COALESCE(m.source,0) AS SIGNED) AS source, \
 CAST(COALESCE(m.status,0) AS SIGNED) AS status, \
 CAST(COALESCE(m.usertype,1) AS SIGNED) AS usertype, \
 COALESCE(r.name,'') AS username_n, \
 COALESCE(NULLIF(r.telphone,''), m.moblie, '') AS telphone, \
 CAST(COALESCE(r.r_status,1) AS SIGNED) AS r_status, \
 CAST(COALESCE(r.idcard_status,0) AS SIGNED) AS idcard_status, \
 CAST(COALESCE(r.email_status,0) AS SIGNED) AS email_status, \
 CAST(COALESCE(r.moblie_status,0) AS SIGNED) AS moblie_status, \
 CAST(COALESCE(r.def_job,0) AS SIGNED) AS def_job, \
 COALESCE(m.wxid,'') AS wxid, COALESCE(m.wxopenid,'') AS wxopenid, \
 COALESCE(m.unionid,'') AS unionid, \
 COALESCE(m.login_address,'') AS login_address, \
 COALESCE(m.moblie_address,'') AS moblie_address, \
 CAST((SELECT COUNT(*) FROM phpyun_userid_job j WHERE j.uid = r.uid) AS SIGNED) AS sq_num \
 FROM phpyun_resume r INNER JOIN phpyun_member m ON m.uid = r.uid WHERE 1=1";

fn push_php_user_member_filters<'a>(qb: &mut QueryBuilder<'a, sqlx::MySql>, f: &PhpUserMemberFilter<'a>) {
    match f.def_job {
        Some(1) => {
            qb.push(" AND COALESCE(r.def_job,0) > 0");
        }
        Some(2) => {
            qb.push(" AND COALESCE(r.def_job,0) = 0");
        }
        _ => {}
    }
    if let Some(st) = f.r_status.filter(|v| *v > 0) {
        qb.push(" AND r.r_status = ");
        qb.push_bind(st);
    }
    if let Some(src) = f.source.filter(|v| *v > 0) {
        qb.push(" AND m.source = ");
        qb.push_bind(src);
    }
    if let Some(kw) = f.keyword.map(str::trim).filter(|s| !s.is_empty()) {
        match f.kw_type {
            2 => {
                qb.push(" AND r.name LIKE ");
                qb.push_bind(format!("%{kw}%"));
            }
            3 => {
                qb.push(" AND r.telphone LIKE ");
                qb.push_bind(format!("%{kw}%"));
            }
            4 => {
                qb.push(" AND r.email LIKE ");
                qb.push_bind(format!("%{kw}%"));
            }
            5 => {
                let uid: u64 = kw.parse().unwrap_or(0);
                qb.push(" AND r.uid = ");
                qb.push_bind(uid);
            }
            6 => {
                qb.push(" AND m.login_ip LIKE ");
                qb.push_bind(format!("%{kw}%"));
            }
            _ => {
                qb.push(" AND m.username LIKE ");
                qb.push_bind(format!("%{kw}%"));
            }
        }
    }
    if let Some(from) = f.login_from {
        qb.push(" AND COALESCE(r.login_date, m.login_date, 0) >= ");
        qb.push_bind(from);
    }
    if let Some(to) = f.login_to {
        qb.push(" AND COALESCE(r.login_date, m.login_date, 0) <= ");
        qb.push_bind(to);
    }
    if let Some(from) = f.reg_from {
        qb.push(" AND COALESCE(m.reg_date,0) >= ");
        qb.push_bind(from);
    }
    if let Some(to) = f.reg_to {
        qb.push(" AND COALESCE(m.reg_date,0) <= ");
        qb.push_bind(to);
    }
}

pub async fn php_list_user_members(
    pool: &MySqlPool,
    f: &PhpUserMemberFilter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<PhpUserMemberRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    let mut qb = QueryBuilder::new(PHP_USER_MEMBER_FIELDS);
    push_php_user_member_filters(&mut qb, f);
    let col = match f.order_t {
        "time" => "r.lastupdate",
        "login_date" => "m.login_date",
        "reg_date" => "m.reg_date",
        _ => "r.uid",
    };
    let dir = if f.order_dir.eq_ignore_ascii_case("asc") {
        " ASC"
    } else {
        " DESC"
    };
    qb.push(" ORDER BY ");
    qb.push(col);
    qb.push(dir);
    qb.push(" LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn php_count_user_members(
    pool: &MySqlPool,
    f: &PhpUserMemberFilter<'_>,
) -> Result<u64, sqlx::Error> {
    let mut qb = QueryBuilder::new(
        "SELECT COUNT(*) FROM phpyun_resume r INNER JOIN phpyun_member m ON m.uid = r.uid WHERE 1=1",
    );
    push_php_user_member_filters(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn php_find_wxpub_temp(pool: &MySqlPool, id: u64) -> Result<Option<WxpubTempRow>, sqlx::Error> {
    sqlx::query_as(&format!(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(title,'') AS title, COALESCE(header,'') AS header, \
         COALESCE(body,'') AS body, COALESCE(footer,'') AS footer, COALESCE(`type`,'') AS `type`, \
         CAST(COALESCE(temptype,0) AS SIGNED) AS temptype, CAST(COALESCE(time,0) AS SIGNED) AS time \
         FROM phpyun_wxpub_temps WHERE id = ? AND {PREDICATE} LIMIT 1"
    ))
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn php_count_wxpub_title(
    pool: &MySqlPool,
    title: &str,
    except: u64,
) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(&format!(
        "SELECT COUNT(*) FROM phpyun_wxpub_temps WHERE title = ? AND id <> ? AND {PREDICATE}"
    ))
    .bind(title)
    .bind(except)
    .fetch_one(pool)
    .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn php_list_wxpub_temps_php(
    pool: &MySqlPool,
    keyword: Option<&str>,
    temptype: Option<i32>,
    offset: u64,
    limit: u64,
) -> Result<Vec<WxpubTempRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    let mut qb = QueryBuilder::new(format!(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(title,'') AS title, COALESCE(header,'') AS header, \
         COALESCE(body,'') AS body, COALESCE(footer,'') AS footer, COALESCE(`type`,'') AS `type`, \
         CAST(COALESCE(temptype,0) AS SIGNED) AS temptype, CAST(COALESCE(time,0) AS SIGNED) AS time \
         FROM phpyun_wxpub_temps WHERE {PREDICATE}"
    ));
    if let Some(t) = temptype {
        qb.push(" AND temptype = ");
        qb.push_bind(t);
    }
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND title LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    qb.push(
        " ORDER BY CASE WHEN `type`='job' THEN 1 WHEN `type`='company' THEN 2 WHEN `type`='resume' THEN 3 ELSE 9 END ASC, id DESC LIMIT ",
    );
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

#[derive(Debug, Clone, FromRow)]
pub struct PhpIdName {
    pub id: u64,
    pub name: String,
}

pub async fn php_list_rating_names(pool: &MySqlPool) -> Result<Vec<PhpIdName>, sqlx::Error> {
    sqlx::query_as(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(`name`,'') AS `name` \
         FROM phpyun_company_rating WHERE category = 1 AND COALESCE(deleted,0)=0 ORDER BY sort ASC, id ASC",
    )
    .fetch_all(pool)
    .await
}

#[derive(Debug, Clone, FromRow)]
pub struct PhpAdminName {
    pub uid: u64,
    pub username: String,
    pub name: String,
}

pub async fn php_list_admin_names(pool: &MySqlPool) -> Result<Vec<PhpAdminName>, sqlx::Error> {
    sqlx::query_as(
        "SELECT CAST(uid AS UNSIGNED) AS uid, COALESCE(username,'') AS username, COALESCE(`name`,'') AS `name` \
         FROM phpyun_admin_user ORDER BY uid ASC",
    )
    .fetch_all(pool)
    .await
}

#[derive(Debug, Clone, FromRow)]
pub struct PhpPubSearchRow {
    pub value: u64,
    pub name: String,
    pub upname: String,
}

pub async fn php_search_wxpub_jobs(
    pool: &MySqlPool,
    keyword: &str,
) -> Result<Vec<PhpPubSearchRow>, sqlx::Error> {
    let kw = keyword.trim();
    if kw.is_empty() {
        return Ok(Vec::new());
    }
    if let Ok(id) = kw.parse::<u64>() {
        return sqlx::query_as(
            "SELECT CAST(id AS UNSIGNED) AS value, COALESCE(`name`,'') AS `name`, COALESCE(com_name,'') AS upname \
             FROM phpyun_company_job WHERE id = ? AND state = 1 AND status = 0 AND r_status = 1 LIMIT 20",
        )
        .bind(id)
        .fetch_all(pool)
        .await;
    }
    sqlx::query_as(
        "SELECT CAST(id AS UNSIGNED) AS value, COALESCE(`name`,'') AS `name`, COALESCE(com_name,'') AS upname \
         FROM phpyun_company_job WHERE state = 1 AND status = 0 AND r_status = 1 \
         AND (`name` LIKE ? OR com_name LIKE ?) ORDER BY lastupdate DESC LIMIT 20",
    )
    .bind(format!("%{kw}%"))
    .bind(format!("%{kw}%"))
    .fetch_all(pool)
    .await
}

pub async fn php_search_wxpub_coms(
    pool: &MySqlPool,
    keyword: &str,
) -> Result<Vec<PhpPubSearchRow>, sqlx::Error> {
    let kw = keyword.trim();
    if kw.is_empty() {
        return Ok(Vec::new());
    }
    sqlx::query_as(
        "SELECT CAST(uid AS UNSIGNED) AS value, COALESCE(`name`,'') AS `name`, '' AS upname \
         FROM phpyun_company WHERE r_status = 1 AND `name` LIKE ? ORDER BY uid DESC LIMIT 10",
    )
    .bind(format!("%{kw}%"))
    .fetch_all(pool)
    .await
}

#[derive(Debug, Clone, FromRow)]
pub struct PhpTwTaskRow {
    pub id: u64,
    pub jobid: u64,
    pub cuid: u64,
    pub jobname: String,
    pub comname: String,
    pub jobsdate: i64,
    pub auid: u64,
    pub content: String,
    pub urgent: i32,
    pub wcmoments: i32,
    pub status: i32,
    pub ctime: i64,
    pub gzh: i32,
    pub r#type: i32,
    pub etime: i64,
    pub job_off: i32,
    pub com_r_status: i32,
    pub admin_username: String,
}

pub struct PhpTwTaskFilter<'a> {
    pub kind: i32,
    pub keyword: Option<&'a str>,
    pub welfare: Option<&'a str>,
    pub auid: Option<u64>,
    pub status: Option<i32>,
    pub urgent: Option<i32>,
    pub wcmoments: Option<i32>,
    pub gzh: Option<i32>,
    pub order_t: &'a str,
    pub order_dir: &'a str,
}

fn push_twtask_filters<'a>(qb: &mut QueryBuilder<'a, sqlx::MySql>, f: &PhpTwTaskFilter<'a>) {
    qb.push(" AND t.`type` = ");
    qb.push_bind(f.kind);
    if let Some(uid) = f.auid.filter(|v| *v > 0) {
        qb.push(" AND t.auid = ");
        qb.push_bind(uid);
    }
    if let Some(st) = f.status {
        qb.push(" AND t.status = ");
        qb.push_bind(st);
    }
    if let Some(v) = f.urgent.filter(|n| *n > 0) {
        qb.push(" AND t.urgent = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.wcmoments.filter(|n| *n > 0) {
        qb.push(" AND t.wcmoments = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.gzh.filter(|n| *n > 0) {
        qb.push(" AND t.gzh = ");
        qb.push_bind(v);
    }
    if let Some(kw) = f.keyword.map(str::trim).filter(|s| !s.is_empty()) {
        if let Ok(id) = kw.parse::<u64>() {
            if f.kind == 1 {
                qb.push(" AND (t.jobid = ");
                qb.push_bind(id);
                qb.push(" OR t.content LIKE ");
                qb.push_bind(format!("%{kw}%"));
                qb.push(")");
            } else {
                qb.push(" AND (t.cuid = ");
                qb.push_bind(id);
                qb.push(" OR t.content LIKE ");
                qb.push_bind(format!("%{kw}%"));
                qb.push(")");
            }
        } else if f.kind == 1 {
            qb.push(" AND (t.jobname LIKE ");
            qb.push_bind(format!("%{kw}%"));
            qb.push(" OR t.comname LIKE ");
            qb.push_bind(format!("%{kw}%"));
            qb.push(" OR t.content LIKE ");
            qb.push_bind(format!("%{kw}%"));
            qb.push(")");
        } else {
            qb.push(" AND (t.comname LIKE ");
            qb.push_bind(format!("%{kw}%"));
            qb.push(" OR t.content LIKE ");
            qb.push_bind(format!("%{kw}%"));
            qb.push(")");
        }
    }
    if let Some(w) = f.welfare.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND t.jobid IN (SELECT id FROM phpyun_company_job WHERE 1=1");
        for part in w.split_whitespace() {
            qb.push(" AND FIND_IN_SET(");
            qb.push_bind(part);
            qb.push(", REPLACE(welfare,'，',','))");
        }
        qb.push(")");
    }
}

const TWTASK_FIELDS: &str = "SELECT CAST(t.id AS UNSIGNED) AS id, CAST(COALESCE(t.jobid,0) AS UNSIGNED) AS jobid, \
 CAST(COALESCE(t.cuid,0) AS UNSIGNED) AS cuid, COALESCE(t.jobname,'') AS jobname, \
 COALESCE(t.comname,'') AS comname, CAST(COALESCE(t.jobsdate,0) AS SIGNED) AS jobsdate, \
 CAST(COALESCE(t.auid,0) AS UNSIGNED) AS auid, COALESCE(t.content,'') AS content, \
 CAST(COALESCE(t.urgent,0) AS SIGNED) AS urgent, CAST(COALESCE(t.wcmoments,0) AS SIGNED) AS wcmoments, \
 CAST(COALESCE(t.status,0) AS SIGNED) AS status, CAST(COALESCE(t.ctime,0) AS SIGNED) AS ctime, \
 CAST(COALESCE(t.gzh,0) AS SIGNED) AS gzh, CAST(COALESCE(t.`type`,1) AS SIGNED) AS `type`, \
 CAST(COALESCE(t.etime,0) AS SIGNED) AS etime, \
 CAST(CASE WHEN j.id IS NULL THEN 1 WHEN j.status = 1 THEN 2 ELSE 0 END AS SIGNED) AS job_off, \
 CAST(COALESCE(c.r_status,0) AS SIGNED) AS com_r_status, \
 COALESCE(NULLIF(a.name,''), a.username, '') AS admin_username \
 FROM phpyun_wxpub_twtask t \
 LEFT JOIN phpyun_company_job j ON j.id = t.jobid \
 LEFT JOIN phpyun_company c ON c.uid = t.cuid \
 LEFT JOIN phpyun_admin_user a ON a.uid = t.auid WHERE 1=1";

pub async fn php_list_twtasks(
    pool: &MySqlPool,
    f: &PhpTwTaskFilter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<PhpTwTaskRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    let mut qb = QueryBuilder::new(TWTASK_FIELDS);
    push_twtask_filters(&mut qb, f);
    let col = match f.order_t {
        "jobsdate" => "t.jobsdate",
        "id" => "t.id",
        _ => "t.ctime",
    };
    let dir = if f.order_dir.eq_ignore_ascii_case("asc") {
        " ASC"
    } else {
        " DESC"
    };
    qb.push(" ORDER BY ");
    qb.push(col);
    qb.push(dir);
    qb.push(" LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn php_count_twtasks(pool: &MySqlPool, f: &PhpTwTaskFilter<'_>) -> Result<u64, sqlx::Error> {
    let mut qb = QueryBuilder::new("SELECT COUNT(*) FROM phpyun_wxpub_twtask t WHERE 1=1");
    push_twtask_filters(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn php_delete_twtasks(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("DELETE FROM phpyun_wxpub_twtask WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn php_finish_twtasks(pool: &MySqlPool, ids: &[u64], now: i64) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb = QueryBuilder::new("UPDATE phpyun_wxpub_twtask SET status = 1, etime = ");
    qb.push_bind(now);
    qb.push(" WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

#[derive(Debug, Clone, FromRow)]
pub struct PhpPubJobRow {
    pub id: u64,
    pub uid: u64,
    pub name: String,
    pub com_name: String,
    pub minsalary: i32,
    pub maxsalary: i32,
    pub number: i32,
    pub age: String,
    pub sex: i32,
    pub exp: i32,
    pub edu: i32,
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    pub description: String,
    pub welfare: String,
    pub content: String,
    pub linktel: String,
    pub linkphone: String,
    pub address: String,
}

pub struct PhpPubJobFilter<'a> {
    pub ids: Option<&'a [u64]>,
    pub rating: Option<&'a str>,
    pub keyword: Option<&'a str>,
    pub provinceid: Option<i32>,
    pub cityid: Option<i32>,
    pub three_cityid: Option<i32>,
    pub job1: Option<i32>,
    pub job1_son: Option<i32>,
    pub job_post: Option<i32>,
    pub lastupdate_gt: Option<i64>,
    pub sdate_gt: Option<i64>,
    pub xsdate: bool,
    pub urgent: bool,
    pub rec: bool,
    pub minsalary: Option<i32>,
    pub maxsalary: Option<i32>,
    pub welfare: Option<&'a str>,
    pub now: i64,
}

fn push_pub_job_filters<'a>(qb: &mut QueryBuilder<'a, sqlx::MySql>, f: &PhpPubJobFilter<'a>) {
    qb.push(" AND j.state = 1 AND j.status = 0 AND j.r_status = 1");
    if let Some(ids) = f.ids.filter(|v| !v.is_empty()) {
        qb.push(" AND j.id IN (");
        let mut sep = qb.separated(", ");
        for id in ids {
            sep.push_bind(*id);
        }
        qb.push(")");
    }
    if let Some(r) = f.rating.map(str::trim).filter(|s| !s.is_empty()) {
        let nums: Vec<i32> = r
            .split([',', '，'])
            .filter_map(|p| p.trim().parse::<i32>().ok())
            .collect();
        if !nums.is_empty() {
            qb.push(" AND j.rating IN (");
            let mut sep = qb.separated(", ");
            for n in nums {
                sep.push_bind(n);
            }
            qb.push(")");
        }
    }
    if let Some(kw) = f.keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND (j.name LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR j.com_name LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(")");
    }
    if let Some(v) = f.provinceid.filter(|n| *n > 0) {
        qb.push(" AND j.provinceid = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.cityid.filter(|n| *n > 0) {
        qb.push(" AND j.cityid = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.three_cityid.filter(|n| *n > 0) {
        qb.push(" AND j.three_cityid = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.job1.filter(|n| *n > 0) {
        qb.push(" AND j.job1 = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.job1_son.filter(|n| *n > 0) {
        qb.push(" AND j.job1_son = ");
        qb.push_bind(v);
    }
    if let Some(v) = f.job_post.filter(|n| *n > 0) {
        qb.push(" AND j.job_post = ");
        qb.push_bind(v);
    }
    if let Some(ts) = f.lastupdate_gt {
        qb.push(" AND j.lastupdate > ");
        qb.push_bind(ts);
    }
    if let Some(ts) = f.sdate_gt {
        qb.push(" AND j.sdate > ");
        qb.push_bind(ts);
    }
    if f.xsdate {
        qb.push(" AND j.xsdate > ");
        qb.push_bind(f.now);
    }
    if f.urgent {
        qb.push(" AND j.urgent_time > ");
        qb.push_bind(f.now);
    }
    if f.rec {
        qb.push(" AND j.rec_time > ");
        qb.push_bind(f.now);
    }
    if let Some(v) = f.minsalary.filter(|n| *n > 0) {
        qb.push(" AND j.minsalary > ");
        qb.push_bind(v);
    }
    if let Some(v) = f.maxsalary.filter(|n| *n > 0) {
        qb.push(" AND j.maxsalary < ");
        qb.push_bind(v);
    }
    if let Some(w) = f.welfare.map(str::trim).filter(|s| !s.is_empty()) {
        for part in w.split(['|', ' ']).map(str::trim).filter(|s| !s.is_empty()) {
            qb.push(" AND j.welfare LIKE ");
            qb.push_bind(format!("%{part}%"));
        }
    }
}

const PUB_JOB_FIELDS: &str = "SELECT CAST(j.id AS UNSIGNED) AS id, CAST(j.uid AS UNSIGNED) AS uid, \
 COALESCE(j.name,'') AS `name`, COALESCE(j.com_name,'') AS com_name, \
 CAST(COALESCE(j.minsalary,0) AS SIGNED) AS minsalary, CAST(COALESCE(j.maxsalary,0) AS SIGNED) AS maxsalary, \
 CAST(COALESCE(j.number,0) AS SIGNED) AS number, COALESCE(j.age,'') AS age, \
 CAST(COALESCE(j.sex,0) AS SIGNED) AS sex, CAST(COALESCE(j.exp,0) AS SIGNED) AS exp, \
 CAST(COALESCE(j.edu,0) AS SIGNED) AS edu, CAST(COALESCE(j.provinceid,0) AS SIGNED) AS provinceid, \
 CAST(COALESCE(j.cityid,0) AS SIGNED) AS cityid, CAST(COALESCE(j.three_cityid,0) AS SIGNED) AS three_cityid, \
 COALESCE(j.description,'') AS description, COALESCE(j.welfare,'') AS welfare, \
 COALESCE(c.content,'') AS content, COALESCE(c.linktel,'') AS linktel, \
 COALESCE(c.linkphone,'') AS linkphone, COALESCE(c.address,'') AS address \
 FROM phpyun_company_job j LEFT JOIN phpyun_company c ON c.uid = j.uid WHERE 1=1";

pub async fn php_list_pubtool_jobs(
    pool: &MySqlPool,
    f: &PhpPubJobFilter<'_>,
    limit: u64,
) -> Result<Vec<PhpPubJobRow>, sqlx::Error> {
    let cap = if limit == 0 { 20 } else { limit.min(200) };
    let mut qb = QueryBuilder::new(PUB_JOB_FIELDS);
    push_pub_job_filters(&mut qb, f);
    qb.push(" ORDER BY j.lastupdate DESC LIMIT ");
    qb.push_bind(cap as i64);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn php_list_pubtool_jobs_by_uid(
    pool: &MySqlPool,
    uid: u64,
    limit: u64,
) -> Result<Vec<PhpPubJobRow>, sqlx::Error> {
    let cap = if limit == 0 { 5 } else { limit.min(50) };
    let mut qb = QueryBuilder::new(PUB_JOB_FIELDS);
    qb.push(" AND j.state = 1 AND j.status = 0 AND j.r_status = 1 AND j.uid = ");
    qb.push_bind(uid);
    qb.push(" ORDER BY j.lastupdate DESC LIMIT ");
    qb.push_bind(cap as i64);
    qb.build_query_as().fetch_all(pool).await
}

#[derive(Debug, Clone, FromRow)]
pub struct PhpPubResumeRow {
    pub id: u64,
    pub uid: u64,
    pub name: String,
    pub uname: String,
    pub edu: i32,
    pub exp: i32,
    pub salary: String,
    pub birthday: String,
    pub photo: String,
}

pub async fn php_list_pubtool_resumes(
    pool: &MySqlPool,
    limit: u64,
    lastupdate_gt: Option<i64>,
    ctime_gt: Option<i64>,
    integrity: Option<i32>,
) -> Result<Vec<PhpPubResumeRow>, sqlx::Error> {
    let cap = if limit == 0 { 20 } else { limit.min(200) };
    let mut qb = QueryBuilder::new(
        "SELECT CAST(e.id AS UNSIGNED) AS id, CAST(e.uid AS UNSIGNED) AS uid, \
         COALESCE(e.name,'') AS `name`, COALESCE(r.name,'') AS uname, \
         CAST(COALESCE(e.edu,0) AS SIGNED) AS edu, CAST(COALESCE(e.exp,0) AS SIGNED) AS exp, \
         COALESCE(e.salary,'') AS salary, COALESCE(r.birthday,'') AS birthday, \
         COALESCE(r.photo,'') AS photo \
         FROM phpyun_resume_expect e INNER JOIN phpyun_resume r ON r.uid = e.uid \
         WHERE e.defaults = 1 AND e.state = 1 AND e.status = 1 AND e.r_status = 1",
    );
    if let Some(ts) = lastupdate_gt {
        qb.push(" AND e.lastupdate > ");
        qb.push_bind(ts);
    }
    if let Some(ts) = ctime_gt {
        qb.push(" AND e.ctime > ");
        qb.push_bind(ts);
    }
    if let Some(n) = integrity.filter(|v| *v > 0) {
        qb.push(" AND e.integrity >= ");
        qb.push_bind(n);
    }
    qb.push(" ORDER BY e.lastupdate DESC LIMIT ");
    qb.push_bind(cap as i64);
    qb.build_query_as().fetch_all(pool).await
}

#[derive(Debug, Clone, FromRow)]
pub struct PhpPubComRow {
    pub uid: u64,
    pub name: String,
    pub content: String,
    pub linkman: String,
    pub linktel: String,
    pub address: String,
    pub welfare: String,
}

pub async fn php_list_pubtool_companies(
    pool: &MySqlPool,
    uids: &[u64],
    rating: Option<&str>,
    limit: u64,
) -> Result<Vec<PhpPubComRow>, sqlx::Error> {
    let cap = if limit == 0 { 20 } else { limit.min(200) };
    let mut qb = QueryBuilder::new(
        "SELECT CAST(uid AS UNSIGNED) AS uid, COALESCE(`name`,'') AS `name`, \
         COALESCE(content,'') AS content, COALESCE(linkman,'') AS linkman, \
         COALESCE(linktel,'') AS linktel, COALESCE(address,'') AS address, \
         COALESCE(welfare,'') AS welfare FROM phpyun_company WHERE r_status = 1 AND `name` <> ''",
    );
    if !uids.is_empty() {
        qb.push(" AND uid IN (");
        let mut sep = qb.separated(", ");
        for id in uids {
            sep.push_bind(*id);
        }
        qb.push(")");
    }
    if let Some(r) = rating.map(str::trim).filter(|s| !s.is_empty()) {
        let nums: Vec<i32> = r
            .split([',', '，'])
            .filter_map(|p| p.trim().parse::<i32>().ok())
            .collect();
        if !nums.is_empty() {
            qb.push(" AND rating IN (");
            let mut sep = qb.separated(", ");
            for n in nums {
                sep.push_bind(n);
            }
            qb.push(")");
        }
    }
    qb.push(" ORDER BY uid DESC LIMIT ");
    qb.push_bind(cap as i64);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn php_list_wxpub_temp_titles(
    pool: &MySqlPool,
    temptype: i32,
    types: &[&str],
) -> Result<Vec<PhpIdName>, sqlx::Error> {
    let mut qb = QueryBuilder::new(format!(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(title,'') AS `name` \
         FROM phpyun_wxpub_temps WHERE {PREDICATE} AND temptype = "
    ));
    qb.push_bind(temptype);
    if !types.is_empty() {
        qb.push(" AND `type` IN (");
        let mut sep = qb.separated(", ");
        for t in types {
            sep.push_bind(*t);
        }
        qb.push(")");
    }
    qb.push(" ORDER BY id ASC");
    qb.build_query_as().fetch_all(pool).await
}
