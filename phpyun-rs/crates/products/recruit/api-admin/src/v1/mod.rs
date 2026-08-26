//! Admin backend (`/v1/admin/*`) — every route calls `require_admin()` first; non-admin requests get 403.

pub mod account_logout;
pub mod ads;
pub mod announcements;
pub mod app_versions;
pub mod articles;
pub mod audit_log;
pub mod broadcasts;
pub mod categories;
pub mod companies;
pub mod company_cert;
pub mod countries;
pub mod cron;
pub mod dashboard;
pub mod descriptions;
pub mod dict_i18n;
pub mod feedback;
pub mod finance;
pub mod friend_links;
pub mod jobs;
pub mod merge;
pub mod msg_logs;
pub mod nav;
pub mod once_jobs;
pub mod ops;
pub mod orders;
pub mod parts;
pub mod questions;
pub mod rbac;
pub mod recycle;
pub mod redeem;
pub mod regions;
pub mod reports;
pub mod resumes;
pub mod site_settings;
pub mod tiny;
pub mod users;
pub mod usertype_change;
pub mod warnings;
pub mod wx_nav;

use axum::Router;
use phpyun_core::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(users::routes())
        .merge(reports::routes())
        .merge(feedback::routes())
        .merge(jobs::routes())
        .merge(company_cert::routes())
        .merge(site_settings::routes())
        .merge(ads::routes())
        .merge(warnings::routes())
        .merge(audit_log::routes())
        .merge(broadcasts::routes())
        .merge(categories::routes())
        .merge(app_versions::routes())
        .merge(nav::routes())
        .merge(dashboard::routes())
        .merge(orders::routes())
        .merge(recycle::routes())
        .merge(redeem::routes())
        .merge(regions::routes())
        .merge(countries::routes())
        .merge(descriptions::routes())
        .merge(merge::routes())
        .merge(account_logout::routes())
        .merge(usertype_change::routes())
        .merge(dict_i18n::routes())
        .merge(articles::routes())
        .merge(announcements::routes())
        .merge(questions::routes())
        .merge(parts::routes())
        .merge(once_jobs::routes())
        .merge(tiny::routes())
        .merge(friend_links::routes())
        .merge(ops::routes())
        .merge(companies::routes())
        .merge(resumes::routes())
        .merge(finance::routes())
        .merge(rbac::routes())
        .merge(cron::routes())
        .merge(wx_nav::routes())
        .merge(msg_logs::routes())
}
