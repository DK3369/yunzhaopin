//! Middleware stack.
//!
//! Order (outer → inner):
//!  1. TraceLayer             — structured span: method / path / status / latency
//!  2. latency_metrics        — record histogram at request end
//!  3. SetRequestIdLayer      — fill in `x-request-id` on ingress
//!  4. PropagateRequestIdLayer— write the id back into the response headers
//!  5. CorsLayer              — origin whitelist read from `Config`
//!  6. CompressionLayer       — response compression
//!  7. GovernorLayer          — per-IP token-bucket rate limiting
//!  8. TimeoutLayer           — single-request 408 timeout
//!  9. ConcurrencyLimitLayer  — process-level backpressure
//! 10. RequestBodyLimitLayer — request-body size cap

use crate::config::Config;
use crate::route_rules::RouteRules;
use axum::{
    extract::{MatchedPath, Request, State},
    http::{header, HeaderName, HeaderValue, Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json, Router,
};
use serde_json::json;
use std::{sync::Arc, time::Duration};
use tower::limit::ConcurrencyLimitLayer;
use tower_governor::{governor::GovernorConfigBuilder, GovernorError, GovernorLayer};
use tower_http::{
    compression::CompressionLayer,
    cors::{AllowOrigin, CorsLayer},
    limit::RequestBodyLimitLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    set_header::SetResponseHeaderLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

/// Install the cross-cutting middleware stack.
///
/// `rules` carries the per-path policy the stack needs — which prefixes are
/// business APIs and which exact paths may be reached with `GET`. It is
/// supplied by the router rather than hardcoded here so this crate stays free
/// of product-specific URLs.
pub fn install<S>(router: Router<S>, cfg: &Config, rules: RouteRules) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let cors = build_cors(cfg);
    let rules = Arc::new(rules);
    let request_id_header = axum::http::HeaderName::from_static("x-request-id");

    // Per-IP token bucket.
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(cfg.rate_limit_per_second)
            .burst_size(cfg.rate_limit_burst)
            .finish()
            .expect("invalid governor config"),
    );

    // Per-request span: method/path/status is auto-added to tracing and also
    // promoted to metric labels.
    let trace = TraceLayer::new_for_http()
        .make_span_with(|req: &axum::http::Request<_>| {
            let path = req
                .extensions()
                .get::<MatchedPath>()
                .map(|p| p.as_str())
                .unwrap_or_else(|| req.uri().path());
            tracing::info_span!(
                "http",
                method = %req.method(),
                path = path,
                status = tracing::field::Empty,
            )
        })
        .on_response(
            |resp: &Response<_>, latency: std::time::Duration, span: &tracing::Span| {
                span.record("status", resp.status().as_u16());
                tracing::debug!(latency_ms = latency.as_millis() as u64, "response");
            },
        );

    router
        // Innermost: request-body limit + backpressure.
        .layer(RequestBodyLimitLayer::new(cfg.max_body_mb * 1024 * 1024))
        .layer(ConcurrencyLimitLayer::new(cfg.global_concurrency_limit))
        .layer(TimeoutLayer::with_status_code(
            axum::http::StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(cfg.request_timeout_secs),
        ))
        .layer(GovernorLayer::new(governor_conf).error_handler(governor_error_response))
        .layer(CompressionLayer::new())
        .layer(cors)
        // Security response headers (added to every response).
        .layer(security_headers())
        .layer(PropagateRequestIdLayer::new(request_id_header.clone()))
        .layer(SetRequestIdLayer::new(request_id_header, MakeRequestUuid))
        // Method whitelist: business APIs are POST-only, everything else
        // accepts GET / POST. See `only_get_post`.
        .layer(axum::middleware::from_fn_with_state(
            Rules(rules.clone()),
            only_get_post,
        ))
        // Bot blocker: 403 known crawlers / scrapers / AI bots before they
        // can spend a rate-limit token or reach a DB query. Sits AFTER the
        // method filter so OPTIONS preflights still work for legit browsers.
        .layer(axum::middleware::from_fn_with_state(
            BotFilter(Arc::new(cfg.bot_ua_denylist.clone())),
            block_bots,
        ))
        // i18n: detect the request language and write it into a task-local for
        // IntoResponse translation. Must sit outside only_get_post so that even
        // 405→404 rejections still go through the translation path.
        .layer(axum::middleware::from_fn(crate::i18n::lang_layer))
        // Outermost: metrics middleware for accurate end-to-end latency.
        .layer(axum::middleware::from_fn(latency_metrics))
        .layer(trace)
        // Normalize framework rejections (method filter, body limit, timeout,
        // unmatched API routes, etc.) into the same JSON/i18n contract as
        // handler errors. This keeps clients from seeing bare text responses.
        .layer(axum::middleware::from_fn_with_state(
            Rules(rules),
            normalize_api_rejections,
        ))
}

