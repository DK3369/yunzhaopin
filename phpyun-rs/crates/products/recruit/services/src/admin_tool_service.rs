//! Additive admin tools: cron inventory, WeChat menu table, email/SMS send logs.

use phpyun_core::{AppResult, AppState, Paged, Pagination};
use phpyun_models::email_msg::{entity::EmailMsg, repo as email_msg_repo};
use phpyun_models::moblie_msg::{entity::MoblieMsg, repo as moblie_msg_repo};
use phpyun_models::wx_nav::{entity::WxNav, repo as wx_nav_repo};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CronJobItem {
    pub name: String,
    pub schedule: String,
    pub kind: String,
}

/// Jobs registered in `apps/server` `start_scheduler`. Read-only inventory (no enable switch).
pub fn list_cron_jobs() -> Vec<CronJobItem> {
    vec![
        CronJobItem {
            name: "expire_jobs".into(),
            schedule: "0 0 * * * *".into(),
            kind: "cron".into(),
        },
        CronJobItem {
            name: "purge_share_tokens".into(),
            schedule: "0 15 3 * * *".into(),
            kind: "cron".into(),
        },
        CronJobItem {
            name: "rotate_audit_log".into(),
            schedule: "0 30 3 * * *".into(),
            kind: "cron".into(),
        },
        CronJobItem {
            name: "purge_recycle_bin".into(),
            schedule: "0 45 3 * * *".into(),
            kind: "cron".into(),
        },
        CronJobItem {
            name: "db_pool_metrics".into(),
            schedule: "30s".into(),
            kind: "interval".into(),
        },
    ]
}

pub async fn list_wx_navs(state: &AppState) -> AppResult<Vec<WxNav>> {
    Ok(wx_nav_repo::list_all(state.db.reader()).await?)
}

pub async fn list_email_logs(state: &AppState, page: Pagination) -> AppResult<Paged<EmailMsg>> {
    let db = state.db.reader();
    let list = email_msg_repo::list_admin(db, page.offset, page.limit).await?;
    let total = email_msg_repo::count_admin(db).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn list_sms_logs(state: &AppState, page: Pagination) -> AppResult<Paged<MoblieMsg>> {
    let db = state.db.reader();
    let list = moblie_msg_repo::list_admin(db, page.offset, page.limit).await?;
    let total = moblie_msg_repo::count_admin(db).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}
