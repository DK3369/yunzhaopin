//! VIP packages / orders / status.

use axum::{extract::State, routing::post, Router};
use phpyun_core::json;
use phpyun_core::utils::{fmt_dt, pay_order_status_name as order_status_name};
use phpyun_core::ApiError;
use phpyun_core::{
    ApiResponse, AppResult, AppState, AuthenticatedUser, ClientIp, Paged, Pagination, ValidatedJson,
};
use phpyun_models::sql::ident_ok;
use phpyun_services::{payment_notify_service, vip_service};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    let r = Router::new()
        .route("/vip/packages", post(list_packages))
        .route("/vip/current", post(get_current))
        .route("/vip/orders", post(create_order))
        .route("/vip/orders/list", post(list_orders))
        .route("/vip/orders/cancel", post(cancel_order))
        .route("/vip/orders/paybank", post(paybank))
        .route("/vip/bank-accounts", post(list_bank_accounts))
        .route("/vip/quote", post(quote_price))
        .route("/vip/orders/integral", post(buy_integral))
        .route("/vip/integral-classes", post(list_integral_classes))
        .route("/vip/recharge", post(recharge))
        .route("/vip/card", post(redeem_card));

    // mock-paid is only mounted in debug builds; the release binary does not include this route.
    #[cfg(debug_assertions)]
    let r = r.route("/vip/orders/mock-paid", post(mock_paid));

    r
}

/// VIP package item — all 10 columns of phpyun_vip_package + derived price_yuan (yuan unit).
#[derive(Debug, Serialize, ToSchema)]
pub struct PackageItem {
    pub id: u32,
    pub code: String,
    pub name: String,
    pub target_usertype: i32,
    pub duration_days: i32,
    pub price_cents: i32,
    /// price_cents / 100 (numeric yuan; front-end does not prefix a currency symbol)
    pub price_yuan: f64,
    pub desc: Option<json::Value>,
    pub is_active: i32,
    pub sort_order: i32,
    pub created_at: i64,
    pub created_at_n: String,
    /// `seeker` | `employer`
    pub role: String,
}

impl PackageItem {
    fn from_pkg(p: phpyun_models::vip::entity::VipPackage, role: &str) -> Self {
        Self {
            id: p.id,
            code: p.code,
            name: p.name,
            target_usertype: p.target_usertype,
            duration_days: p.duration_days,
            price_yuan: f64::from(p.price_cents) / 100.0,
            price_cents: p.price_cents,
            desc: p.desc_json,
            is_active: p.is_active,
            sort_order: p.sort_order,
            created_at_n: fmt_dt(p.created_at),
            created_at: p.created_at,
            role: role.to_string(),
        }
    }
}

#[derive(Debug, Default, Deserialize, Validate, ToSchema)]
pub struct ListPackagesForm {
    /// Parsed then discarded. Catalog follows JWT `usertype` (seeker packs vs employer VIP 1–6).
    #[serde(default)]
    #[validate(length(max = 16))]
    pub kind: Option<String>,
}

/// List of purchasable packages (filtered by current user's usertype)
#[utoipa::path(
    post,
    path = "/v1/mcenter/vip/packages",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = ListPackagesForm,
    responses((status = 200, description = "ok"))
)]
pub async fn list_packages(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(f): ValidatedJson<ListPackagesForm>,
) -> AppResult<ApiResponse<Vec<PackageItem>>> {
    let kind = match f.kind.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
        None => None,
        Some(raw) => {
            if !ident_ok(raw) {
                return Err(ApiError::param_invalid("kind"));
            }
            match raw {
                "package" => Some("package"),
                "time" => Some("time"),
                _ => return Err(ApiError::param_invalid("kind")),
            }
        }
    };
    let list = vip_service::list_packages(&state, &user, kind).await?;
    let role = if user.usertype == 1 { "seeker" } else { "employer" };
    Ok(ApiResponse::data(
        list.into_iter()
            .map(|p| PackageItem::from_pkg(p, role))
            .collect(),
    ))
}

#[derive(Debug, Clone, Default, Serialize, ToSchema)]
pub struct VipCaps {
    pub job_num: i32,
    pub resume: i32,
    pub interview: i32,
    pub breakjob_num: i32,
    pub top_num: i32,
    pub urgent_num: i32,
    pub rec_num: i32,
    pub zph_num: i32,
    pub sons_num: i32,
}

