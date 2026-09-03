//! `phpyun_userid_msg` — PHP `job.model.php::addYqms` interview invitations.

use sqlx::MySqlPool;

pub struct UseridMsgCreate<'a> {
    pub uid: u64,
    pub title: &'a str,
    pub content: &'a str,
    pub fid: u64,
    pub fname: &'a str,
    pub datetime: i64,
    pub address: &'a str,
    pub intertime: &'a str,
    pub linkman: &'a str,
    pub linktel: &'a str,
    pub jobid: u64,
    pub jobname: &'a str,
    pub did: u64,
    pub x: &'a str,
    pub y: &'a str,
}

pub async fn insert(pool: &MySqlPool, c: UseridMsgCreate<'_>) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        r#"INSERT INTO phpyun_userid_msg
           (uid, title, content, fid, fname, type, datetime, `default`, is_browse,
            address, intertime, linkman, linktel, jobid, jobname, did, x, y, isdel)
           VALUES (?, ?, ?, ?, ?, 0, ?, 0, 1, ?, ?, ?, ?, ?, ?, ?, ?, ?, 9)"#,
    )
    .bind(c.uid)
    .bind(c.title)
    .bind(c.content)
    .bind(c.fid)
    .bind(c.fname)
    .bind(c.datetime)
    .bind(c.address)
    .bind(c.intertime)
    .bind(c.linkman)
    .bind(c.linktel)
    .bind(c.jobid)
    .bind(c.jobname)
    .bind(c.did)
    .bind(c.x)
    .bind(c.y)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}
