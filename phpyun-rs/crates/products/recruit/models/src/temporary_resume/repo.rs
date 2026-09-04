//! `phpyun_temporary_resume` — guest quick-apply snapshot (PHP `temporary_resume`).

use sqlx::MySqlPool;

pub struct Insert<'a> {
    pub name: &'a str,
    pub uname: &'a str,
    pub edu: i32,
    pub sex: i32,
    pub exp: i32,
    pub telphone: &'a str,
    pub birthday: &'a str,
    pub hy: i32,
    pub job_classid: &'a str,
    pub city_classid: &'a str,
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    pub minsalary: i32,
    pub maxsalary: i32,
    pub rid: i32,
}

pub async fn insert(pool: &MySqlPool, row: Insert<'_>) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_temporary_resume \
         (name, uname, edu, sex, exp, telphone, birthday, hy, job_classid, city_classid, \
          provinceid, cityid, three_cityid, minsalary, maxsalary, rid) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(row.name)
    .bind(row.uname)
    .bind(row.edu)
    .bind(row.sex)
    .bind(row.exp)
    .bind(row.telphone)
    .bind(row.birthday)
    .bind(row.hy)
    .bind(row.job_classid)
    .bind(row.city_classid)
    .bind(row.provinceid)
    .bind(row.cityid)
    .bind(row.three_cityid)
    .bind(row.minsalary)
    .bind(row.maxsalary)
    .bind(row.rid)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}
