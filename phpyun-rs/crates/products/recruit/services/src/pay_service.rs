//! Third-party payment gateway (OV6 + HMAC merchants).

use phpyun_core::clock;
use phpyun_core::hmac_sha256::verify_merchant_sign;
use phpyun_core::utils::fmt_dt;
use phpyun_core::{ApiError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::pay::entity::{
    MerchantWrite, MethodWrite, OrderInsert, PayMerchant, PayMethod, PayOrderListRow,
};
use phpyun_models::pay::repo as pay_repo;
use phpyun_models::sql::ident_ok;
use phpyun_models::vip::repo as vip_repo;
use serde::Serialize;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::pay_adapter;
use crate::pay_config;
use crate::site_setting_service;
use crate::stripe_service;

const SIG_TOLERANCE: i64 = 300;
const OV6: &str = "ov6";

#[derive(Debug, Clone, Serialize)]
pub struct MerchantView {
    pub id: u64,
    pub code: String,
    pub name: String,
    pub api_key: String,
    pub notify_url: String,
    pub return_url: String,
    pub status: String,
    pub ctime: i64,
    pub ctime_n: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MerchantCreated {
    pub id: u64,
    pub code: String,
    pub api_key: String,
    pub api_secret: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct MethodView {
    pub id: u64,
    pub merchant_id: u64,
    pub code: String,
    pub name: String,
    pub status: String,
    pub sort: i32,
    pub charge_ready: bool,
    pub secret_key_set: bool,
    pub webhook_secret_set: bool,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrderView {
    pub id: u64,
    pub pay_no: String,
    pub merchant_code: String,
    pub merchant_name: String,
    pub merchant_order_no: String,
    pub method_code: String,
    pub amount_cents: i32,
    pub amount_yuan: f64,
    pub currency: String,
    pub status: String,
    pub channel_ref: String,
    pub subject: String,
    pub paid_at: i64,
    pub ctime: i64,
    pub ctime_n: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GatewayPay {
    pub pay_no: String,
    pub pay_url: String,
    pub method: String,
    pub status: String,
}

pub struct MerchantAuth {
    pub merchant: PayMerchant,
}

pub fn status_ok(s: &str) -> bool {
    matches!(s, "active" | "paused")
}

pub fn order_status_ok(s: &str) -> bool {
    matches!(s, "pending" | "paid" | "failed" | "cancelled")
}

fn merchant_view(m: &PayMerchant) -> MerchantView {
    MerchantView {
        id: m.id,
        code: m.code.clone(),
        name: m.name.clone(),
        api_key: m.api_key.clone(),
        notify_url: m.notify_url.clone(),
        return_url: m.return_url.clone(),
        status: m.status.clone(),
        ctime: m.ctime,
        ctime_n: fmt_dt(m.ctime),
    }
}

fn method_view(m: &PayMethod) -> MethodView {
    let sk = pay_config::config_str(&m.config_json, "secret_key");
    let sk = if sk.is_empty() {
        pay_config::config_str(&m.config_json, "sk")
    } else {
        sk
    };
    let wh = pay_config::config_str(&m.config_json, "webhook_secret");
    let currency = pay_config::config_str(&m.config_json, "currency");
    MethodView {
        id: m.id,
        merchant_id: m.merchant_id,
        code: m.code.clone(),
        name: m.name.clone(),
        status: m.status.clone(),
        sort: m.sort,
        charge_ready: pay_adapter::charge_ready(&m.code),
        secret_key_set: !sk.is_empty(),
        webhook_secret_set: !wh.is_empty(),
        currency,
    }
}

fn order_view_from_list(r: PayOrderListRow) -> OrderView {
    OrderView {
        id: r.id,
        pay_no: r.pay_no,
        merchant_code: r.merchant_code,
        merchant_name: r.merchant_name,
        merchant_order_no: r.merchant_order_no,
        method_code: r.method_code,
        amount_yuan: f64::from(r.amount_cents.max(0)) / 100.0,
        amount_cents: r.amount_cents,
        currency: r.currency,
        status: r.status,
        channel_ref: r.channel_ref,
        subject: r.subject,
        paid_at: r.paid_at,
        ctime: r.ctime,
        ctime_n: fmt_dt(r.ctime),
    }
}

fn new_pay_no(now: i64) -> String {
    let hex = Uuid::new_v4().simple().to_string();
    format!("p{now}{}", &hex[..8])
}

fn new_key(prefix: &str) -> String {
    let hex = Uuid::new_v4().simple().to_string();
    format!("{prefix}{}", &hex[..16])
}

fn new_secret() -> String {
    Uuid::new_v4().simple().to_string() + &Uuid::new_v4().simple().to_string()
}

fn parse_auth(header: &str) -> Option<(String, i64, String)> {
    let h = header.trim();
    let rest = h
        .strip_prefix("HMAC-SHA256 ")
        .or_else(|| h.strip_prefix("HMAC-SHA256"))
        .unwrap_or(h)
        .trim();
    let mut key_id = String::new();
    let mut ts: Option<i64> = None;
    let mut sign = String::new();
    for part in rest.split(|c: char| c == ',' || c.is_whitespace()) {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        if let Some(v) = part.strip_prefix("key_id=") {
            key_id = v.trim_matches('"').to_string();
        } else if let Some(v) = part.strip_prefix("ts=") {
            ts = v.trim_matches('"').parse().ok();
        } else if let Some(v) = part.strip_prefix("sign=") {
            sign = v.trim_matches('"').to_string();
        }
    }
    if !ident_ok(&key_id) || sign.is_empty() {
        return None;
    }
    Some((key_id, ts?, sign))
}

pub async fn authenticate(
    state: &AppState,
    method: &str,
    path: &str,
    auth_header: &str,
    body: &[u8],
) -> AppResult<MerchantAuth> {
    let Some((key_id, ts, sign)) = parse_auth(auth_header) else {
        return Err(ApiError::unauth());
    };
    let now = clock::now_ts();
    if now.abs_diff(ts) > SIG_TOLERANCE as u64 {
        return Err(ApiError::unauth());
    }
    let Some(m) = pay_repo::find_merchant_by_api_key(state.db.reader(), &key_id).await? else {
        return Err(ApiError::unauth());
    };
    if m.status != "active" || m.api_secret.trim().is_empty() {
        return Err(ApiError::unauth());
    }
    let meth = method.to_ascii_uppercase();
    if !verify_merchant_sign(&m.api_secret, ts, &meth, path, body, &sign) {
        return Err(ApiError::unauth());
    }
    Ok(MerchantAuth { merchant: m })
}

pub async fn ov6_merchant(state: &AppState) -> AppResult<PayMerchant> {
    pay_repo::find_merchant_by_code(state.db.reader(), OV6)
        .await?
        .ok_or_else(|| ApiError::business("pay_not_configured"))
}

pub async fn available_channels(state: &AppState) -> AppResult<Vec<String>> {
    let Ok(Some(m)) = pay_repo::find_merchant_by_code(state.db.reader(), OV6).await else {
        return Ok(Vec::new());
    };
    if m.status != "active" {
        return Ok(Vec::new());
    }
    let rows = pay_repo::list_active_methods(state.db.reader(), m.id).await?;
    Ok(rows.into_iter().map(|x| x.code).collect())
}

pub async fn assert_create_channel(state: &AppState, channel: &str) -> AppResult<()> {
    let ch = match channel {
        "wxh5" => "wxpay",
        other => other,
    };
    let channels = available_channels(state).await?;
    if channels.is_empty() {
        return stripe_service::assert_create_channel(state, ch).await;
    }
    if !channels.iter().any(|c| c == ch) {
        return Err(ApiError::param_invalid("channel"));
    }
    Ok(())
}

async fn resolve_currency(state: &AppState, method: &PayMethod) -> String {
    let mut c = pay_config::config_str(&method.config_json, "currency");
    if c.is_empty() {
        c = site_setting_service::get(state, "sy_stripe_currency")
            .await
            .ok()
            .flatten()
            .map(|r| r.value)
            .unwrap_or_default();
    }
    let c = c.trim().to_ascii_lowercase();
    if c.is_empty() || !ident_ok(&c) {
        "usd".into()
    } else {
        c
    }
}

async fn web_base(state: &AppState) -> String {
    let mut u = site_setting_service::get(state, "sy_weburl")
        .await
        .ok()
        .flatten()
        .map(|r| r.value)
        .unwrap_or_default();
    while u.ends_with('/') {
        u.pop();
    }
    if u.is_empty() {
        "https://job1.ov6.com".into()
    } else {
        u
    }
}

fn merge_config(existing: &str, secret_key: Option<&str>, webhook_secret: Option<&str>, currency: Option<&str>) -> String {
    let mut v: Value = serde_json::from_str(existing).unwrap_or_else(|_| json!({}));
    if !v.is_object() {
        v = json!({});
    }
    if let Some(sk) = secret_key {
        let t = sk.trim();
        if !t.is_empty() {
            v["secret_key"] = json!(t);
        }
    }
    if let Some(wh) = webhook_secret {
        let t = wh.trim();
        if !t.is_empty() {
            v["webhook_secret"] = json!(t);
        }
    }
    if let Some(c) = currency {
        let t = c.trim().to_ascii_lowercase();
        if !t.is_empty() && ident_ok(&t) {
            v["currency"] = json!(t);
        }
    }
    v.to_string()
}

pub async fn admin_list_merchants(state: &AppState) -> AppResult<Vec<MerchantView>> {
    let rows = pay_repo::list_merchants(state.db.reader()).await?;
    Ok(rows.iter().map(merchant_view).collect())
}

pub struct MerchantSaveIn {
    pub id: u64,
    pub code: String,
    pub name: String,
    pub notify_url: String,
    pub return_url: String,
    pub status: String,
    pub rotate_secret: bool,
}

pub async fn admin_save_merchant(
    state: &AppState,
    _user: &AuthenticatedUser,
    f: MerchantSaveIn,
) -> AppResult<MerchantCreated> {
    let status = if f.status.is_empty() {
        "active".into()
    } else {
        f.status
    };
    if !status_ok(&status) {
        return Err(ApiError::param_invalid("status"));
    }
    let name = f.name.trim();
    if name.is_empty() || name.len() > 128 {
        return Err(ApiError::param_invalid("name"));
    }
    let now = clock::now_ts();
    if f.id == 0 {
        let code = f.code.trim().to_ascii_lowercase();
        if !ident_ok(&code) {
            return Err(ApiError::param_invalid("code"));
        }
        if pay_repo::find_merchant_by_code(state.db.reader(), &code)
            .await?
            .is_some()
        {
            return Err(ApiError::business("code_exists"));
        }
        let api_key = new_key("k");
        let api_secret = new_secret();
        let id = pay_repo::insert_merchant(
            state.db.pool(),
            MerchantWrite {
                code: &code,
                name,
                api_key: &api_key,
                api_secret: &api_secret,
                notify_url: f.notify_url.trim(),
                return_url: f.return_url.trim(),
                status: &status,
            },
            now,
        )
        .await?;
        return Ok(MerchantCreated {
            id,
            code,
            api_key,
            api_secret,
        });
    }
    let Some(old) = pay_repo::find_merchant_by_id(state.db.reader(), f.id).await? else {
        return Err(ApiError::param_invalid("id"));
    };
    let secret = if f.rotate_secret {
        Some(new_secret())
    } else {
        None
    };
    pay_repo::update_merchant(
        state.db.pool(),
        f.id,
        name,
        f.notify_url.trim(),
        f.return_url.trim(),
        &status,
        secret.as_deref(),
        now,
    )
    .await?;
    Ok(MerchantCreated {
        id: f.id,
        code: old.code,
        api_key: old.api_key,
        api_secret: secret.unwrap_or_default(),
    })
}

pub async fn admin_set_merchant_status(
    state: &AppState,
    id: u64,
    status: &str,
) -> AppResult<()> {
    if !status_ok(status) {
        return Err(ApiError::param_invalid("status"));
    }
    let n = pay_repo::set_merchant_status(state.db.pool(), id, status, clock::now_ts()).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("id"));
    }
    Ok(())
}

pub async fn admin_list_methods(
    state: &AppState,
    merchant_id: Option<u64>,
) -> AppResult<Vec<MethodView>> {
    let rows = pay_repo::list_methods(state.db.reader(), merchant_id).await?;
    Ok(rows.iter().map(method_view).collect())
}

pub struct MethodSaveIn {
    pub id: u64,
    pub merchant_id: u64,
    pub code: String,
    pub name: String,
    pub status: String,
    pub sort: i32,
    pub secret_key: String,
    pub webhook_secret: String,
    pub currency: String,
}

pub async fn admin_save_method(state: &AppState, f: MethodSaveIn) -> AppResult<u64> {
    let code = f.code.trim().to_ascii_lowercase();
    if !pay_adapter::method_ok(&code) {
        return Err(ApiError::param_invalid("code"));
    }
    let status = if f.status.is_empty() {
        "paused".into()
    } else {
        f.status
    };
    if !status_ok(&status) {
        return Err(ApiError::param_invalid("status"));
    }
    let name = f.name.trim();
    if name.is_empty() || name.len() > 128 {
        return Err(ApiError::param_invalid("name"));
    }
    if pay_repo::find_merchant_by_id(state.db.reader(), f.merchant_id)
        .await?
        .is_none()
    {
        return Err(ApiError::param_invalid("merchant_id"));
    }
    let now = clock::now_ts();
    if f.id == 0 {
        if pay_repo::find_method(state.db.reader(), f.merchant_id, &code)
            .await?
            .is_some()
        {
            return Err(ApiError::business("code_exists"));
        }
        let cfg = merge_config(
            "{}",
            Some(f.secret_key.as_str()),
            Some(f.webhook_secret.as_str()),
            Some(f.currency.as_str()),
        );
        let id = pay_repo::insert_method(
            state.db.pool(),
            MethodWrite {
                merchant_id: f.merchant_id,
                code: &code,
                name,
                status: &status,
                config_json: &cfg,
                sort: f.sort,
            },
            now,
        )
        .await?;
        return Ok(id);
    }
    let Some(old) = pay_repo::find_method_by_id(state.db.reader(), f.id).await? else {
        return Err(ApiError::param_invalid("id"));
    };
    let cfg = merge_config(
        &old.config_json,
        Some(f.secret_key.as_str()),
        Some(f.webhook_secret.as_str()),
        Some(f.currency.as_str()),
    );
    pay_repo::update_method(
        state.db.pool(),
        f.id,
        name,
        &status,
        &cfg,
        f.sort,
        now,
    )
    .await?;
    Ok(f.id)
}

pub async fn admin_set_method_status(state: &AppState, id: u64, status: &str) -> AppResult<()> {
    if !status_ok(status) {
        return Err(ApiError::param_invalid("status"));
    }
    let n = pay_repo::set_method_status(state.db.pool(), id, status, clock::now_ts()).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("id"));
    }
    Ok(())
}

pub async fn admin_delete_method(state: &AppState, id: u64) -> AppResult<()> {
    let Some(m) = pay_repo::find_method_by_id(state.db.reader(), id).await? else {
        return Err(ApiError::param_invalid("id"));
    };
    let pending = pay_repo::count_pending_for_method(state.db.reader(), m.merchant_id, &m.code).await?;
    if pending > 0 {
        return Err(ApiError::business("method_in_use"));
    }
    let n = pay_repo::delete_method(state.db.pool(), id).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("id"));
    }
    Ok(())
}