/// `RouteRules` in the shape `from_fn_with_state` wants: cheap to clone, and a
/// distinct type so two middlewares can each take their own state.
#[derive(Clone)]
struct Rules(Arc<RouteRules>);

/// Convert non-JSON failures on business routes into the public response
/// envelope. Handler-produced JSON is left untouched, including its detailed
/// validation payloads; only framework-generated text/empty responses are
/// replaced.
async fn normalize_api_rejections(
    State(rules): State<Rules>,
    req: Request,
    next: Next,
) -> Response {
    let path = req.uri().path().to_string();
    let response = next.run(req).await;
    if !rules.0.is_api_path(&path) || response.status().as_u16() < 400 {
        return response;
    }

    let is_json = response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.to_ascii_lowercase().starts_with("application/json"));
    if is_json {
        return response;
    }

    let status = response.status();
    let key = rejection_key(status);
    let mut normalized = envelope(status, key).into_response();

    // Keep operational headers such as Retry-After and request IDs, but let
    // Json set the correct content type and content length.
    for (name, value) in response.headers() {
        if name != header::CONTENT_TYPE && name != header::CONTENT_LENGTH {
            normalized.headers_mut().insert(name.clone(), value.clone());
        }
    }
    normalized
}

/// Map a framework-generated status onto the same stable `key` vocabulary that
/// `ApiError` uses, so a client sees one error taxonomy regardless of whether
/// the rejection came from a handler or from a tower layer.
fn rejection_key(status: StatusCode) -> &'static str {
    match status {
        StatusCode::UNAUTHORIZED => "unauth",
        StatusCode::FORBIDDEN => "forbidden",
        StatusCode::NOT_FOUND => "not_found",
        StatusCode::METHOD_NOT_ALLOWED => "method_not_allowed",
        StatusCode::REQUEST_TIMEOUT => "timeout",
        StatusCode::PAYLOAD_TOO_LARGE => "body_too_large",
        StatusCode::TOO_MANY_REQUESTS => "rate_limit",
        StatusCode::SERVICE_UNAVAILABLE => "unavailable",
        StatusCode::BAD_REQUEST | StatusCode::UNPROCESSABLE_ENTITY => "param_invalid",
        s if s.is_server_error() => "internal",
        _ => "request_failed",
    }
}

/// The one and only failure envelope shape. Must stay byte-compatible with
/// `ApiError::into_response` — `crates/products/recruit/api/tests/response_contract.rs`
/// asserts both paths agree.
fn envelope(status: StatusCode, key: &'static str) -> (StatusCode, Json<serde_json::Value>) {
    let lang = crate::i18n::current_lang();
    (
        status,
        Json(json!({
            "code": status.as_u16(),
            "key": key,
            "msg": crate::i18n::t(&format!("errors.{key}"), lang),
            "data": "",
        })),
    )
}

fn localized_rejection(status: StatusCode, key: &'static str) -> Response {
    envelope(status, key).into_response()
}

/// Convert rate-limit middleware rejections into the same JSON/i18n envelope
/// used by handler errors. The default implementation is plain text, which
/// breaks clients and the response contract under load.
fn governor_error_response(error: GovernorError) -> Response {
    let (status, headers) = match error {
        GovernorError::TooManyRequests { headers, .. } => (StatusCode::TOO_MANY_REQUESTS, headers),
        GovernorError::UnableToExtractKey => (StatusCode::INTERNAL_SERVER_ERROR, None),
        GovernorError::Other { code, headers, .. } => (code, headers),
    };

    let mut response = envelope(status, rejection_key(status)).into_response();
    if let Some(headers) = headers {
        // Retry-After and the X-RateLimit-* family must survive; the envelope
        // owns content-type / content-length.
        response.headers_mut().extend(headers);
    }
    response
}