#[derive(Debug, Clone, Default, Serialize, ToSchema)]
pub struct SeekerCapsView {
    pub chat: bool,
    pub resume_top: bool,
    pub tpl_all: bool,
    pub refresh_free: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CurrentVip {
    pub active: bool,
    pub package_code: Option<String>,
    pub started_at: Option<i64>,
    pub expires_at: Option<i64>,
    pub rating: i32,
    pub rating_name: String,
    pub rating_type: i32,
    pub job_num: i32,
    pub breakjob_num: i32,
    pub down_resume: i32,
    pub invite_resume: i32,
    pub zph_num: i32,
    pub top_num: i32,
    pub urgent_num: i32,
    pub rec_num: i32,
    pub integral: i64,
    pub sons_num: i32,
    pub caps: VipCaps,
    /// 站点 `com_vip_type`：0 套餐+时间 / 1 仅时间 / 2 仅套餐。
    pub com_vip_type: i32,
    pub can_chat: bool,
    /// `seeker` | `employer`
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seeker_caps: Option<SeekerCapsView>,
}

fn seeker_caps_view(c: phpyun_services::seeker_vip_service::SeekerCaps) -> SeekerCapsView {
    SeekerCapsView {
        chat: c.chat,
        resume_top: c.resume_top,
        tpl_all: c.tpl_all,
        refresh_free: c.refresh_free,
    }
}

/// My current VIP status
#[utoipa::path(
    post,
    path = "/v1/mcenter/vip/current",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = CurrentVip))
)]
pub async fn get_current(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<CurrentVip>> {
    let v = vip_service::get_current_vip(&state, &user).await?;
    let now = phpyun_core::clock::now_ts();
    let can_chat = phpyun_services::seeker_vip_service::can_initiate_chat(&state, &user).await?;
    let com_vip_type = phpyun_models::site_setting::repo::find_many(
        state.db.reader(),
        &["com_vip_type"],
    )
    .await
    .ok()
    .and_then(|m| m.get("com_vip_type").and_then(|s| s.trim().parse::<i32>().ok()))
    .unwrap_or(0);
    if user.usertype == 1 {
        let caps = phpyun_services::seeker_vip_service::seeker_caps(&state, user.uid).await?;
        let integral = phpyun_models::member_statis::repo::get_balance(state.db.reader(), user.uid)
            .await
            .map(|b| b.balance)
            .unwrap_or(0);
        let rating_name = if let Some(ref vv) = v {
            phpyun_models::seeker_vip::repo::find_by_code(state.db.reader(), &vv.package_code)
                .await
                .ok()
                .flatten()
                .map(|p| p.name)
                .unwrap_or_else(|| vv.package_code.clone())
        } else {
            String::new()
        };
        return Ok(ApiResponse::data(CurrentVip {
            active: v.as_ref().map(|x| x.expires_at > now).unwrap_or(false),
            package_code: v.as_ref().map(|x| x.package_code.clone()),
            started_at: v.as_ref().map(|x| x.started_at),
            expires_at: v.as_ref().map(|x| x.expires_at),
            rating: 0,
            rating_name,
            rating_type: 2,
            job_num: 0,
            breakjob_num: 0,
            down_resume: 0,
            invite_resume: 0,
            zph_num: 0,
            top_num: 0,
            urgent_num: 0,
            rec_num: 0,
            integral,
            sons_num: 0,
            caps: VipCaps::default(),
            com_vip_type,
            can_chat,
            role: "seeker".into(),
            seeker_caps: Some(seeker_caps_view(caps)),
        }));
    }
    let st = phpyun_models::company_statis::repo::find_admin(state.db.reader(), user.uid)
        .await
        .ok()
        .flatten();
    let caps = if let Some(s) = &st {
        phpyun_models::com_stats::repo::find_rating_caps(state.db.reader(), s.rating)
            .await
            .ok()
            .flatten()
            .map(|c| VipCaps {
                job_num: c.job_num,
                resume: c.resume,
                interview: c.interview,
                breakjob_num: c.breakjob_num,
                top_num: c.top_num,
                urgent_num: c.urgent_num,
                rec_num: c.rec_num,
                zph_num: c.zph_num,
                sons_num: c.sons_num,
            })
            .unwrap_or_default()
    } else {
        VipCaps::default()
    };
    let empty = || CurrentVip {
        active: false,
        package_code: None,
        started_at: None,
        expires_at: None,
        rating: 0,
        rating_name: String::new(),
        rating_type: 0,
        job_num: 0,
        breakjob_num: 0,
        down_resume: 0,
        invite_resume: 0,
        zph_num: 0,
        top_num: 0,
        urgent_num: 0,
        rec_num: 0,
        integral: 0,
        sons_num: 0,
        caps: VipCaps::default(),
        com_vip_type,
        can_chat,
        role: "employer".into(),
        seeker_caps: None,
    };
    Ok(ApiResponse::data(match (v, st) {
        (Some(v), Some(s)) => CurrentVip {
            active: v.expires_at == 0 || v.expires_at >= now,
            package_code: Some(v.package_code),
            started_at: Some(v.started_at),
            expires_at: Some(v.expires_at),
            rating: s.rating,
            rating_name: s.rating_name,
            rating_type: s.rating_type,
            job_num: s.job_num,
            breakjob_num: s.breakjob_num,
            down_resume: s.down_resume,
            invite_resume: s.invite_resume,
            zph_num: s.zph_num,
            top_num: s.top_num,
            urgent_num: s.urgent_num,
            rec_num: s.rec_num,
            integral: s.integral.parse().unwrap_or(0),
            sons_num: s.sons_num,
            caps,
            com_vip_type,
            can_chat,
            role: "employer".into(),
            seeker_caps: None,
        },
        (Some(v), None) => CurrentVip {
            active: v.expires_at == 0 || v.expires_at >= now,
            package_code: Some(v.package_code),
            started_at: Some(v.started_at),
            expires_at: Some(v.expires_at),
            ..empty()
        },
        _ => empty(),
    }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateOrderForm {
    #[validate(length(min = 1, max = 32))]
    pub package_code: String,
    /// alipay / wechat / stripe / stub
    #[validate(length(min = 1, max = 16))]
    pub channel: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OrderCreated {
    pub order_no: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_url: Option<String>,
    pub channel: String,
}

/// Create an order (returns order_no, hand it to the frontend to call the payment gateway)
#[utoipa::path(
    post,
    path = "/v1/mcenter/vip/orders",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = CreateOrderForm,
    responses((status = 200, description = "ok", body = OrderCreated))
)]
pub async fn create_order(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<CreateOrderForm>,
) -> AppResult<ApiResponse<OrderCreated>> {
    if f.channel != "alipay" && f.channel != "wxpay" && f.channel != "wxh5" && f.channel != "bank" {
        return Err(ApiError::param_invalid("channel"));
    }
    let created = vip_service::create_order_ex(&state, &user, &f.package_code, &f.channel, &ip).await?;
    // Order is already inserted. Missing Alipay keys must not 400 the whole create —
    // the client still needs `order_no` to open cashier.
    let pay_url = if f.channel == "alipay" {
        match payment_notify_service::ensure_alipay_page(&state).await {
            Ok(()) => payment_notify_service::build_alipay_page_url(
                &state,
                &created.order_no,
                &created.subject,
                created.amount_cents,
                None,
            )
            .await
            .ok(),
            Err(_) => None,
        }
    } else {
        None
    };
    Ok(ApiResponse::data(OrderCreated {
        order_no: created.order_no,
        pay_url,
        channel: f.channel,
    }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct IntegralBuyForm {
    #[validate(length(min = 1, max = 32))]
    pub package_code: String,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/vip/orders/integral",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = IntegralBuyForm,
    responses((status = 200, description = "ok"))
)]
pub async fn buy_integral(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<IntegralBuyForm>,
) -> AppResult<ApiResponse<json::Value>> {
    let order_no = vip_service::buy_with_integral(&state, &user, &f.package_code, &ip).await?;
    Ok(ApiResponse::data(json::json!({ "ok": true, "order_no": order_no })))
}

/// Pay order item — all 10 columns of phpyun_pay_order + yuan-unit amount + time formatting.
#[derive(Debug, Serialize, ToSchema)]
pub struct OrderItem {
    pub id: u64,
    pub order_no: String,
    pub uid: u64,
    pub package_code: String,
    pub amount_cents: i32,
    pub amount_yuan: f64,
    pub channel: String,
    /// 0 pending / 1 paid / 2 refunded / 3 cancelled
    pub status: i32,
    pub status_n: String,
    pub pay_tx_id: Option<String>,
    pub created_at: i64,
    pub created_at_n: String,
    pub paid_at: i64,
    pub paid_at_n: String,
    pub order_kind: i32,
    pub integral: i32,
}

impl From<phpyun_models::vip::entity::PayOrder> for OrderItem {
    fn from(o: phpyun_models::vip::entity::PayOrder) -> Self {
        let status_n = if o.channel == "bank" && o.status == 3 {
            "awaiting_confirm".to_string()
        } else {
            order_status_name(o.status).to_string()
        };
        Self {
            id: o.id,
            order_no: o.order_no,
            uid: o.uid,
            package_code: o.package_code,
            amount_yuan: f64::from(o.amount_cents) / 100.0,
            amount_cents: o.amount_cents,
            channel: o.channel,
            status_n,
            status: o.status,
            pay_tx_id: o.pay_tx_id,
            created_at_n: fmt_dt(o.created_at),
            created_at: o.created_at,
            paid_at_n: fmt_dt(o.paid_at),
            paid_at: o.paid_at,
            order_kind: o.order_kind,
            integral: o.integral,
        }
    }
}

/// My orders list
#[utoipa::path(
    post,
    path = "/v1/mcenter/vip/orders/list",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_orders(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    page: Pagination,
) -> AppResult<ApiResponse<Paged<OrderItem>>> {
    let r = vip_service::list_orders(&state, &user, page).await?;
    Ok(ApiResponse::data(Paged::from_listing(
        r.list, r.total, page,
    )))
}

/// Cancel an unpaid order (orders with status=0). Cannot cancel paid / cancelled orders.
#[utoipa::path(post,
    path = "/v1/mcenter/vip/orders/cancel",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = CancelOrderBody,
    responses(
        (status = 200, description = "ok"),
        (status = 400, description = "Order not found / does not belong to you / already paid / cancelled"),
    )
)]
pub async fn cancel_order(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<CancelOrderBody>,
) -> AppResult<ApiResponse<json::Value>> {
    let order_no = b.order_no;
    phpyun_core::validators::ensure_path_token(&order_no)?;
    vip_service::cancel_order(&state, &user, &order_no).await?;
    Ok(ApiResponse::data(json::json!({ "ok": true })))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct BankAccountView {
    pub id: u64,
    pub name: String,
    pub bank_name: String,
    pub bank_number: String,
    pub bank_address: String,
}

/// Site bank-transfer accounts (PHP `getBankList`).
#[utoipa::path(
    post,
    path = "/v1/mcenter/vip/bank-accounts",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok"))
)]
pub async fn list_bank_accounts(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<Vec<BankAccountView>>> {
    let list = vip_service::list_bank_accounts(&state, &user).await?;
    Ok(ApiResponse::data(
        list.into_iter()
            .map(|a| BankAccountView {
                id: a.id,
                name: a.name,
                bank_name: a.bank_name,
                bank_number: a.bank_number,
                bank_address: a.bank_address,
            })
            .collect(),
    ))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PayBankForm {
    #[validate(
        length(min = 1, max = 64),
        custom(function = "phpyun_core::validators::path_token")
    )]
    pub order_no: String,
    #[validate(length(min = 1, max = 80))]
    pub bank_name: String,
    #[validate(length(min = 1, max = 80))]
    pub bank_number: String,
    #[validate(length(min = 1, max = 32))]
    pub bank_price: String,
    #[serde(default, deserialize_with = "phpyun_core::date_parse::de_loose_ts")]
    #[validate(range(min = 1i64, max = 4_102_444_800i64))]
    pub bank_time: i64,
    #[serde(default)]
    #[validate(length(max = 500))]
    pub order_remark: String,
    #[serde(default)]
    #[validate(length(max = 255))]
    pub order_pic: Option<String>,
}

/// PHP `payment::paybank` — submit bank transfer voucher.
#[utoipa::path(
    post,
    path = "/v1/mcenter/vip/orders/paybank",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = PayBankForm,
    responses((status = 200, description = "ok"))
)]
pub async fn paybank(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<PayBankForm>,
) -> AppResult<ApiResponse<json::Value>> {
    vip_service::submit_bank_pay(
        &state,
        &user,
        vip_service::BankPayInput {
            order_no: &f.order_no,
            bank_name: &f.bank_name,
            bank_number: &f.bank_number,
            bank_price: &f.bank_price,
            bank_time: f.bank_time,
            order_remark: &f.order_remark,
            order_pic: f.order_pic.as_deref(),
        },
        &ip,
    )
    .await?;
    Ok(ApiResponse::data(json::json!({ "ok": true })))
}