pub async fn admin_list_orders(
    state: &AppState,
    merchant_code: Option<&str>,
    method_code: Option<&str>,
    status: Option<&str>,
    offset: u64,
    limit: u64,
) -> AppResult<(Vec<OrderView>, u64)> {
    if let Some(c) = merchant_code {
        if !ident_ok(c) {
            return Err(ApiError::param_invalid("merchant_code"));
        }
    }
    if let Some(c) = method_code {
        if !ident_ok(c) {
            return Err(ApiError::param_invalid("method_code"));
        }
    }
    if let Some(s) = status {
        if !order_status_ok(s) {
            return Err(ApiError::param_invalid("status"));
        }
    }
    let total = pay_repo::count_orders(
        state.db.reader(),
        merchant_code,
        method_code,
        status,
    )
    .await?;
    let rows = pay_repo::list_orders(
        state.db.reader(),
        merchant_code,
        method_code,
        status,
        offset,
        limit,
    )
    .await?;
    Ok((rows.into_iter().map(order_view_from_list).collect(), total))
}

async fn ensure_method(
    state: &AppState,
    merchant: &PayMerchant,
    method: &str,
) -> AppResult<PayMethod> {
    if !ident_ok(method) || !pay_adapter::method_ok(method) {
        return Err(ApiError::param_invalid("method"));
    }
    if merchant.status != "active" {
        return Err(ApiError::business("merchant_paused"));
    }
    let Some(row) = pay_repo::find_method(state.db.reader(), merchant.id, method).await? else {
        return Err(ApiError::param_invalid("method"));
    };
    if row.status != "active" {
        return Err(ApiError::business("method_paused"));
    }
    if !pay_adapter::charge_ready(method) {
        return Err(ApiError::business("not_configured"));
    }
    Ok(row)
}

