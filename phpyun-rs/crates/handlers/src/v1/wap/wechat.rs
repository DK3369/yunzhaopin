//! WeChat Official Account integration webhook (mirrors PHPYun `weixin/index_action`).
//!
//! Path: `/v1/wap/wechat/callback`
//!
//! - **GET**: integration verification, echoes back `echostr` verbatim (plain text).
//! - **POST**: parse XML message -> business routing -> return XML reply (or `success`).
//!
//! **The response must be raw text/xml or text/plain**, not our standard JSON envelope.
//! That is why we use `axum::response::Response` directly to return the raw body.

use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::Response,
    routing::get,
    Router,
};
use phpyun_core::i18n::{t, Lang};
use phpyun_core::{ApiJson, AppError, AppResult, AppState, InfraError};
use phpyun_services::wechat_api_service;
use phpyun_services::wechat_service::{
    self, default_reply, parse_incoming, verify_signature, SUCCESS_ACK,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/wechat/callback", get(verify).post(receive))
        .route("/wechat/qr/{kind}/{id}", get(qr_for_resource))
}

#[derive(Debug, Deserialize)]
pub struct VerifyQuery {
    pub signature: String,
    pub timestamp: String,
    pub nonce: String,
    pub echostr: Option<String>,
}

/// WeChat integration verification: if the signature matches, echo `echostr` back verbatim.
pub async fn verify(
    State(state): State<AppState>,
    Query(q): Query<VerifyQuery>,
) -> Response {
    let Some(token) = state.config.wechat_token.as_deref() else {
        return plain(StatusCode::SERVICE_UNAVAILABLE, "wechat_not_configured");
    };
    if !verify_signature(token, &q.timestamp, &q.nonce, &q.signature) {
        return plain(StatusCode::FORBIDDEN, "bad_signature");
    }
    plain(StatusCode::OK, &q.echostr.unwrap_or_default())
}

/// WeChat event/message entry point: signature + XML parsing + reply (or `success`).
pub async fn receive(
    State(state): State<AppState>,
    Query(q): Query<VerifyQuery>,
    body: String,
) -> Response {
    let Some(token) = state.config.wechat_token.as_deref() else {
        return plain(StatusCode::SERVICE_UNAVAILABLE, "wechat_not_configured");
    };
    if !verify_signature(token, &q.timestamp, &q.nonce, &q.signature) {
        return plain(StatusCode::FORBIDDEN, "bad_signature");
    }

    let msg = parse_incoming(&body);
    // Emit inbound audit log (for downstream offline analysis of user behaviour)
    tracing::debug!(
        msg_type = %msg.msg_type,
        event = ?msg.event,
        "wechat inbound"
    );

    // If the operator configured a fixed welcome message, use that; otherwise pull i18n `wechat.welcome_default` in the system default language.
    let welcome_default = t("wechat.welcome_default", Lang::ZhCN);
    let welcome = state
        .config
        .wechat_welcome_message
        .as_deref()
        .unwrap_or(&welcome_default);

    match default_reply(&msg, welcome) {
        Some(xml) => xml_response(xml),
        None => plain(StatusCode::OK, SUCCESS_ACK),
    }
}

// ---- QR code generation ----

#[derive(Debug, Serialize, ToSchema)]
pub struct QrView {
    pub ticket: String,
    pub show_url: String,
    pub expire_seconds: u64,
}

#[derive(Debug, Deserialize, utoipa::IntoParams)]
pub struct QrOpts {
    /// Optional `scene_str` prefix (defaults to `weixin`).
    #[serde(default)]
    pub tag: Option<String>,
    /// QR code lifetime in seconds (default 7 days; range 60..=2592000)
    #[serde(default = "default_expire")]
    pub expire: u64,
}
fn default_expire() -> u64 {
    7 * 24 * 3600
}

/// Generate a parameterised QR code for a given business resource (mirrors the PHPYun `pubWxQrcode` branch).
/// `kind` is one of: job / resume / company / article / announcement / jobtel /
///         parttel / comtel / part / register / gongzhao
#[utoipa::path(
    get,
    path = "/v1/wap/wechat/qr/{kind}/{id}",
    tag = "wap",
    params(
        ("kind" = String, Path),
        ("id" = u64, Path),
        QrOpts,
    ),
    responses(
        (status = 200, description = "ok", body = QrView),
        (status = 400, description = "kind not in allow list / wechat not configured"),
        (status = 502, description = "WeChat upstream error"),
    )
)]
pub async fn qr_for_resource(
    State(state): State<AppState>,
    Path((kind, id)): Path<(String, u64)>,
    Query(opts): Query<QrOpts>,
) -> AppResult<ApiJson<QrView>> {
    let tag = opts.tag.as_deref().unwrap_or("weixin");
    let scene = wechat_api_service::scene_str_for(&kind, id, tag)
        .ok_or_else(|| AppError::new(InfraError::InvalidParam(format!("kind={kind}"))))?;
    let qr = wechat_api_service::create_qr_scene(&state, &scene, opts.expire).await?;
    Ok(ApiJson(QrView {
        ticket: qr.ticket,
        show_url: qr.show_url,
        expire_seconds: qr.expire_seconds,
    }))
}

// ---- Utilities ----

fn plain(status: StatusCode, body: &str) -> Response {
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
        .body(body.to_string().into())
        .unwrap_or_else(|_| fallback(status))
}

fn xml_response(body: String) -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
        .body(body.into())
        .unwrap_or_else(|_| fallback(StatusCode::INTERNAL_SERVER_ERROR))
}

fn fallback(status: StatusCode) -> Response {
    Response::builder()
        .status(status)
        .body(String::new().into())
        .unwrap()
}

// Silence the unused-import warning for wechat_service
#[allow(dead_code)]
fn _keep_service_imported_reference() -> &'static str {
    wechat_service::SUCCESS_ACK
}
