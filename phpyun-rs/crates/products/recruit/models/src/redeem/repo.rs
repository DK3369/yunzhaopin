use super::entity::{RedeemClass, RedeemOrder, Reward};
use crate::soft_delete::{self, PREDICATE};
use sqlx::{MySql, MySqlPool, QueryBuilder, Transaction};

// Strictly aligned with PHPYun:
//   phpyun_reward       columns: id/name/nid/tnid/integral/num/restriction/stock/pic/sort/content/status/sdate/rec/hot
//   phpyun_redeem_class columns: id/keyid/name/sort
//   phpyun_change       gift-redemption work orders (PHP `redeem.model.php::AddChange`)
//
// Rust field -> PHP column (bridged via SELECT aliases):
//   Reward.sold       <-> num        |  is_rec <-> rec  |  is_hot <-> hot  |  created_at <-> sdate
//   RedeemClass.parent_id <-> keyid  |  created_at = 0
//   RedeemOrder.gid      <-> gid     |  address <-> body |  created_at <-> ctime

const REWARD_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    COALESCE(name, '') AS name, \
    COALESCE(pic, '') AS pic, \
    COALESCE(content, '') AS content, \
    CAST(COALESCE(integral, 0) AS UNSIGNED) AS integral, \
    CAST(COALESCE(stock, 0) AS UNSIGNED) AS stock, \
    CAST(COALESCE(num, 0) AS UNSIGNED) AS sold, \
    CAST(COALESCE(restriction, 0) AS UNSIGNED) AS restriction, \
    CAST(COALESCE(nid, 0) AS UNSIGNED) AS nid, \
    CAST(COALESCE(tnid, 0) AS UNSIGNED) AS tnid, \
    CAST(COALESCE(status, 0) AS SIGNED) AS status, \
    CAST(COALESCE(rec, 0) AS SIGNED) AS is_rec, \
    CAST(COALESCE(hot, 0) AS SIGNED) AS is_hot, \
    CAST(COALESCE(sdate, 0) AS SIGNED) AS created_at";

const ORDER_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(COALESCE(uid, 0) AS UNSIGNED) AS uid, \
    CAST(COALESCE(gid, 0) AS UNSIGNED) AS gid, \
    COALESCE(name, '') AS name, \
    COALESCE(linkman, '') AS linkman, \
    COALESCE(linktel, '') AS linktel, \
    COALESCE(body, '') AS address, \
    CAST(COALESCE(integral, 0) AS UNSIGNED) AS integral, \
    CAST(COALESCE(num, 0) AS UNSIGNED) AS num, \
    CAST(COALESCE(status, 0) AS SIGNED) AS status, \
    CAST(COALESCE(ctime, 0) AS SIGNED) AS created_at";

const CLASS_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(COALESCE(keyid, 0) AS UNSIGNED) AS parent_id, \
    COALESCE(name, '') AS name, \
    CAST(COALESCE(sort, 0) AS SIGNED) AS sort, \
    CAST(0 AS SIGNED) AS created_at";

// ---------- classes ----------

pub async fn list_classes(
    pool: &MySqlPool,
    parent_id: Option<u64>,
) -> Result<Vec<RedeemClass>, sqlx::Error> {
    let sql = match parent_id {
        Some(_) => format!(
            "SELECT {CLASS_FIELDS} FROM phpyun_redeem_class \
             WHERE keyid = ? AND {PREDICATE} ORDER BY sort ASC, id ASC"
        ),
        None => format!(
            "SELECT {CLASS_FIELDS} FROM phpyun_redeem_class \
             WHERE {PREDICATE} ORDER BY keyid ASC, sort ASC, id ASC"
        ),
    };
    let q = sqlx::query_as::<_, RedeemClass>(&sql);
    match parent_id {
        Some(p) => q.bind(p).fetch_all(pool).await,
        None => q.fetch_all(pool).await,
    }
}

