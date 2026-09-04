//! `phpyun_company_statis` — per-company counter row.
//!
//! Same shape as `phpyun_member_statis` for jobseekers but on the company
//! side. PHP creates the row lazily when an employer activates their account.
//!
//! This is the **single repo** owning every column. `vip::repo` and
//! `special::repo` re-export the integral / rating accessors from here.

use sqlx::MySqlPool;

/// INSERT IGNORE — create the per-company counter row if it doesn't already
/// exist. Idempotent.
pub async fn ensure_row(pool: &MySqlPool, uid: u64) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT IGNORE INTO phpyun_company_statis (uid) VALUES (?)")
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn dec_zph_num(pool: &MySqlPool, uid: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_company_statis SET zph_num = zph_num - 1 WHERE uid = ? AND zph_num > 0",
    )
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn insert_admin_created<'e, E>(
    exec: E,
    uid: u64,
    rating: i32,
    rating_name: &str,
    rating_type: i32,
    job_num: i32,
    down_resume: i32,
    breakjob_num: i32,
    invite_resume: i32,
    zph_num: i32,
    top_num: i32,
    urgent_num: i32,
    rec_num: i32,
    integral: i64,
    vip_stime: i64,
    vip_etime: i64,
) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::MySql>,
{
    sqlx::query(
        "INSERT INTO phpyun_company_statis (\
            uid, rating, rating_name, rating_type, job_num, down_resume, breakjob_num, \
            invite_resume, zph_num, top_num, urgent_num, rec_num, integral, vip_stime, vip_etime, \
            sq_job, fav_job, all_pay, consum_pay\
         ) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,0,0,0,0)",
    )
    .bind(uid)
    .bind(rating)
    .bind(rating_name)
    .bind(rating_type)
    .bind(job_num)
    .bind(down_resume)
    .bind(breakjob_num)
    .bind(invite_resume)
    .bind(zph_num)
    .bind(top_num)
    .bind(urgent_num)
    .bind(rec_num)
    .bind(integral.to_string())
    .bind(vip_stime)
    .bind(vip_etime)
    .execute(exec)
    .await?;
    Ok(())
}

pub async fn delete_by_uid<'e, E>(exec: E, uid: u64) -> Result<(), sqlx::Error>
where
    E: sqlx::Executor<'e, Database = sqlx::MySql>,
{
    sqlx::query("DELETE FROM phpyun_company_statis WHERE uid = ?")
        .bind(uid)
        .execute(exec)
        .await?;
    Ok(())
}

/// Read the integral balance. Stored as VARCHAR in PHP and validated in Rust.
/// Returns 0 when the row doesn't exist.
pub async fn read_integral(pool: &MySqlPool, uid: u64) -> Result<i64, sqlx::Error> {
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT COALESCE(integral, '') FROM phpyun_company_statis \
         WHERE uid = ? LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    row.map(|(raw,)| {
        crate::member_statis::repo::parse_stored_balance(&raw, "phpyun_company_statis.integral")
    })
    .transpose()
    .map(|balance| balance.unwrap_or(0))
}

/// Atomic deduction on the integral column. Returns `1` on success, `0` when
/// balance is insufficient.
pub async fn try_deduct_integral(
    pool: &MySqlPool,
    uid: u64,
    points: i64,
) -> Result<u64, sqlx::Error> {
    if points <= 0 {
        return Err(sqlx::Error::Protocol(format!(
            "phpyun_company_statis.integral: deduction must be positive, got {points}"
        )));
    }
    let mut tx = pool.begin().await?;
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT COALESCE(integral, '') FROM phpyun_company_statis \
         WHERE uid = ? FOR UPDATE",
    )
    .bind(uid)
    .fetch_optional(&mut *tx)
    .await?;
    let Some((raw,)) = row else {
        tx.rollback().await?;
        return Ok(0);
    };
    let balance =
        crate::member_statis::repo::parse_stored_balance(&raw, "phpyun_company_statis.integral")?;
    if balance < points {
        tx.rollback().await?;
        return Ok(0);
    }
    let next = balance.checked_sub(points).ok_or_else(|| {
        sqlx::Error::Protocol(format!(
            "phpyun_company_statis.integral: subtraction overflow for {balance} - {points}"
        ))
    })?;
    sqlx::query("UPDATE phpyun_company_statis SET integral = ? WHERE uid = ?")
        .bind(next.to_string())
        .bind(uid)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(1)
}

