//! `phpyun_rs_seeker_vip_pack` — job-seeker monthly VIP SKUs.

use super::entity::SeekerVipPack;
use sqlx::MySqlPool;

const FIELDS: &str = "CAST(id AS UNSIGNED) AS id, \
    COALESCE(code,'') AS code, \
    COALESCE(name,'') AS name, \
    CAST(COALESCE(months,1) AS SIGNED) AS months, \
    CAST(COALESCE(price_cents,0) AS SIGNED) AS price_cents, \
    CAST(COALESCE(chat,0) AS SIGNED) AS chat, \
    CAST(COALESCE(resume_top,0) AS SIGNED) AS resume_top, \
    CAST(COALESCE(tpl_all,0) AS SIGNED) AS tpl_all, \
    CAST(COALESCE(refresh_free,0) AS SIGNED) AS refresh_free, \
    CAST(COALESCE(sort,0) AS SIGNED) AS sort, \
    CAST(COALESCE(display,0) AS SIGNED) AS display, \
    CAST(COALESCE(deleted,0) AS SIGNED) AS deleted, \
    CAST(COALESCE(created_at,0) AS SIGNED) AS created_at, \
    CAST(COALESCE(updated_at,0) AS SIGNED) AS updated_at";

pub async fn list_buyable(pool: &MySqlPool) -> Result<Vec<SeekerVipPack>, sqlx::Error> {
    let r = sqlx::query_as::<_, SeekerVipPack>(&format!(
        "SELECT {FIELDS} FROM phpyun_rs_seeker_vip_pack \
         WHERE COALESCE(deleted,0)=0 AND COALESCE(display,0)=1 \
         ORDER BY sort ASC, months ASC, id ASC"
    ))
    .fetch_all(pool)
    .await;
    match r {
        Ok(v) => Ok(v),
        Err(e) if phpyun_core::db::is_missing_table(&e) => Ok(Vec::new()),
        Err(e) => Err(e),
    }
}

pub async fn list_admin(pool: &MySqlPool) -> Result<Vec<SeekerVipPack>, sqlx::Error> {
    let r = sqlx::query_as::<_, SeekerVipPack>(&format!(
        "SELECT {FIELDS} FROM phpyun_rs_seeker_vip_pack \
         WHERE COALESCE(deleted,0)=0 \
         ORDER BY sort ASC, id ASC"
    ))
    .fetch_all(pool)
    .await;
    match r {
        Ok(v) => Ok(v),
        Err(e) if phpyun_core::db::is_missing_table(&e) => Ok(Vec::new()),
        Err(e) => Err(e),
    }
}

pub async fn find_by_code(pool: &MySqlPool, code: &str) -> Result<Option<SeekerVipPack>, sqlx::Error> {
    let r = sqlx::query_as::<_, SeekerVipPack>(&format!(
        "SELECT {FIELDS} FROM phpyun_rs_seeker_vip_pack \
         WHERE code = ? AND COALESCE(deleted,0)=0 LIMIT 1"
    ))
    .bind(code)
    .fetch_optional(pool)
    .await;
    match r {
        Ok(v) => Ok(v),
        Err(e) if phpyun_core::db::is_missing_table(&e) => Ok(None),
        Err(e) => Err(e),
    }
}

pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<Option<SeekerVipPack>, sqlx::Error> {
    let r = sqlx::query_as::<_, SeekerVipPack>(&format!(
        "SELECT {FIELDS} FROM phpyun_rs_seeker_vip_pack \
         WHERE id = ? AND COALESCE(deleted,0)=0 LIMIT 1"
    ))
    .bind(id)
    .fetch_optional(pool)
    .await;
    match r {
        Ok(v) => Ok(v),
        Err(e) if phpyun_core::db::is_missing_table(&e) => Ok(None),
        Err(e) => Err(e),
    }
}

pub struct PackWrite<'a> {
    pub code: &'a str,
    pub name: &'a str,
    pub months: i32,
    pub price_cents: i32,
    pub chat: i32,
    pub resume_top: i32,
    pub tpl_all: i32,
    pub refresh_free: i32,
    pub sort: i32,
    pub display: i32,
}

pub async fn insert(pool: &MySqlPool, w: &PackWrite<'_>, now: i64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_rs_seeker_vip_pack \
         (code, name, months, price_cents, chat, resume_top, tpl_all, refresh_free, sort, display, deleted, created_at, updated_at) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0, ?, ?)",
    )
    .bind(w.code)
    .bind(w.name)
    .bind(w.months)
    .bind(w.price_cents)
    .bind(w.chat)
    .bind(w.resume_top)
    .bind(w.tpl_all)
    .bind(w.refresh_free)
    .bind(w.sort)
    .bind(w.display)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn update(pool: &MySqlPool, id: u64, w: &PackWrite<'_>, now: i64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_rs_seeker_vip_pack SET \
           code=?, name=?, months=?, price_cents=?, chat=?, resume_top=?, tpl_all=?, refresh_free=?, \
           sort=?, display=?, updated_at=? \
         WHERE id=? AND COALESCE(deleted,0)=0",
    )
    .bind(w.code)
    .bind(w.name)
    .bind(w.months)
    .bind(w.price_cents)
    .bind(w.chat)
    .bind(w.resume_top)
    .bind(w.tpl_all)
    .bind(w.refresh_free)
    .bind(w.sort)
    .bind(w.display)
    .bind(now)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn soft_delete(pool: &MySqlPool, id: u64, now: i64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_rs_seeker_vip_pack SET deleted=1, display=0, updated_at=? WHERE id=? AND COALESCE(deleted,0)=0",
    )
    .bind(now)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}
