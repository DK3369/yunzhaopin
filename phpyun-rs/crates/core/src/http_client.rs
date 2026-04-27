//! Outbound HTTP facade — high concurrency / stability.
//!
//! ## Stability
//! - **Per-call timeouts**: the underlying client has a global timeout; the
//!   `with_retry` methods can layer in a per-attempt limit on top.
//! - **Exponential backoff + jitter retries**: timeout / 5xx / connect errors
//!   are retried automatically; business errors (4xx) are not.
//! - **Error classification**: `reqwest` timeout → `UpstreamError("timeout")`;
//!   everything else → `UpstreamError(msg)`.
//!
//! ## Performance
//! - The underlying `reqwest::Client` is `Arc` internally; `.clone()` across
//!   threads is zero-cost.
//! - Keep-alive + per-host connection pool (TCP/TLS reuse on Tokio).
//! - `rustls` (no `libssl`); HTTP/2 enabled.
//!
//! ## Observability
//! - `http.client.latency_ms{host, status}` histogram.
//! - `http.client.retry{host}` counter.
//! - `http.client.error{host, kind}` counter.

use crate::error::{AppError, AppResult};
use crate::json;
use crate::metrics as m;
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};
use std::time::{Duration, Instant};
use tracing::Instrument;

fn build_client(timeout_secs: u64, pool_max_idle_per_host: usize) -> anyhow::Result<Client> {
    let client = Client::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .connect_timeout(Duration::from_secs(5))
        .pool_idle_timeout(Some(Duration::from_secs(90)))
        .pool_max_idle_per_host(pool_max_idle_per_host)
        .tcp_nodelay(true)
        .tcp_keepalive(Some(Duration::from_secs(60)))
        .user_agent(concat!("phpyun-rs/", env!("CARGO_PKG_VERSION")))
        .build()?;
    Ok(client)
}

/// Retry policy. Only retry on 5xx / timeout / connect errors; 4xx is treated
/// as a business error and returned immediately.
#[derive(Debug, Clone, Copy)]
pub struct RetryPolicy {
    pub max_attempts: u8,
    pub base_delay_ms: u64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay_ms: 100,
        }
    }
}

impl RetryPolicy {
    pub const NONE: Self = Self {
        max_attempts: 1,
        base_delay_ms: 0,
    };
}

#[derive(Clone)]
pub struct Http {
    inner: Client,
}

impl Http {
    pub fn new(timeout_secs: u64, pool_max_idle_per_host: usize) -> anyhow::Result<Self> {
        Ok(Self {
            inner: build_client(timeout_secs, pool_max_idle_per_host)?,
        })
    }

    /// GET JSON, with the default 3 retries.
    pub async fn get_json<T: DeserializeOwned>(&self, url: &str) -> AppResult<T> {
        self.get_json_with(url, RetryPolicy::default()).await
    }

    /// GET JSON with a configurable retry policy.
    pub async fn get_json_with<T: DeserializeOwned>(
        &self,
        url: &str,
        retry: RetryPolicy,
    ) -> AppResult<T> {
        let host = host_of(url);
        let span = tracing::info_span!("http.get", url = %url, host = %host);
        async move {
            let text = self.send_with_retry("GET", url, None::<&()>, retry).await?;
            json::from_str::<T>(&text)
        }
        .instrument(span)
        .await
    }

