//! After Stripe Session is paid: mark gateway order; notify external merchant.

use phpyun_core::{clock, AppState};
use phpyun_models::pay::repo as pay_repo;
use serde_json::json;

pub async fn on_stripe_paid(
    state: &AppState,
    pay_no: &str,
    merchant_order_no: &str,
    session_id: &str,
    amount_total: Option<i32>,
) {
    let now = clock::now_ts();
    let row = if !pay_no.trim().is_empty() {
        pay_repo::find_order_by_pay_no(state.db.reader(), pay_no)
            .await
            .ok()
            .flatten()
    } else if !session_id.trim().is_empty() {
        pay_repo::find_order_by_channel_ref(state.db.reader(), session_id)
            .await
            .ok()
            .flatten()
    } else if !merchant_order_no.trim().is_empty() {
        if let Ok(Some(m)) = pay_repo::find_merchant_by_code(state.db.reader(), "ov6").await {
            pay_repo::find_order_by_merchant_ref(
                state.db.reader(),
                m.id,
                merchant_order_no,
                "stripe",
            )
            .await
            .ok()
            .flatten()
        } else {
            None
        }
    } else {
        None
    };
    let Some(row) = row else {
        return;
    };
    if let Some(paid) = amount_total {
        if paid != row.amount_cents {
            tracing::warn!(
                pay_no = %row.pay_no,
                expected = row.amount_cents,
                paid,
                "pay gateway amount mismatch"
            );
            return;
        }
    }
    let n = pay_repo::mark_paid(state.db.pool(), &row.pay_no, session_id, now)
        .await
        .unwrap_or(0);
    if n == 0 {
        return;
    }
    let Ok(Some(merchant)) = pay_repo::find_merchant_by_id(state.db.reader(), row.merchant_id).await
    else {
        return;
    };
    if merchant.code == "ov6" {
        return;
    }
    let url = merchant.notify_url.trim();
    if url.is_empty() || !(url.starts_with("https://") || url.starts_with("http://")) {
        return;
    }
    let body = json!({
        "pay_no": row.pay_no,
        "merchant_order_no": row.merchant_order_no,
        "method": row.method_code,
        "amount_cents": row.amount_cents,
        "currency": row.currency,
        "status": "paid",
        "channel_ref": session_id,
    });
    let text = body.to_string();
    if let Err(e) = state.http.post_text(url, text).await {
        tracing::warn!(error = %e, pay_no = %row.pay_no, "pay merchant notify failed");
    }
}