/// **Dev only**: simulates a payment callback (in production, signature verification of the third-party payment gateway is used).
/// Only compiled in debug builds — this function does not exist in the release binary.
#[cfg(debug_assertions)]
#[utoipa::path(post,
    path = "/v1/mcenter/vip/orders/mock-paid",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = MockPaidBody,
    responses((status = 200, description = "ok"))
)]
pub async fn mock_paid(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<MockPaidBody>,
) -> axum::response::Response {
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    if !(state.config.dev_tokens && state.config.env.is_dev_or_test()) {
        return StatusCode::NOT_FOUND.into_response();
    }
    match mock_paid_inner(state, user, b).await {
        Ok(body) => body.into_response(),
        Err(e) => e.into_response(),
    }
}

#[cfg(debug_assertions)]
async fn mock_paid_inner(
    state: AppState,
    user: AuthenticatedUser,
    b: MockPaidBody,
) -> AppResult<ApiResponse<json::Value>> {
    let order_no = b.order_no;
    phpyun_core::validators::ensure_path_token(&order_no)?;
    // Defensive check: the order must belong to the currently logged-in user, to avoid marking someone else's order as paid.
    let fake_tx = format!("MOCK-{}", uuid::Uuid::now_v7().simple());
    if let Some(order) =
        phpyun_models::vip::repo::find_order_by_no_and_type(state.db.reader(), &order_no, 2).await?
    {
        if order.uid != user.uid {
            return Err(ApiError::param_invalid("order_not_owned"));
        }
        vip_service::mark_recharge_paid(&state, &order_no, &fake_tx).await?;
        return Ok(ApiResponse::data(
            json::json!({ "ok": true, "pay_tx_id": fake_tx }),
        ));
    }
    if let Some(any) = phpyun_models::vip::repo::find_any_order_by_no(state.db.reader(), &order_no).await?
    {
        if any.uid != user.uid {
            return Err(ApiError::param_invalid("order_not_owned"));
        }
        if any.order_kind == 28 || any.order_kind == 19 || any.order_kind == 23 || any.order_kind == 31 {
            phpyun_services::payment_notify_service::settle_paid(&state, &order_no, &fake_tx)
                .await?;
            return Ok(ApiResponse::data(
                json::json!({ "ok": true, "pay_tx_id": fake_tx }),
            ));
        }
    }
    let order = phpyun_models::vip::repo::find_order_by_no(state.db.reader(), &order_no)
        .await?
        .ok_or_else(|| -> phpyun_core::ApiError { ApiError::param_invalid("order_not_found") })?;
    if order.uid != user.uid {
        return Err(ApiError::param_invalid("order_not_owned"));
    }
    vip_service::mark_paid(&state, &order_no, &fake_tx).await?;
    Ok(ApiResponse::data(
        json::json!({ "ok": true, "pay_tx_id": fake_tx }),
    ))
}

