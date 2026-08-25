//! Third-party callbacks that are **not** versioned (`/callback/*`).
//!
//! Alipay / WeChat Pay notify with raw `success`/`fail` bodies, and Locoy
//! collector ingest with the historic numeric codes.

use std::collections::{BTreeMap, HashMap};

use axum::body::Bytes;
use axum::extract::{Query, State};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::Form;
use axum::Router;
use phpyun_core::{AppState, ClientIp};
use serde::Deserialize;
use validator::Validate;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/alipay", post(alipay))
        .route("/wechat-pay", post(wechat_pay))
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