/// Method filter.
///
/// - **Business API paths** (the namespaces registered on [`RouteRules`]):
///   POST only. HEAD and OPTIONS pass through for probes and browser
///   preflights; GET / PUT / PATCH / DELETE get a 405. This is the
///   project-wide convention — every business parameter travels in a JSON
///   body, never in the URL.
/// - **Paths explicitly exempted via [`RouteRules::allow_get`]**: GET is
///   allowed too. These are third-party protocol handshakes whose verb we do
///   not control.
/// - **Everything else** (`/health`, `/ready`, `/docs/*`, `openapi.json`):
///   GET / POST / HEAD / OPTIONS, anything else 404.
async fn only_get_post(State(rules): State<Rules>, req: Request, next: Next) -> Response {
    use axum::http::Method;
    let path = req.uri().path();
    let method = req.method();

    let post_only = rules.0.is_api_path(path) && !rules.0.allows_get(path);
    if post_only {
        return match *method {
            Method::POST | Method::HEAD | Method::OPTIONS => next.run(req).await,
            _ => localized_rejection(StatusCode::METHOD_NOT_ALLOWED, "method_not_allowed"),
        };
    }

    match *method {
        Method::GET | Method::POST | Method::HEAD | Method::OPTIONS => next.run(req).await,
        // Non-API paths keep the historical 404 (rather than 405) so we do not
        // advertise which ops endpoints exist.
        _ if rules.0.is_api_path(path) => {
            localized_rejection(StatusCode::METHOD_NOT_ALLOWED, "method_not_allowed")
        }
        _ => localized_rejection(StatusCode::NOT_FOUND, "not_found"),
    }
}

/// Immutable, pre-lowercased UA denylist shared by every request. Cloning is
/// an `Arc` bump, which is what `from_fn_with_state` needs.
#[derive(Clone)]
struct BotFilter(Arc<Vec<String>>);

/// Block crawler / scraper User-Agents with a flat 403 before they can spend a
/// rate-limit token or reach a DB query. An empty UA is allowed (k8s probes,
/// internal monitoring, and curl smoke-tests often send none), and an empty
/// denylist disables the check entirely.
///
/// The patterns come from `Config::bot_ua_denylist`, so an operator can widen
/// or disable the filter without a rebuild. Note that generic HTTP-library UAs
/// are deliberately absent from the default list — see
/// `config::DEFAULT_BOT_UA_DENYLIST`.
async fn block_bots(State(filter): State<BotFilter>, req: Request, next: Next) -> Response {
    if !filter.0.is_empty() {
        if let Some(ua) = req
            .headers()
            .get(header::USER_AGENT)
            .and_then(|v| v.to_str().ok())
        {
            let ua_lower = ua.to_ascii_lowercase();
            if filter.0.iter().any(|p| ua_lower.contains(p.as_str())) {
                return localized_rejection(StatusCode::FORBIDDEN, "forbidden");
            }
        }
    }
    next.run(req).await
}

/// Security response headers — **applied to every response**. Mitigates
/// clickjacking / MIME sniffing / referrer leakage.
/// HSTS isn't added here: it's typically added by the reverse proxy
/// (nginx / Cloudflare) so the HTTP→HTTPS redirect still works; adding HSTS at
/// the application layer would, in direct-to-internal-network scenarios, lock
/// users out of HTTP access entirely.
type SecurityHeaders = tower::layer::util::Stack<
    SetResponseHeaderLayer<HeaderValue>,
    tower::layer::util::Stack<
        SetResponseHeaderLayer<HeaderValue>,
        tower::layer::util::Stack<
            SetResponseHeaderLayer<HeaderValue>,
            SetResponseHeaderLayer<HeaderValue>,
        >,
    >,
>;

