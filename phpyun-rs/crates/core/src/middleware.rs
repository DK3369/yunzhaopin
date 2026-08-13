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
use axum::{
    extract::{MatchedPath, Request},
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

pub fn install<S>(router: Router<S>, cfg: &Config) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let cors = build_cors(cfg);
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
        // Method whitelist: only allow GET / POST; everything
        // else (PUT / DELETE / PATCH / OPTIONS / HEAD, etc.) is uniformly turned into 404.
        .layer(axum::middleware::from_fn(only_get_post))
        // Bot blocker: 403 known crawlers / scrapers / AI bots before they
        // can spend a rate-limit token or reach a DB query. Sits AFTER the
        // method filter so OPTIONS preflights still work for legit browsers.
        .layer(axum::middleware::from_fn(block_bots))
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
        .layer(axum::middleware::from_fn(normalize_api_rejections))
}

/// Convert non-JSON failures on business routes into the public response
/// envelope. Handler-produced JSON is left untouched, including its detailed
/// validation payloads; only framework-generated text/empty responses are
/// replaced.
async fn normalize_api_rejections(req: Request, next: Next) -> Response {
    let path = req.uri().path().to_string();
    let response = next.run(req).await;
    if !path.starts_with("/v1/") || response.status().as_u16() < 400 {
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

    let source_status = response.status();
    let status = public_error_status(source_status);
    let key = match source_status {
        StatusCode::UNAUTHORIZED => "errors.unauth",
        StatusCode::FORBIDDEN => "errors.forbidden",
        StatusCode::NOT_FOUND => "errors.not_found",
        StatusCode::METHOD_NOT_ALLOWED => "errors.method_not_allowed",
        StatusCode::REQUEST_TIMEOUT => "errors.timeout",
        StatusCode::PAYLOAD_TOO_LARGE => "errors.body_too_large",
        StatusCode::TOO_MANY_REQUESTS => "errors.rate_limit",
        StatusCode::SERVICE_UNAVAILABLE => "errors.unavailable",
        StatusCode::BAD_REQUEST | StatusCode::UNPROCESSABLE_ENTITY => "errors.param_invalid",
        _ if status.is_server_error() => "errors.internal",
        _ => "errors.request_failed",
    };
    let lang = crate::i18n::current_lang();
    let msg = crate::i18n::t(key, lang);
    let mut normalized = Json(json!({
        "code": status.as_u16(),
        "key": key,
        "msg": msg,
        "data": null,
    }))
    .into_response();
    *normalized.status_mut() = status;

    // Keep operational headers such as Retry-After and request IDs, but let
    // Json set the correct content type and content length.
    for (name, value) in response.headers() {
        if name != header::CONTENT_TYPE && name != header::CONTENT_LENGTH {
            normalized.headers_mut().insert(name.clone(), value.clone());
        }
    }
    normalized
}

/// Public API error status policy: keep only authentication/session failures
/// as 401; all other failures are exposed as 500.
fn public_error_status(status: StatusCode) -> StatusCode {
    if status == StatusCode::UNAUTHORIZED {
        StatusCode::UNAUTHORIZED
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

fn localized_rejection(status: StatusCode, key: &'static str) -> Response {
    let lang = crate::i18n::current_lang();
    let msg = crate::i18n::t(key, lang);
    let status = public_error_status(status);
    (
        status,
        Json(json!({
            "code": status.as_u16(),
            "key": key,
            "msg": msg,
            "data": null,
        })),
    )
        .into_response()
}

/// Convert rate-limit middleware rejections into the same JSON/i18n envelope
/// used by handler errors. The default implementation is plain text, which
/// breaks clients and the response contract under load.
fn governor_error_response(error: GovernorError) -> Response {
    let (source_status, key, msg, headers) = match error {
        GovernorError::TooManyRequests { headers, .. } => {
            let lang = crate::i18n::current_lang();
            (
                axum::http::StatusCode::TOO_MANY_REQUESTS,
                "errors.rate_limit",
                crate::i18n::t("errors.rate_limit", lang),
                headers,
            )
        }
        GovernorError::UnableToExtractKey => {
            let lang = crate::i18n::current_lang();
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "errors.internal",
                crate::i18n::t("errors.internal", lang),
                None,
            )
        }
        GovernorError::Other { code, headers, .. } => {
            let lang = crate::i18n::current_lang();
            let source_status = if code.is_server_error() {
                axum::http::StatusCode::INTERNAL_SERVER_ERROR
            } else {
                code
            };
            let key = if source_status == axum::http::StatusCode::TOO_MANY_REQUESTS {
                "errors.rate_limit"
            } else {
                "errors.internal"
            };
            (source_status, key, crate::i18n::t(key, lang), headers)
        }
    };
    let status = public_error_status(source_status);

    let mut response = Json(json!({
        "code": status.as_u16(),
        "key": key,
        "msg": msg,
        "data": null,
    }))
    .into_response();
    *response.status_mut() = status;
    if let Some(headers) = headers {
        response.headers_mut().extend(headers);
    }
    response
}

/// Method filter:
///
/// - **`/v1/*` business endpoints**: only POST is allowed (HEAD / OPTIONS pass
///   through for browser preflights & probes). GET / PUT / PATCH / DELETE all
///   return 405. This is the project-wide convention — every business param
///   travels in a JSON body, never in the URL.
/// - **Everything else** (e.g. `/health`, `/ready`, `/openapi.json`,
///   `/docs/*` Swagger UI assets, the `/v1/wap/wechat/callback` exception
///   for the WeChat protocol): GET / POST / HEAD / OPTIONS are all allowed;
///   anything else returns 404 (preserves the original behaviour).
///
/// `/v1/wap/wechat/callback` is the only `/v1/*` route that legitimately
/// needs GET (WeChat's verification handshake mandates GET + query string —
/// outside our control). We allow-list it explicitly.
async fn only_get_post(req: Request, next: Next) -> Response {
    use axum::http::Method;
    let path = req.uri().path();
    let method = req.method();

    // 1) WeChat callback exception — GET allowed.
    if path == "/v1/wap/wechat/callback" {
        return match *method {
            Method::GET | Method::POST | Method::HEAD | Method::OPTIONS => next.run(req).await,
            _ => localized_rejection(StatusCode::METHOD_NOT_ALLOWED, "errors.method_not_allowed"),
        };
    }

    // 2) Dictionary clients historically use GET for this endpoint; keep it
    // compatible while the rest of the business API remains POST-only.
    if path == "/v1/wap/dict/industries" {
        return match *method {
            Method::GET | Method::POST | Method::HEAD | Method::OPTIONS => next.run(req).await,
            _ => localized_rejection(StatusCode::METHOD_NOT_ALLOWED, "errors.method_not_allowed"),
        };
    }

    // 3) /v1/* business endpoints — POST only (+ HEAD/OPTIONS for preflight).
    if path.starts_with("/v1/") {
        return match *method {
            Method::POST | Method::HEAD | Method::OPTIONS => next.run(req).await,
            _ => localized_rejection(StatusCode::METHOD_NOT_ALLOWED, "errors.method_not_allowed"),
        };
    }

    // 4) All other paths (ops probes, swagger UI, openapi.json) — keep the
    //    permissive GET / POST allowance.
    match *method {
        Method::GET | Method::POST | Method::HEAD | Method::OPTIONS => next.run(req).await,
        _ => localized_rejection(StatusCode::NOT_FOUND, "errors.not_found"),
    }
}

/// User-Agent substrings (lowercased) that get a flat 403. This is a backend
/// API — there is zero reason for any indexing / archiving / training crawler
/// to touch us. Block them at the door so they don't even consume a rate-limit
/// token. False positives are an acceptable cost: if a legit client UA happens
/// to contain "spider" we'd rather they 403 once and switch UA than let the
/// real swarm in.
const BOT_UA_PATTERNS: &[&str] = &[
    // -- search engines --
    "googlebot",
    "bingbot",
    "slurp",
    "duckduckbot",
    "yandexbot",
    "baiduspider",
    "sogou web spider",
    "sogou inst spider",
    "yisouspider",
    "360spider",
    "haosouspider",
    "sosospider",
    "exabot",
    "facebot",
    "ia_archiver",
    "petalbot",
    "yahoo! slurp",
    // -- SEO / data brokers --
    "ahrefsbot",
    "semrushbot",
    "mj12bot",
    "dotbot",
    "seznambot",
    "blexbot",
    "megaindex",
    "linkdexbot",
    "screaming frog",
    "sitebulb",
    "serpstatbot",
    "barkrowler",
    "dataforseobot",
    // -- AI training scrapers --
    "gptbot",
    "chatgpt-user",
    "oai-searchbot",
    "claudebot",
    "claude-web",
    "anthropic-ai",
    "ccbot",
    "perplexitybot",
    "bytespider",
    "applebot-extended",
    "amazonbot",
    "diffbot",
    "cohere-ai",
    "img2dataset",
    "timpibot",
    "google-extended",
    // -- generic catch-all (substring match) --
    "spider",
    "crawler",
    "scraper",
    "headlesschrome",
    "phantomjs",
    "puppeteer",
    "playwright",
    "scrapy",
    "httrack",
    "wget",
    "libwww-perl",
    "python-urllib",
    "go-http-client",
    "java/",
    "okhttp",
];

/// Block crawler / scraper User-Agents with a flat 403. Empty UA is allowed
/// (k8s probes, internal monitoring, our own curl smoke-tests often have none),
/// but anything matching the blacklist is dropped before rate-limit / DB.
async fn block_bots(req: Request, next: Next) -> Response {
    if let Some(ua) = req
        .headers()
        .get(header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
    {
        let ua_lower = ua.to_ascii_lowercase();
        if BOT_UA_PATTERNS.iter().any(|p| ua_lower.contains(p)) {
            return localized_rejection(StatusCode::FORBIDDEN, "errors.forbidden");
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
fn security_headers() -> tower::layer::util::Stack<
    SetResponseHeaderLayer<HeaderValue>,
    tower::layer::util::Stack<
        SetResponseHeaderLayer<HeaderValue>,
        tower::layer::util::Stack<
            SetResponseHeaderLayer<HeaderValue>,
            SetResponseHeaderLayer<HeaderValue>,
        >,
    >,
> {
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
