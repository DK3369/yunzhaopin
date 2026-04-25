//! Prometheus metrics, tracing-context bridging, and a business metric facade.
//!
//! Business code should not call `metrics::counter!(...)` directly — use the typed
//! helpers in this module instead:
//! - `auth_event(kind, reason)` — login/logout/registration-class events.
//! - `cache_hit(level, scope)` / `cache_miss(scope)` — cache hit ratio.
//! - `counter(name)` / `counter_with(name, tags)` — generic channel (add new
//!   metrics here first).
//! - `gauge(name).set(v)` / `histogram_ms(name, ms)` — numeric observations.
//!
//! This way metric names live in one file, and `"auth.foo.bar"` style hard-coded
//! strings don't get scattered across the workspace.

use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_tracing_context::{MetricsLayer, TracingContextLayer};
use metrics_util::layers::Layer;
use std::net::SocketAddr;

/// Install the Prometheus recorder + tracing-context bridge. Starts an HTTP
/// listener that exposes `/metrics`.
pub fn install_prometheus(bind: &str) -> anyhow::Result<()> {
    let addr: SocketAddr = bind.parse()?;
    let (recorder, exporter) = PrometheusBuilder::new()
        .with_http_listener(addr)
        .build()?;

    let layered = TracingContextLayer::all().layer(recorder);
    metrics::set_global_recorder(layered)
        .map_err(|e| anyhow::anyhow!("metrics recorder already set: {e}"))?;

    tokio::spawn(exporter);
    tracing::info!(%bind, "prometheus endpoint up");
    Ok(())
}

/// `tracing_subscriber` layer that bridges tracing spans into metrics labels.
pub fn tracing_metrics_layer() -> MetricsLayer {
    MetricsLayer::new()
}

// ==================== Business metric facade ====================

/// Plain counter: no labels, simple increment.
#[inline]
pub fn counter(name: &'static str) {
    metrics::counter!(name).increment(1);
}

/// Plain counter with a set of labels.
#[inline]
pub fn counter_with(name: &'static str, labels: &[(&'static str, &'static str)]) {
    let labels: Vec<(&'static str, String)> =
        labels.iter().map(|(k, v)| (*k, (*v).to_string())).collect();
    metrics::counter!(name, &labels).increment(1);
}

/// Unified entry point for authentication-class events. `kind` is something like
/// `"login_success"`, `"login_fail"`, `"logout"`. `reason` may be `None` (no
/// sub-category) or `Some("bad_password")` etc.
pub fn auth_event(kind: &'static str, reason: Option<&'static str>) {
    let name = match kind {
        "login_success" => "auth.login_success",
        "login_blocked" => "auth.login_blocked",
        "login_fail" => "auth.login_fail",
        "logout" => "auth.logout",
        "token_refreshed" => "auth.token_refreshed",
        "password_upgraded" => "auth.password_upgraded",
        other => {
            // Unregistered kinds are still allowed through, but a warn lets us
            // catch any that slipped past the typed list.
            tracing::warn!(kind = other, "unregistered auth_event kind");
            "auth.other"
        }
    };
    match reason {
        Some(r) => counter_with(name, &[("reason", r)]),
        None => counter(name),
    }
}

/// Tiered cache hit. `level` = "l1" / "l2"; `scope` is something like `"user.profile"`.
#[inline]
pub fn cache_hit(level: &'static str, scope: &'static str) {
    counter_with("cache.hit", &[("level", level), ("scope", scope)]);
}

#[inline]
pub fn cache_miss(scope: &'static str) {
    counter_with("cache.miss", &[("scope", scope)]);
}

/// Counter for rate-limit rejections.
#[inline]
pub fn rate_limit_blocked(key_prefix: &'static str) {
    counter_with("rate_limit.blocked", &[("key_prefix", key_prefix)]);
}

/// Read/write a gauge (e.g. connection-pool size). Returns a raw handle and the
/// caller invokes `.set(val)`.
#[inline]
pub fn gauge_set(name: &'static str, value: f64) {
    metrics::gauge!(name).set(value);
}

/// Record a duration (milliseconds) into a histogram.
#[inline]
pub fn histogram_ms(name: &'static str, ms: f64) {
    metrics::histogram!(name).record(ms);
}
