//! Third-party callbacks that are **not** versioned (`/callback/*`).
//!
//! Alipay / WeChat Pay notify with raw `success`/`fail` bodies, and Locoy
//! collector ingest with the historic numeric codes.

use std::collections::{BTreeMap, HashMap};

use axum::body::Bytes;
use axum::extract::{Path, Query, State};
use axum::http::{header, HeaderMap, HeaderName, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::Form;
use axum::Router;
use phpyun_core::{AppState, ClientIp};
use phpyun_services::stripe_service::WebhookAck;
use serde::Deserialize;
use validator::Validate;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/alipay", post(alipay))
        .route("/wechat-pay", post(wechat_pay))
        .route("/stripe", post(stripe))
        .route("/pay/{method}", post(pay_method))
        .route("/locoy", post(locoy))
}

fn plain(status: StatusCode, body: &'static str) -> Response {
    (
        status,
        [(header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        body,
    )
        .into_response()
}

async fn alipay(
    State(state): State<AppState>,
    Form(map): Form<HashMap<String, String>>,
) -> Response {
    let params: BTreeMap<String, String> = map.into_iter().collect();
    match phpyun_services::payment_notify_service::handle_alipay(&state, &params).await {
        Ok(body) => plain(StatusCode::OK, body),
        Err(e) => {
            tracing::warn!(error = %e, "alipay notify rejected");
            plain(StatusCode::OK, "fail")
        }
    }
}

fn stripe_signature_header(headers: &HeaderMap) -> String {
    for name in ["stripe-signature", "stripe_signature"] {
        if let Ok(n) = HeaderName::from_bytes(name.as_bytes()) {
            if let Some(v) = headers.get(&n).and_then(|v| v.to_str().ok()) {
                if !v.is_empty() {
                    return v.to_string();
                }
            }
        }
    }
    for (name, value) in headers.iter() {
        let n = name.as_str();
        if n.eq_ignore_ascii_case("stripe-signature") || n.eq_ignore_ascii_case("stripe_signature")
        {
            if let Ok(v) = value.to_str() {
                if !v.is_empty() {
                    return v.to_string();
                }
            }
        }
    }
    String::new()
}

fn header_names(headers: &HeaderMap) -> String {
    let mut names: Vec<&str> = headers.keys().map(|k| k.as_str()).collect();
    names.sort_unstable();
    names.join(",")
}

fn webhook_plain(ack: WebhookAck) -> Response {
    let status = StatusCode::from_u16(ack.status()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    plain(status, ack.body())
}

async fn stripe(State(state): State<AppState>, headers: HeaderMap, body: Bytes) -> Response {
    let sig = stripe_signature_header(&headers);
    if sig.is_empty() {
        tracing::warn!(
            headers = %header_names(&headers),
            body_len = body.len(),
            "stripe webhook missing Stripe-Signature"
        );
    }
    let ack = phpyun_services::stripe_service::handle_webhook(&state, &body, &sig).await;
    if !ack.is_ok_status() {
        tracing::warn!(status = ack.status(), "stripe notify rejected");
    }
    webhook_plain(ack)
}

async fn pay_method(
    State(state): State<AppState>,
    Path(method): Path<String>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    if method != "stripe" {
        return webhook_plain(WebhookAck::BadRequest);
    }
    stripe(State(state), headers, body).await
}

async fn wechat_pay(State(state): State<AppState>, body: Bytes) -> Response {
    let xml = String::from_utf8_lossy(&body);
    match phpyun_services::payment_notify_service::handle_wechat_pay(&state, &xml).await {
        Ok(_) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/xml; charset=utf-8")],
            "<xml><return_code><![CDATA[SUCCESS]]></return_code><return_msg><![CDATA[OK]]></return_msg></xml>",
        )
            .into_response(),
        Err(e) => {
            tracing::warn!(error = %e, "wechat pay notify rejected");
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "application/xml; charset=utf-8")],
                "<xml><return_code><![CDATA[FAIL]]></return_code><return_msg><![CDATA[FAIL]]></return_msg></xml>",
            )
                .into_response()
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
struct LocoyQuery {
    #[serde(default)]
    #[validate(length(max = 32))]
    m: String,
    #[serde(default)]
    #[validate(length(max = 32))]
    c: String,
    #[serde(default)]
    #[validate(length(max = 256))]
    key: String,
}

async fn locoy(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    Query(q): Query<LocoyQuery>,
    Form(post): Form<HashMap<String, String>>,
) -> Response {
    if q.validate().is_err() {
        return plain(StatusCode::OK, phpyun_services::locoy_service::CODE_BAD);
    }
    let action_ok = matches!(
        (q.m.as_str(), q.c.as_str()),
        ("news", "addnews") | ("job", "add") | ("partjob", "add") | ("user", "add")
    );
    if !action_ok {
        return plain(StatusCode::OK, phpyun_services::locoy_service::CODE_BAD);
    }
    match phpyun_services::locoy_service::ingest(&state, &q.m, &q.key, &ip, post).await {
        Ok(code) => plain(StatusCode::OK, code),
        Err(e) => {
            tracing::error!(error = %e, "locoy ingest failed");
            plain(StatusCode::OK, phpyun_services::locoy_service::CODE_BAD)
        }
    }
}