// ==================== Price quote ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct PriceQuoteView {
    pub id: u64,
    pub name: String,
    pub service_price: f64,
    pub yh_price: f64,
    pub price: f64,
    /// 1=cash, 2=integral-pay, 3=insufficient-integral fallback to cash.
    pub style: i32,
    pub promo_active: bool,
    pub user_integral: i64,
}

impl From<phpyun_services::vip_service::PriceQuote> for PriceQuoteView {
    fn from(p: phpyun_services::vip_service::PriceQuote) -> Self {
        Self {
            id: p.id,
            name: p.name,
            service_price: p.service_price,
            yh_price: p.yh_price,
            price: p.price,
            style: p.style,
            promo_active: p.promo_active,
            user_integral: p.user_integral,
        }
    }
}

/// Compute the effective price the current user pays for a package.
///
/// Counterpart of PHP `ajax::getPackPrice_action` (`kind=pack`) and
/// `ajax::getVipPrice_action` (`kind=vip`). Combines:
///   - rating-tier discount (`service_discount`)
///   - active promo window (`time_start < now < time_end`)
///   - integral-payment availability (`com_integral_online == 3` and `kind`
///     not in `sy_only_price`)
///   - the user's integral balance
///
/// Returns 403 for non-employers; 400 when the package id is unknown or
/// `kind` is not `pack` / `vip`.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct QuotePriceBody {
    /// `pack` or `vip` — selects which price table to read.
    #[validate(
        length(min = 1, max = 32),
        custom(function = "phpyun_core::validators::path_token")
    )]
    pub kind: String,
    /// VIP package id (`phpyun_member_pricing.id`).
    #[validate(range(min = 1, max = 999_999_999))]
    pub id: u64,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/vip/quote",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = QuotePriceBody,
    responses(
        (status = 200, description = "ok", body = PriceQuoteView),
        (status = 400, description = "Invalid kind / package not found"),
        (status = 403, description = "Not an employer"),
    )
)]
pub async fn quote_price(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ValidatedJson(b): ValidatedJson<QuotePriceBody>,
) -> AppResult<ApiResponse<PriceQuoteView>> {
    let q = vip_service::quote_package_price(&state, &user, b.id, &b.kind).await?;
    Ok(ApiResponse::data(PriceQuoteView::from(q)))
}