fn extra_json(url: &str, channel_ref: &str, expires_at: i64) -> String {
    json!({
        "pay_url": url,
        "channel_ref": channel_ref,
        "expires_at": expires_at,
    })
    .to_string()
}

pub async fn create_for_ov6(
    state: &AppState,
    user: &AuthenticatedUser,
    order_no: &str,
    method: &str,
    client_ip: &str,
) -> AppResult<GatewayPay> {
    let merchant = ov6_merchant(state).await?;
    let row = ensure_method(state, &merchant, method).await?;
    let o = vip_repo::find_any_order_by_no(state.db.reader(), order_no)
        .await?
        .ok_or_else(|| ApiError::param_invalid("order_not_found"))?;
    if o.uid != user.uid {
        return Err(ApiError::param_invalid("order_not_found"));
    }
    if o.status != 0 {
        return Err(ApiError::business("order_not_pending"));
    }
    let now = clock::now_ts();
    let existing =
        pay_repo::find_order_by_merchant_ref(state.db.reader(), merchant.id, order_no, method)
            .await?;
    let pay_no = if let Some(ex) = existing {
        if ex.status == "paid" {
            return Err(ApiError::business("order_already_processed"));
        }
        if ex.status != "pending" {
            return Err(ApiError::business("order_not_pending"));
        }
        ex.pay_no
    } else {
        let pay_no = new_pay_no(now);
        let subject = if o.package_code.trim().is_empty() {
            format!("order-{}", o.order_kind)
        } else {
            o.package_code.clone()
        };
        pay_repo::insert_order(
            state.db.pool(),
            OrderInsert {
                pay_no: &pay_no,
                merchant_id: merchant.id,
                merchant_order_no: order_no,
                method_code: method,
                amount_cents: o.amount_cents,
                currency: &resolve_currency(state, &row).await,
                subject: &subject,
            },
            now,
        )
        .await?;
        pay_no
    };
    let url =
        stripe_service::create_checkout_url(state, user, order_no, client_ip, Some(&pay_no)).await?;
    let sid = phpyun_models::stripe_order::repo::find_by_order_no(state.db.reader(), order_no)
        .await?
        .map(|r| r.stripe_session_id)
        .unwrap_or_default();
    pay_repo::update_checkout(
        state.db.pool(),
        &pay_no,
        &url,
        &sid,
        &extra_json(&url, &sid, now + 86_400),
        now,
    )
    .await?;
    Ok(GatewayPay {
        pay_no,
        pay_url: url,
        method: method.into(),
        status: "pending".into(),
    })
}