pub async fn set_rating(pool: &MySqlPool, uid: u64, rating: i32) -> Result<u64, sqlx::Error> {
    ensure_row(pool, uid).await?;
    let res = sqlx::query("UPDATE phpyun_company_statis SET rating = ? WHERE uid = ?")
        .bind(rating)
        .bind(uid)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub async fn read_rating(pool: &MySqlPool, uid: u64) -> Result<i32, sqlx::Error> {
    let row: Option<(i32,)> = sqlx::query_as(
        "SELECT CAST(COALESCE(rating, 0) AS SIGNED) FROM phpyun_company_statis \
         WHERE uid = ? LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(r,)| r).unwrap_or(0))
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct AdminStatisRow {
    pub rating: i32,
    pub rating_name: String,
    pub job_num: i32,
    pub down_resume: i32,
    pub breakjob_num: i32,
    pub invite_resume: i32,
    pub zph_num: i32,
    pub top_num: i32,
    pub urgent_num: i32,
    pub rec_num: i32,
    pub vip_stime: i64,
    pub vip_etime: i64,
    pub integral: String,
    pub rating_type: i32,
    pub suspend_num: i32,
    pub max_time: i64,
}

pub async fn find_admin(pool: &MySqlPool, uid: u64) -> Result<Option<AdminStatisRow>, sqlx::Error> {
    sqlx::query_as(
        "SELECT CAST(COALESCE(rating,0) AS SIGNED) AS rating, COALESCE(rating_name,'') AS rating_name, \
         CAST(COALESCE(job_num,0) AS SIGNED) AS job_num, CAST(COALESCE(down_resume,0) AS SIGNED) AS down_resume, \
         CAST(COALESCE(breakjob_num,0) AS SIGNED) AS breakjob_num, \
         CAST(COALESCE(invite_resume,0) AS SIGNED) AS invite_resume, \
         CAST(COALESCE(zph_num,0) AS SIGNED) AS zph_num, CAST(COALESCE(top_num,0) AS SIGNED) AS top_num, \
         CAST(COALESCE(urgent_num,0) AS SIGNED) AS urgent_num, CAST(COALESCE(rec_num,0) AS SIGNED) AS rec_num, \
         CAST(COALESCE(vip_stime,0) AS SIGNED) AS vip_stime, CAST(COALESCE(vip_etime,0) AS SIGNED) AS vip_etime, \
         COALESCE(integral,'') AS integral, CAST(COALESCE(rating_type,0) AS SIGNED) AS rating_type, \
         CAST(COALESCE(suspend_num,0) AS SIGNED) AS suspend_num, CAST(COALESCE(max_time,0) AS SIGNED) AS max_time \
         FROM phpyun_company_statis WHERE uid = ? LIMIT 1",
    )
    .bind(uid)
    .fetch_optional(pool)
    .await
}

pub async fn update_admin_quotas(
    pool: &MySqlPool,
    uid: u64,
    s: &AdminStatisRow,
) -> Result<u64, sqlx::Error> {
    ensure_row(pool, uid).await?;
    let res = sqlx::query(
        "UPDATE phpyun_company_statis SET \
            rating=?, rating_name=?, rating_type=?, integral=?, vip_stime=?, vip_etime=?, \
            job_num=?, breakjob_num=?, down_resume=?, invite_resume=?, zph_num=?, \
            top_num=?, urgent_num=?, rec_num=?, suspend_num=?, max_time=? \
         WHERE uid=?",
    )
    .bind(s.rating)
    .bind(&s.rating_name)
    .bind(s.rating_type)
    .bind(&s.integral)
    .bind(s.vip_stime)
    .bind(s.vip_etime)
    .bind(s.job_num)
    .bind(s.breakjob_num)
    .bind(s.down_resume)
    .bind(s.invite_resume)
    .bind(s.zph_num)
    .bind(s.top_num)
    .bind(s.urgent_num)
    .bind(s.rec_num)
    .bind(s.suspend_num)
    .bind(s.max_time)
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

/// PHP `finance_recharge` jifen：在现有 VARCHAR 积分上加正数。
pub async fn add_integral(pool: &MySqlPool, uid: u64, points: i64) -> Result<i64, sqlx::Error> {
    if points <= 0 {
        return Err(sqlx::Error::Protocol(format!(
            "phpyun_company_statis.integral: add must be positive, got {points}"
        )));
    }
    ensure_row(pool, uid).await?;
    let mut tx = pool.begin().await?;
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT COALESCE(integral, '') FROM phpyun_company_statis \
         WHERE uid = ? FOR UPDATE",
    )
    .bind(uid)
    .fetch_optional(&mut *tx)
    .await?;
    let raw = row.map(|(s,)| s).unwrap_or_default();
    let balance =
        crate::member_statis::repo::parse_stored_balance(&raw, "phpyun_company_statis.integral")?;
    let next = balance.checked_add(points).ok_or_else(|| {
        sqlx::Error::Protocol(format!(
            "phpyun_company_statis.integral: add overflow for {balance} + {points}"
        ))
    })?;
    sqlx::query("UPDATE phpyun_company_statis SET integral = ? WHERE uid = ?")
        .bind(next.to_string())
        .bind(uid)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(next)
}

pub async fn adjust_integral(pool: &MySqlPool, uid: u64, points: i64) -> Result<i64, sqlx::Error> {
    if points == 0 {
        return Ok(0);
    }
    if points > 0 {
        return add_integral(pool, uid, points).await;
    }
    let deduct = (-points) as u32;
    ensure_row(pool, uid).await?;
    let mut tx = pool.begin().await?;
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT COALESCE(integral, '') FROM phpyun_company_statis WHERE uid = ? FOR UPDATE",
    )
    .bind(uid)
    .fetch_optional(&mut *tx)
    .await?;
    let raw = row.map(|(s,)| s).unwrap_or_default();
    let balance =
        crate::member_statis::repo::parse_stored_balance(&raw, "phpyun_company_statis.integral")?;
    let Some(next) = crate::member_statis::repo::balance_after_deduction(balance, deduct) else {
        tx.rollback().await?;
        return Err(sqlx::Error::Protocol(
            "phpyun_company_statis.integral: insufficient".into(),
        ));
    };
    sqlx::query("UPDATE phpyun_company_statis SET integral = ? WHERE uid = ?")
        .bind(next.to_string())
        .bind(uid)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(next)
}

pub async fn add_service_nums(
    pool: &MySqlPool,
    uid: u64,
    job_num: i32,
    breakjob_num: i32,
    down_resume: i32,
    invite_resume: i32,
    zph_num: i32,
    top_num: i32,
    rec_num: i32,
    urgent_num: i32,
) -> Result<u64, sqlx::Error> {
    ensure_row(pool, uid).await?;
    Ok(
        sqlx::query(
            "UPDATE phpyun_company_statis SET \
             job_num = job_num + ?, breakjob_num = breakjob_num + ?, \
             down_resume = down_resume + ?, invite_resume = invite_resume + ?, \
             zph_num = zph_num + ?, top_num = top_num + ?, rec_num = rec_num + ?, \
             urgent_num = urgent_num + ? WHERE uid = ?",
        )
        .bind(job_num)
        .bind(breakjob_num)
        .bind(down_resume)
        .bind(invite_resume)
        .bind(zph_num)
        .bind(top_num)
        .bind(rec_num)
        .bind(urgent_num)
        .bind(uid)
        .execute(pool)
        .await?
        .rows_affected(),
    )
}

/// PHP 开通套餐天数：从 max(now, vip_etime) 起加 `days` 天。
pub async fn extend_vip_days(
    pool: &MySqlPool,
    uid: u64,
    days: i64,
    now: i64,
) -> Result<i64, sqlx::Error> {
    if days <= 0 {
        return Err(sqlx::Error::Protocol(format!(
            "phpyun_company_statis.vip_etime: days must be positive, got {days}"
        )));
    }
    ensure_row(pool, uid).await?;
    let mut tx = pool.begin().await?;
    let row: Option<(i64, i64)> = sqlx::query_as(
        "SELECT CAST(COALESCE(vip_stime, 0) AS SIGNED), \
                CAST(COALESCE(vip_etime, 0) AS SIGNED) \
         FROM phpyun_company_statis WHERE uid = ? FOR UPDATE",
    )
    .bind(uid)
    .fetch_optional(&mut *tx)
    .await?;
    let (stime, etime) = row.unwrap_or((0, 0));
    let base = if etime > now { etime } else { now };
    let extra = days.checked_mul(86_400).ok_or_else(|| {
        sqlx::Error::Protocol(format!(
            "phpyun_company_statis.vip_etime: days overflow {days}"
        ))
    })?;
    let next = base.checked_add(extra).ok_or_else(|| {
        sqlx::Error::Protocol("phpyun_company_statis.vip_etime: timestamp overflow".into())
    })?;
    let next_stime = if stime > 0 { stime } else { now };
    sqlx::query("UPDATE phpyun_company_statis SET vip_stime = ?, vip_etime = ? WHERE uid = ?")
        .bind(next_stime)
        .bind(next)
        .bind(uid)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(next)
}

pub async fn try_consume_down_resume(pool: &MySqlPool, uid: u64) -> Result<bool, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_company_statis SET down_resume = down_resume - 1 \
         WHERE uid = ? AND down_resume > 0",
    )
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected() > 0)
}