#[derive(Debug, serde::Deserialize, validator::Validate, utoipa::ToSchema)]
pub struct CancelOrderBody {
    #[validate(
        length(min = 1, max = 64),
        custom(function = "phpyun_core::validators::path_token")
    )]
    pub order_no: String,
}

#[derive(Debug, serde::Deserialize, validator::Validate, utoipa::ToSchema)]
pub struct MockPaidBody {
    #[validate(
        length(min = 1, max = 64),
        custom(function = "phpyun_core::validators::path_token")
    )]
    pub order_no: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct IntegralClassItem {
    pub id: u64,
    pub integral: i32,
    pub discount: i32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct IntegralClassPack {
    pub list: Vec<IntegralClassItem>,
    pub min_recharge: i64,
    pub proportion: i64,
    pub pricename: String,
    pub priceunit: String,
    pub balance: i64,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/vip/integral-classes",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = IntegralClassPack))
)]
pub async fn list_integral_classes(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<IntegralClassPack>> {
    let p = vip_service::list_integral_classes(&state, &user).await?;
    Ok(ApiResponse::data(IntegralClassPack {
        list: p
            .list
            .into_iter()
            .map(|c| IntegralClassItem {
                id: c.id,
                integral: c.integral,
                discount: c.discount,
            })
            .collect(),
        min_recharge: p.min_recharge,
        proportion: p.proportion,
        pricename: p.pricename,
        priceunit: p.priceunit,
        balance: p.balance,
    }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RechargeForm {
    #[validate(range(min = 1, max = 10_000_000))]
    pub price_int: i64,
    #[serde(default)]
    pub integralid: u64,
    #[validate(length(min = 1, max = 16))]
    pub channel: String,
    #[serde(default)]
    #[validate(length(max = 500))]
    pub remark: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RechargeCreated {
    pub order_no: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_url: Option<String>,
    pub channel: String,
    pub amount_cents: i32,
    pub amount_yuan: f64,
    pub integral: i64,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/vip/recharge",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = RechargeForm,
    responses((status = 200, description = "ok", body = RechargeCreated))
)]
pub async fn recharge(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<RechargeForm>,
) -> AppResult<ApiResponse<RechargeCreated>> {
    if f.channel != "alipay" && f.channel != "wxpay" && f.channel != "wxh5" && f.channel != "bank" {
        return Err(ApiError::param_invalid("channel"));
    }
    if f.channel == "alipay" {
        payment_notify_service::ensure_alipay_page(&state).await?;
    }
    let created = vip_service::create_recharge(
        &state,
        &user,
        f.price_int,
        f.integralid,
        &f.channel,
        &f.remark,
        &ip,
    )
    .await?;
    let pay_url = if f.channel == "alipay" {
        Some(
            payment_notify_service::build_alipay_page_url(
                &state,
                &created.order_no,
                &created.subject,
                created.amount_cents,
                Some("/com/pay"),
            )
            .await?,
        )
    } else {
        None
    };
    Ok(ApiResponse::data(RechargeCreated {
        order_no: created.order_no,
        pay_url,
        channel: f.channel,
        amount_cents: created.amount_cents,
        amount_yuan: f64::from(created.amount_cents) / 100.0,
        integral: created.integral,
    }))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CardForm {
    #[validate(length(min = 1, max = 20))]
    pub card: String,
    #[validate(length(min = 1, max = 20))]
    pub password: String,
}

#[utoipa::path(
    post,
    path = "/v1/mcenter/vip/card",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = CardForm,
    responses((status = 200, description = "ok"))
)]
pub async fn redeem_card(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<CardForm>,
) -> AppResult<ApiResponse<json::Value>> {
    let quota = vip_service::redeem_card(&state, &user, &f.card, &f.password, &ip).await?;
    Ok(ApiResponse::data(json::json!({ "ok": true, "quota": quota })))
}
