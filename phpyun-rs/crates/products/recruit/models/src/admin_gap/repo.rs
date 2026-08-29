//! Admin SQL for unmigrated PHP tables. Table names are literals.

use super::entity::*;
use sqlx::{MySqlPool, QueryBuilder};

pub(super) fn lim(limit: u64, offset: u64) -> Result<(i64, i64), sqlx::Error> {
    Ok((
        phpyun_core::numeric::checked_db_i64(limit, "pagination.limit")?,
        phpyun_core::numeric::checked_db_i64(offset, "pagination.offset")?,
    ))
}

pub(super) async fn delete_in(pool: &MySqlPool, prefix: &str, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(prefix);
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

// ---------- resume photos / certs ----------

pub async fn list_user_photos(
    pool: &MySqlPool,
    status: Option<i32>,
    keyword: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<UserPhotoRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(uid AS UNSIGNED) AS uid, COALESCE(name,'') AS name, \
         COALESCE(name,'') AS username, \
         CAST(COALESCE(sex,0) AS SIGNED) AS sex, COALESCE(photo,'') AS photo, \
         CAST(COALESCE(photo_status,0) AS SIGNED) AS photo_status \
         FROM phpyun_resume WHERE photo <> '' AND COALESCE(defphoto,1) = 1",
    );
    if let Some(s) = status {
        qb.push(" AND photo_status = ");
        qb.push_bind(s);
    }
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND (name LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR uid = ");
        qb.push_bind(kw.parse::<u64>().unwrap_or(0));
        qb.push(")");
    }
    qb.push(" ORDER BY photo_status DESC, uid DESC LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_user_photos(
    pool: &MySqlPool,
    status: Option<i32>,
    keyword: Option<&str>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT COUNT(*) FROM phpyun_resume WHERE photo <> '' AND COALESCE(defphoto,1) = 1",
    );
    if let Some(s) = status {
        qb.push(" AND photo_status = ");
        qb.push_bind(s);
    }
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND (name LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR uid = ");
        qb.push_bind(kw.parse::<u64>().unwrap_or(0));
        qb.push(")");
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn set_photo_status(pool: &MySqlPool, uid: u64, status: i32) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_resume SET photo_status = ? WHERE uid = ?")
            .bind(status)
            .bind(uid)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

pub async fn list_user_certs(
    pool: &MySqlPool,
    status: Option<i32>,
    keyword: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<UserCertRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(uid AS UNSIGNED) AS uid, COALESCE(name,'') AS name, \
         COALESCE(idcard,'') AS idcard, COALESCE(idcard_pic,'') AS idcard_pic, \
         CAST(COALESCE(idcard_status,0) AS SIGNED) AS idcard_status, \
         CAST(COALESCE(cert_time,0) AS SIGNED) AS cert_time \
         FROM phpyun_resume WHERE idcard_pic <> ''",
    );
    if let Some(s) = status {
        qb.push(" AND idcard_status = ");
        qb.push_bind(s);
    }
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND name LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    qb.push(" ORDER BY idcard_status ASC, cert_time DESC LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_user_certs(
    pool: &MySqlPool,
    status: Option<i32>,
    keyword: Option<&str>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_resume WHERE idcard_pic <> ''");
    if let Some(s) = status {
        qb.push(" AND idcard_status = ");
        qb.push_bind(s);
    }
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND name LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn set_idcard_status(pool: &MySqlPool, uid: u64, status: i32) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_resume SET idcard_status = ? WHERE uid = ?")
            .bind(status)
            .bind(uid)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

// ---------- job consult msgs ----------

pub async fn list_user_msgs(
    pool: &MySqlPool,
    keyword: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<UserMsgRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(id AS UNSIGNED) AS id, CAST(COALESCE(uid,0) AS UNSIGNED) AS uid, \
         COALESCE(username,'') AS username, COALESCE(job_name,'') AS job_name, \
         COALESCE(com_name,'') AS com_name, COALESCE(content,'') AS content, \
         COALESCE(reply,'') AS reply, CAST(COALESCE(datetime,0) AS SIGNED) AS datetime, \
         CAST(COALESCE(reply_time,0) AS SIGNED) AS reply_time, \
         CAST(COALESCE(status,0) AS SIGNED) AS status \
         FROM phpyun_msg WHERE COALESCE(del_status,0) = 0",
    );
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND (username LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR job_name LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR com_name LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR content LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(")");
    }
    qb.push(" ORDER BY id DESC LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_user_msgs(pool: &MySqlPool, keyword: Option<&str>) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_msg WHERE COALESCE(del_status,0) = 0");
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND (username LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR job_name LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR com_name LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR content LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(")");
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn delete_user_msgs(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    delete_in(pool, "UPDATE phpyun_msg SET del_status = 1 WHERE id IN (", ids).await
}

// ---------- member logs ----------

pub async fn list_member_logs(
    pool: &MySqlPool,
    usertype: Option<i32>,
    uid: Option<u64>,
    offset: u64,
    limit: u64,
) -> Result<Vec<MemberLogRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(id AS UNSIGNED) AS id, CAST(COALESCE(uid,0) AS UNSIGNED) AS uid, \
         CAST(COALESCE(opera,0) AS SIGNED) AS opera, CAST(COALESCE(`type`,0) AS SIGNED) AS `type`, \
         CAST(COALESCE(usertype,0) AS SIGNED) AS usertype, COALESCE(content,'') AS content, \
         COALESCE(ip,'') AS ip, CAST(COALESCE(ctime,0) AS SIGNED) AS ctime \
         FROM phpyun_member_log WHERE 1=1",
    );
    if let Some(u) = usertype {
        qb.push(" AND usertype = ");
        qb.push_bind(u);
    }
    if let Some(id) = uid.filter(|v| *v > 0) {
        qb.push(" AND uid = ");
        qb.push_bind(id);
    }
    qb.push(" ORDER BY id DESC LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_member_logs(
    pool: &MySqlPool,
    usertype: Option<i32>,
    uid: Option<u64>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_member_log WHERE 1=1");
    if let Some(u) = usertype {
        qb.push(" AND usertype = ");
        qb.push_bind(u);
    }
    if let Some(id) = uid.filter(|v| *v > 0) {
        qb.push(" AND uid = ");
        qb.push_bind(id);
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

// ---------- company logo / shows / content ----------

pub async fn list_company_photos(
    pool: &MySqlPool,
    status: Option<i32>,
    keyword: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<CompanyPhotoRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(uid AS UNSIGNED) AS uid, COALESCE(name,'') AS name, \
         COALESCE(logo,'') AS logo, CAST(COALESCE(logo_status,0) AS SIGNED) AS logo_status \
         FROM phpyun_company WHERE logo <> ''",
    );
    if let Some(s) = status {
        qb.push(" AND logo_status = ");
        qb.push_bind(s);
    }
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND (name LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR uid = ");
        qb.push_bind(kw.parse::<u64>().unwrap_or(0));
        qb.push(")");
    }
    qb.push(" ORDER BY logo_status DESC, uid DESC LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_company_photos(
    pool: &MySqlPool,
    status: Option<i32>,
    keyword: Option<&str>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_company WHERE logo <> ''");
    if let Some(s) = status {
        qb.push(" AND logo_status = ");
        qb.push_bind(s);
    }
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND (name LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR uid = ");
        qb.push_bind(kw.parse::<u64>().unwrap_or(0));
        qb.push(")");
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn set_logo_status(pool: &MySqlPool, uid: u64, status: i32) -> Result<u64, sqlx::Error> {
    Ok(
        sqlx::query("UPDATE phpyun_company SET logo_status = ? WHERE uid = ?")
            .bind(status)
            .bind(uid)
            .execute(pool)
            .await?
            .rows_affected(),
    )
}

fn gallery_table(kind: &str) -> &'static str {
    if kind == "resume" {
        "phpyun_resume_show"
    } else {
        "phpyun_company_show"
    }
}

pub async fn list_gallery(
    pool: &MySqlPool,
    kind: &str,
    status: Option<i32>,
    offset: u64,
    limit: u64,
) -> Result<Vec<GalleryAdminRow>, sqlx::Error> {
    let table = gallery_table(kind);
    let (l, o) = lim(limit, offset)?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(format!(
        "SELECT CAST(id AS UNSIGNED) AS id, CAST(COALESCE(uid,0) AS UNSIGNED) AS uid, \
         COALESCE(title,'') AS title, COALESCE(picurl,'') AS picurl, \
         CAST(COALESCE(status,0) AS SIGNED) AS status, CAST(COALESCE(sort,0) AS SIGNED) AS sort \
         FROM {table} WHERE status != 2"
    ));
    if let Some(s) = status {
        qb.push(" AND status = ");
        qb.push_bind(s);
    }
    qb.push(" ORDER BY id DESC LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_gallery(
    pool: &MySqlPool,
    kind: &str,
    status: Option<i32>,
) -> Result<u64, sqlx::Error> {
    let table = gallery_table(kind);
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new(format!("SELECT COUNT(*) FROM {table} WHERE status != 2"));
    if let Some(s) = status {
        qb.push(" AND status = ");
        qb.push_bind(s);
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn set_gallery_status(
    pool: &MySqlPool,
    kind: &str,
    ids: &[u64],
    status: i32,
) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let table = gallery_table(kind);
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new(format!("UPDATE {table} SET status = "));
    qb.push_bind(status);
    qb.push(" WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

fn content_table(kind: &str) -> &'static str {
    if kind == "product" {
        "phpyun_company_product"
    } else {
        "phpyun_company_news"
    }
}

pub async fn list_company_content(
    pool: &MySqlPool,
    kind: &str,
    status: Option<i32>,
    offset: u64,
    limit: u64,
) -> Result<Vec<CompanyContentAdminRow>, sqlx::Error> {
    let table = content_table(kind);
    let (l, o) = lim(limit, offset)?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(format!(
        "SELECT CAST(id AS UNSIGNED) AS id, CAST(COALESCE(uid,0) AS UNSIGNED) AS uid, \
         COALESCE(title,'') AS title, CAST(COALESCE(status,0) AS SIGNED) AS status, \
         COALESCE(statusbody,'') AS statusbody, CAST(COALESCE(ctime,0) AS SIGNED) AS ctime \
         FROM {table} WHERE 1=1"
    ));
    if let Some(s) = status {
        qb.push(" AND status = ");
        qb.push_bind(s);
    }
    qb.push(" ORDER BY id DESC LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_company_content(
    pool: &MySqlPool,
    kind: &str,
    status: Option<i32>,
) -> Result<u64, sqlx::Error> {
    let table = content_table(kind);
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new(format!("SELECT COUNT(*) FROM {table} WHERE 1=1"));
    if let Some(s) = status {
        qb.push(" AND status = ");
        qb.push_bind(s);
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn set_company_content_status(
    pool: &MySqlPool,
    kind: &str,
    ids: &[u64],
    status: i32,
    statusbody: &str,
) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let table = content_table(kind);
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new(format!("UPDATE {table} SET status = "));
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

pub async fn list_interviews(
    pool: &MySqlPool,
    keyword: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<InterviewAdminRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(id AS UNSIGNED) AS id, CAST(COALESCE(uid,0) AS UNSIGNED) AS uid, \
         COALESCE(title,'') AS title, COALESCE(fname,'') AS fname, \
         COALESCE(jobname,'') AS jobname, COALESCE(content,'') AS content, \
         CAST(COALESCE(datetime,0) AS SIGNED) AS datetime, \
         CAST(COALESCE(is_browse,0) AS SIGNED) AS is_browse \
         FROM phpyun_userid_msg WHERE COALESCE(isdel,0) = 0",
    );
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND (title LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR jobname LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR fname LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(")");
    }
    qb.push(" ORDER BY id DESC LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_interviews(pool: &MySqlPool, keyword: Option<&str>) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_userid_msg WHERE COALESCE(isdel,0) = 0");
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND (title LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR jobname LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR fname LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(")");
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn list_company_statis(
    pool: &MySqlPool,
    keyword: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<CompanyStatisAdminRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(s.uid AS UNSIGNED) AS uid, COALESCE(c.name,'') AS com_name, \
         CAST(COALESCE(s.rating,0) AS SIGNED) AS rating, COALESCE(s.rating_name,'') AS rating_name, \
         COALESCE(s.integral,'') AS integral, CAST(COALESCE(s.vip_stime,0) AS SIGNED) AS vip_stime, \
         CAST(COALESCE(s.vip_etime,0) AS SIGNED) AS vip_etime \
         FROM phpyun_company_statis s LEFT JOIN phpyun_company c ON c.uid = s.uid WHERE 1=1",
    );
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND c.name LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    qb.push(" ORDER BY s.uid DESC LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_company_statis(pool: &MySqlPool, keyword: Option<&str>) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT COUNT(*) FROM phpyun_company_statis s LEFT JOIN phpyun_company c ON c.uid = s.uid WHERE 1=1",
    );
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND c.name LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn save_company_statis(
    pool: &MySqlPool,
    uid: u64,
    rating: i32,
    rating_name: &str,
    integral: &str,
    vip_stime: i64,
    vip_etime: i64,
) -> Result<u64, sqlx::Error> {
    Ok(sqlx::query(
        "UPDATE phpyun_company_statis SET rating=?, rating_name=?, integral=?, vip_stime=?, vip_etime=? WHERE uid=?",
    )
    .bind(rating)
    .bind(rating_name)
    .bind(integral)
    .bind(vip_stime)
    .bind(vip_etime)
    .bind(uid)
    .execute(pool)
    .await?
    .rows_affected())
}

pub async fn list_refresh_logs(
    pool: &MySqlPool,
    r#type: Option<i32>,
    uid: Option<u64>,
    offset: u64,
    limit: u64,
) -> Result<Vec<JobRefreshLogRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(id AS UNSIGNED) AS id, CAST(COALESCE(uid,0) AS UNSIGNED) AS uid, \
         CAST(COALESCE(jobid,0) AS UNSIGNED) AS jobid, CAST(COALESCE(usertype,0) AS SIGNED) AS usertype, \
         CAST(COALESCE(`type`,0) AS SIGNED) AS `type`, COALESCE(r_time,'') AS r_time, \
         COALESCE(ip,'') AS ip, COALESCE(remark,'') AS remark \
         FROM phpyun_job_refresh_log WHERE 1=1",
    );
    if let Some(t) = r#type {
        qb.push(" AND `type` = ");
        qb.push_bind(t);
    }
    if let Some(id) = uid.filter(|v| *v > 0) {
        qb.push(" AND uid = ");
        qb.push_bind(id);
    }
    qb.push(" ORDER BY id DESC LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_refresh_logs(
    pool: &MySqlPool,
    r#type: Option<i32>,
    uid: Option<u64>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_job_refresh_log WHERE 1=1");
    if let Some(t) = r#type {
        qb.push(" AND `type` = ");
        qb.push_bind(t);
    }
    if let Some(id) = uid.filter(|v| *v > 0) {
        qb.push(" AND uid = ");
        qb.push_bind(id);
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

// ---------- keywords / cron / error / sysmsg / navmap / domain ----------

const HOTKEY_FIELDS: &str = "CAST(id AS UNSIGNED) AS id, COALESCE(key_name,'') AS key_name, \
    CAST(COALESCE(num,0) AS SIGNED) AS num, CAST(COALESCE(`type`,0) AS SIGNED) AS `type`, \
    CAST(COALESCE(`check`,0) AS SIGNED) AS `check`, CAST(COALESCE(bold,0) AS SIGNED) AS bold, \
    CAST(COALESCE(tuijian,0) AS SIGNED) AS tuijian, COALESCE(color,'') AS color, COALESCE(size,'') AS size";

pub async fn list_hot_keys(
    pool: &MySqlPool,
    r#type: Option<i32>,
    keyword: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<HotKeyAdminRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(HOTKEY_FIELDS);
    qb.push(" FROM phpyun_hot_key WHERE 1=1");
    if let Some(t) = r#type.filter(|v| *v > 0) {
        qb.push(" AND `type` = ");
        qb.push_bind(t);
    }
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND key_name LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    qb.push(" ORDER BY id DESC LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_hot_keys(
    pool: &MySqlPool,
    r#type: Option<i32>,
    keyword: Option<&str>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_hot_key WHERE 1=1");
    if let Some(t) = r#type.filter(|v| *v > 0) {
        qb.push(" AND `type` = ");
        qb.push_bind(t);
    }
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND key_name LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn upsert_hot_key(
    pool: &MySqlPool,
    id: Option<u64>,
    key_name: &str,
    r#type: i32,
    check: i32,
    bold: i32,
    tuijian: i32,
    color: &str,
    size: &str,
) -> Result<u64, sqlx::Error> {
    if let Some(id) = id.filter(|v| *v > 0) {
        sqlx::query(
            "UPDATE phpyun_hot_key SET key_name=?, `type`=?, `check`=?, bold=?, tuijian=?, color=?, size=? WHERE id=?",
        )
        .bind(key_name)
        .bind(r#type)
        .bind(check)
        .bind(bold)
        .bind(tuijian)
        .bind(color)
        .bind(size)
        .bind(id)
        .execute(pool)
        .await?;
        return Ok(id);
    }
    Ok(sqlx::query(
        "INSERT INTO phpyun_hot_key (key_name, num, `type`, `check`, bold, tuijian, color, size) \
         VALUES (?, 0, ?, ?, ?, ?, ?, ?)",
    )
    .bind(key_name)
    .bind(r#type)
    .bind(check)
    .bind(bold)
    .bind(tuijian)
    .bind(color)
    .bind(size)
    .execute(pool)
    .await?
    .last_insert_id())
}

pub async fn delete_hot_keys(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    delete_in(pool, "DELETE FROM phpyun_hot_key WHERE id IN (", ids).await
}

const CRON_FIELDS: &str = "CAST(id AS UNSIGNED) AS id, COALESCE(name,'') AS name, COALESCE(dir,'') AS dir, \
    CAST(COALESCE(`type`,0) AS SIGNED) AS `type`, CAST(COALESCE(week,0) AS SIGNED) AS week, \
    CAST(COALESCE(month,0) AS SIGNED) AS month, CAST(COALESCE(hour,0) AS SIGNED) AS hour, \
    CAST(COALESCE(minute,0) AS SIGNED) AS minute, CAST(COALESCE(display,0) AS SIGNED) AS display, \
    CAST(COALESCE(nowtime,0) AS SIGNED) AS nowtime, CAST(COALESCE(nexttime,0) AS SIGNED) AS nexttime";

pub async fn list_cron(pool: &MySqlPool, offset: u64, limit: u64) -> Result<Vec<CronRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    let sql = format!("SELECT {CRON_FIELDS} FROM phpyun_cron ORDER BY id DESC LIMIT ? OFFSET ?");
    sqlx::query_as::<_, CronRow>(&sql)
        .bind(l)
        .bind(o)
        .fetch_all(pool)
        .await
}

pub async fn count_cron(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_cron")
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn upsert_cron(
    pool: &MySqlPool,
    id: Option<u64>,
    name: &str,
    dir: &str,
    r#type: i32,
    week: i32,
    month: i32,
    hour: i32,
    minute: i32,
    display: i32,
    now: i64,
) -> Result<u64, sqlx::Error> {
    if let Some(id) = id.filter(|v| *v > 0) {
        sqlx::query(
            "UPDATE phpyun_cron SET name=?, dir=?, `type`=?, week=?, month=?, hour=?, minute=?, display=? WHERE id=?",
        )
        .bind(name)
        .bind(dir)
        .bind(r#type)
        .bind(week)
        .bind(month)
        .bind(hour)
        .bind(minute)
        .bind(display)
        .bind(id)
        .execute(pool)
        .await?;
        return Ok(id);
    }
    Ok(sqlx::query(
        "INSERT INTO phpyun_cron (name, dir, `type`, week, month, hour, minute, display, ctime) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(name)
    .bind(dir)
    .bind(r#type)
    .bind(week)
    .bind(month)
    .bind(hour)
    .bind(minute)
    .bind(display)
    .bind(now)
    .execute(pool)
    .await?
    .last_insert_id())
}

pub async fn delete_cron(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    delete_in(pool, "DELETE FROM phpyun_cron WHERE id IN (", ids).await
}

pub async fn list_error_logs(
    pool: &MySqlPool,
    keyword: Option<&str>,
    logtype: Option<i32>,
    offset: u64,
    limit: u64,
) -> Result<Vec<ErrorLogRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(id AS UNSIGNED) AS id, CAST(COALESCE(uid,0) AS SIGNED) AS uid, \
         CAST(COALESCE(`type`,0) AS SIGNED) AS `type`, COALESCE(content,'') AS content, \
         CAST(COALESCE(ctime,0) AS SIGNED) AS ctime, CAST(COALESCE(isread,0) AS SIGNED) AS isread \
         FROM phpyun_error_log WHERE 1=1",
    );
    if let Some(t) = logtype.filter(|v| *v > 0) {
        qb.push(" AND `type` = ");
        qb.push_bind(t);
    }
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND (content LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR uid = ");
        qb.push_bind(kw.parse::<i64>().unwrap_or(0));
        qb.push(")");
    }
    qb.push(" ORDER BY id DESC LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_error_logs(
    pool: &MySqlPool,
    keyword: Option<&str>,
    logtype: Option<i32>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_error_log WHERE 1=1");
    if let Some(t) = logtype.filter(|v| *v > 0) {
        qb.push(" AND `type` = ");
        qb.push_bind(t);
    }
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND (content LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR uid = ");
        qb.push_bind(kw.parse::<i64>().unwrap_or(0));
        qb.push(")");
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn delete_error_logs(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(sqlx::query("DELETE FROM phpyun_error_log")
            .execute(pool)
            .await?
            .rows_affected());
    }
    delete_in(pool, "DELETE FROM phpyun_error_log WHERE id IN (", ids).await
}

pub async fn list_sysmsgs(
    pool: &MySqlPool,
    keyword: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<SysmsgAdminRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(id AS UNSIGNED) AS id, CAST(COALESCE(fa_uid,0) AS UNSIGNED) AS fa_uid, \
         COALESCE(username,'') AS username, COALESCE(content,'') AS content, \
         CAST(COALESCE(usertype,0) AS SIGNED) AS usertype, CAST(COALESCE(ctime,0) AS SIGNED) AS ctime \
         FROM phpyun_sysmsg WHERE 1=1",
    );
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND (username LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR content LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(")");
    }
    qb.push(" ORDER BY id DESC LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_sysmsgs(pool: &MySqlPool, keyword: Option<&str>) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_sysmsg WHERE 1=1");
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND (username LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR content LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(")");
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn insert_sysmsg(
    pool: &MySqlPool,
    fa_uid: u64,
    usertype: i32,
    content: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    Ok(sqlx::query(
        "INSERT INTO phpyun_sysmsg (fa_uid, usertype, content, remind_status, ctime) VALUES (?, ?, ?, 1, ?)",
    )
    .bind(fa_uid)
    .bind(usertype)
    .bind(content)
    .bind(now)
    .execute(pool)
    .await?
    .last_insert_id())
}

pub async fn list_member_uids_by_usertype(
    pool: &MySqlPool,
    usertype: i32,
    offset: u64,
    limit: u64,
) -> Result<Vec<u64>, sqlx::Error> {
    let rows: Vec<(u64,)> = sqlx::query_as(
        "SELECT CAST(uid AS UNSIGNED) FROM phpyun_member WHERE usertype = ? ORDER BY uid ASC LIMIT ? OFFSET ?",
    )
    .bind(usertype)
    .bind(phpyun_core::numeric::checked_db_i64(limit, "pagination.limit")?)
    .bind(phpyun_core::numeric::checked_db_i64(offset, "pagination.offset")?)
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|(u,)| u).collect())
}

const NAVMAP_FIELDS: &str = "CAST(id AS UNSIGNED) AS id, CAST(COALESCE(nid,0) AS SIGNED) AS nid, \
    COALESCE(name,'') AS name, COALESCE(url,'') AS url, CAST(COALESCE(sort,0) AS SIGNED) AS sort, \
    CAST(COALESCE(display,0) AS SIGNED) AS display, CAST(COALESCE(eject,0) AS SIGNED) AS eject, \
    CAST(COALESCE(`type`,0) AS SIGNED) AS `type`, COALESCE(furl,'') AS furl";

pub async fn list_navmap(
    pool: &MySqlPool,
    keyword: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<NavmapRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(NAVMAP_FIELDS);
    qb.push(" FROM phpyun_navmap WHERE 1=1");
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND (name LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR url LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(")");
    }
    qb.push(" ORDER BY sort ASC, id ASC LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_navmap(pool: &MySqlPool, keyword: Option<&str>) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_navmap WHERE 1=1");
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND (name LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR url LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(")");
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn upsert_navmap(
    pool: &MySqlPool,
    id: Option<u64>,
    nid: i32,
    name: &str,
    url: &str,
    sort: i32,
    display: i32,
    eject: i32,
    r#type: i32,
    furl: &str,
) -> Result<u64, sqlx::Error> {
    if let Some(id) = id.filter(|v| *v > 0) {
        sqlx::query(
            "UPDATE phpyun_navmap SET nid=?, name=?, url=?, sort=?, display=?, eject=?, `type`=?, furl=? WHERE id=?",
        )
        .bind(nid)
        .bind(name)
        .bind(url)
        .bind(sort)
        .bind(display)
        .bind(eject)
        .bind(r#type)
        .bind(furl)
        .bind(id)
        .execute(pool)
        .await?;
        return Ok(id);
    }
    Ok(sqlx::query(
        "INSERT INTO phpyun_navmap (nid, name, url, sort, display, eject, `type`, furl) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(nid)
    .bind(name)
    .bind(url)
    .bind(sort)
    .bind(display)
    .bind(eject)
    .bind(r#type)
    .bind(furl)
    .execute(pool)
    .await?
    .last_insert_id())
}

pub async fn delete_navmap(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    delete_in(pool, "DELETE FROM phpyun_navmap WHERE id IN (", ids).await
}

pub async fn list_domains(
    pool: &MySqlPool,
    keyword: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<DomainAdminRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(title,'') AS title, COALESCE(domain,'') AS domain, \
         CAST(COALESCE(fz_type,0) AS SIGNED) AS fz_type, CAST(COALESCE(mode,0) AS SIGNED) AS mode, \
         COALESCE(webtitle,'') AS web_title, COALESCE(indexdir,'') AS indexdir, \
         COALESCE(style,'') AS style, CAST(COALESCE(hy,0) AS SIGNED) AS hy, \
         CAST(COALESCE(cityid,0) AS SIGNED) AS cityid, CAST(COALESCE(province,0) AS SIGNED) AS province, \
         COALESCE(tpl,'') AS tpl \
         FROM phpyun_domain WHERE 1=1",
    );
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND (title LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR domain LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(")");
    }
    qb.push(" ORDER BY id ASC LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_domains(pool: &MySqlPool, keyword: Option<&str>) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_domain WHERE 1=1");
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND (title LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR domain LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(")");
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn upsert_domain(
    pool: &MySqlPool,
    id: Option<u64>,
    title: &str,
    domain: &str,
    fz_type: i32,
    mode: i32,
    web_title: &str,
    indexdir: &str,
) -> Result<u64, sqlx::Error> {
    if let Some(id) = id.filter(|v| *v > 0) {
        sqlx::query(
            "UPDATE phpyun_domain SET title=?, domain=?, fz_type=?, mode=?, webtitle=?, indexdir=? WHERE id=?",
        )
        .bind(title)
        .bind(domain)
        .bind(fz_type)
        .bind(mode)
        .bind(web_title)
        .bind(indexdir)
        .bind(id)
        .execute(pool)
        .await?;
        return Ok(id);
    }
    Ok(sqlx::query(
        "INSERT INTO phpyun_domain (title, domain, fz_type, mode, webtitle, indexdir) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(title)
    .bind(domain)
    .bind(fz_type)
    .bind(mode)
    .bind(web_title)
    .bind(indexdir)
    .execute(pool)
    .await?
    .last_insert_id())
}

pub async fn delete_domains(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    delete_in(pool, "DELETE FROM phpyun_domain WHERE id IN (", ids).await
}

pub async fn list_domain_admins(
    pool: &MySqlPool,
    keyword: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<DomainAdminUserRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(uid AS UNSIGNED) AS uid, COALESCE(username,'') AS username, \
         COALESCE(name,'') AS name, CAST(COALESCE(m_id,0) AS SIGNED) AS m_id, \
         CAST(COALESCE(did,0) AS UNSIGNED) AS did, CAST(COALESCE(status,0) AS SIGNED) AS status \
         FROM phpyun_admin_user WHERE did > 0",
    );
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND (username LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR name LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(")");
    }
    qb.push(" ORDER BY uid ASC LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_domain_admins(pool: &MySqlPool, keyword: Option<&str>) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_admin_user WHERE did > 0");
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND (username LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR name LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(")");
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

// ---------- special companies / wx / outside / hr log / marketing ----------

pub async fn list_special_coms(
    pool: &MySqlPool,
    sid: Option<u64>,
    offset: u64,
    limit: u64,
) -> Result<Vec<SpecialComAdminRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(sc.id AS UNSIGNED) AS id, CAST(COALESCE(sc.sid,0) AS UNSIGNED) AS sid, \
         CAST(COALESCE(sc.uid,0) AS UNSIGNED) AS uid, CAST(COALESCE(sc.integral,0) AS SIGNED) AS integral, \
         CAST(COALESCE(sc.status,0) AS SIGNED) AS status, COALESCE(sc.statusbody,'') AS statusbody, \
         CAST(COALESCE(sc.sort,0) AS SIGNED) AS sort, CAST(COALESCE(sc.famous,0) AS SIGNED) AS famous, \
         CAST(COALESCE(sc.`time`,0) AS SIGNED) AS created_at, \
         COALESCE(c.name,'') AS name \
         FROM phpyun_special_com sc \
         LEFT JOIN phpyun_company c ON c.uid = sc.uid \
         WHERE 1=1",
    );
    if let Some(s) = sid.filter(|v| *v > 0) {
        qb.push(" AND sc.sid = ");
        qb.push_bind(s);
    }
    qb.push(" ORDER BY sc.id DESC LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_special_coms(pool: &MySqlPool, sid: Option<u64>) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_special_com WHERE 1=1");
    if let Some(s) = sid.filter(|v| *v > 0) {
        qb.push(" AND sid = ");
        qb.push_bind(s);
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn set_special_com_status(
    pool: &MySqlPool,
    id: u64,
    status: i32,
    statusbody: &str,
) -> Result<u64, sqlx::Error> {
    Ok(sqlx::query("UPDATE phpyun_special_com SET status=?, statusbody=? WHERE id=?")
        .bind(status)
        .bind(statusbody)
        .bind(id)
        .execute(pool)
        .await?
        .rows_affected())
}

pub async fn list_wxqrcodes(
    pool: &MySqlPool,
    status: Option<i32>,
    keyword: Option<&str>,
    offset: u64,
    limit: u64,
) -> Result<Vec<WxQrcodeRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(wxloginid,'') AS wxloginid, \
         COALESCE(ticket,'') AS ticket, CAST(COALESCE(`time`,0) AS SIGNED) AS time, \
         CAST(COALESCE(status,0) AS SIGNED) AS status, COALESCE(wxid,'') AS wxid, \
         CAST(COALESCE(uid,0) AS UNSIGNED) AS uid \
         FROM phpyun_wxqrcode WHERE 1=1",
    );
    if let Some(s) = status {
        qb.push(" AND status = ");
        qb.push_bind(s);
    }
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND (wxloginid LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR wxid LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(")");
    }
    qb.push(" ORDER BY time DESC LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_wxqrcodes(
    pool: &MySqlPool,
    status: Option<i32>,
    keyword: Option<&str>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_wxqrcode WHERE 1=1");
    if let Some(s) = status {
        qb.push(" AND status = ");
        qb.push_bind(s);
    }
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND (wxloginid LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(" OR wxid LIKE ");
        qb.push_bind(format!("%{kw}%"));
        qb.push(")");
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn list_wxpub_temps(
    pool: &MySqlPool,
    keyword: Option<&str>,
    temptype: Option<i32>,
    offset: u64,
    limit: u64,
) -> Result<Vec<WxpubTempRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(title,'') AS title, COALESCE(header,'') AS header, \
         COALESCE(body,'') AS body, COALESCE(footer,'') AS footer, COALESCE(`type`,'') AS `type`, \
         CAST(COALESCE(temptype,0) AS SIGNED) AS temptype, CAST(COALESCE(time,0) AS SIGNED) AS time \
         FROM phpyun_wxpub_temps WHERE 1=1",
    );
    if let Some(t) = temptype {
        qb.push(" AND temptype = ");
        qb.push_bind(t);
    }
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND title LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    qb.push(" ORDER BY id DESC LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_wxpub_temps(
    pool: &MySqlPool,
    keyword: Option<&str>,
    temptype: Option<i32>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_wxpub_temps WHERE 1=1");
    if let Some(t) = temptype {
        qb.push(" AND temptype = ");
        qb.push_bind(t);
    }
    if let Some(kw) = keyword.map(str::trim).filter(|s| !s.is_empty()) {
        qb.push(" AND title LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn upsert_wxpub_temp(
    pool: &MySqlPool,
    id: Option<u64>,
    title: &str,
    header: &str,
    body: &str,
    footer: &str,
    r#type: &str,
    temptype: i32,
    now: i64,
) -> Result<u64, sqlx::Error> {
    if let Some(id) = id.filter(|v| *v > 0) {
        sqlx::query(
            "UPDATE phpyun_wxpub_temps SET title=?, header=?, body=?, footer=?, `type`=?, temptype=? WHERE id=?",
        )
        .bind(title)
        .bind(header)
        .bind(body)
        .bind(footer)
        .bind(r#type)
        .bind(temptype)
        .bind(id)
        .execute(pool)
        .await?;
        return Ok(id);
    }
    Ok(sqlx::query(
        "INSERT INTO phpyun_wxpub_temps (title, header, body, footer, `type`, temptype, time) \
         VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(title)
    .bind(header)
    .bind(body)
    .bind(footer)
    .bind(r#type)
    .bind(temptype)
    .bind(now)
    .execute(pool)
    .await?
    .last_insert_id())
}

pub async fn delete_wxpub_temps(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    delete_in(pool, "DELETE FROM phpyun_wxpub_temps WHERE id IN (", ids).await
}

pub async fn list_outside(
    pool: &MySqlPool,
    offset: u64,
    limit: u64,
) -> Result<Vec<OutsideRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    sqlx::query_as::<_, OutsideRow>(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(name,'') AS name, COALESCE(`type`,'') AS `type`, \
         CAST(COALESCE(titlelen,0) AS SIGNED) AS titlelen, CAST(COALESCE(infolen,0) AS SIGNED) AS infolen, \
         CAST(COALESCE(num,0) AS SIGNED) AS num, COALESCE(code,'') AS code, \
         CAST(COALESCE(lasttime,0) AS SIGNED) AS lasttime \
         FROM phpyun_outside ORDER BY id DESC LIMIT ? OFFSET ?",
    )
    .bind(l)
    .bind(o)
    .fetch_all(pool)
    .await
}

pub async fn count_outside(pool: &MySqlPool) -> Result<u64, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM phpyun_outside")
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn upsert_outside(
    pool: &MySqlPool,
    id: Option<u64>,
    name: &str,
    r#type: &str,
    titlelen: i32,
    infolen: i32,
    num: i32,
    code: &str,
    now: i64,
) -> Result<u64, sqlx::Error> {
    if let Some(id) = id.filter(|v| *v > 0) {
        sqlx::query(
            "UPDATE phpyun_outside SET name=?, `type`=?, titlelen=?, infolen=?, num=?, code=?, lasttime=? WHERE id=?",
        )
        .bind(name)
        .bind(r#type)
        .bind(titlelen)
        .bind(infolen)
        .bind(num)
        .bind(code)
        .bind(now)
        .bind(id)
        .execute(pool)
        .await?;
        return Ok(id);
    }
    Ok(sqlx::query(
        "INSERT INTO phpyun_outside (name, `type`, titlelen, infolen, num, code, lasttime) \
         VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(name)
    .bind(r#type)
    .bind(titlelen)
    .bind(infolen)
    .bind(num)
    .bind(code)
    .bind(now)
    .execute(pool)
    .await?
    .last_insert_id())
}

pub async fn delete_outside(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    delete_in(pool, "DELETE FROM phpyun_outside WHERE id IN (", ids).await
}

pub async fn list_hr_logs(
    pool: &MySqlPool,
    uid: Option<u64>,
    offset: u64,
    limit: u64,
) -> Result<Vec<HrLogRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(id AS UNSIGNED) AS id, CAST(COALESCE(uid,0) AS UNSIGNED) AS uid, \
         CAST(COALESCE(job,0) AS SIGNED) AS job, CAST(COALESCE(lookjob,0) AS SIGNED) AS lookjob, \
         CAST(COALESCE(lookresume,0) AS SIGNED) AS lookresume, CAST(COALESCE(sqjob,0) AS SIGNED) AS sqjob, \
         CAST(COALESCE(yq,0) AS SIGNED) AS yq, CAST(COALESCE(login,0) AS SIGNED) AS login, \
         CAST(COALESCE(ctime,0) AS SIGNED) AS ctime, CAST(COALESCE(uptime,0) AS SIGNED) AS uptime \
         FROM phpyun_hr_log WHERE 1=1",
    );
    if let Some(id) = uid.filter(|v| *v > 0) {
        qb.push(" AND uid = ");
        qb.push_bind(id);
    }
    qb.push(" ORDER BY id DESC LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_hr_logs(pool: &MySqlPool, uid: Option<u64>) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_hr_log WHERE 1=1");
    if let Some(id) = uid.filter(|v| *v > 0) {
        qb.push(" AND uid = ");
        qb.push_bind(id);
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn last_email_msgs(pool: &MySqlPool, limit: u64) -> Result<Vec<LastMsgAt>, sqlx::Error> {
    sqlx::query_as::<_, LastMsgAt>(
        "SELECT COALESCE(title,'') AS title, CAST(COALESCE(ctime,0) AS SIGNED) AS ctime \
         FROM phpyun_email_msg WHERE del = 0 ORDER BY id DESC LIMIT ?",
    )
    .bind(phpyun_core::numeric::checked_db_i64(limit, "pagination.limit")?)
    .fetch_all(pool)
    .await
}

pub async fn last_sms_msgs(pool: &MySqlPool, limit: u64) -> Result<Vec<LastMsgAt>, sqlx::Error> {
    sqlx::query_as::<_, LastMsgAt>(
        "SELECT COALESCE(content,'') AS title, CAST(COALESCE(ctime,0) AS SIGNED) AS ctime \
         FROM phpyun_moblie_msg WHERE del = 0 ORDER BY id DESC LIMIT ?",
    )
    .bind(phpyun_core::numeric::checked_db_i64(limit, "pagination.limit")?)
    .fetch_all(pool)
    .await
}

pub async fn insert_email_log(
    pool: &MySqlPool,
    uid: u64,
    email: &str,
    title: &str,
    content: &str,
    now: i64,
    state: i32,
) -> Result<u64, sqlx::Error> {
    Ok(sqlx::query(
        "INSERT INTO phpyun_email_msg (uid, email, title, content, ctime, state, del) VALUES (?, ?, ?, ?, ?, ?, 0)",
    )
    .bind(uid)
    .bind(email)
    .bind(title)
    .bind(content)
    .bind(now)
    .bind(state)
    .execute(pool)
    .await?
    .last_insert_id())
}

pub async fn insert_sms_log(
    pool: &MySqlPool,
    uid: u64,
    mobile: &str,
    content: &str,
    now: i64,
    state: i32,
) -> Result<u64, sqlx::Error> {
    Ok(sqlx::query(
        "INSERT INTO phpyun_moblie_msg (uid, moblie, content, ctime, state, del) VALUES (?, ?, ?, ?, ?, 0)",
    )
    .bind(uid)
    .bind(mobile)
    .bind(content)
    .bind(now)
    .bind(state)
    .execute(pool)
    .await?
    .last_insert_id())
}

pub async fn list_company_tpls(
    pool: &MySqlPool,
) -> Result<Vec<(u64, String, String, i32)>, sqlx::Error> {
    sqlx::query_as(
        "SELECT CAST(id AS UNSIGNED), COALESCE(name,''), COALESCE(url,''), CAST(COALESCE(status,0) AS SIGNED) \
         FROM phpyun_company_tpl ORDER BY id DESC",
    )
    .fetch_all(pool)
    .await
}

pub async fn find_members_by_usernames(
    pool: &MySqlPool,
    names: &[String],
) -> Result<Vec<(u64, i32, String)>, sqlx::Error> {
    if names.is_empty() {
        return Ok(vec![]);
    }
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(uid AS UNSIGNED), CAST(COALESCE(usertype,0) AS SIGNED), COALESCE(username,'') \
         FROM phpyun_member WHERE username IN (",
    );
    let mut sep = qb.separated(", ");
    for n in names {
        sep.push_bind(n);
    }
    qb.push(")");
    qb.build_query_as().fetch_all(pool).await
}

pub async fn list_member_emails(
    pool: &MySqlPool,
    usertype: i32,
    limit: i64,
) -> Result<Vec<String>, sqlx::Error> {
    sqlx::query_scalar(
        "SELECT COALESCE(email,'') FROM phpyun_member WHERE usertype = ? AND email IS NOT NULL AND email <> '' LIMIT ?",
    )
    .bind(usertype)
    .bind(limit)
    .fetch_all(pool)
    .await
}

pub async fn list_member_mobiles(
    pool: &MySqlPool,
    usertype: i32,
    limit: i64,
) -> Result<Vec<String>, sqlx::Error> {
    sqlx::query_scalar(
        "SELECT COALESCE(moblie,'') FROM phpyun_member WHERE usertype = ? AND moblie IS NOT NULL AND moblie <> '' LIMIT ?",
    )
    .bind(usertype)
    .bind(limit)
    .fetch_all(pool)
    .await
}

const RATING_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, COALESCE(name,'') AS name, \
    COALESCE(service_price,'') AS service_price, COALESCE(integral_buy,'') AS integral_buy, \
    COALESCE(yh_price,'') AS yh_price, COALESCE(yh_integral,'') AS yh_integral, \
    CAST(COALESCE(time_start,0) AS SIGNED) AS time_start, CAST(COALESCE(time_end,0) AS SIGNED) AS time_end, \
    CAST(COALESCE(resume,0) AS SIGNED) AS resume, CAST(COALESCE(job_num,0) AS SIGNED) AS job_num, \
    CAST(COALESCE(interview,0) AS SIGNED) AS interview, CAST(COALESCE(editjob_num,0) AS SIGNED) AS editjob_num, \
    CAST(COALESCE(breakjob_num,0) AS SIGNED) AS breakjob_num, CAST(COALESCE(sort,0) AS SIGNED) AS sort, \
    CAST(COALESCE(display,0) AS SIGNED) AS display, COALESCE(explains,'') AS explains, \
    COALESCE(com_pic,'') AS com_pic, CAST(COALESCE(`type`,0) AS SIGNED) AS `type`, \
    CAST(COALESCE(category,0) AS SIGNED) AS category, CAST(COALESCE(service_time,0) AS SIGNED) AS service_time, \
    CAST(COALESCE(zph_num,0) AS SIGNED) AS zph_num, CAST(COALESCE(service_discount,0) AS SIGNED) AS service_discount, \
    CAST(COALESCE(top_num,0) AS SIGNED) AS top_num, CAST(COALESCE(urgent_num,0) AS SIGNED) AS urgent_num, \
    CAST(COALESCE(rec_num,0) AS SIGNED) AS rec_num, CAST(COALESCE(freelook_num,0) AS SIGNED) AS freelook_num, \
    CAST(COALESCE(freerefresh_num,0) AS SIGNED) AS freerefresh_num, \
    CAST(COALESCE(suspend_num,0) AS SIGNED) AS suspend_num, CAST(COALESCE(max_time,0) AS SIGNED) AS max_time";

pub async fn list_rating_packages(
    pool: &MySqlPool,
    id: Option<u64>,
    offset: u64,
    limit: u64,
) -> Result<Vec<RatingPackageRow>, sqlx::Error> {
    let (l, o) = lim(limit, offset)?;
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(format!(
        "SELECT {RATING_FIELDS} FROM phpyun_company_rating WHERE category = 1"
    ));
    if let Some(id) = id.filter(|v| *v > 0) {
        qb.push(" AND id = ");
        qb.push_bind(id);
    }
    qb.push(" ORDER BY `type` ASC, sort DESC, id ASC LIMIT ");
    qb.push_bind(l);
    qb.push(" OFFSET ");
    qb.push_bind(o);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn count_rating_packages(pool: &MySqlPool, id: Option<u64>) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_company_rating WHERE category = 1");
    if let Some(id) = id.filter(|v| *v > 0) {
        qb.push(" AND id = ");
        qb.push_bind(id);
    }
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn find_rating_package(
    pool: &MySqlPool,
    id: u64,
) -> Result<Option<RatingPackageRow>, sqlx::Error> {
    sqlx::query_as(&format!(
        "SELECT {RATING_FIELDS} FROM phpyun_company_rating WHERE id = ? LIMIT 1"
    ))
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub struct RatingPackageWrite<'a> {
    pub name: &'a str,
    pub service_price: &'a str,
    pub integral_buy: &'a str,
    pub yh_price: &'a str,
    pub yh_integral: &'a str,
    pub time_start: i64,
    pub time_end: i64,
    pub resume: i32,
    pub job_num: i32,
    pub interview: i32,
    pub editjob_num: i32,
    pub breakjob_num: i32,
    pub sort: i32,
    pub display: i32,
    pub explains: &'a str,
    pub com_pic: &'a str,
    pub r#type: i32,
    pub category: i32,
    pub service_time: i32,
    pub zph_num: i32,
    pub service_discount: i32,
    pub top_num: i32,
    pub urgent_num: i32,
    pub rec_num: i32,
    pub freelook_num: i32,
    pub freerefresh_num: i32,
    pub suspend_num: i32,
    pub max_time: i32,
}

pub async fn insert_rating_package(
    pool: &MySqlPool,
    w: RatingPackageWrite<'_>,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_company_rating (\
            name, service_price, integral_buy, yh_price, yh_integral, time_start, time_end, \
            resume, job_num, interview, editjob_num, breakjob_num, sort, display, explains, com_pic, \
            `type`, category, service_time, zph_num, service_discount, top_num, urgent_num, rec_num, \
            freelook_num, freerefresh_num, suspend_num, max_time\
         ) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)",
    )
    .bind(w.name)
    .bind(w.service_price)
    .bind(w.integral_buy)
    .bind(w.yh_price)
    .bind(w.yh_integral)
    .bind(w.time_start)
    .bind(w.time_end)
    .bind(w.resume)
    .bind(w.job_num)
    .bind(w.interview)
    .bind(w.editjob_num)
    .bind(w.breakjob_num)
    .bind(w.sort)
    .bind(w.display)
    .bind(w.explains)
    .bind(w.com_pic)
    .bind(w.r#type)
    .bind(w.category)
    .bind(w.service_time)
    .bind(w.zph_num)
    .bind(w.service_discount)
    .bind(w.top_num)
    .bind(w.urgent_num)
    .bind(w.rec_num)
    .bind(w.freelook_num)
    .bind(w.freerefresh_num)
    .bind(w.suspend_num)
    .bind(w.max_time)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn update_rating_package(
    pool: &MySqlPool,
    id: u64,
    w: RatingPackageWrite<'_>,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_company_rating SET \
            name=?, service_price=?, integral_buy=?, yh_price=?, yh_integral=?, time_start=?, time_end=?, \
            resume=?, job_num=?, interview=?, editjob_num=?, breakjob_num=?, sort=?, display=?, explains=?, com_pic=?, \
            `type`=?, category=?, service_time=?, zph_num=?, service_discount=?, top_num=?, urgent_num=?, rec_num=?, \
            freelook_num=?, freerefresh_num=?, suspend_num=?, max_time=? \
         WHERE id=?",
    )
    .bind(w.name)
    .bind(w.service_price)
    .bind(w.integral_buy)
    .bind(w.yh_price)
    .bind(w.yh_integral)
    .bind(w.time_start)
    .bind(w.time_end)
    .bind(w.resume)
    .bind(w.job_num)
    .bind(w.interview)
    .bind(w.editjob_num)
    .bind(w.breakjob_num)
    .bind(w.sort)
    .bind(w.display)
    .bind(w.explains)
    .bind(w.com_pic)
    .bind(w.r#type)
    .bind(w.category)
    .bind(w.service_time)
    .bind(w.zph_num)
    .bind(w.service_discount)
    .bind(w.top_num)
    .bind(w.urgent_num)
    .bind(w.rec_num)
    .bind(w.freelook_num)
    .bind(w.freerefresh_num)
    .bind(w.suspend_num)
    .bind(w.max_time)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn delete_rating_packages(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    delete_in(pool, "DELETE FROM phpyun_company_rating WHERE category=1 AND id IN (", ids).await
}

pub async fn clear_rating_pic(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_company_rating SET com_pic='' WHERE id=?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
