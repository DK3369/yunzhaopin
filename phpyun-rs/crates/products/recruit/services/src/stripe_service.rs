//! Stripe Checkout Sessions (`2026-08-26.dahlia`, `ui_mode=hosted_page`).
//!
//! Outbound HTTP goes through `phpyun_core::http_client`. Ledger rows live in
//! `phpyun_rs_stripe_order` (one row per site order).

use phpyun_core::clock;
use phpyun_core::hmac_sha256::verify_stripe_signature;
use phpyun_core::{ApiError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::stripe_order::entity::{EventPatch, LocalOrderIn, RequestPatch, SessionPatch};
use phpyun_models::stripe_order::repo as stripe_repo;
use phpyun_models::user::repo as user_repo;
use phpyun_models::vip::repo as vip_repo;
use serde_json::{json, Value};

use crate::payment_notify_service;
use crate::site_setting_service;

pub const STRIPE_API_VERSION: &str = "2026-08-26.dahlia";
const SESSIONS_URL: &str = "https://api.stripe.com/v1/checkout/sessions";
const WEBHOOKS_URL: &str = "https://api.stripe.com/v1/webhook_endpoints";
const SIG_TOLERANCE: i64 = 300;

async fn cfg_val(state: &AppState, key: &str) -> String {
    site_setting_service::get(state, key)
        .await
        .ok()
        .flatten()
        .map(|r| r.value)
        .unwrap_or_default()
}

pub async fn stripe_enabled(state: &AppState) -> bool {
    let flag = cfg_val(state, "stripe").await;
    if flag.trim() != "1" {
        return false;
    }
    !cfg_val(state, "sy_stripe_sk").await.trim().is_empty()
}

fn secret_key(state: &AppState) -> impl std::future::Future<Output = AppResult<String>> + '_ {
    async move {
        let sk = cfg_val(state, "sy_stripe_sk").await;
        let sk = sk.trim().to_string();
        if sk.is_empty() {
            return Err(ApiError::business("pay_not_configured"));
        }
        Ok(sk)
    }
}

fn currency(state: &AppState) -> impl std::future::Future<Output = String> + '_ {
    async move {
        let c = cfg_val(state, "sy_stripe_currency").await;
        let c = c.trim().to_ascii_lowercase();
        if c.is_empty() || !phpyun_models::sql::ident_ok(&c) {
            "usd".into()
        } else {
            c
        }
    }
}

fn web_base(state: &AppState) -> impl std::future::Future<Output = String> + '_ {
    async move {
        let mut u = cfg_val(state, "sy_weburl").await;
        while u.ends_with('/') {
            u.pop();
        }
        if u.is_empty() {
            "https://job1.ov6.com".into()
        } else {
            u
        }
    }
}

fn form_encode(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(char::from(b));
            }
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

fn form_push(buf: &mut String, k: &str, v: &str) {
    if !buf.is_empty() {
        buf.push('&');
    }
    buf.push_str(&form_encode(k));
    buf.push('=');
    buf.push_str(&form_encode(v));
}

pub fn session_id_ok(s: &str) -> bool {
    let n = s.len();
    if n < 8 || n > 255 {
        return false;
    }
    let Some(rest) = s.strip_prefix("cs_") else {
        return false;
    };
    rest.bytes()
        .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
}

async fn stripe_call(
    state: &AppState,
    method: &'static str,
    url: &str,
    sk: &str,
    form: Option<&str>,
) -> AppResult<Value> {
    let auth = format!("Bearer {sk}");
    let headers = [
        ("authorization", auth.as_str()),
        ("stripe-version", STRIPE_API_VERSION),
    ];
    let (status, text) = state
        .http
        .exchange_with_headers(method, url, &headers, form)
        .await?;
    if !(200..300).contains(&status) {
        let msg = serde_json::from_str::<Value>(&text)
            .ok()
            .and_then(|v| {
                v.get("error")
                    .and_then(|e| e.get("message"))
                    .and_then(|m| m.as_str())
                    .map(str::to_string)
            })
            .unwrap_or_else(|| {
                let t = text.trim();
                if t.len() > 240 {
                    format!("stripe {status}")
                } else if t.is_empty() {
                    format!("stripe {status}")
                } else {
                    t.to_string()
                }
            });
        return Err(ApiError::upstream(msg));
    }
    if text.is_empty() {
        return Ok(json!({}));
    }
    serde_json::from_str(&text).map_err(|_| ApiError::upstream("stripe_json"))
}

