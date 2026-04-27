//! API **v1** — stable release used by the initial client cohort.
//!
//! Admin endpoints (`/v1/admin/*`) are gated by a router-level guard
//! (`phpyun_core::admin_guard::layer`) so an unguarded handler can never
//! escape the role check; per-handler `user.require_admin()` calls remain
//! as a defense-in-depth audit signal.

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