pub async fn insert_class(
    pool: &MySqlPool,
    parent_id: u64,
    name: &str,
    sort: i32,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let _ = now;
    let res = sqlx::query("INSERT INTO phpyun_redeem_class (keyid, name, sort) VALUES (?, ?, ?)")
        .bind(parent_id)
        .bind(name)
        .bind(sort)
        .execute(pool)
        .await?;
    Ok(res.last_insert_id())
}

pub async fn delete_class(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    soft_delete::mark_col_in(pool, "phpyun_redeem_class", "keyid", &[id]).await?;
    soft_delete::mark_id(pool, "phpyun_redeem_class", id).await
}

pub async fn find_class(pool: &MySqlPool, id: u64) -> Result<Option<RedeemClass>, sqlx::Error> {
    let sql = format!(
        "SELECT {CLASS_FIELDS} FROM phpyun_redeem_class WHERE id = ? AND {PREDICATE} LIMIT 1"
    );
    sqlx::query_as(&sql).bind(id).fetch_optional(pool).await
}

pub async fn update_class(
    pool: &MySqlPool,
    id: u64,
    name: Option<&str>,
    sort: Option<i32>,
) -> Result<u64, sqlx::Error> {
    if name.is_none() && sort.is_none() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("UPDATE phpyun_redeem_class SET ");
    let mut first = true;
    if let Some(n) = name {
        qb.push("name = ");
        qb.push_bind(n);
        first = false;
    }
    if let Some(s) = sort {
        if !first {
            qb.push(", ");
        }
        qb.push("sort = ");
        qb.push_bind(s);
    }
    qb.push(" WHERE id = ");
    qb.push_bind(id);
    Ok(qb.build().execute(pool).await?.rows_affected())
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PhpRewardRow {
    pub id: u64,
    pub name: String,
    pub nid: u64,
    pub tnid: u64,
    pub integral: i32,
    pub restriction: i32,
    pub stock: i32,
    pub sort: i32,
    pub status: i32,
    pub rec: i32,
    pub hot: i32,
    pub pic: String,
    pub content: String,
}

pub struct PhpRewardFilter<'a> {
    pub name_kw: Option<&'a str>,
    pub integral: Option<i32>,
    pub nid: Option<u64>,
    pub status: Option<i32>,
    pub rec: Option<i32>,
    pub hot: Option<i32>,
    pub sort: &'a str,
    pub dir: &'a str,
}

fn push_reward_where(qb: &mut QueryBuilder<'_, sqlx::MySql>, f: &PhpRewardFilter<'_>) {
    qb.push(format!(" FROM phpyun_reward WHERE {PREDICATE}"));
    if let Some(kw) = f.name_kw.filter(|s| !s.is_empty()) {
        qb.push(" AND name LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    if let Some(n) = f.integral {
        qb.push(" AND integral = ");
        qb.push_bind(n);
    }
    if let Some(n) = f.nid.filter(|n| *n > 0) {
        qb.push(" AND nid = ");
        qb.push_bind(n);
    }
    if let Some(s) = f.status {
        qb.push(" AND COALESCE(status,0) = ");
        qb.push_bind(s);
    }
    if let Some(s) = f.rec {
        qb.push(" AND COALESCE(rec,0) = ");
        qb.push_bind(s);
    }
    if let Some(s) = f.hot {
        qb.push(" AND COALESCE(hot,0) = ");
        qb.push_bind(s);
    }
}

fn reward_order(sort: &str, dir: &str) -> &'static str {
    let desc = !dir.eq_ignore_ascii_case("asc");
    match sort {
        "integral" => {
            if desc {
                " ORDER BY integral DESC, id DESC"
            } else {
                " ORDER BY integral ASC, id ASC"
            }
        }
        "stock" => {
            if desc {
                " ORDER BY stock DESC, id DESC"
            } else {
                " ORDER BY stock ASC, id ASC"
            }
        }
        "sort" => {
            if desc {
                " ORDER BY sort DESC, id DESC"
            } else {
                " ORDER BY sort ASC, id ASC"
            }
        }
        _ => {
            if desc {
                " ORDER BY id DESC"
            } else {
                " ORDER BY id ASC"
            }
        }
    }
}