pub async fn try_consume_breakpart(pool: &MySqlPool, uid: u64, n: i32) -> Result<bool, sqlx::Error> {
    if n <= 0 {
        return Ok(true);
    }
    let res = sqlx::query(
        "UPDATE phpyun_company_statis SET breakpart_num = breakpart_num - ? \
         WHERE uid = ? AND breakpart_num >= ?",
    )
    .bind(n)
    .bind(uid)
    .bind(n)
    .execute(pool)
    .await?;
    Ok(res.rows_affected() > 0)
}

pub async fn try_consume_breakjob(pool: &MySqlPool, uid: u64, n: i32) -> Result<bool, sqlx::Error> {
    if n <= 0 {
        return Ok(true);
    }
    let res = sqlx::query(
        "UPDATE phpyun_company_statis SET breakjob_num = breakjob_num - ? \
         WHERE uid = ? AND breakjob_num >= ?",
    )
    .bind(n)
    .bind(uid)
    .bind(n)
    .execute(pool)
    .await?;
    Ok(res.rows_affected() > 0)
}

pub async fn try_consume_invite_resume(pool: &MySqlPool, uid: u64) -> Result<bool, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_company_statis SET invite_resume = invite_resume - 1 \
         WHERE uid = ? AND invite_resume > 0",
    )
    .bind(uid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected() > 0)
}

pub async fn freelook_num(pool: &MySqlPool, rating_id: i32) -> Result<i32, sqlx::Error> {
    let row: Option<(i32,)> = sqlx::query_as(
        "SELECT CAST(COALESCE(freelook_num, 0) AS SIGNED) FROM phpyun_company_rating WHERE id = ? LIMIT 1",
    )
    .bind(rating_id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|(n,)| n).unwrap_or(0))
}
