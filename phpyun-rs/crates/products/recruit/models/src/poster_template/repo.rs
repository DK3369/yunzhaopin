use super::entity::{PosterKind, PosterTemplate};
use serde::Serialize;
use sqlx::{FromRow, MySqlPool};

const FIELDS: &str = "id, title, pic, `type`, isopen, sort, num, config_pos";

pub async fn list_by_kind(
    pool: &MySqlPool,
    kind: PosterKind,
) -> Result<Vec<PosterTemplate>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_admin_jobwhb \
         WHERE `type` = ? AND isopen = 1 ORDER BY num DESC, sort DESC, id DESC"
    );
    sqlx::query_as::<_, PosterTemplate>(&sql)
        .bind(kind.code())
        .fetch_all(pool)
        .await
}

pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<Option<PosterTemplate>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_admin_jobwhb WHERE id = ? LIMIT 1");
    sqlx::query_as::<_, PosterTemplate>(&sql)
        .bind(id)
        .fetch_optional(pool)
        .await
}

/// Fetch the first (hottest) enabled template for a given kind; used as
/// the default template when the `hb` parameter is omitted.
pub async fn default_for_kind(
    pool: &MySqlPool,
    kind: PosterKind,
) -> Result<Option<PosterTemplate>, sqlx::Error> {
    let sql = format!(
        "SELECT {FIELDS} FROM phpyun_admin_jobwhb \
         WHERE `type` = ? AND isopen = 1 ORDER BY sort DESC, num DESC, id DESC LIMIT 1"
    );
    sqlx::query_as::<_, PosterTemplate>(&sql)
        .bind(kind.code())
        .fetch_optional(pool)
        .await
}

/// Increment usage count.
pub async fn incr_num(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE phpyun_admin_jobwhb SET num = num + 1 WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct AdminWhbRow {
    #[sqlx(try_from = "i32")]
    pub id: u64,
    pub name: String,
    pub pic: String,
    pub sort: i32,
    pub isopen: i32,
    pub r#type: i32,
    pub num: i32,
    pub style: i32,
}

pub async fn list_admin_by_type(pool: &MySqlPool, typ: i32) -> Result<Vec<AdminWhbRow>, sqlx::Error> {
    sqlx::query_as::<_, AdminWhbRow>(
        "SELECT id, COALESCE(name,'') AS name, COALESCE(pic,'') AS pic, \
         COALESCE(sort,0) AS sort, COALESCE(isopen,0) AS isopen, COALESCE(`type`,0) AS `type`, \
         COALESCE(num,0) AS num, COALESCE(style,0) AS style \
         FROM phpyun_admin_jobwhb WHERE `type` = ? ORDER BY sort DESC, id DESC",
    )
    .bind(typ)
    .fetch_all(pool)
    .await
}

pub async fn set_open_ids(pool: &MySqlPool, typ: i32, open_ids: &[u64]) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE phpyun_admin_jobwhb SET isopen = 0 WHERE `type` = ?")
        .bind(typ)
        .execute(pool)
        .await?;
    for id in open_ids {
        sqlx::query("UPDATE phpyun_admin_jobwhb SET isopen = 1 WHERE id = ? AND `type` = ?")
            .bind(*id)
            .bind(typ)
            .execute(pool)
            .await?;
    }
    Ok(())
}

pub async fn delete_whb(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_admin_jobwhb WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub struct WhbSave<'a> {
    pub id: Option<u64>,
    pub name: &'a str,
    pub pic: Option<&'a str>,
    pub sort: i32,
    pub isopen: i32,
    pub r#type: i32,
    pub num: i32,
    pub style: i32,
}

pub async fn upsert_whb(pool: &MySqlPool, s: WhbSave<'_>) -> Result<u64, sqlx::Error> {
    if let Some(id) = s.id.filter(|n| *n > 0) {
        if let Some(pic) = s.pic.filter(|p| !p.is_empty()) {
            sqlx::query(
                "UPDATE phpyun_admin_jobwhb SET name=?, pic=?, sort=?, isopen=?, `type`=?, num=?, style=? WHERE id=?",
            )
            .bind(s.name)
            .bind(pic)
            .bind(s.sort)
            .bind(s.isopen)
            .bind(s.r#type)
            .bind(s.num)
            .bind(s.style)
            .bind(id)
            .execute(pool)
            .await?;
        } else {
            sqlx::query(
                "UPDATE phpyun_admin_jobwhb SET name=?, sort=?, isopen=?, `type`=?, num=?, style=? WHERE id=?",
            )
            .bind(s.name)
            .bind(s.sort)
            .bind(s.isopen)
            .bind(s.r#type)
            .bind(s.num)
            .bind(s.style)
            .bind(id)
            .execute(pool)
            .await?;
        }
        return Ok(id);
    }
    Ok(sqlx::query(
        "INSERT INTO phpyun_admin_jobwhb (name, pic, sort, isopen, `type`, num, style) VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(s.name)
    .bind(s.pic.unwrap_or(""))
    .bind(s.sort)
    .bind(s.isopen)
    .bind(s.r#type)
    .bind(s.num)
    .bind(s.style)
    .execute(pool)
    .await?
    .last_insert_id())
}
