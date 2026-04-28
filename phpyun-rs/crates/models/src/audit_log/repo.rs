use super::entity::AuditLog;
use sqlx::{MySqlPool, QueryBuilder};

const FIELDS: &str =
    "id, actor_uid, actor_ip, actor_ua, action, target, success, meta, created_at";

pub struct AuditFilter<'a> {
    pub action_prefix: Option<&'a str>,
    pub actor_uid: Option<u64>,
    pub since: Option<i64>,
    pub until: Option<i64>,
}

pub async fn list(
    pool: &MySqlPool,
    f: &AuditFilter<'_>,
    offset: u64,
    limit: u64,
) -> Result<Vec<AuditLog>, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> = QueryBuilder::new("SELECT ");
    qb.push(FIELDS);
    qb.push(" FROM phpyun_rs_audit_log WHERE 1=1");
    push_filters(&mut qb, f);
    qb.push(" ORDER BY id DESC LIMIT ");
    qb.push_bind(limit);
    qb.push(" OFFSET ");
    qb.push_bind(offset);
    // The PHPyun-port `phpyun_rs_audit_log` table is optional — when the host
    // PHP install hasn't been provisioned with it, return an empty page rather
    // than 500.
    phpyun_core::db::ok_default_if_object_missing(
        qb.build_query_as::<AuditLog>().fetch_all(pool).await,
    )
}

pub async fn count(pool: &MySqlPool, f: &AuditFilter<'_>) -> Result<u64, sqlx::Error> {
    let mut qb: QueryBuilder<sqlx::MySql> =
        QueryBuilder::new("SELECT COUNT(*) FROM phpyun_rs_audit_log WHERE 1=1");
    push_filters(&mut qb, f);
    let res: Result<(i64,), _> = qb.build_query_as().fetch_one(pool).await;
    match res {
        Ok((n,)) => Ok(n.max(0) as u64),
        Err(e) if phpyun_core::db::is_missing_table(&e) => Ok(0),
        Err(e) => Err(e),
    }
}

/// Scheduled rotation: delete audit entries with `created_at < cutoff`.
pub async fn rotate(pool: &MySqlPool, cutoff: i64) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM phpyun_rs_audit_log WHERE created_at < ?")
        .bind(cutoff)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

fn push_filters<'a>(qb: &mut QueryBuilder<'a, sqlx::MySql>, f: &AuditFilter<'a>) {
    if let Some(a) = f.action_prefix {
        qb.push(" AND action LIKE ");
        qb.push_bind(format!("{a}%"));
    }
    if let Some(u) = f.actor_uid {
        qb.push(" AND actor_uid = ");
        qb.push_bind(u);
    }
    if let Some(s) = f.since {
        qb.push(" AND created_at >= ");
        qb.push_bind(s);
    }
    if let Some(u) = f.until {
        qb.push(" AND created_at <= ");
        qb.push_bind(u);
    }
}
