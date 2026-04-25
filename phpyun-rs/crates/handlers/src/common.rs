//! Version-independent "ops" endpoints: `/health`, `/ready`, `/robots.txt`.
//!
//! These exist for k8s / load balancers / monitoring / search engines, not for
//! business clients — they have no version concept and are always mounted on
//! the root path, OUTSIDE the bot-block / rate-limit / concurrency middleware.

use axum::{extract::State, http::header, response::IntoResponse, routing::get, Router};
use phpyun_core::{json, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
        .route("/robots.txt", get(robots_txt))
}

/// Depends on no external resource; just proves the process is alive.
async fn health() -> axum::Json<json::Value> {
    axum::Json(json::json!({ "ok": true }))
}

/// Ready only when both DB and Redis are OK. db.healthy() has a 1s cache to absorb high-frequency k8s probes.
async fn ready(State(state): State<AppState>) -> axum::Json<json::Value> {
    let (db_ok, redis_ok) = tokio::join!(state.db.healthy(), state.redis.ping());
    axum::Json(json::json!({
        "db": db_ok,
        "redis": redis_ok,
        "overall": db_ok && redis_ok,
    }))
}

/// Blanket Disallow — this is an API, not a public site. Polite crawlers obey
/// this; rude ones are stopped at the UA blacklist in core::middleware.
async fn robots_txt() -> impl IntoResponse {
    const BODY: &str = "User-agent: *\nDisallow: /\n";
    (
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        BODY,
    )
}
