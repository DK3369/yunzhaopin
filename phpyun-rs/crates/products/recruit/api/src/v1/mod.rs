//! API **v1** — stable release used by the initial client cohort (App + Web).
//!
//! Admin lives in the sibling crate `phpyun-api-admin`.

pub mod mcenter;
pub mod wap;

use axum::Router;
use phpyun_core::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/wap", wap::router())
        .nest("/mcenter", mcenter::router())
}

/// Paths in this version that accept `GET` despite the POST-only convention.
pub fn get_allowed_paths() -> Vec<&'static str> {
    wap::get_allowed_paths()
}
