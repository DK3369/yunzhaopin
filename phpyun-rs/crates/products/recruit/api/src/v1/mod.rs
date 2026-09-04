//! API **v1** — stable release used by the initial client cohort (App + Web).
//!
//! Admin lives in the sibling crate `phpyun-api-admin`.

pub mod mcenter;
pub mod wap;

use axum::Router;
use phpyun_core::AppState;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/wap", wap::router())
        .nest("/mcenter", mcenter::router())
        .layer(axum::middleware::from_fn_with_state(
            state,
            wap::site_gate_layer,
        ))
}

/// Paths in this version that accept `GET` despite the POST-only convention.
pub fn get_allowed_paths() -> Vec<&'static str> {
    wap::get_allowed_paths()
}