fn json_str(v: &Value, k: &str) -> String {
    match v.get(k) {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Number(n)) => n.to_string(),
        Some(Value::Bool(true)) => "true".into(),
        Some(Value::Bool(false)) => "false".into(),
        _ => String::new(),
    }
}

fn json_i64(v: &Value, k: &str) -> i64 {
    v.get(k).and_then(Value::as_i64).unwrap_or(0)
}

fn json_opt_i32(v: &Value, k: &str) -> Option<i32> {
    v.get(k)
        .and_then(Value::as_i64)
        .and_then(|n| i32::try_from(n).ok())
}

fn json_opt_bool_i32(v: &Value, k: &str) -> Option<i32> {
    match v.get(k) {
        Some(Value::Bool(true)) => Some(1),
        Some(Value::Bool(false)) => Some(0),
        Some(Value::Number(n)) => n.as_i64().and_then(|x| i32::try_from(x).ok()),
        _ => None,
    }
}

fn json_join(v: &Value, k: &str) -> String {
    match v.get(k) {
        Some(Value::Array(arr)) => arr
            .iter()
            .filter_map(|x| x.as_str())
            .collect::<Vec<_>>()
            .join(","),
        Some(Value::String(s)) => s.clone(),
        _ => String::new(),
    }
}

fn json_blob(v: &Value, k: &str) -> Option<String> {
    match v.get(k) {
        None | Some(Value::Null) => None,
        Some(other) => serde_json::to_string(other).ok(),
    }
}

fn id_or_expand(v: &Value, k: &str) -> String {
    match v.get(k) {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Object(o)) => o
            .get("id")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string(),
        _ => String::new(),
    }
}

fn nested_str(v: &Value, a: &str, b: &str) -> String {
    v.get(a)
        .and_then(|x| x.get(b))
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string()
}

fn nested_i32(v: &Value, a: &str, b: &str) -> Option<i32> {
    v.get(a)
        .and_then(|x| x.get(b))
        .and_then(Value::as_i64)
        .and_then(|n| i32::try_from(n).ok())
}

