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

pub mod delete_guard;
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
            .merge(
                v1::router()
                    .layer(axum::middleware::from_fn_with_state(
                        state.clone(),
                        crate::delete_guard::layer,
                    ))
                    .layer(axum::middleware::from_fn_with_state(
                        state,
                        phpyun_core::admin_guard::layer,
                    )),
            ),
    )
}

pub fn openapi() -> utoipa::openapi::OpenApi {
    use utoipa::OpenApi;
    openapi::AdminDoc::openapi()
}

pub fn get_allowed_paths() -> Vec<&'static str> {
    Vec::new()
}

#[cfg(test)]
mod snapshot_tests {
    use std::path::PathBuf;

    #[test]
    fn admin_paths_match_repo_snapshot() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../../../../doc/snapshots/admin_paths.txt");
        let expected: Vec<String> = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
            .lines()
            .filter(|l| !l.is_empty())
            .map(str::to_string)
            .collect();
        let mut actual: Vec<_> = crate::openapi().paths.paths.keys().cloned().collect();
        actual.sort();
        assert_eq!(actual, expected);
        assert_eq!(actual.len(), 297);
    }
}
