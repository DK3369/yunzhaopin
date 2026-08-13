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
//! - `/docs`, `/api-docs/vN/openapi.json` — Swagger UI (dev/test only; one spec per version)
//!
//! ## Middleware mounted on demand
//! - Global middleware is installed by `mw::install`;
//! - **Idempotency middleware** `idempotency::layer` is mounted only on the write-endpoint subtree (e.g. /v1/wap/upload).

use axum::Router;
use phpyun_core::{middleware as mw, AppEnvironment, AppState};

use crate::{common, openapi, v1, v2};

/// Expose interactive API documentation only outside production. Keeping the
/// decision in the router means `/docs` and the raw OpenAPI JSON endpoints do
/// not exist at all when `APP_ENV=prod`; they are not merely hidden by the UI.
fn mount_api_docs<S>(router: Router<S>, env: AppEnvironment) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    if env.is_dev_or_test() {
        router.merge(openapi::swagger_ui())
    } else {
        router
    }
}

pub fn build_router(cfg: &phpyun_core::Config) -> Router<AppState> {
    // ---- Business APIs: full middleware stack (incl. IP rate limit / global concurrency limit / body size limit) ----
    let api = Router::new()
        .nest("/v1", v1::router())
        .nest("/v2", v2::router());
    let api = mount_api_docs(api, cfg.env);
    let api_with_mw = mw::install(api, cfg);

    // ---- Ops probes: **bypass rate limit / concurrency limit / body limit** (k8s LB probes hit these frequently) ----
    //
    // /health and /ready must respond reliably; if they get rate-limited even once, the LB will mark
    // the instance unhealthy and pull the entire process out of the load balancer — so these two
    // endpoints **must** run outside the middleware stack. This also avoids the
    // ConcurrencyLimitLayer making health checks queue up and time out under traffic spikes.
    Router::new().merge(common::router()).merge(api_with_mw)
}

/// State-aware variant — wires a router-level admin guard onto `/v1/admin/*`
/// in addition to everything `build_router` does. Production callers should
/// prefer this entry-point so an unguarded admin handler can never escape
/// the role check; per-handler `user.require_admin()` calls remain as a
/// defense-in-depth audit signal.
pub fn build_router_with_state(cfg: &phpyun_core::Config, state: AppState) -> Router<AppState> {
    let v1 = Router::new()
        .nest("/wap", v1::wap::router())
        .nest("/mcenter", v1::mcenter::router())
        .nest(
            "/admin",
            v1::admin::router().layer(axum::middleware::from_fn_with_state(
                state,
                phpyun_core::admin_guard::layer,
            )),
        );

    let api = Router::new().nest("/v1", v1).nest("/v2", v2::router());
    let api = mount_api_docs(api, cfg.env);
    let api_with_mw = mw::install(api, cfg);

    Router::new().merge(common::router()).merge(api_with_mw)
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
        mount_api_docs(Router::new(), env)
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
            assert_ne!(status_for(env, "/docs/").await, StatusCode::NOT_FOUND);
        }
    }

    #[tokio::test]
    async fn api_docs_do_not_exist_in_production() {
        for path in [
            "/docs",
            "/docs/",
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