fn session_patch(sess: &Value) -> SessionPatch {
    let mut payment_intent = id_or_expand(sess, "payment_intent");
    let mut payment_method = String::new();
    let mut latest_charge = String::new();
    if let Some(pi) = sess.get("payment_intent") {
        if pi.is_object() {
            payment_method = id_or_expand(pi, "payment_method");
            latest_charge = id_or_expand(pi, "latest_charge");
            if payment_intent.is_empty() {
                payment_intent = json_str(pi, "id");
            }
        }
    }
    SessionPatch {
        stripe_session_id: json_str(sess, "id"),
        stripe_object: json_str(sess, "object"),
        stripe_livemode: json_opt_bool_i32(sess, "livemode").unwrap_or(0),
        stripe_created: json_i64(sess, "created"),
        stripe_expires_at: json_i64(sess, "expires_at"),
        stripe_status: json_str(sess, "status"),
        stripe_payment_status: json_str(sess, "payment_status"),
        stripe_mode: json_str(sess, "mode"),
        stripe_ui_mode: json_str(sess, "ui_mode"),
        stripe_currency: json_str(sess, "currency"),
        stripe_amount_subtotal: json_opt_i32(sess, "amount_subtotal"),
        stripe_amount_total: json_opt_i32(sess, "amount_total"),
        stripe_amount_discount: nested_i32(sess, "total_details", "amount_discount"),
        stripe_amount_shipping: nested_i32(sess, "total_details", "amount_shipping"),
        stripe_amount_tax: nested_i32(sess, "total_details", "amount_tax"),
        stripe_customer: id_or_expand(sess, "customer"),
        stripe_customer_email: json_str(sess, "customer_email"),
        stripe_customer_account: json_str(sess, "customer_account"),
        stripe_customer_creation: json_str(sess, "customer_creation"),
        stripe_customer_name: nested_str(sess, "customer_details", "name"),
        stripe_customer_phone: nested_str(sess, "customer_details", "phone"),
        stripe_customer_tax_exempt: nested_str(sess, "customer_details", "tax_exempt"),
        stripe_payment_intent: payment_intent,
        stripe_payment_link: id_or_expand(sess, "payment_link"),
        stripe_setup_intent: id_or_expand(sess, "setup_intent"),
        stripe_subscription: id_or_expand(sess, "subscription"),
        stripe_invoice: id_or_expand(sess, "invoice"),
        stripe_url: json_str(sess, "url"),
        stripe_success_url: json_str(sess, "success_url"),
        stripe_cancel_url: json_str(sess, "cancel_url"),
        stripe_return_url: json_str(sess, "return_url"),
        stripe_client_reference_id: json_str(sess, "client_reference_id"),
        stripe_locale: json_str(sess, "locale"),
        stripe_submit_type: json_str(sess, "submit_type"),
        stripe_billing_address_collection: json_str(sess, "billing_address_collection"),
        stripe_payment_method_collection: json_str(sess, "payment_method_collection"),
        stripe_payment_method_types: json_join(sess, "payment_method_types"),
        stripe_allowed_payment_method_types: json_join(sess, "allowed_payment_method_types"),
        stripe_excluded_payment_method_types: json_join(sess, "excluded_payment_method_types"),
        stripe_allow_promotion_codes: json_opt_bool_i32(sess, "allow_promotion_codes"),
        stripe_recovered_from: json_str(sess, "recovered_from"),
        stripe_redirect_on_completion: json_str(sess, "redirect_on_completion"),
        stripe_origin_context: json_str(sess, "origin_context"),
        stripe_integration_identifier: json_str(sess, "integration_identifier"),
        stripe_automatic_tax_enabled: sess
            .get("automatic_tax")
            .and_then(|t| json_opt_bool_i32(t, "enabled")),
        stripe_automatic_tax_status: nested_str(sess, "automatic_tax", "status"),
        stripe_payment_method: payment_method,
        stripe_latest_charge: latest_charge,
        stripe_adaptive_pricing_json: json_blob(sess, "adaptive_pricing"),
        stripe_after_expiration_json: json_blob(sess, "after_expiration"),
        stripe_automatic_tax_json: json_blob(sess, "automatic_tax"),
        stripe_branding_settings_json: json_blob(sess, "branding_settings"),
        stripe_collected_information_json: json_blob(sess, "collected_information"),
        stripe_consent_json: json_blob(sess, "consent"),
        stripe_consent_collection_json: json_blob(sess, "consent_collection"),
        stripe_custom_fields_json: json_blob(sess, "custom_fields"),
        stripe_custom_text_json: json_blob(sess, "custom_text"),
        stripe_customer_details_json: json_blob(sess, "customer_details"),
        stripe_currency_conversion_json: json_blob(sess, "currency_conversion"),
        stripe_discounts_json: json_blob(sess, "discounts"),
        stripe_invoice_creation_json: json_blob(sess, "invoice_creation"),
        stripe_line_items_json: json_blob(sess, "line_items"),
        stripe_metadata_json: json_blob(sess, "metadata"),
        stripe_name_collection_json: json_blob(sess, "name_collection"),
        stripe_optional_items_json: json_blob(sess, "optional_items"),
        stripe_payment_method_options_json: json_blob(sess, "payment_method_options"),
        stripe_payment_method_configuration_json: json_blob(
            sess,
            "payment_method_configuration_details",
        ),
        stripe_permissions_json: json_blob(sess, "permissions"),
        stripe_phone_number_collection_json: json_blob(sess, "phone_number_collection"),
        stripe_presentment_details_json: json_blob(sess, "presentment_details"),
        stripe_saved_payment_method_options_json: json_blob(sess, "saved_payment_method_options"),
        stripe_shipping_address_collection_json: json_blob(sess, "shipping_address_collection"),
        stripe_shipping_cost_json: json_blob(sess, "shipping_cost"),
        stripe_shipping_options_json: json_blob(sess, "shipping_options"),
        stripe_tax_id_collection_json: json_blob(sess, "tax_id_collection"),
        stripe_total_details_json: json_blob(sess, "total_details"),
        stripe_wallet_options_json: json_blob(sess, "wallet_options"),
        stripe_managed_payments_json: json_blob(sess, "managed_payments"),
        stripe_session_json: serde_json::to_string(sess).ok(),
    }
}

fn rating_from_code(code: &str) -> i32 {
    code.strip_prefix("pkg_")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}

fn member_ut(usertype: i32) -> i32 {
    if usertype == 1 {
        1
    } else {
        2
    }
}

fn success_path_for(usertype: i32, order_no: &str) -> String {
    if usertype == 1 {
        format!("/user/cashier/{order_no}")
    } else {
        format!("/com/cashier/{order_no}")
    }
}

