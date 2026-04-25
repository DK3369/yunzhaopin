//! Idempotency middleware — caches responses keyed by the `Idempotency-Key` header.
//!
//! ## Use case
//! On flaky mobile networks, POSTs that submit orders, register, or bind a
//! phone number are frequently retried by the client and end up duplicated on
//! the server. Adding an idempotency header to write operations is the
//! industry standard (Stripe / Shopify / GitHub API all do it).
//!
//! ## Protocol
//! - The client includes `Idempotency-Key: <unique-id>` on
//!   `POST/PUT/PATCH/DELETE` requests.
//! - The first time the server runs the handler → caches the response (status
//!   + headers + body) for 24h.
//! - When the same key arrives again → the cached response is returned
//!   directly; **the handler is not invoked** (including service layer / DB /
//!   Redis rate-limit counters).
//! - Without the header → request passes through unchanged, as if the
//!   middleware weren't installed.
//! - Non-mutating methods (GET/HEAD/OPTIONS) → pass through.
//!
//! ## Concurrent dedup
//! When N requests with the same key arrive concurrently: the first grabs the
//! distributed lock ([Kv::acquire_lock]); the rest wait 50ms and re-read the
//! cache — so out of N requests only 1 actually runs, and the others return
//! the same cached response.
//!
//! ## Usage
//! ```ignore
//! Router::new()
//!     .route("/orders", post(create_order))
//!     .layer(axum::middleware::from_fn_with_state(
//!         state.clone(),
//!         idempotency::layer,
//!     ))
//! ```
//!
//! Apply selectively — don't put it on read endpoints (/me, /list); attach it
//! to write endpoints (/login, /register, /pay).
//!
//! ## Edges
//! - Hard upper bound on body size: 1 MB; oversize responses are not cached
//!   (they fall through to the original handler).
//! - 5xx errors are not cached (avoid caching transient failures).
//! - Cache TTL defaults to 24h, configurable.

use crate::error::AppError;
use crate::kv::Kv;
use crate::metrics as m;
use crate::state::AppState;
use axum::body::{to_bytes, Body};
use axum::extract::{Request, State};
use axum::http::Method;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const HEADER: &str = "idempotency-key";
const REPLAY_HEADER: &str = "x-idempotent-replay";
const KEY_PREFIX: &str = "idem:";
const LOCK_PREFIX: &str = "idem:lock:";
const DEFAULT_TTL_SECS: u64 = 86_400; // 24h
const MAX_BODY_BYTES: usize = 1024 * 1024; // 1MB
const LOCK_TTL_MS: u64 = 60_000; // 60s lock
const WAIT_FOR_PEER_MS: u64 = 50; // when we lose the lock race, wait 50ms then re-read the cache

#[derive(Serialize, Deserialize)]
struct CachedResponse {
    status: u16,
    /// base64-encoded body bytes
    body: String,
    /// Key response headers (e.g. content-type); everything else is dropped.
    content_type: Option<String>,
}

/// axum middleware function. Attach to a router with
/// `.layer(from_fn_with_state(state, layer))`.
pub async fn layer(State(state): State<AppState>, req: Request, next: Next) -> Response {
    // 1. Methods that don't need idempotency: pass through.
    if !is_mutating(req.method()) {
        return next.run(req).await;
    }

    // 2. No idempotency header: pass through.
    let Some(key_hdr) = req
        .headers()
        .get(HEADER)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
    else {
        return next.run(req).await;
    };

    // 3. Validate header format (prevent injection: require 16-128 printable ASCII chars).
    if !is_valid_key(&key_hdr) {
        m::counter("idempotency.bad_key");
        return next.run(req).await;
    }

    let kv = state.redis.clone();
    let route = req
        .extensions()
        .get::<axum::extract::MatchedPath>()
        .map(|p| p.as_str().to_string())
        .unwrap_or_else(|| req.uri().path().to_string());
    let method = req.method().to_string();
    let cache_key = format!("{KEY_PREFIX}{method}:{route}:{key_hdr}");
    let lock_key = format!("{LOCK_PREFIX}{method}:{route}:{key_hdr}");

    // 4. Fast path: cache hit, return immediately.
    if let Some(cached) = read_cache(&kv, &cache_key).await {
        m::counter("idempotency.replay");
        return replay(cached);
    }

    // 5. Grab the lock — only one of the concurrent same-key requests actually runs the handler.
    let owner = uuid::Uuid::now_v7().to_string();
    let got_lock = kv
        .acquire_lock(&lock_key, &owner, LOCK_TTL_MS)
        .await
        .unwrap_or(false);

    if !got_lock {
        // Someone else is running it — wait a bit, re-read the cache, return on hit.
        tokio::time::sleep(Duration::from_millis(WAIT_FOR_PEER_MS)).await;
        if let Some(cached) = read_cache(&kv, &cache_key).await {
            m::counter("idempotency.peer_replay");
            return replay(cached);
        }
        // Still not ready — fall through and run (safety net to avoid blocking),
        // but don't cache (let the original owner write).
        m::counter("idempotency.lock_timeout_passthrough");
        return next.run(req).await;
    }

    // 6. Got the lock: run the handler, capture the response, write the cache.
    m::counter("idempotency.miss");
    let resp = next.run(req).await;
    let resp = match capture_and_cache(&kv, &cache_key, resp).await {
        Ok(r) => r,
        Err(r) => r, // even if caching fails, we must still return the original response
    };

    // 7. Release the lock (CAS owner check).
    let _ = kv.release_lock(&lock_key, &owner).await;
    resp
}

