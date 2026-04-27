//! Payment callback (aligned with PHPYun's payment gateway notification endpoint).
//!
//! Design:
//! - Real production should implement signature verification per provider
//!   (alipay/wechat/stripe/...) and wrap it in a pluggable `PaymentProvider` trait.
//! - The current migration version uses a **pre-shared token** to quickly stitch the
//!   whole flow together: the gateway is configured with a fixed token, passed in
//!   the `X-Pay-Token` header, and the server does a simple comparison.
//!
//! On success, call `vip_service::mark_paid` to reuse the existing mark-paid + activate-VIP + emit-event flow.

use axum::{
    extract::State,
    http::{header::HeaderMap, StatusCode},
    routing::post,
    Router,
};
use phpyun_core::error::InfraError;
use phpyun_core::{dto::OkResp, ApiJson, AppError, AppResult, AppState, ValidatedJson};
use phpyun_services::vip_service;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new().route("/pay/callback", post(callback))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CallbackForm {
    #[validate(length(min = 1, max = 64))]
    pub order_no: String,
    #[validate(length(min = 1, max = 128))]
    pub pay_tx_id: String,
}

/// Gateway callback: authenticate via the shared secret in the `X-Pay-Token` header -> mark-paid
#[utoipa::path(
    post,
    path = "/v1/wap/pay/callback",
    tag = "wap",
    request_body = CallbackForm,
    responses(
        (status = 200, description = "ok", body = OkResp),
        (status = 401, description = "bad token"),
        (status = 503, description = "server not configured")
    )
)]
pub async fn callback(
    State(state): State<AppState>,
    headers: HeaderMap,
    ValidatedJson(f): ValidatedJson<CallbackForm>,
) -> AppResult<ApiJson<OkResp>> {
    // 1. The server must have a token configured; if not, return 503 (avoid running the endpoint unprotected)
    let expected = state
        .config
        .payment_callback_token
        .as_deref()
        .ok_or_else(|| {
            AppError::new(InfraError::Upstream(
                "payment_callback_token not configured".into(),
            ))
        })?;
    // Startup config::validate() already requires >= 32 characters; keep a runtime safety net here.
    if expected.len() < 32 {
        return Err(AppError::new(InfraError::Upstream(
            "payment_callback_token too short".into(),
        )));
    }

    // 2. Header verification (constant-time comparison to prevent timing attacks)
    let got = headers
        .get("X-Pay-Token")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if !constant_time_eq(got.as_bytes(), expected.as_bytes()) {
        tracing::warn!(order = %f.order_no, "pay callback: bad token");
        return Err(AppError::unauth());
    }

    // 3. Mark as paid
    vip_service::mark_paid(&state, &f.order_no, &f.pay_tx_id).await?;
    Ok(ApiJson(OkResp { ok: true }))
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut d = 0u8;
    for i in 0..a.len() {
        d |= a[i] ^ b[i];
    }
    d == 0
}

#[allow(dead_code)]
const _STATUS_MARKER: StatusCode = StatusCode::OK;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ct_eq_basic() {
        assert!(constant_time_eq(b"abc", b"abc"));
        assert!(!constant_time_eq(b"abc", b"abd"));
        assert!(!constant_time_eq(b"abc", b"abcd"));
        assert!(constant_time_eq(b"", b""));
    }
}