const PHP_REWARD_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    COALESCE(name,'') AS name, \
    CAST(COALESCE(nid,0) AS UNSIGNED) AS nid, \
    CAST(COALESCE(tnid,0) AS UNSIGNED) AS tnid, \
    CAST(COALESCE(integral,0) AS SIGNED) AS integral, \
    CAST(COALESCE(restriction,0) AS SIGNED) AS restriction, \
    CAST(COALESCE(stock,0) AS SIGNED) AS stock, \
    CAST(COALESCE(sort,0) AS SIGNED) AS sort, \
    CAST(COALESCE(status,0) AS SIGNED) AS status, \
    CAST(COALESCE(rec,0) AS SIGNED) AS rec, \
    CAST(COALESCE(hot,0) AS SIGNED) AS hot, \
    COALESCE(pic,'') AS pic, \
    COALESCE(content,'') AS content";

pub async fn php_list_rewards(
    pool: &MySqlPool,
    f: &PhpRewardFilter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<PhpRewardRow>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new(format!("SELECT {PHP_REWARD_FIELDS}"));
    push_reward_where(&mut qb, f);
    qb.push(reward_order(f.sort, f.dir));
    qb.push(" LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn php_count_rewards(pool: &MySqlPool, f: &PhpRewardFilter<'_>) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT COUNT(*)");
    push_reward_where(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

// ---------- rewards ----------

pub async fn list_rewards(
    pool: &MySqlPool,
    only_active: bool,
    nid: Option<u64>,
    tnid: Option<u64>,
    offset: u64,
    limit: u64,
) -> Result<Vec<Reward>, sqlx::Error> {
    let mut sql = format!("SELECT {REWARD_FIELDS} FROM phpyun_reward WHERE {PREDICATE}");
    if only_active {
        sql.push_str(" AND status = 1");
    }
    if nid.is_some() {
        sql.push_str(" AND nid = ?");
    }
    if tnid.is_some() {
        sql.push_str(" AND tnid = ?");
    }
    sql.push_str(" ORDER BY rec DESC, hot DESC, id DESC LIMIT ? OFFSET ?");

    let mut q = sqlx::query_as::<_, Reward>(&sql);
    if let Some(n) = nid {
        q = q.bind(n);
    }
    if let Some(t) = tnid {
        q = q.bind(t);
    }
    q.bind(limit).bind(offset).fetch_all(pool).await
}

pub async fn count_rewards(
    pool: &MySqlPool,
    only_active: bool,
    nid: Option<u64>,
    tnid: Option<u64>,
) -> Result<u64, sqlx::Error> {
    let mut sql = format!("SELECT COUNT(*) FROM phpyun_reward WHERE {PREDICATE}");
    if only_active {
        sql.push_str(" AND status = 1");
    }
    if nid.is_some() {
        sql.push_str(" AND nid = ?");
    }
    if tnid.is_some() {
        sql.push_str(" AND tnid = ?");
    }
    let mut q = sqlx::query_as::<_, (i64,)>(&sql);
    if let Some(n) = nid {
        q = q.bind(n);
    }
    if let Some(t) = tnid {
        q = q.bind(t);
    }
    let (n,) = q.fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn get_reward(pool: &MySqlPool, id: u64) -> Result<Option<Reward>, sqlx::Error> {
    let sql = format!("SELECT {REWARD_FIELDS} FROM phpyun_reward WHERE id = ? AND {PREDICATE}");
    sqlx::query_as::<_, Reward>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub struct NewReward<'a> {
    pub name: &'a str,
    pub pic: &'a str,
    pub content: &'a str,
    pub integral: u32,
    pub stock: u32,
    pub restriction: u32,
    pub nid: u64,
    pub tnid: u64,
}

pub async fn insert_reward(
    pool: &MySqlPool,
    r: &NewReward<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_reward \
         (name, pic, content, integral, stock, num, restriction, nid, tnid, status, rec, hot, sdate) \
         VALUES (?, ?, ?, ?, ?, 0, ?, ?, ?, 1, 0, 0, ?)",
    )
    .bind(r.name)
    .bind(r.pic)
    .bind(r.content)
    .bind(r.integral)
    .bind(r.stock)
    .bind(r.restriction)
    .bind(r.nid)
    .bind(r.tnid)
    .bind(now)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn set_reward_status(pool: &MySqlPool, id: u64, status: i32) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_reward SET status = ? WHERE id = ?")
        .bind(status)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub async fn set_reward_flags(
    pool: &MySqlPool,
    id: u64,
    is_rec: Option<i32>,
    is_hot: Option<i32>,
) -> Result<u64, sqlx::Error> {
    let mut clauses = vec![];
    if is_rec.is_some() {
        clauses.push("rec = ?");
    }
    if is_hot.is_some() {
        clauses.push("hot = ?");
    }
    if clauses.is_empty() {
        return Ok(0);
    }
    let sql = format!(
        "UPDATE phpyun_reward SET {} WHERE id = ?",
        clauses.join(", ")
    );
    let mut q = sqlx::query(&sql);
    if let Some(r) = is_rec {
        q = q.bind(r);
    }
    if let Some(h) = is_hot {
        q = q.bind(h);
    }
    let res = q.bind(id).execute(pool).await?;
    Ok(res.rows_affected())
}

pub async fn delete_reward(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    soft_delete::mark_id(pool, "phpyun_reward", id).await
}

/// Deduct stock and increment the sold counter inside a transaction.
/// Returns affected rows.
pub async fn tx_reserve_stock(
    tx: &mut Transaction<'_, MySql>,
    reward_id: u64,
    n: u32,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_reward \
         SET stock = stock - ?, num = num + ? \
         WHERE id = ? AND stock >= ? AND status = 1",
    )
    .bind(n)
    .bind(n)
    .bind(reward_id)
    .bind(n)
    .execute(&mut **tx)
    .await?;
    Ok(res.rows_affected())
}

/// Return stock when cancelling.
pub async fn tx_return_stock(
    tx: &mut Transaction<'_, MySql>,
    reward_id: u64,
    n: u32,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_reward \
         SET stock = stock + ?, num = GREATEST(CAST(num AS SIGNED) - ?, 0) \
         WHERE id = ?",
    )
    .bind(n)
    .bind(n)
    .bind(reward_id)
    .execute(&mut **tx)
    .await?;
    Ok(res.rows_affected())
}

// ---------- orders ----------

pub async fn count_user_orders_for_reward(
    pool: &MySqlPool,
    uid: u64,
    reward_id: u64,
) -> Result<u32, sqlx::Error> {
    let (n,): (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(num), 0) FROM phpyun_change \
         WHERE uid = ? AND gid = ?",
    )
        .bind(uid)
        .bind(reward_id)
        .fetch_one(pool)
        .await?;
    Ok(phpyun_core::numeric::saturating_count_u32(n))
}

