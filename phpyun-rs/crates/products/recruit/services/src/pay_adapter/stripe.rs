//! Stripe adapter: Hosted Checkout using method `config_json` (fallback `sy_stripe_sk`).

use phpyun_core::{ApiError, AppResult, AppState};
use super::{CheckoutIn, CheckoutOut};
use crate::pay_config;

pub async fn gateway_checkout(state: &AppState, input: CheckoutIn<'_>) -> AppResult<CheckoutOut> {
    let mut sk = pay_config::config_str(input.config_json, "secret_key");
    if sk.is_empty() {
        sk = pay_config::config_str(input.config_json, "sk");
    }
    if sk.is_empty() {
        return Err(ApiError::business("pay_not_configured"));
    }
    let cur = if input.currency.is_empty() {
        "usd"
    } else {
        input.currency
    };
    crate::stripe_service::create_gateway_session(
        state,
        &sk,
        input.pay_no,
        input.merchant_order_no,
        input.amount_cents,
        cur,
        input.subject,
        input.customer_email,
        input.success_url,
        input.cancel_url,
    )
    .await
}