pub async fn upsert_local(
    state: &AppState,
    order_no: &str,
    client_ip: &str,
    subject: Option<&str>,
) -> AppResult<()> {
    let o = vip_repo::find_any_order_by_no(state.db.reader(), order_no)
        .await?
        .ok_or_else(|| ApiError::param_invalid("order_not_found"))?;
    let ut = member_ut(o.usertype);
    let subj = subject
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or(o.package_code.trim());
    let subject = if subj.is_empty() {
        format!("order-{}", o.order_kind)
    } else {
        subj.to_string()
    };
    let yuan = f64::from(o.amount_cents.max(0)) / 100.0;
    let now = clock::now_ts();
    stripe_repo::upsert_local(
        state.db.pool(),
        LocalOrderIn {
            uid: o.uid,
            usertype: ut,
            order_no: &o.order_no,
            company_order_id: o.id,
            order_kind: o.order_kind,
            package_code: &o.package_code,
            rating: rating_from_code(&o.package_code),
            subject: &subject,
            amount_cents: o.amount_cents,
            amount_yuan: yuan,
            channel: "stripe",
            order_state: o.status,
            client_ip,
            success_path: &success_path_for(ut, &o.order_no),
            created_at: o.created_at,
        },
        now,
    )
    .await?;
    Ok(())
}

async fn customer_email(state: &AppState, uid: u64) -> String {
    match user_repo::find_by_uid(state.db.reader(), uid).await {
        Ok(Some(m)) => m
            .email
            .unwrap_or_default()
            .trim()
            .to_string(),
        _ => String::new(),
    }
}

fn email_ok(s: &str) -> bool {
    let s = s.trim();
    s.len() >= 5 && s.len() <= 128 && s.contains('@') && !s.contains(' ')
}

pub async fn create_checkout_url(
    state: &AppState,
    user: &AuthenticatedUser,
    order_no: &str,
    client_ip: &str,
) -> AppResult<String> {
    let o = vip_repo::find_any_order_by_no(state.db.reader(), order_no)
        .await?
        .ok_or_else(|| ApiError::param_invalid("order_not_found"))?;
    if o.uid != user.uid {
        return Err(ApiError::param_invalid("order_not_found"));
    }
    if o.status != 0 {
        return Err(ApiError::business("order_not_pending"));
    }
    upsert_local(state, order_no, client_ip, None).await?;
    let sk = secret_key(state).await?;
    let cur = currency(state).await;
    let base = web_base(state).await;
    let ut = member_ut(if o.usertype > 0 {
        o.usertype
    } else {
        i32::from(user.usertype)
    });
    let path = success_path_for(ut, order_no);
    let success_url = format!("{base}{path}?session_id={{CHECKOUT_SESSION_ID}}");
    let cancel_url = format!("{base}{path}");
    let subject = if o.package_code.trim().is_empty() {
        format!("order-{}", o.order_kind)
    } else {
        o.package_code.clone()
    };
    let email = customer_email(state, o.uid).await;
    let meta = json!({
        "order_no": order_no,
        "uid": o.uid.to_string(),
        "usertype": ut.to_string(),
        "package_code": o.package_code,
        "order_kind": o.order_kind.to_string(),
    });
    let meta_s = meta.to_string();
    let mut form = String::new();
    form_push(&mut form, "mode", "payment");
    form_push(&mut form, "ui_mode", "hosted_page");
    form_push(&mut form, "client_reference_id", order_no);
    form_push(&mut form, "success_url", &success_url);
    form_push(&mut form, "cancel_url", &cancel_url);
    form_push(&mut form, "managed_payments[enabled]", "false");
    form_push(&mut form, "adaptive_pricing[enabled]", "false");
    form_push(&mut form, "line_items[0][quantity]", "1");
    form_push(&mut form, "line_items[0][price_data][currency]", &cur);
    form_push(
        &mut form,
        "line_items[0][price_data][unit_amount]",
        &o.amount_cents.max(0).to_string(),
    );
    form_push(
        &mut form,
        "line_items[0][price_data][product_data][name]",
        &subject,
    );
    form_push(&mut form, "metadata[order_no]", order_no);
    form_push(&mut form, "metadata[uid]", &o.uid.to_string());
    form_push(&mut form, "metadata[usertype]", &ut.to_string());
    form_push(&mut form, "metadata[package_code]", &o.package_code);
    form_push(&mut form, "metadata[order_kind]", &o.order_kind.to_string());
    form_push(&mut form, "expand[0]", "payment_intent");
    if email_ok(&email) {
        form_push(&mut form, "customer_email", email.trim());
    }
    let now = clock::now_ts();
    stripe_repo::update_request(
        state.db.pool(),
        order_no,
        RequestPatch {
            req_mode: "payment",
            req_ui_mode: "hosted_page",
            req_currency: &cur,
            req_unit_amount: o.amount_cents,
            req_product_name: &subject,
            req_quantity: 1,
            req_success_url: &success_url,
            req_cancel_url: &cancel_url,
            req_client_reference_id: order_no,
            req_customer_email: if email_ok(&email) { email.trim() } else { "" },
            req_metadata_json: Some(meta_s.as_str()),
            req_body: &form,
            req_stripe_version: STRIPE_API_VERSION,
            req_at: now,
        },
        now,
    )
    .await?;
    let sess = stripe_call(state, "POST", SESSIONS_URL, &sk, Some(&form)).await?;
    let patch = session_patch(&sess);
    let url = patch.stripe_url.clone();
    if url.is_empty() {
        return Err(ApiError::upstream("stripe_no_url"));
    }
    stripe_repo::update_session(state.db.pool(), order_no, &patch, now).await?;
    Ok(url)
}

