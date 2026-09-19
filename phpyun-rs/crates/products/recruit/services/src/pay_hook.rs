//! After Stripe Session is paid: mark gateway order; notify external merchant.

use phpyun_core::{clock, AppState};
use phpyun_models::pay::entity::{NotifyInsert, PayOrder};
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
    notify_status(state, &row, "paid", session_id).await;
}

pub async fn notify_status(state: &AppState, order: &PayOrder, event: &str, channel_ref: &str) {
    let event = match event {
        "paid" => "paid",
        "refunded" => "refunded",
        _ => return,
    };
    let Ok(Some(merchant)) = pay_repo::find_merchant_by_id(state.db.reader(), order.merchant_id).await
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
        "pay_no": order.pay_no,
        "merchant_order_no": order.merchant_order_no,
        "method": order.method_code,
        "amount_cents": order.amount_cents,
        "currency": order.currency,
        "status": event,
        "channel_ref": channel_ref,
    });
    deliver_notify(
        state,
        order.merchant_id,
        &order.pay_no,
        event,
        url,
        &body.to_string(),
    )
    .await;
}

pub async fn retry_notify(state: &AppState, id: u64) -> phpyun_core::AppResult<()> {
    let Some(row) = pay_repo::find_notify_by_id(state.db.reader(), id).await? else {
        return Err(phpyun_core::ApiError::param_invalid("id"));
    };
    let mut url = row.url.trim().to_string();
    let mut body = row.body.clone();
    if url.is_empty() || body.trim().is_empty() {
        let Some(order) = pay_repo::find_order_by_pay_no(state.db.reader(), &row.pay_no).await?
        else {
            return Err(phpyun_core::ApiError::param_invalid("id"));
        };
        let Some(merchant) =
            pay_repo::find_merchant_by_id(state.db.reader(), order.merchant_id).await?
        else {
            return Err(phpyun_core::ApiError::param_invalid("id"));
        };
        if url.is_empty() {
            url = merchant.notify_url.trim().to_string();
        }
        if body.trim().is_empty() {
            let event = match row.event.as_str() {
                "refunded" => "refunded",
                _ => "paid",
            };
            body = json!({
                "pay_no": order.pay_no,
                "merchant_order_no": order.merchant_order_no,
                "method": order.method_code,
                "amount_cents": order.amount_cents,
                "currency": order.currency,
                "status": event,
                "channel_ref": order.channel_ref,
            })
            .to_string();
        }
    }
    if url.is_empty() || !(url.starts_with("https://") || url.starts_with("http://")) {
        return Err(phpyun_core::ApiError::param_invalid("url"));
    }
    let event = match row.event.as_str() {
        "refunded" => "refunded",
        _ => "paid",
    };
    deliver_notify(state, row.merchant_id, &row.pay_no, event, &url, &body).await;
    Ok(())
}

async fn deliver_notify(
    state: &AppState,
    merchant_id: u64,
    pay_no: &str,
    event: &'static str,
    url: &str,
    body: &str,
) {
    let now = clock::now_ts();
    let url_store = clip(url, 512);
    let body_store = clip(body, 4096);
    let id = pay_repo::insert_notify(
        state.db.pool(),
        NotifyInsert {
            pay_no,
            merchant_id,
            event,
            url: &url_store,
            body: &body_store,
            http_status: 0,
            ok: 0,
            error: "",
        },
        now,
    )
    .await
    .unwrap_or(0);
    let (ok, status, err) = match state
        .http
        .post_body_status(url, "application/json", body.to_string())
        .await
    {
        Ok((s, _)) => {
            if (200..300).contains(&s) {
                (1, s as i32, String::new())
            } else {
                (0, s as i32, format!("http {s}"))
            }
        }
        Err(e) => {
            tracing::warn!(error = %e, pay_no, "pay merchant notify failed");
            (0, 0, clip(&e.to_string(), 255))
        }
    };
    if id > 0 {
        let _ = pay_repo::update_notify(state.db.pool(), id, status, ok, &err).await;
    } else {
        let _ = pay_repo::insert_notify(
            state.db.pool(),
            NotifyInsert {
                pay_no,
                merchant_id,
                event,
                url: &url_store,
                body: &body_store,
                http_status: status,
                ok,
                error: &err,
            },
            now,
        )
        .await;
    }
}

fn clip(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        s.chars().take(max).collect()
    }
}