fn security_headers() -> SecurityHeaders {
    use tower::layer::util::Stack;
    let xcto = SetResponseHeaderLayer::overriding(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    let xfo = SetResponseHeaderLayer::overriding(
        HeaderName::from_static("x-frame-options"),
        HeaderValue::from_static("DENY"),
    );
    let referrer = SetResponseHeaderLayer::overriding(
        header::REFERRER_POLICY,
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    // X-Robots-Tag tells crawlers that even read responses must not be
    // indexed / archived / used for AI training. Backed by the UA blacklist
    // above; this header catches the polite crawlers that bypass our UA list.
    let robots = SetResponseHeaderLayer::overriding(
        HeaderName::from_static("x-robots-tag"),
        HeaderValue::from_static("noindex, nofollow, noarchive, nosnippet, noai, noimageai"),
    );
    Stack::new(robots, Stack::new(referrer, Stack::new(xfo, xcto)))
}

/// Request-latency histogram + QPS counter. One timeseries per route.
async fn latency_metrics(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let route = req
        .extensions()
        .get::<MatchedPath>()
        .map(|p| p.as_str().to_string())
        .unwrap_or_else(|| req.uri().path().to_string());

    let start = std::time::Instant::now();
    let resp = next.run(req).await;
    let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
    let status = resp.status().as_u16().to_string();

    metrics::counter!(
        "http.requests_total",
        "method" => method.to_string(),
        "route" => route.clone(),
        "status" => status.clone()
    )
    .increment(1);

    metrics::histogram!(
        "http.latency_ms",
        "method" => method.to_string(),
        "route" => route,
        "status" => status
    )
    .record(elapsed_ms);

    resp
}

fn build_cors(cfg: &Config) -> CorsLayer {
    // If the only entry is "*", allow any origin; otherwise use the whitelist.
    let origin = if cfg.cors_allowed_origins.len() == 1 && cfg.cors_allowed_origins[0] == "*" {
        AllowOrigin::any()
    } else {
        let list: Vec<HeaderValue> = cfg
            .cors_allowed_origins
            .iter()
            .filter_map(|o| o.parse::<HeaderValue>().ok())
            .collect();
        AllowOrigin::list(list)
    };

    // Keep this list in sync with `only_get_post` above — advertising methods
    // we'd reject misleads clients into firing requests that 404 in the actual
    // router. PHPYun's controller convention only ever used GET / POST.
    CorsLayer::new()
        .allow_origin(origin)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(tower_http::cors::Any)
        .max_age(Duration::from_secs(600))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every status the middleware stack can emit must resolve to a key that
    /// actually exists in the locale tables, otherwise clients get the raw
    /// `errors.<key>` string as their user-facing message.
    #[test]
    fn every_rejection_key_is_translatable() {
        let statuses = [
            StatusCode::BAD_REQUEST,
            StatusCode::UNAUTHORIZED,
            StatusCode::FORBIDDEN,
            StatusCode::NOT_FOUND,
            StatusCode::METHOD_NOT_ALLOWED,
            StatusCode::REQUEST_TIMEOUT,
            StatusCode::PAYLOAD_TOO_LARGE,
            StatusCode::UNPROCESSABLE_ENTITY,
            StatusCode::TOO_MANY_REQUESTS,
            StatusCode::INTERNAL_SERVER_ERROR,
            StatusCode::BAD_GATEWAY,
            StatusCode::SERVICE_UNAVAILABLE,
            StatusCode::CONFLICT,
        ];
        for status in statuses {
            let key = rejection_key(status);
            let i18n_key = format!("errors.{key}");
            for lang in [
                crate::i18n::Lang::ZhCN,
                crate::i18n::Lang::ZhTW,
                crate::i18n::Lang::En,
            ] {
                let msg = crate::i18n::t(&i18n_key, lang);
                assert_ne!(
                    msg, i18n_key,
                    "{status} -> {i18n_key} has no {lang:?} translation"
                );
            }
        }
    }

    /// The framework-rejection envelope must carry exactly the same members as
    /// `ApiError::into_response`, so clients see one shape everywhere.
    #[test]
    fn envelope_has_the_canonical_member_set() {
        let (status, Json(body)) = envelope(StatusCode::NOT_FOUND, "not_found");
        assert_eq!(status, StatusCode::NOT_FOUND);

        let mut members: Vec<&str> = body
            .as_object()
            .expect("envelope is an object")
            .keys()
            .map(String::as_str)
            .collect();
        members.sort_unstable();
        assert_eq!(members, ["code", "data", "key", "msg"]);

        assert_eq!(body["code"], 404);
        assert_eq!(body["key"], "not_found");
        assert_eq!(body["data"], "");
    }

    /// Framework rejections must not be re-labelled into a different status the
    /// way the old `public_error_status` collapse did.
    #[test]
    fn envelope_preserves_the_source_status() {
        for status in [
            StatusCode::TOO_MANY_REQUESTS,
            StatusCode::PAYLOAD_TOO_LARGE,
            StatusCode::REQUEST_TIMEOUT,
        ] {
            let (out, Json(body)) = envelope(status, rejection_key(status));
            assert_eq!(out, status);
            assert_eq!(body["code"], status.as_u16());
        }
    }
}