async fn retrieve_session(state: &AppState, sk: &str, session_id: &str) -> AppResult<Value> {
    let url = format!(
        "{SESSIONS_URL}/{}?expand[0]=payment_intent",
        form_encode(session_id)
    );
    stripe_call(state, "GET", &url, sk, None).await
}

fn metadata_str(sess: &Value, k: &str) -> String {
    sess.get("metadata")
        .and_then(|m| m.get(k))
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string()
}

fn session_paid(sess: &Value) -> bool {
    let pay = json_str(sess, "payment_status");
    pay == "paid" || pay == "no_payment_required"
}

async fn apply_session_and_maybe_settle(
    state: &AppState,
    order_no: &str,
    expected_uid: Option<u64>,
    sess: &Value,
) -> AppResult<bool> {
    let o = vip_repo::find_any_order_by_no(state.db.reader(), order_no)
        .await?
        .ok_or_else(|| ApiError::param_invalid("order_not_found"))?;
    if let Some(uid) = expected_uid {
        if o.uid != uid {
            return Err(ApiError::param_invalid("order_not_found"));
        }
    }
    let meta_no = metadata_str(sess, "order_no");
    let href = json_str(sess, "client_reference_id");
    if (!meta_no.is_empty() && meta_no != order_no) || (!href.is_empty() && href != order_no) {
        return Err(ApiError::param_invalid("order_mismatch"));
    }
    let meta_uid = metadata_str(sess, "uid");
    if !meta_uid.is_empty() && meta_uid != o.uid.to_string() {
        return Err(ApiError::param_invalid("order_mismatch"));
    }
    let now = clock::now_ts();
    let patch = session_patch(sess);
    stripe_repo::update_session(state.db.pool(), order_no, &patch, now).await?;
    if o.status == 1 {
        stripe_repo::mark_settled(
            state.db.pool(),
            order_no,
            if patch.stripe_payment_intent.is_empty() {
                &patch.stripe_session_id
            } else {
                &patch.stripe_payment_intent
            },
            now,
        )
        .await?;
        return Ok(true);
    }
    if !session_paid(sess) {
        return Ok(false);
    }
    let paid = patch.stripe_amount_total.unwrap_or(-1);
    if paid != o.amount_cents {
        tracing::warn!(
            order_no,
            expected = o.amount_cents,
            paid,
            "stripe amount mismatch; refuse settle"
        );
        return Err(ApiError::param_invalid("amount_mismatch"));
    }
    let tx = if patch.stripe_payment_intent.is_empty() {
        patch.stripe_session_id.clone()
    } else {
        patch.stripe_payment_intent.clone()
    };
    payment_notify_service::settle_paid_checked(state, order_no, &tx, paid).await?;
    stripe_repo::mark_settled(state.db.pool(), order_no, &tx, now).await?;
    Ok(true)
}

