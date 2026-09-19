//! HMAC merchant API (`/v1/pay/*`).

use axum::body::Bytes;
use axum::extract::State;
use axum::http::{HeaderMap, Method};
use axum::routing::post;
use axum::Router;
use phpyun_core::{ApiError, ApiResponse, AppResult, AppState, ClientIp};
use phpyun_services::pay_service::{self, GatewayPay, MerchantOrderIn, MethodView, OrderView};
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/methods/list", post(methods_list))
        .route("/orders", post(create_order))
        .route("/orders/detail", post(order_detail))
        .route("/orders/close", post(order_close))
        .route("/orders/refund", post(order_refund))
}

fn auth_header(headers: &HeaderMap) -> String {
    headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string()
}

async fn merchant_at(
    state: &AppState,
    method: &Method,
    path: &str,
    headers: &HeaderMap,
    body: &[u8],
    client_ip: &str,
) -> AppResult<pay_service::MerchantAuth> {
    pay_service::authenticate(
        state,
        method.as_str(),
        path,
        &auth_header(headers),
        body,
        client_ip,
    )
    .await
}

fn parse_json<T: serde::de::DeserializeOwned + Validate>(body: &[u8]) -> AppResult<T> {
    let v: T = serde_json::from_slice(body).map_err(|_| ApiError::param_invalid("json"))?;
    v.validate()
        .map_err(|_| ApiError::param_invalid("json"))?;
    Ok(v)
}

pub async fn methods_list(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
) -> AppResult<ApiResponse<Vec<MethodView>>> {
    let auth = merchant_at(&state, &method, "/v1/pay/methods/list", &headers, &body, &ip).await?;
    Ok(ApiResponse::data(
        pay_service::merchant_methods(&state, &auth.merchant).await?,
    ))
}

#[utoipa::path(
    post,
    path = "/v1/pay/methods/list",
    tag = "pay",
    responses((status = 200, description = "HMAC merchant methods"))
)]
pub fn spec_methods_list() {}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateOrderForm {
    #[validate(length(min = 1, max = 64))]
    pub merchant_order_no: String,
    #[validate(length(min = 1, max = 64))]
    pub method: String,
    #[validate(range(min = 1, max = 100_000_000))]
    pub amount_cents: i32,
    #[serde(default)]
    #[validate(length(max = 16))]
    pub currency: String,
    #[validate(length(min = 1, max = 255))]
    pub subject: String,
    #[serde(default)]
    #[validate(length(max = 128))]
    pub customer_email: String,
    #[serde(default)]
    #[validate(length(max = 512))]
    pub success_url: String,
    #[serde(default)]
    #[validate(length(max = 512))]
    pub cancel_url: String,
}

#[utoipa::path(
    post,
    path = "/v1/pay/orders",
    tag = "pay",
    request_body = CreateOrderForm,
    responses((status = 200, description = "HMAC create order"))
)]
pub fn spec_create_order() {}

pub async fn create_order(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
) -> AppResult<ApiResponse<GatewayPay>> {
    let auth = merchant_at(&state, &method, "/v1/pay/orders", &headers, &body, &ip).await?;
    let f: CreateOrderForm = parse_json(&body)?;
    let g = pay_service::create_for_merchant(
        &state,
        &auth.merchant,
        MerchantOrderIn {
            merchant_order_no: f.merchant_order_no,
            method: f.method,
            amount_cents: f.amount_cents,
            currency: f.currency,
            subject: f.subject,
            customer_email: f.customer_email,
            success_url: f.success_url,
            cancel_url: f.cancel_url,
        },
    )
    .await?;
    Ok(ApiResponse::data(g))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PayNoForm {
    #[validate(length(min = 1, max = 64))]
    pub pay_no: String,
}

#[utoipa::path(
    post,
    path = "/v1/pay/orders/detail",
    tag = "pay",
    request_body = PayNoForm,
    responses((status = 200, description = "HMAC order detail"))
)]
pub fn spec_order_detail() {}

pub async fn order_detail(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
) -> AppResult<ApiResponse<OrderView>> {
    let auth = merchant_at(&state, &method, "/v1/pay/orders/detail", &headers, &body, &ip).await?;
    let f: PayNoForm = parse_json(&body)?;
    Ok(ApiResponse::data(
        pay_service::merchant_order(&state, &auth.merchant, &f.pay_no).await?,
    ))
}

#[utoipa::path(
    post,
    path = "/v1/pay/orders/close",
    tag = "pay",
    request_body = PayNoForm,
    responses((status = 200, description = "HMAC close pending order"))
)]
pub fn spec_order_close() {}

pub async fn order_close(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
) -> AppResult<ApiResponse<serde_json::Value>> {
    let auth = merchant_at(&state, &method, "/v1/pay/orders/close", &headers, &body, &ip).await?;
    let f: PayNoForm = parse_json(&body)?;
    pay_service::close_order(&state, Some(&auth.merchant), &f.pay_no).await?;
    Ok(ApiResponse::data(serde_json::json!({ "ok": true })))
}

#[utoipa::path(
    post,
    path = "/v1/pay/orders/refund",
    tag = "pay",
    request_body = PayNoForm,
    responses((status = 200, description = "HMAC refund paid Stripe order"))
)]
pub fn spec_order_refund() {}

pub async fn order_refund(
    State(state): State<AppState>,
    ClientIp(ip): ClientIp,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
) -> AppResult<ApiResponse<serde_json::Value>> {
    let auth = merchant_at(&state, &method, "/v1/pay/orders/refund", &headers, &body, &ip).await?;
    let f: PayNoForm = parse_json(&body)?;
    pay_service::refund_order(&state, Some(&auth.merchant), &f.pay_no).await?;
    Ok(ApiResponse::data(serde_json::json!({ "ok": true })))
}