pub async fn tx_insert_order(
    tx: &mut Transaction<'_, MySql>,
    o: &NewOrder<'_>,
    now: i64,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO phpyun_change \
         (uid, username, usertype, name, gid, linkman, linktel, body, integral, num, ctime, status) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0)",
    )
    .bind(o.uid)
    .bind(o.username)
    .bind(o.usertype)
    .bind(o.name)
    .bind(o.gid)
    .bind(o.linkman)
    .bind(o.linktel)
    .bind(o.address)
    .bind(o.integral)
    .bind(o.num)
    .bind(now)
    .execute(&mut **tx)
    .await?;
    Ok(res.last_insert_id())
}

pub struct NewOrder<'a> {
    pub uid: u64,
    pub username: &'a str,
    pub usertype: i32,
    pub gid: u64,
    pub name: &'a str,
    pub linkman: &'a str,
    pub linktel: &'a str,
    pub address: &'a str,
    pub integral: u32,
    pub num: u32,
}

pub async fn list_orders(
    pool: &MySqlPool,
    uid: Option<u64>,
    status: Option<i32>,
    offset: u64,
    limit: u64,
) -> Result<Vec<RedeemOrder>, sqlx::Error> {
    let mut sql = format!("SELECT {ORDER_FIELDS} FROM phpyun_change WHERE 1=1");
    if uid.is_some() {
        sql.push_str(" AND uid = ?");
    }
    if status.is_some() {
        sql.push_str(" AND status = ?");
    }
    sql.push_str(" ORDER BY id DESC LIMIT ? OFFSET ?");
    let mut q = sqlx::query_as::<_, RedeemOrder>(&sql);
    if let Some(u) = uid {
        q = q.bind(u);
    }
    if let Some(s) = status {
        q = q.bind(s);
    }
    q.bind(limit).bind(offset).fetch_all(pool).await
}

