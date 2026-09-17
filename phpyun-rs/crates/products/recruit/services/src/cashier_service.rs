//! Cashier: look up any `company_order` by `order_id` and continue payment.

use phpyun_core::utils::pay_order_status_name;
use phpyun_core::utils::fmt_dt;
use phpyun_core::{ApiError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::bank::entity::BankAccount;
use phpyun_models::bank::repo as bank_repo;
use phpyun_models::vip::repo::{self as vip_repo, AnyOrder};
use crate::payment_notify_service;
use crate::site_setting_service;

#[derive(Debug, Clone)]
pub struct CashierDetail {
    pub order_no: String,
    pub r#type: i32,
    pub subject: String,
    pub amount_yuan: f64,
    pub status: i32,
    pub status_n: String,
    pub channel: String,
    pub created_at_n: String,
    pub usertype: i32,
    pub payable: bool,
    pub channels: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CashierPay {
    pub pay_url: Option<String>,
    pub channel: String,
    pub bank_accounts: Vec<BankAccount>,
}

async fn setting_nonempty(state: &AppState, key: &str) -> bool {
    match site_setting_service::get(state, key).await {
        Ok(Some(row)) => !row.value.trim().is_empty(),
        _ => false,
    }
}

async fn available_channels(state: &AppState) -> AppResult<Vec<String>> {
    let mut out = vec!["alipay".to_string()];
    if !bank_repo::list_all(state.db.reader()).await?.is_empty() {
        out.push("bank".into());
    }
    if setting_nonempty(state, "sy_wxpayid").await {
        out.push("wxpay".into());
    }
    Ok(out)
}

fn map_detail(o: AnyOrder, channels: Vec<String>) -> CashierDetail {
    let status_n = if o.channel == "bank" && o.status == 3 {
        "awaiting_confirm".into()
    } else {
        pay_order_status_name(o.status).to_string()
    };
    let subject = if o.package_code.trim().is_empty() {
        format!("order-{}", o.order_kind)
    } else {
        o.package_code.clone()
    };
    CashierDetail {
        order_no: o.order_no,
        r#type: o.order_kind,
        subject,
        amount_yuan: f64::from(o.amount_cents.max(0)) / 100.0,
        status: o.status,
        status_n,
        channel: o.channel,
        created_at_n: fmt_dt(o.created_at),
        usertype: o.usertype,
        payable: o.status == 0,
        channels,
    }
}

pub async fn detail(
    state: &AppState,
    user: &AuthenticatedUser,
    order_no: &str,
) -> AppResult<CashierDetail> {
    phpyun_core::validators::ensure_path_token(order_no)?;
    let o = vip_repo::find_any_order_by_no(state.db.reader(), order_no)
        .await?
        .ok_or_else(|| ApiError::param_invalid("order_not_found"))?;
    if o.uid != user.uid {
        return Err(ApiError::param_invalid("order_not_found"));
    }
    let channels = available_channels(state).await?;
    Ok(map_detail(o, channels))
}

pub async fn pay(
    state: &AppState,
    user: &AuthenticatedUser,
    order_no: &str,
    channel: &str,
) -> AppResult<CashierPay> {
    phpyun_core::validators::ensure_path_token(order_no)?;
    let ch = match channel {
        "wxh5" => "wxpay",
        other => other,
    };
    let channels = available_channels(state).await?;
    if ch != "alipay" && ch != "bank" && ch != "wxpay" {
        return Err(ApiError::param_invalid("channel"));
    }
    if ch == "wxpay" && !channels.iter().any(|c| c == "wxpay") {
        return Err(ApiError::param_invalid("channel"));
    }
    let o = vip_repo::find_any_order_by_no(state.db.reader(), order_no)
        .await?
        .ok_or_else(|| ApiError::param_invalid("order_not_found"))?;
    if o.uid != user.uid {
        return Err(ApiError::param_invalid("order_not_found"));
    }
    if o.status != 0 {
        return Err(ApiError::business("order_not_pending"));
    }
    let n = vip_repo::set_order_channel(state.db.pool(), order_no, user.uid, ch).await?;
    if n == 0 {
        return Err(ApiError::business("order_not_pending"));
    }
    if ch == "alipay" {
        payment_notify_service::ensure_alipay_page(state).await?;
        let subject = if o.package_code.trim().is_empty() {
            format!("order-{}", o.order_kind)
        } else {
            o.package_code.clone()
        };
        let pay_url = payment_notify_service::build_alipay_page_url(
            state,
            order_no,
            &subject,
            o.amount_cents,
            None,
        )
        .await?;
        return Ok(CashierPay {
            pay_url: Some(pay_url),
            channel: ch.into(),
            bank_accounts: Vec::new(),
        });
    }
    if ch == "bank" {
        let bank_accounts = bank_repo::list_all(state.db.reader()).await?;
        return Ok(CashierPay {
            pay_url: None,
            channel: ch.into(),
            bank_accounts,
        });
    }
    Ok(CashierPay {
        pay_url: None,
        channel: ch.into(),
        bank_accounts: Vec::new(),
    })
}
