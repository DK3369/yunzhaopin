//! Shared admin soft-delete: `deleted` TINYINT `0`=active, `1`=deleted.
//! List/detail queries must add [`PREDICATE`]. Physical `DELETE` is reserved
//! for logs, KV config, and recycle-bin purge.

use sqlx::{MySqlPool, QueryBuilder};

pub const PREDICATE: &str = "COALESCE(deleted,0)=0";

const TABLES: &[&str] = &[
    "phpyun_admin_announcement",
    "phpyun_admin_link",
    "phpyun_app_version",
    "phpyun_banner",
    "phpyun_city_class",
    "phpyun_comclass",
    "phpyun_company_rating",
    "phpyun_company_service",
    "phpyun_company_service_detail",
    "phpyun_company_show",
    "phpyun_cron",
    "phpyun_desc_class",
    "phpyun_description",
    "phpyun_domain",
    "phpyun_evaluate",
    "phpyun_evaluate_group",
    "phpyun_evaluate_leave_message",
    "phpyun_evaluate_log",
    "phpyun_gongzhao",
    "phpyun_hot_key",
    "phpyun_hotjob",
    "phpyun_job_class",
    "phpyun_navmap",
    "phpyun_news_base",
    "phpyun_outside",
    "phpyun_partclass",
    "phpyun_q_class",
    "phpyun_question",
    "phpyun_reason",
    "phpyun_redeem_class",
    "phpyun_resume_show",
    "phpyun_reward",
    "phpyun_special_com",
    "phpyun_toolbox_class",
    "phpyun_toolbox_doc",
    "phpyun_userclass",
    "phpyun_wxnav",
    "phpyun_wxpub_temps",
    "phpyun_zhaopinhui_space",
];

const COLS: &[&str] = &[
    "id", "gid", "cid", "examid", "uid", "pid", "keyid", "nbid", "type", "sid",
];

fn assert_ident(name: &str, allowed: &[&str]) {
    if !allowed.contains(&name) {
        panic!("soft_delete: identifier not allowed: {name}");
    }
}

fn col_sql(col: &str) -> &'static str {
    if col == "type" {
        assert_ident(col, COLS);
        return "`type`";
    }
    COLS.iter()
        .copied()
        .find(|c| *c == col)
        .unwrap_or_else(|| panic!("soft_delete: identifier not allowed: {col}"))
}

pub async fn mark_ids(pool: &MySqlPool, table: &'static str, ids: &[u64]) -> Result<u64, sqlx::Error> {
    mark_col_in(pool, table, "id", ids).await
}

pub async fn mark_id(pool: &MySqlPool, table: &'static str, id: u64) -> Result<u64, sqlx::Error> {
    mark_ids(pool, table, &[id]).await
}

pub async fn mark_col_in(
    pool: &MySqlPool,
    table: &'static str,
    col: &'static str,
    ids: &[u64],
) -> Result<u64, sqlx::Error> {
    assert_ident(table, TABLES);
    let col = col_sql(col);
    if ids.is_empty() {
        return Ok(0);
    }
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new(format!(
        "UPDATE {table} SET deleted=1 WHERE COALESCE(deleted,0)=0 AND {col} IN ("
    ));
    let mut sep = qb.separated(", ");
    for id in ids {
        sep.push_bind(*id);
    }
    qb.push(")");
    Ok(qb.build().execute(pool).await?.rows_affected())
}