pub async fn count_orders(
    pool: &MySqlPool,
    uid: Option<u64>,
    status: Option<i32>,
) -> Result<u64, sqlx::Error> {
    let mut sql = String::from("SELECT COUNT(*) FROM phpyun_change WHERE 1=1");
    if uid.is_some() {
        sql.push_str(" AND uid = ?");
    }
    if status.is_some() {
        sql.push_str(" AND status = ?");
    }
    let mut q = sqlx::query_as::<_, (i64,)>(&sql);
    if let Some(u) = uid {
        q = q.bind(u);
    }
    if let Some(s) = status {
        q = q.bind(s);
    }
    let (n,) = q.fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn get_order(pool: &MySqlPool, id: u64) -> Result<Option<RedeemOrder>, sqlx::Error> {
    let sql = format!("SELECT {ORDER_FIELDS} FROM phpyun_change WHERE id = ?");
    sqlx::query_as::<_, RedeemOrder>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn tx_set_order_status(
    tx: &mut Transaction<'_, MySql>,
    id: u64,
    expected: i32,
    new_status: i32,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_change SET status = ? WHERE id = ? AND status = ?")
        .bind(new_status)
        .bind(id)
        .bind(expected)
        .execute(&mut **tx)
        .await?;
    Ok(res.rows_affected())
}

pub async fn count_class_names(pool: &MySqlPool, names: &[String]) -> Result<u64, sqlx::Error> {
    if names.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(format!(
        "SELECT COUNT(*) FROM phpyun_redeem_class WHERE {PREDICATE} AND name IN ("
    ));
    let mut sep = qb.separated(", ");
    for n in names {
        sep.push_bind(n);
    }
    qb.push(")");
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn php_delete_classes(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("DELETE FROM phpyun_redeem_class WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(") OR keyid IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn php_get_reward(pool: &MySqlPool, id: u64) -> Result<Option<PhpRewardRow>, sqlx::Error> {
    let sql = format!(
        "SELECT {PHP_REWARD_FIELDS} FROM phpyun_reward WHERE id = ? AND {PREDICATE} LIMIT 1"
    );
    sqlx::query_as(&sql).bind(id).fetch_optional(pool).await
}

pub struct PhpRewardSave<'a> {
    pub id: Option<u64>,
    pub name: &'a str,
    pub pic: Option<&'a str>,
    pub content: &'a str,
    pub integral: i32,
    pub stock: i32,
    pub restriction: i32,
    pub nid: u64,
    pub tnid: u64,
    pub status: i32,
    pub sort: i32,
    pub now: i64,
}

pub async fn php_save_reward(pool: &MySqlPool, a: &PhpRewardSave<'_>) -> Result<u64, sqlx::Error> {
    if let Some(id) = a.id.filter(|i| *i > 0) {
        let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("UPDATE phpyun_reward SET name = ");
        qb.push_bind(a.name);
        qb.push(", content = ");
        qb.push_bind(a.content);
        qb.push(", integral = ");
        qb.push_bind(a.integral);
        qb.push(", stock = ");
        qb.push_bind(a.stock);
        qb.push(", restriction = ");
        qb.push_bind(a.restriction);
        qb.push(", nid = ");
        qb.push_bind(a.nid);
        qb.push(", tnid = ");
        qb.push_bind(a.tnid);
        qb.push(", status = ");
        qb.push_bind(a.status);
        qb.push(", sort = ");
        qb.push_bind(a.sort);
        qb.push(", sdate = ");
        qb.push_bind(a.now);
        qb.push(", hot = 0");
        if let Some(pic) = a.pic {
            qb.push(", pic = ");
            qb.push_bind(pic);
        }
        qb.push(" WHERE id = ");
        qb.push_bind(id);
        qb.build().execute(pool).await?;
        Ok(id)
    } else {
        let pic = a.pic.unwrap_or("");
        let res = sqlx::query(
            "INSERT INTO phpyun_reward \
             (name, pic, content, integral, stock, num, restriction, nid, tnid, status, rec, hot, sdate, sort) \
             VALUES (?, ?, ?, ?, ?, 0, ?, ?, ?, ?, 0, 0, ?, ?)",
        )
        .bind(a.name)
        .bind(pic)
        .bind(a.content)
        .bind(a.integral)
        .bind(a.stock)
        .bind(a.restriction)
        .bind(a.nid)
        .bind(a.tnid)
        .bind(a.status)
        .bind(a.now)
        .bind(a.sort)
        .execute(pool)
        .await?;
        Ok(res.last_insert_id())
    }
}

pub async fn php_delete_rewards(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("DELETE FROM phpyun_reward WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

const PHP_CHANGE_FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    CAST(COALESCE(uid,0) AS UNSIGNED) AS uid, \
    COALESCE(username,'') AS username, \
    CAST(COALESCE(usertype,0) AS SIGNED) AS usertype, \
    COALESCE(name,'') AS name, \
    CAST(COALESCE(gid,0) AS UNSIGNED) AS gid, \
    CAST(COALESCE(integral,0) AS SIGNED) AS integral, \
    CAST(COALESCE(ctime,0) AS SIGNED) AS ctime, \
    CAST(COALESCE(num,0) AS SIGNED) AS num, \
    COALESCE(linktel,'') AS linktel, \
    COALESCE(linkman,'') AS linkman, \
    COALESCE(body,'') AS body, \
    CAST(COALESCE(status,0) AS SIGNED) AS status, \
    COALESCE(statusbody,'') AS statusbody, \
    COALESCE(express,'') AS express, \
    COALESCE(expnum,'') AS expnum";

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PhpChangeRow {
    pub id: u64,
    pub uid: u64,
    pub username: String,
    pub usertype: i32,
    pub name: String,
    pub gid: u64,
    pub integral: i32,
    pub ctime: i64,
    pub num: i32,
    pub linktel: String,
    pub linkman: String,
    pub body: String,
    pub status: i32,
    pub statusbody: String,
    pub express: String,
    pub expnum: String,
}

pub struct PhpChangeFilter<'a> {
    pub name_kw: Option<&'a str>,
    pub username_kw: Option<&'a str>,
    pub status: Option<i32>,
    pub time_min: Option<i64>,
    pub sort: &'a str,
    pub dir: &'a str,
}

fn push_change_where(qb: &mut QueryBuilder<'_, sqlx::MySql>, f: &PhpChangeFilter<'_>) {
    qb.push(" FROM phpyun_change WHERE 1=1");
    if let Some(kw) = f.name_kw.filter(|s| !s.is_empty()) {
        qb.push(" AND name LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    if let Some(kw) = f.username_kw.filter(|s| !s.is_empty()) {
        qb.push(" AND username LIKE ");
        qb.push_bind(format!("%{kw}%"));
    }
    if let Some(s) = f.status {
        qb.push(" AND COALESCE(status,0) = ");
        qb.push_bind(s);
    }
    if let Some(t) = f.time_min {
        qb.push(" AND ctime >= ");
        qb.push_bind(t);
    }
}

fn change_order(sort: &str, dir: &str) -> &'static str {
    let desc = !dir.eq_ignore_ascii_case("asc");
    match sort {
        "ctime" | "ctime_n" => {
            if desc {
                " ORDER BY ctime DESC, id DESC"
            } else {
                " ORDER BY ctime ASC, id ASC"
            }
        }
        "status" | "zt" => {
            if desc {
                " ORDER BY status DESC, id DESC"
            } else {
                " ORDER BY status ASC, id ASC"
            }
        }
        "integral" => {
            if desc {
                " ORDER BY integral DESC, id DESC"
            } else {
                " ORDER BY integral ASC, id ASC"
            }
        }
        _ => {
            if sort.is_empty() {
                " ORDER BY status ASC, id DESC"
            } else if desc {
                " ORDER BY id DESC"
            } else {
                " ORDER BY id ASC"
            }
        }
    }
}

pub async fn php_list_changes(
    pool: &MySqlPool,
    f: &PhpChangeFilter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<PhpChangeRow>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new(format!("SELECT {PHP_CHANGE_FIELDS}"));
    push_change_where(&mut qb, f);
    qb.push(change_order(f.sort, f.dir));
    qb.push(" LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    qb.build_query_as().fetch_all(pool).await
}

pub async fn php_count_changes(
    pool: &MySqlPool,
    f: &PhpChangeFilter<'_>,
) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT COUNT(*)");
    push_change_where(&mut qb, f);
    let (n,): (i64,) = qb.build_query_as().fetch_one(pool).await?;
    Ok(phpyun_core::numeric::nonnegative_count(n))
}

pub async fn php_get_change(pool: &MySqlPool, id: u64) -> Result<Option<PhpChangeRow>, sqlx::Error> {
    let sql = format!("SELECT {PHP_CHANGE_FIELDS} FROM phpyun_change WHERE id = ? LIMIT 1");
    sqlx::query_as(&sql).bind(id).fetch_optional(pool).await
}

pub async fn php_get_changes(
    pool: &MySqlPool,
    ids: &[u64],
) -> Result<Vec<PhpChangeRow>, sqlx::Error> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new(format!("SELECT {PHP_CHANGE_FIELDS} FROM phpyun_change WHERE id IN ("));
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    qb.build_query_as().fetch_all(pool).await
}

pub struct PhpChangeReview<'a> {
    pub status: i32,
    pub linkman: &'a str,
    pub linktel: &'a str,
    pub statusbody: &'a str,
    pub express: &'a str,
    pub expnum: &'a str,
}

pub async fn php_review_change(
    pool: &MySqlPool,
    id: u64,
    a: &PhpChangeReview<'_>,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_change SET status = ?, linkman = ?, linktel = ?, statusbody = ?, express = ?, expnum = ? WHERE id = ?",
    )
    .bind(a.status)
    .bind(a.linkman)
    .bind(a.linktel)
    .bind(a.statusbody)
    .bind(a.express)
    .bind(a.expnum)
    .bind(id)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}

pub async fn php_delete_changes(pool: &MySqlPool, ids: &[u64]) -> Result<u64, sqlx::Error> {
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("DELETE FROM phpyun_change WHERE id IN (");
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}

pub async fn php_adjust_reward_stock(
    pool: &MySqlPool,
    gid: u64,
    num: i32,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "UPDATE phpyun_reward SET stock = GREATEST(CAST(stock AS SIGNED) + ?, 0), \
         num = GREATEST(CAST(num AS SIGNED) - ?, 0) WHERE id = ?",
    )
    .bind(num)
    .bind(num)
    .bind(gid)
    .execute(pool)
    .await?;
    Ok(res.rows_affected())
}