pub struct MerchantOrderIn {
    pub merchant_order_no: String,
    pub method: String,
    pub amount_cents: i32,
    pub currency: String,
    pub subject: String,
    pub customer_email: String,
    pub success_url: String,
    pub cancel_url: String,
}

pub async fn create_for_merchant(
    state: &AppState,
    merchant: &PayMerchant,
    f: MerchantOrderIn,
) -> AppResult<GatewayPay> {
    let method = f.method.trim().to_ascii_lowercase();
    let row = ensure_method(state, merchant, &method).await?;
    phpyun_core::validators::ensure_path_token(&f.merchant_order_no)?;
    if f.amount_cents < 1 {
        return Err(ApiError::param_invalid("amount_cents"));
    }
    let mut currency = f.currency.trim().to_ascii_lowercase();
    if currency.is_empty() {
        currency = resolve_currency(state, &row).await;
    }
    if !ident_ok(&currency) {
        return Err(ApiError::param_invalid("currency"));
    }
    let subject = f.subject.trim();
    if subject.is_empty() || subject.len() > 255 {
        return Err(ApiError::param_invalid("subject"));
    }
    let now = clock::now_ts();
    if let Some(ex) =
        pay_repo::find_order_by_merchant_ref(state.db.reader(), merchant.id, &f.merchant_order_no, &method)
            .await?
    {
        if ex.status == "paid" {
            return Err(ApiError::business("order_already_processed"));
        }
        if ex.status == "pending" && !ex.pay_url.is_empty() {
            return Ok(GatewayPay {
                pay_no: ex.pay_no,
                pay_url: ex.pay_url,
                method,
                status: ex.status,
            });
        }
        if ex.status != "pending" {
            return Err(ApiError::business("order_not_pending"));
        }
        let base = web_base(state).await;
        let success = if f.success_url.trim().is_empty() {
            merchant
                .return_url
                .clone()
                .if_empty(&format!("{base}/pay/stripe?pay_no={}", ex.pay_no))
        } else {
            f.success_url.trim().to_string()
        };
        let cancel = if f.cancel_url.trim().is_empty() {
            format!("{base}/pay/stripe?pay_no={}&canceled=1", ex.pay_no)
        } else {
            f.cancel_url.trim().to_string()
        };
        let out = pay_adapter::create_checkout(
            state,
            &method,
            pay_adapter::CheckoutIn {
                pay_no: &ex.pay_no,
                merchant_order_no: &f.merchant_order_no,
                amount_cents: f.amount_cents,
                currency: &currency,
                subject,
                customer_email: f.customer_email.trim(),
                success_url: &success,
                cancel_url: &cancel,
                config_json: &resolved_config(state, merchant, &row).await,
            },
        )
        .await?;
        pay_repo::update_checkout(
            state.db.pool(),
            &ex.pay_no,
            &out.pay_url,
            &out.channel_ref,
            &extra_json(&out.pay_url, &out.channel_ref, now + 86_400),
            now,
        )
        .await?;
        return Ok(GatewayPay {
            pay_no: ex.pay_no,
            pay_url: out.pay_url,
            method,
            status: "pending".into(),
        });
    }
    let pay_no = new_pay_no(now);
    pay_repo::insert_order(
        state.db.pool(),
        OrderInsert {
            pay_no: &pay_no,
            merchant_id: merchant.id,
            merchant_order_no: &f.merchant_order_no,
            method_code: &method,
            amount_cents: f.amount_cents,
            currency: &currency,
            subject,
        },
        now,
    )
    .await?;
    let base = web_base(state).await;
    let success = if f.success_url.trim().is_empty() {
        if merchant.return_url.trim().is_empty() {
            format!("{base}/pay/stripe?pay_no={pay_no}")
        } else {
            merchant.return_url.clone()
        }
    } else {
        f.success_url.trim().to_string()
    };
    let cancel = if f.cancel_url.trim().is_empty() {
        format!("{base}/pay/stripe?pay_no={pay_no}&canceled=1")
    } else {
        f.cancel_url.trim().to_string()
    };
    let out = pay_adapter::create_checkout(
        state,
        &method,
        pay_adapter::CheckoutIn {
            pay_no: &pay_no,
            merchant_order_no: &f.merchant_order_no,
            amount_cents: f.amount_cents,
            currency: &currency,
            subject,
            customer_email: f.customer_email.trim(),
            success_url: &success,
            cancel_url: &cancel,
            config_json: &resolved_config(state, merchant, &row).await,
        },
    )
    .await?;
    pay_repo::update_checkout(
        state.db.pool(),
        &pay_no,
        &out.pay_url,
        &out.channel_ref,
        &extra_json(&out.pay_url, &out.channel_ref, now + 86_400),
        now,
    )
    .await?;
    Ok(GatewayPay {
        pay_no,
        pay_url: out.pay_url,
        method,
        status: "pending".into(),
    })
}

