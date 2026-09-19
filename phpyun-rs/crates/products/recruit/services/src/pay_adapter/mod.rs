//! Channel adapters. Stripe is live; GCash / PayMaya return `not_configured`.

pub mod stripe;

use phpyun_core::{ApiError, AppResult, AppState};

pub fn method_ok(code: &str) -> bool {
    matches!(code, "stripe" | "gcash" | "paymaya")
}

pub fn charge_ready(code: &str) -> bool {
    code == "stripe"
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
    if !charge_ready(method) {
        return Err(ApiError::business("not_configured"));
    }
    stripe::gateway_checkout(state, input).await
}
