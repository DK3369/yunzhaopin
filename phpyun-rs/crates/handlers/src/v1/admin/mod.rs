//! Admin backend (`/v1/admin/*`) — every route calls `require_admin()` first; non-admin requests get 403.

pub mod account_logout;
pub mod ads;
pub mod app_versions;
pub mod audit_log;
pub mod broadcasts;
pub mod categories;
pub mod company_cert;
pub mod dashboard;
pub mod descriptions;
pub mod dict_i18n;
pub mod feedback;
pub mod jobs;
pub mod merge;
pub mod nav;
pub mod orders;
pub mod recycle;
pub mod redeem;
pub mod reports;
pub mod site_settings;
pub mod users;
pub mod usertype_change;
pub mod warnings;

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
        .merge(descriptions::routes())
        .merge(merge::routes())
        .merge(account_logout::routes())
        .merge(usertype_change::routes())
        .merge(dict_i18n::routes())
}
