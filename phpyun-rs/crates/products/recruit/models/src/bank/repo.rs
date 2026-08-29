//! PHPYun `phpyun_bank`. PHP `config.model` getBankList / addBank / upBank / delBank
//! uses physical DELETE (not `deleted`).

use super::entity::BankAccount;
use sqlx::MySqlPool;

const FIELDS: &str = "\
    CAST(id AS UNSIGNED) AS id, \
    COALESCE(name, '') AS name, \
    COALESCE(bank_name, '') AS bank_name, \
    COALESCE(bank_number, '') AS bank_number, \
    COALESCE(bank_address, '') AS bank_address";

pub async fn list_all(pool: &MySqlPool) -> Result<Vec<BankAccount>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_bank ORDER BY id ASC");
    sqlx::query_as::<_, BankAccount>(&sql).fetch_all(pool).await
}

pub async fn find_by_number(
    pool: &MySqlPool,
    bank_number: &str,
) -> Result<Option<BankAccount>, sqlx::Error> {
    let sql = format!("SELECT {FIELDS} FROM phpyun_bank WHERE bank_number = ? LIMIT 1");
    sqlx::query_as::<_, BankAccount>(&sql)
        .bind(bank_number)
        .fetch_optional(pool)
        .await
}

pub struct BankUpsert<'a> {
    pub id: Option<u64>,
    pub name: &'a str,
    pub bank_name: &'a str,
    pub bank_number: &'a str,
    pub bank_address: &'a str,
}

pub async fn upsert(pool: &MySqlPool, a: BankUpsert<'_>) -> Result<u64, sqlx::Error> {
    if let Some(id) = a.id.filter(|i| *i > 0) {
        sqlx::query(
            "UPDATE phpyun_bank SET name=?, bank_name=?, bank_number=?, bank_address=? WHERE id=?",
        )
        .bind(a.name)
        .bind(a.bank_name)
        .bind(a.bank_number)
        .bind(a.bank_address)
        .bind(id)
        .execute(pool)
        .await?;
        return Ok(id);
    }
    let res = sqlx::query(
        "INSERT INTO phpyun_bank (name, bank_name, bank_number, bank_address) VALUES (?, ?, ?, ?)",
    )
    .bind(a.name)
    .bind(a.bank_name)
    .bind(a.bank_number)
    .bind(a.bank_address)
    .execute(pool)
    .await?;
    Ok(res.last_insert_id())
}

pub async fn delete(pool: &MySqlPool, id: u64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_bank WHERE id=?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
