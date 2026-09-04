//! Top-level route assembly.
//!
//! ## Versioned
//! - `/v1/*` — current stable version
//! - `/v2/*` — latest version (only overrides endpoints with breaking changes; merges v1 for the rest)
//! - `/vN/*` — future N: just add one line `.nest("/vN", vN::router())`
//!
//! ## Unversioned
//! - `/health`, `/ready` — ops probes
//! - `/files/*` — static uploaded files (local FS backend for dev; prod uses CDN, mounting here is optional)
//! - `/api-docs/vN/openapi.json` — OpenAPI JSON (dev/test only; one spec per version)
//!
//! ## Middleware mounted on demand
//! - Global middleware is installed by `mw::install`;
//! - **Idempotency middleware** `idempotency::layer` is mounted only on the write-endpoint subtree (e.g. /v1/wap/upload).

use axum::Router;
use phpyun_core::{middleware as mw, route_rules::RouteRules, AppEnvironment, AppState};
use utoipa::openapi::OpenApi;

use crate::{common, openapi, v1, v2};

/// Per-path HTTP policy for the middleware stack.
///
/// Registering a new API namespace means adding one line here; the middleware
/// itself knows nothing about our URLs. GET exemptions come from the modules
/// that own the routes (see `v1::wap::wechat::GET_ALLOWED_PATHS`).
fn route_rules() -> RouteRules {
    RouteRules::new()
        .api_namespace("/v1")
        .api_namespace("/v2")
        .allow_get_all(v1::get_allowed_paths())
}

/// Extra GET exemptions from sibling API crates (e.g. none today for admin).
fn route_rules_with(extra_get: impl IntoIterator<Item = &'static str>) -> RouteRules {
    route_rules().allow_get_all(extra_get)
}

fn mount_api_docs<S>(
    router: Router<S>,
    env: AppEnvironment,
    extra_docs: Option<(&'static str, OpenApi)>,
) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    if env.is_dev_or_test() {
        router.merge(openapi::api_docs_router(extra_docs))
    } else {
        router
    }
}

pub fn build_router(cfg: &phpyun_core::Config, state: AppState) -> Router<AppState> {
    assemble(cfg, Router::new(), None, [], state)
}

/// Production assembly. `extra` is merged at the router root (typically the
/// admin crate's `/v1/admin` tree). `extra_docs` is an optional extra Swagger
/// spec (`(url, spec)`).
pub fn assemble(
    cfg: &phpyun_core::Config,
    extra: Router<AppState>,
    extra_docs: Option<(&'static str, OpenApi)>,
    extra_get: impl IntoIterator<Item = &'static str>,
    state: AppState,
) -> Router<AppState> {
    let api = Router::new()
        .nest("/v1", v1::router(state))
        .nest("/v2", v2::router())
        .nest("/callback", crate::callback::router())
        .merge(extra);
    let api = mount_api_docs(api, cfg.env, extra_docs);
    let api_with_mw = mw::install(api, cfg, route_rules_with(extra_get));
    Router::new().merge(common::router()).merge(api_with_mw)
}

/// State-aware variant kept for tests and in-process smoke probes that do not
/// mount the sibling admin crate.
pub fn build_router_with_state(cfg: &phpyun_core::Config, state: AppState) -> Router<AppState> {
    assemble(cfg, Router::new(), None, [], state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    async fn status_for(env: AppEnvironment, path: &str) -> StatusCode {
        mount_api_docs(Router::new(), env, None)
            .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
            .await
            .unwrap()
            .status()
    }

    #[tokio::test]
    async fn api_docs_are_available_in_dev_and_test() {
        for env in [AppEnvironment::Dev, AppEnvironment::Test] {
            assert_eq!(
                status_for(env, "/api-docs/v1/openapi.json").await,
                StatusCode::OK
            );
            assert_eq!(
                status_for(env, "/api-docs/v2/openapi.json").await,
                StatusCode::OK
            );
        }
    }

    #[tokio::test]
    async fn api_docs_do_not_exist_in_production() {
        for path in [
            "/api-docs/v1/openapi.json",
            "/api-docs/v2/openapi.json",
        ] {
            assert_eq!(
                status_for(AppEnvironment::Prod, path).await,
                StatusCode::NOT_FOUND,
                "production unexpectedly exposed {path}"
            );
        }
    }
}