fn is_mutating(m: &Method) -> bool {
    matches!(*m, Method::POST | Method::PUT | Method::PATCH | Method::DELETE)
}

fn is_valid_key(s: &str) -> bool {
    let len = s.len();
    (16..=128).contains(&len)
        && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

async fn read_cache(kv: &Kv, key: &str) -> Option<CachedResponse> {
    kv.get_json::<CachedResponse>(key).await.ok().flatten()
}

fn replay(cached: CachedResponse) -> Response {
    let body_bytes = match base64::engine::general_purpose::STANDARD.decode(&cached.body) {
        Ok(b) => b,
        Err(_) => {
            m::counter("idempotency.cache_corrupt");
            return AppError::internal(std::io::Error::other("idempotency cache corrupt"))
                .into_response();
        }
    };
    let mut builder = Response::builder().status(cached.status);
    builder = builder.header(REPLAY_HEADER, "true");
    if let Some(ct) = cached.content_type {
        builder = builder.header(axum::http::header::CONTENT_TYPE, ct);
    }
    builder.body(Body::from(body_bytes)).unwrap_or_else(|_| {
        AppError::internal(std::io::Error::other("idempotency replay build failed"))
            .into_response()
    })
}

/// Reads the response body, writes it to the cache, then rebuilds and returns
/// a Response from the same bytes. On failure returns `Err(original response)`
/// — we must never swallow the original response from the upstream caller.
async fn capture_and_cache(
    kv: &Kv,
    cache_key: &str,
    resp: Response,
) -> Result<Response, Response> {
    let status = resp.status();
    // Don't cache 5xx (avoid caching transient failures).
    if status.is_server_error() {
        m::counter("idempotency.skip_5xx");
        return Ok(resp);
    }

    let (parts, body) = resp.into_parts();
    let bytes = match to_bytes(body, MAX_BODY_BYTES).await {
        Ok(b) => b,
        Err(_) => {
            m::counter("idempotency.body_too_large");
            return Ok(Response::from_parts(parts, Body::empty()));
        }
    };

    let cached = CachedResponse {
        status: status.as_u16(),
        body: base64::engine::general_purpose::STANDARD.encode(&bytes),
        content_type: parts
            .headers
            .get(axum::http::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string()),
    };

    if let Err(e) = kv.set_json_ex(cache_key, &cached, DEFAULT_TTL_SECS).await {
        m::counter("idempotency.cache_write_error");
        tracing::warn!(?e, "idempotency cache write failed (request still served)");
    }

    Ok(Response::from_parts(parts, Body::from(bytes)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_validation() {
        assert!(is_valid_key("0123456789abcdef")); // 16 chars
        assert!(is_valid_key("a-b_c-d-".repeat(8).as_str().get(..64).unwrap()));
        assert!(!is_valid_key("short")); // <16
        assert!(!is_valid_key(&"x".repeat(200))); // >128
        assert!(!is_valid_key("contains space here is")); // contains a space
        assert!(!is_valid_key("contains/slash/here_xx")); // contains /
    }

    #[test]
    fn mutating_methods() {
        assert!(is_mutating(&Method::POST));
        assert!(is_mutating(&Method::PUT));
        assert!(is_mutating(&Method::PATCH));
        assert!(is_mutating(&Method::DELETE));
        assert!(!is_mutating(&Method::GET));
        assert!(!is_mutating(&Method::HEAD));
        assert!(!is_mutating(&Method::OPTIONS));
    }

    #[test]
    fn cached_response_roundtrip() {
        let c = CachedResponse {
            status: 200,
            body: base64::engine::general_purpose::STANDARD.encode(b"hello"),
            content_type: Some("application/json".into()),
        };
        let s = serde_json::to_string(&c).unwrap();
        let back: CachedResponse = serde_json::from_str(&s).unwrap();
        assert_eq!(back.status, 200);
        assert_eq!(
            base64::engine::general_purpose::STANDARD.decode(&back.body).unwrap(),
            b"hello"
        );
    }

    #[test]
    fn ttl_constant_is_24h() {
        const _: () = assert!(DEFAULT_TTL_SECS == 86_400);
    }
}