trait IfEmpty {
    fn if_empty(self, fallback: &str) -> String;
}

impl IfEmpty for String {
    fn if_empty(self, fallback: &str) -> String {
        if self.trim().is_empty() {
            fallback.to_string()
        } else {
            self
        }
    }
}

async fn resolved_config(state: &AppState, merchant: &PayMerchant, method: &PayMethod) -> String {
    let cfg = method.config_json.clone();
    if merchant.code != OV6 {
        return cfg;
    }
    let sk = pay_config::config_str(&cfg, "secret_key");
    if !sk.is_empty() {
        return cfg;
    }
    let site_sk = site_setting_service::get(state, "sy_stripe_sk")
        .await
        .ok()
        .flatten()
        .map(|r| r.value)
        .unwrap_or_default();
    let site_sk = site_sk.trim();
    if site_sk.is_empty() {
        return cfg;
    }
    merge_config(&cfg, Some(site_sk), None, None)
}

pub async fn merchant_methods(state: &AppState, merchant: &PayMerchant) -> AppResult<Vec<MethodView>> {
    let rows = pay_repo::list_active_methods(state.db.reader(), merchant.id).await?;
    Ok(rows.iter().map(method_view).collect())
}

pub async fn merchant_order(
    state: &AppState,
    merchant: &PayMerchant,
    pay_no: &str,
) -> AppResult<OrderView> {
    if !ident_ok(pay_no) {
        return Err(ApiError::param_invalid("pay_no"));
    }
    let Some(o) = pay_repo::find_order_by_pay_no(state.db.reader(), pay_no).await? else {
        return Err(ApiError::param_invalid("order_not_found"));
    };
    if o.merchant_id != merchant.id {
        return Err(ApiError::param_invalid("order_not_found"));
    }
    Ok(OrderView {
        id: o.id,
        pay_no: o.pay_no,
        merchant_code: merchant.code.clone(),
        merchant_name: merchant.name.clone(),
        merchant_order_no: o.merchant_order_no,
        method_code: o.method_code,
        amount_yuan: f64::from(o.amount_cents.max(0)) / 100.0,
        amount_cents: o.amount_cents,
        currency: o.currency,
        status: o.status,
        channel_ref: o.channel_ref,
        subject: o.subject,
        paid_at: o.paid_at,
        ctime: o.ctime,
        ctime_n: fmt_dt(o.ctime),
    })
}

pub async fn stripe_config_for_ov6(state: &AppState) -> String {
    pay_repo::method_config_by_merchant_code(state.db.reader(), OV6, "stripe")
        .await
        .ok()
        .flatten()
        .unwrap_or_default()
}