pub async fn retrieve_and_settle(
    state: &AppState,
    user: &AuthenticatedUser,
    order_no: &str,
    session_id: &str,
) -> AppResult<bool> {
    if !session_id_ok(session_id) {
        return Err(ApiError::param_invalid("session_id"));
    }
    let o = vip_repo::find_any_order_by_no(state.db.reader(), order_no)
        .await?
        .ok_or_else(|| ApiError::param_invalid("order_not_found"))?;
    if o.uid != user.uid {
        return Err(ApiError::param_invalid("order_not_found"));
    }
    let sk = secret_key(state).await?;
    let sess = retrieve_session(state, &sk, session_id).await?;
    apply_session_and_maybe_settle(state, order_no, Some(user.uid), &sess).await
}

pub async fn handle_webhook(state: &AppState, body: &[u8], sig_header: &str) -> AppResult<&'static str> {
    let secret = cfg_val(state, "sy_stripe_whsec").await;
    let secret = secret.trim().to_string();
    if secret.is_empty() {
        tracing::warn!("stripe webhook missing sy_stripe_whsec");
        return Err(ApiError::unauth());
    }
    let now = clock::now_ts();
    if !verify_stripe_signature(&secret, body, sig_header, now, SIG_TOLERANCE) {
        tracing::warn!(
            sig_len = sig_header.len(),
            body_len = body.len(),
            secret_len = secret.len(),
            now,
            "stripe webhook signature mismatch"
        );
        return Err(ApiError::unauth());
    }
    let event: Value = serde_json::from_slice(body).map_err(|_| ApiError::param_invalid("json"))?;
    let etype = json_str(&event, "type");
    if etype != "checkout.session.completed" && etype != "checkout.session.async_payment_succeeded"
    {
        return Ok("ignored");
    }
    let sess = event
        .get("data")
        .and_then(|d| d.get("object"))
        .cloned()
        .unwrap_or(Value::Null);
    if !sess.is_object() {
        return Err(ApiError::param_invalid("session"));
    }
    let mut order_no = metadata_str(&sess, "order_no");
    if order_no.is_empty() {
        order_no = json_str(&sess, "client_reference_id");
    }
    if order_no.is_empty() {
        let sid = json_str(&sess, "id");
        if let Ok(Some(row)) = stripe_repo::find_by_session_id(state.db.reader(), &sid).await {
            order_no = row.order_no;
        }
    }
    if order_no.is_empty() {
        return Err(ApiError::param_invalid("order_no"));
    }
    let event_json = serde_json::to_string(&event).ok();
    stripe_repo::update_event(
        state.db.pool(),
        &order_no,
        EventPatch {
            stripe_event_id: &json_str(&event, "id"),
            stripe_event_type: &etype,
            stripe_event_created: json_i64(&event, "created"),
            stripe_api_version: &json_str(&event, "api_version"),
            stripe_event_json: event_json.as_deref(),
        },
        now,
    )
    .await?;
    if let Err(e) = apply_session_and_maybe_settle(state, &order_no, None, &sess).await {
        tracing::warn!(error = %e, order_no, "stripe webhook settle skipped");
    }
    Ok("ok")
}

pub async fn ensure_webhook(state: &AppState, user: &AuthenticatedUser) -> AppResult<()> {
    user.require_admin()?;
    let sk = secret_key(state).await?;
    let base = web_base(state).await;
    let url = format!("{base}/callback/stripe");
    let existing = cfg_val(state, "sy_stripe_whsec").await;
    if !existing.trim().is_empty() {
        return Ok(());
    }
    let mut form = String::new();
    form_push(&mut form, "url", &url);
    form_push(&mut form, "enabled_events[0]", "checkout.session.completed");
    form_push(
        &mut form,
        "enabled_events[1]",
        "checkout.session.async_payment_succeeded",
    );
    form_push(&mut form, "api_version", STRIPE_API_VERSION);
    match stripe_call(state, "POST", WEBHOOKS_URL, &sk, Some(&form)).await {
        Ok(v) => {
            if let Some(sec) = v.get("secret").and_then(Value::as_str) {
                if !sec.is_empty() {
                    site_setting_service::admin_upsert(
                        state,
                        user,
                        crate::site_setting_service::UpsertInput {
                            key: "sy_stripe_whsec",
                            value: sec,
                            description: "",
                            is_public: false,
                        },
                    )
                    .await?;
                }
            }
            Ok(())
        }
        Err(e) => {
            tracing::warn!(error = %e, "stripe webhook endpoint create skipped");
            Ok(())
        }
    }
}
