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
//! - `/docs`, `/api-docs/vN/openapi.json` — Swagger UI (one spec per version)
//!
//! ## Middleware mounted on demand
//! - Global middleware is installed by `mw::install`;
//! - **Idempotency middleware** `idempotency::layer` is mounted only on the write-endpoint subtree (e.g. /v1/wap/upload).

use axum::Router;
use phpyun_core::{middleware as mw, AppState};

use crate::{common, openapi, v1, v2};

pub fn build_router(cfg: &phpyun_core::Config) -> Router<AppState> {
    // ---- Business APIs: full middleware stack (incl. IP rate limit / global concurrency limit / body size limit) ----
    let api = Router::new()
        .nest("/v1", v1::router())
        .nest("/v2", v2::router())
        .merge(openapi::swagger_ui());
    let api_with_mw = mw::install(api, cfg);

    // ---- Ops probes: **bypass rate limit / concurrency limit / body limit** (k8s LB probes hit these frequently) ----
    //
    // /health and /ready must respond reliably; if they get rate-limited even once, the LB will mark
    // the instance unhealthy and pull the entire process out of the load balancer — so these two
    // endpoints **must** run outside the middleware stack. This also avoids the
    // ConcurrencyLimitLayer making health checks queue up and time out under traffic spikes.
    Router::new()
        .merge(common::router())
        .merge(api_with_mw)
}

/// State-aware variant — wires a router-level admin guard onto `/v1/admin/*`
/// in addition to everything `build_router` does. Production callers should
/// prefer this entry-point so an unguarded admin handler can never escape
/// the role check; per-handler `user.require_admin()` calls remain as a
/// defense-in-depth audit signal.
pub fn build_router_with_state(
    cfg: &phpyun_core::Config,
    state: AppState,
) -> Router<AppState> {
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

    let api = Router::new()
        .nest("/v1", v1)
        .nest("/v2", v2::router())
        .merge(openapi::swagger_ui());
    let api_with_mw = mw::install(api, cfg);

    Router::new()
        .merge(common::router())
        .merge(api_with_mw)
}
