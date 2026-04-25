//! API **v1** — stable release used by the initial client cohort.

pub mod admin;
pub mod mcenter;
pub mod wap;

use axum::Router;
use phpyun_core::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/wap", wap::router())
        .nest("/mcenter", mcenter::router())
        .nest("/admin", admin::router())
}
