//! Channel adapters. Stripe is live; others in the catalog return `not_configured`.

pub mod stripe;

use phpyun_core::{ApiError, AppResult, AppState};

/// Static catalog: `code` is matched to a `&'static str` (never interpolated into SQL).
pub const CHANNELS: &[(&str, &str, bool)] = &[
    ("stripe", "Stripe", true),
    ("gcash", "GCash", false),
    ("paymaya", "PayMaya", false),
    ("paypal", "PayPal", false),
    ("grabpay", "GrabPay", false),
];

pub fn method_ok(code: &str) -> bool {
    catalog_name(code).is_some()
}

pub fn catalog_name(code: &str) -> Option<&'static str> {
    match code {
        "stripe" => Some("Stripe"),
        "gcash" => Some("GCash"),
        "paymaya" => Some("PayMaya"),
        "paypal" => Some("PayPal"),
        "grabpay" => Some("GrabPay"),
        _ => None,
    }
}

pub fn charge_ready(code: &str, config_json: &str) -> bool {
    if code != "stripe" {
        return false;
    }
    !crate::pay_config::secret_key(config_json).is_empty()
}

pub struct CheckoutIn<'a> {
    pub pay_no: &'a str,
    pub merchant_order_no: &'a str,
    pub amount_cents: i32,
    pub currency: &'a str,
    pub subject: &'a str,
    pub customer_email: &'a str,
    pub success_url: &'a str,
    pub cancel_url: &'a str,
    pub config_json: &'a str,
}

pub struct CheckoutOut {
    pub pay_url: String,
    pub channel_ref: String,
}

pub async fn create_checkout(
    state: &AppState,
    method: &str,
    input: CheckoutIn<'_>,
) -> AppResult<CheckoutOut> {
    if !method_ok(method) {
        return Err(ApiError::param_invalid("method"));
    }
    if method != "stripe" {
        return Err(ApiError::business("not_configured"));
    }
    stripe::gateway_checkout(state, input).await
}