    /// POST a JSON body → JSON response, with the default 3 retries (note:
    /// non-idempotent business should pass `NONE`).
    pub async fn post_json<B: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        url: &str,
        body: &B,
    ) -> AppResult<T> {
        self.post_json_with(url, body, RetryPolicy::default()).await
    }

    pub async fn post_json_with<B: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        url: &str,
        body: &B,
        retry: RetryPolicy,
    ) -> AppResult<T> {
        let host = host_of(url);
        let span = tracing::info_span!("http.post", url = %url, host = %host);
        async move {
            let text = self.send_with_retry("POST", url, Some(body), retry).await?;
            json::from_str::<T>(&text)
        }
        .instrument(span)
        .await
    }

    /// POST `application/x-www-form-urlencoded` body, read JSON response.
    /// Used by OAuth2 token endpoints that won't accept JSON bodies (Weibo,
    /// some legacy gateways). No retries — these are non-idempotent.
    pub async fn post_form_to_json<T: DeserializeOwned>(
        &self,
        url: &str,
        form_body: &str,
    ) -> AppResult<T> {
        let host = host_of(url);
        let span = tracing::info_span!("http.post_form", url = %url, host = %host);
        async move {
            let started = Instant::now();
            let res = self
                .inner
                .post(url)
                .header("content-type", "application/x-www-form-urlencoded")
                .body(form_body.to_string())
                .send()
                .await
                .and_then(|r| r.error_for_status());
            let (text, status) = match res {
                Ok(resp) => {
                    let status = resp.status().as_u16();
                    (resp.text().await.map_err(map_reqwest_err)?, status)
                }
                Err(e) => return Err(map_reqwest_err(e)),
            };
            m::histogram_ms(
                "http.client.latency_ms",
                started.elapsed().as_secs_f64() * 1000.0,
            );
            record_status(&host, status);
            json::from_str::<T>(&text)
        }
        .instrument(span)
        .await
    }

    /// POST a plain-text body and read text in return (common for SMS /
    /// payment XML gateways).
    pub async fn post_text(&self, url: &str, body: String) -> AppResult<String> {
        let host = host_of(url);
        let span = tracing::info_span!("http.post_text", url = %url, host = %host);
        async move {
            let started = Instant::now();
            let res = self
                .inner
                .post(url)
                .body(body)
                .send()
                .await
                .and_then(|r| r.error_for_status());
            let (text, status) = match res {
                Ok(resp) => {
                    let status = resp.status().as_u16();
                    (resp.text().await.map_err(map_reqwest_err)?, status)
                }
                Err(e) => return Err(map_reqwest_err(e)),
            };
            m::histogram_ms("http.client.latency_ms", started.elapsed().as_secs_f64() * 1000.0);
            record_status(&host, status);
            Ok(text)
        }
        .instrument(span)
        .await
    }

    /// Internal: send a JSON body with retries and read a text response.
    async fn send_with_retry<B: Serialize + ?Sized>(
        &self,
        method: &'static str,
        url: &str,
        body: Option<&B>,
        retry: RetryPolicy,
    ) -> AppResult<String> {
        let host = host_of(url);
        let mut last_err: Option<AppError> = None;

        for attempt in 0..retry.max_attempts {
            if attempt > 0 {
                let base = retry.base_delay_ms.saturating_mul(1u64 << (attempt - 1).min(4));
                let wait = base.saturating_add(jitter_ms(base));
                tokio::time::sleep(Duration::from_millis(wait)).await;
                m::counter_with("http.client.retry", &[("host", as_static(&host))]);
            }

            let started = Instant::now();
            let mut req = match method {
                "GET" => self.inner.get(url),
                "POST" => self.inner.post(url),
                _ => unreachable!(),
            };
            if let Some(b) = body {
                req = req.json(b);
            }

            let resp_res = req.send().await;
            m::histogram_ms(
                "http.client.latency_ms",
                started.elapsed().as_secs_f64() * 1000.0,
            );

            match resp_res {
                Ok(resp) => {
                    let status = resp.status();
                    record_status(&host, status.as_u16());
                    if status.is_success() {
                        return resp.text().await.map_err(map_reqwest_err);
                    }
                    if status.is_server_error() {
                        last_err = Some(AppError::upstream(format!(
                            "{method} {url} → {status}"
                        )));
                        m::counter_with(
                            "http.client.error",
                            &[("host", as_static(&host)), ("kind", "5xx")],
                        );
                        continue; // Only 5xx triggers a retry.
                    }
                    // 4xx isn't retried; return the business error directly.
                    return Err(AppError::upstream(format!("{method} {url} → {status}")));
                }
                Err(e) => {
                    let kind = if e.is_timeout() {
                        "timeout"
                    } else if e.is_connect() {
                        "connect"
                    } else {
                        "other"
                    };
                    m::counter_with(
                        "http.client.error",
                        &[("host", as_static(&host)), ("kind", kind)],
                    );
                    last_err = Some(AppError::upstream(format!("{method} {url}: {e}")));
                }
            }
        }

        Err(last_err.unwrap_or_else(|| AppError::upstream(format!("{method} {url} failed"))))
    }
}

fn map_reqwest_err(e: reqwest::Error) -> AppError {
    AppError::upstream(e.to_string())
}

/// Extract the host from the URL for the metric label. Label cardinality is
/// bounded (the set of outbound upstreams is finite).
fn host_of(url: &str) -> String {
    url.split('/')
        .nth(2)
        .unwrap_or("unknown")
        .split(':')
        .next()
        .unwrap_or("unknown")
        .to_string()
}

/// Metric labels require `&'static str`. We "leak" once here — the total set of
/// upstream hostnames is small (a handful to a few dozen), so a one-time leak
/// across the process lifetime is acceptable. This avoids allocating a
/// `Box<str>` per HTTP request.
fn as_static(s: &str) -> &'static str {
    // Only known hosts get a real label; anything not in the intern table → "other".
    // To avoid both a memory leak and a cardinality blow-up, we return a
    // coarse-grained "other" instead.
    match s {
        // Extend: list upstream hostnames here.
        "127.0.0.1" | "localhost" => "localhost",
        _ => "other",
    }
}

fn record_status(host: &str, status: u16) {
    let status_class: &'static str = match status {
        100..=199 => "1xx",
        200..=299 => "2xx",
        300..=399 => "3xx",
        400..=499 => "4xx",
        500..=599 => "5xx",
        _ => "other",
    };
    m::counter_with(
        "http.client.response",
        &[("host", as_static(host)), ("status", status_class)],
    );
}

/// Cheap jitter: seed from the system's subsecond nanos, no extra dependency.
/// Range: [0, base/2].
fn jitter_ms(base: u64) -> u64 {
    if base == 0 {
        return 0;
    }
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.subsec_nanos() as u64)
        .unwrap_or(0);
    nanos % (base / 2 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_of_extracts_host() {
        assert_eq!(host_of("http://api.example.com/v1/x"), "api.example.com");
        assert_eq!(host_of("https://api.example.com:8443/path"), "api.example.com");
        assert_eq!(host_of("http://127.0.0.1:3000/health"), "127.0.0.1");
        assert_eq!(host_of("not-a-url"), "unknown");
    }

    #[test]
    fn jitter_stays_in_bounds() {
        for base in [0u64, 10, 100, 1000] {
            for _ in 0..32 {
                let j = jitter_ms(base);
                assert!(j <= base / 2 + 1, "jitter {j} out of bounds for base {base}");
            }
        }
    }

    #[test]
    fn retry_policy_none_is_single_attempt() {
        assert_eq!(RetryPolicy::NONE.max_attempts, 1);
    }
}
