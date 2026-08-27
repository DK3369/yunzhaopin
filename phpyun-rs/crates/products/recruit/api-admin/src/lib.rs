//! Admin HTTP adapters (`/v1/admin/*`).
//!
//! Split from `phpyun-handlers` so App (`/v1/wap`, `/v1/mcenter`) and admin
//! compile independently. Business logic still lives in `phpyun-services`.
//!
//! ## Architecture rules (same as handlers)
//!
//! Handlers parse input, call services, map `ApiResponse`. Forbidden:
//! `sqlx` / `redis` / `moka` / `reqwest` / business rules.
//!
//! The admin JWT role check is applied inside [`router`] — callers cannot
//! obtain an unguarded admin tree.

pub mod dto;
pub mod openapi;
pub mod v1;

use axum::Router;
use phpyun_core::AppState;

/// Admin routes nested at `/v1/admin`. Login is public; everything else
/// is wrapped in the router-level admin guard.
pub fn router(state: AppState) -> Router<AppState> {
    Router::new().nest(
        "/v1/admin",
        Router::new()
            .merge(v1::auth::public_routes())
            .merge(v1::router().layer(axum::middleware::from_fn_with_state(
                state,
                phpyun_core::admin_guard::layer,
            ))),
    )
}

pub fn openapi() -> utoipa::openapi::OpenApi {
    use utoipa::OpenApi;
    openapi::AdminDoc::openapi()
}

pub fn get_allowed_paths() -> Vec<&'static str> {
    Vec::new()
}
