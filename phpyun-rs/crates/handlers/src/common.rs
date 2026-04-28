//! Version-independent "ops" endpoints: `/health`, `/ready`, `/robots.txt`,
//! `/dev/token`.
//!
//! These exist for k8s / load balancers / monitoring / search engines, not for
//! business clients — they have no version concept and are always mounted on
//! the root path, OUTSIDE the bot-block / rate-limit / concurrency middleware.

use axum::{extract::State, http::header, http::StatusCode, response::IntoResponse, routing::get, Router};
use phpyun_core::{json, AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
        .route("/robots.txt", get(robots_txt))
        .route("/dev/token", get(dev_token))
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

/// Returns long-lived dev/test tokens (one per role) in non-prod, or 404 in
/// prod. Used by Swagger UI bootstrap and by curl/Postman during development.
///
/// `token` defaults to the admin token (works for `/admin/*` plus everything
/// that doesn't role-check); the `tokens` map exposes all three roles so a
/// jobseeker-only endpoint like `/v1/mcenter/resume/expects` can be tested
/// with the matching token.
async fn dev_token() -> axum::response::Response {
    match phpyun_core::dev_token::tokens() {
        Some(t) => axum::Json(json::json!({
            "uid": 1,
            "ttl_years": 30,
            "default_role": "admin",
            "token": t.admin,
            "tokens": {
                "jobseeker": t.jobseeker,
                "employer": t.employer,
                "admin": t.admin,
            },
            "note": "dev/test only. Pick by role: jobseeker/employer/admin. Paste into Swagger Authorize.",
        }))
        .into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}
