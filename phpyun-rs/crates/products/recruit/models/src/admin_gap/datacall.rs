//! PHP `datacall.class.php` list queries. Preview never writes plus cache files.

use sqlx::{FromRow, MySqlPool, QueryBuilder, Row};

#[derive(Debug, Clone, FromRow)]
pub struct OutsideCall {
    pub id: u64,
    pub code: String,
    pub r#type: String,
    pub num: i32,
    pub byorder: String,
    pub titlelen: i32,
    pub infolen: i32,
    pub urltype: i32,
    pub timetype: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct ResumeCallRow {
    pub id: u64,
    pub uid: u64,
    pub resumename: String,
    pub lastupdate: i64,
    pub hits: i64,
    pub hy: i32,
    pub job_classid: String,
    pub report: i32,
    pub salary: i32,
    #[sqlx(rename = "type")]
    pub kind: i32,
    pub qw_provinceid: i32,
    pub qw_cityid: i32,
    pub name: String,
    pub birthday: String,
    pub edu: i32,
    pub photo: String,
    pub email: String,
    pub telhome: String,
    pub telphone: String,
    pub exp: i32,
    pub address: String,
    pub description: String,
    pub homepage: String,
    pub idcard: String,
    pub living: String,
    pub domicile: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct CompanyCallRow {
    pub uid: u64,
    pub name: String,
    pub hy: i32,
    pub pr: i32,
    pub provinceid: i32,
    pub cityid: i32,
    pub mun: i32,
    pub sdate: String,
    pub money: String,
    pub address: String,
    pub zip: String,
    pub linkman: String,
    pub linkjob: String,
    pub linkqq: String,
    pub linkphone: String,
    pub linktel: String,
    pub linkmail: String,
    pub website: String,
    pub logo: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct JobCallRow {
    pub id: u64,
    pub uid: u64,
    pub name: String,
    pub com_name: String,
    pub hy: i32,
    pub job1_son: i32,
    pub job_post: i32,
    pub provinceid: i32,
    pub cityid: i32,
    pub minsalary: i32,
    pub maxsalary: i32,
    pub number: i32,
    pub age: i32,
    pub exp: i32,
    pub report: i32,
    pub edu: i32,
    pub lang: String,
    pub welfare: String,
    pub lastupdate: i64,
}

#[derive(Debug, Clone, FromRow)]
pub struct ZphCallRow {
    pub id: u64,
    pub title: String,
    pub organizers: String,
    pub starttime: String,
    pub start_at: i64,
    pub address: String,
    pub phone: String,
    pub user: String,
    pub weburl: String,
    pub pic: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct NewsCallRow {
    pub id: u64,
    pub title: String,
    pub keyword: String,
    pub author: String,
    pub datetime: i64,
    pub hits: i64,
    pub description: String,
    pub s_thumb: String,
    pub source: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct AskCallRow {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub nickname: String,
    pub add_time: i64,
    pub answer_num: i64,
}

#[derive(Debug, Clone, FromRow)]
pub struct LinkCallRow {
    pub link_name: String,
    pub link_url: String,
    pub pic: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct OnceCallRow {
    pub id: u64,
    pub title: String,
    pub companyname: String,
    pub mans: String,
    pub require: String,
    pub phone: String,
    pub linkman: String,
    pub address: String,
    pub ctime: i64,
}

#[derive(Debug, Clone, FromRow)]
pub struct TinyCallRow {
    pub id: u64,
    pub username: String,
    pub sex: i32,
    pub exp: i32,
    pub job: String,
    pub mobile: String,
    pub production: String,
    pub time: i64,
}

fn order_parts(byorder: &str, allowed: &[&str], lastedit_to: &str) -> (String, &'static str) {
    let s = byorder.replace("lastedit", lastedit_to);
    let mut parts = s.split(',');
    let col = parts.next().unwrap_or("").trim();
    let dir = parts.next().unwrap_or("desc").trim();
    let col = if allowed.iter().any(|c| *c == col) {
        col.to_string()
    } else {
        allowed.first().copied().unwrap_or("id").to_string()
    };
    let dir = if dir.eq_ignore_ascii_case("asc") {
        "ASC"
    } else {
        "DESC"
    };
    (col, dir)
}

fn order_sql(byorder: &str, allowed: &[&str], lastedit_to: &str) -> String {
    let (col, dir) = order_parts(byorder, allowed, lastedit_to);
    format!("`{col}` {dir}")
}

fn lim(num: i32) -> u64 {
    u64::try_from(num).unwrap_or(10).clamp(1, 200)
}

pub async fn find_outside(pool: &MySqlPool, id: u64) -> Result<Option<OutsideCall>, sqlx::Error> {
    sqlx::query_as(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(code,'') AS code, COALESCE(`type`,'') AS `type`, \
         CAST(COALESCE(num,0) AS SIGNED) AS num, COALESCE(byorder,'') AS byorder, \
         CAST(COALESCE(titlelen,0) AS SIGNED) AS titlelen, CAST(COALESCE(infolen,0) AS SIGNED) AS infolen, \
         CAST(COALESCE(urltype,0) AS SIGNED) AS urltype, COALESCE(timetype,'') AS timetype \
         FROM phpyun_outside WHERE id = ? AND COALESCE(deleted,0)=0 LIMIT 1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn list_resume(
    pool: &MySqlPool,
    byorder: &str,
    num: i32,
) -> Result<Vec<ResumeCallRow>, sqlx::Error> {
    let (col, dir) = order_parts(
        byorder,
        &["id", "lastupdate", "hits", "uid"],
        "lastupdate",
    );
    let sql = format!(
        "SELECT CAST(e.id AS UNSIGNED) AS id, CAST(e.uid AS UNSIGNED) AS uid, \
         COALESCE(e.name,'') AS resumename, CAST(COALESCE(e.lastupdate,0) AS SIGNED) AS lastupdate, \
         CAST(COALESCE(e.hits,0) AS SIGNED) AS hits, CAST(COALESCE(e.hy,0) AS SIGNED) AS hy, \
         COALESCE(e.job_classid,'') AS job_classid, CAST(COALESCE(e.report,0) AS SIGNED) AS report, \
         CAST(COALESCE(e.salary,0) AS SIGNED) AS salary, CAST(COALESCE(e.`type`,0) AS SIGNED) AS `type`, \
         CAST(COALESCE(e.provinceid,0) AS SIGNED) AS qw_provinceid, CAST(COALESCE(e.cityid,0) AS SIGNED) AS qw_cityid, \
         COALESCE(r.name,'') AS name, COALESCE(r.birthday,'') AS birthday, \
         CAST(COALESCE(r.edu,0) AS SIGNED) AS edu, COALESCE(r.photo,'') AS photo, \
         COALESCE(r.email,'') AS email, COALESCE(r.telhome,'') AS telhome, COALESCE(r.telphone,'') AS telphone, \
         CAST(COALESCE(r.exp,0) AS SIGNED) AS exp, COALESCE(r.address,'') AS address, \
         COALESCE(r.description,'') AS description, COALESCE(r.homepage,'') AS homepage, \
         COALESCE(r.idcard,'') AS idcard, COALESCE(r.living,'') AS living, COALESCE(r.domicile,'') AS domicile \
         FROM phpyun_resume_expect e INNER JOIN phpyun_resume r ON r.uid = e.uid AND r.def_job = e.id \
         WHERE e.defaults = 1 ORDER BY e.`{col}` {dir} LIMIT ?"
    );
    sqlx::query_as(&sql).bind(lim(num)).fetch_all(pool).await
}

pub async fn list_company(
    pool: &MySqlPool,
    byorder: &str,
    num: i32,
) -> Result<Vec<CompanyCallRow>, sqlx::Error> {
    let order = order_sql(byorder, &["uid", "lastupdate", "name"], "lastupdate");
    let sql = format!(
        "SELECT CAST(uid AS UNSIGNED) AS uid, COALESCE(name,'') AS name, CAST(COALESCE(hy,0) AS SIGNED) AS hy, \
         CAST(COALESCE(pr,0) AS SIGNED) AS pr, CAST(COALESCE(provinceid,0) AS SIGNED) AS provinceid, \
         CAST(COALESCE(cityid,0) AS SIGNED) AS cityid, CAST(COALESCE(mun,0) AS SIGNED) AS mun, \
         COALESCE(sdate,'') AS sdate, COALESCE(CAST(money AS CHAR),'') AS money, COALESCE(address,'') AS address, \
         COALESCE(zip,'') AS zip, COALESCE(linkman,'') AS linkman, COALESCE(linkjob,'') AS linkjob, \
         COALESCE(linkqq,'') AS linkqq, COALESCE(linkphone,'') AS linkphone, COALESCE(linktel,'') AS linktel, \
         COALESCE(linkmail,'') AS linkmail, COALESCE(website,'') AS website, COALESCE(logo,'') AS logo \
         FROM phpyun_company WHERE COALESCE(name,'') <> '' AND COALESCE(hy,0) <> 0 \
         ORDER BY {order} LIMIT ?"
    );
    sqlx::query_as(&sql).bind(lim(num)).fetch_all(pool).await
}

pub async fn job_counts(pool: &MySqlPool, uids: &[u64]) -> Result<std::collections::HashMap<u64, i64>, sqlx::Error> {
    if uids.is_empty() {
        return Ok(std::collections::HashMap::new());
    }
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(uid AS UNSIGNED) AS uid, COUNT(*) AS cnt FROM phpyun_company_job WHERE uid IN (",
    );
    let mut sep = qb.separated(", ");
    for id in uids {
        sep.push_bind(*id);
    }
    qb.push(") GROUP BY uid");
    let rows = qb.build().fetch_all(pool).await?;
    Ok(rows
        .into_iter()
        .filter_map(|r| Some((r.try_get::<u64, _>("uid").ok()?, r.try_get::<i64, _>("cnt").unwrap_or(0))))
        .collect())
}

pub async fn list_job(
    pool: &MySqlPool,
    byorder: &str,
    num: i32,
) -> Result<Vec<JobCallRow>, sqlx::Error> {
    let order = order_sql(
        byorder,
        &["id", "lastupdate", "sdate", "uid"],
        "lastupdate",
    );
    let sql = format!(
        "SELECT CAST(id AS UNSIGNED) AS id, CAST(uid AS UNSIGNED) AS uid, COALESCE(name,'') AS name, \
         COALESCE(com_name,'') AS com_name, CAST(COALESCE(hy,0) AS SIGNED) AS hy, \
         CAST(COALESCE(job1_son,0) AS SIGNED) AS job1_son, CAST(COALESCE(job_post,0) AS SIGNED) AS job_post, \
         CAST(COALESCE(provinceid,0) AS SIGNED) AS provinceid, CAST(COALESCE(cityid,0) AS SIGNED) AS cityid, \
         CAST(COALESCE(minsalary,0) AS SIGNED) AS minsalary, CAST(COALESCE(maxsalary,0) AS SIGNED) AS maxsalary, \
         CAST(COALESCE(number,0) AS SIGNED) AS number, CAST(COALESCE(age,0) AS SIGNED) AS age, \
         CAST(COALESCE(exp,0) AS SIGNED) AS exp, CAST(COALESCE(report,0) AS SIGNED) AS report, \
         CAST(COALESCE(edu,0) AS SIGNED) AS edu, COALESCE(lang,'') AS lang, COALESCE(welfare,'') AS welfare, \
         CAST(COALESCE(lastupdate,0) AS SIGNED) AS lastupdate \
         FROM phpyun_company_job WHERE state = 1 ORDER BY {order} LIMIT ?"
    );
    sqlx::query_as(&sql).bind(lim(num)).fetch_all(pool).await
}

pub async fn list_zph(
    pool: &MySqlPool,
    byorder: &str,
    num: i32,
) -> Result<Vec<ZphCallRow>, sqlx::Error> {
    let order = order_sql(byorder, &["id", "starttime", "ctime"], "lastupdate");
    let sql = format!(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(title,'') AS title, COALESCE(organizers,'') AS organizers, \
         COALESCE(starttime,'') AS starttime, CAST(COALESCE(UNIX_TIMESTAMP(starttime),0) AS SIGNED) AS start_at, \
         COALESCE(address,'') AS address, COALESCE(phone,'') AS phone, COALESCE(`user`,'') AS user, \
         COALESCE(weburl,'') AS weburl, COALESCE(pic,'') AS pic \
         FROM phpyun_zhaopinhui ORDER BY {order} LIMIT ?"
    );
    sqlx::query_as(&sql).bind(lim(num)).fetch_all(pool).await
}

pub async fn zph_com_counts(pool: &MySqlPool, ids: &[u64]) -> Result<std::collections::HashMap<u64, i64>, sqlx::Error> {
    if ids.is_empty() {
        return Ok(std::collections::HashMap::new());
    }
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(
        "SELECT CAST(zid AS UNSIGNED) AS zid, COUNT(*) AS cnt FROM phpyun_zhaopinhui_com WHERE zid IN (",
    );
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(") GROUP BY zid");
    let rows = qb.build().fetch_all(pool).await?;
    Ok(rows
        .into_iter()
        .filter_map(|r| Some((r.try_get::<u64, _>("zid").ok()?, r.try_get::<i64, _>("cnt").unwrap_or(0))))
        .collect())
}

pub async fn list_news(
    pool: &MySqlPool,
    byorder: &str,
    num: i32,
) -> Result<Vec<NewsCallRow>, sqlx::Error> {
    let order = order_sql(byorder, &["id", "datetime", "hits"], "lastupdate");
    let sql = format!(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(title,'') AS title, COALESCE(keyword,'') AS keyword, \
         COALESCE(author,'') AS author, CAST(COALESCE(datetime,0) AS SIGNED) AS datetime, \
         CAST(COALESCE(hits,0) AS SIGNED) AS hits, COALESCE(description,'') AS description, \
         COALESCE(s_thumb,'') AS s_thumb, COALESCE(source,'') AS source \
         FROM phpyun_news_base ORDER BY {order} LIMIT ?"
    );
    sqlx::query_as(&sql).bind(lim(num)).fetch_all(pool).await
}

pub async fn list_ask(
    pool: &MySqlPool,
    byorder: &str,
    num: i32,
) -> Result<Vec<AskCallRow>, sqlx::Error> {
    let order = order_sql(byorder, &["id", "add_time"], "lastupdate");
    let sql = format!(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(title,'') AS title, COALESCE(content,'') AS content, \
         COALESCE(nickname,'') AS nickname, CAST(COALESCE(add_time,0) AS SIGNED) AS add_time, \
         CAST(COALESCE(answer_num,0) AS SIGNED) AS answer_num \
         FROM phpyun_question ORDER BY {order} LIMIT ?"
    );
    sqlx::query_as(&sql).bind(lim(num)).fetch_all(pool).await
}

pub async fn list_link(
    pool: &MySqlPool,
    byorder: &str,
    num: i32,
) -> Result<Vec<LinkCallRow>, sqlx::Error> {
    let order = order_sql(byorder, &["id", "link_sorting"], "lastupdate");
    let sql = format!(
        "SELECT COALESCE(link_name,'') AS link_name, COALESCE(link_url,'') AS link_url, COALESCE(pic,'') AS pic \
         FROM phpyun_admin_link ORDER BY {order} LIMIT ?"
    );
    sqlx::query_as(&sql).bind(lim(num)).fetch_all(pool).await
}

pub async fn list_once(
    pool: &MySqlPool,
    byorder: &str,
    num: i32,
) -> Result<Vec<OnceCallRow>, sqlx::Error> {
    let order = order_sql(byorder, &["id", "ctime"], "ctime");
    let sql = format!(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(title,'') AS title, COALESCE(companyname,'') AS companyname, \
         COALESCE(mans,'') AS mans, COALESCE(`require`,'') AS `require`, COALESCE(phone,'') AS phone, \
         COALESCE(linkman,'') AS linkman, COALESCE(address,'') AS address, CAST(COALESCE(ctime,0) AS SIGNED) AS ctime \
         FROM phpyun_once_job ORDER BY {order} LIMIT ?"
    );
    sqlx::query_as(&sql).bind(lim(num)).fetch_all(pool).await
}

pub async fn list_tiny(
    pool: &MySqlPool,
    byorder: &str,
    num: i32,
) -> Result<Vec<TinyCallRow>, sqlx::Error> {
    let order = order_sql(byorder, &["id", "time", "lastupdate"], "time");
    let sql = format!(
        "SELECT CAST(id AS UNSIGNED) AS id, COALESCE(username,'') AS username, CAST(COALESCE(sex,0) AS SIGNED) AS sex, \
         CAST(COALESCE(exp,0) AS SIGNED) AS exp, COALESCE(job,'') AS job, COALESCE(mobile,'') AS mobile, \
         COALESCE(production,'') AS production, CAST(COALESCE(time,0) AS SIGNED) AS time \
         FROM phpyun_resume_tiny ORDER BY {order} LIMIT ?"
    );
    sqlx::query_as(&sql).bind(lim(num)).fetch_all(pool).await
}
