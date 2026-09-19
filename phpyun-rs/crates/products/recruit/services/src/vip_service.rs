//! VIP membership service: list packages / create order / mock payment success / query current VIP status.
//!
//! Payment currently uses `channel=stub` which goes through a "fake payment" flow for dev.
//! In production this needs to integrate with alipay / wechat / stripe.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::vip::{
    entity::{PayOrder, UserVip, VipPackage},
    repo as vip_repo,
};

pub async fn list_packages(
    state: &AppState,
    user: &AuthenticatedUser,
    kind: Option<&str>,
) -> AppResult<Vec<VipPackage>> {
    if user.usertype == 1 {
        let _ = kind;
        return crate::seeker_vip_service::list_buyable(state).await;
    }
    user.require_employer()?;
    let _ = kind;
    Ok(phpyun_models::vip::repo::list_time_packages(state.db.reader()).await?)
}

/// Create an order -- returns order_no, the client uses it to call the payment gateway.
pub async fn create_order(
    state: &AppState,
    user: &AuthenticatedUser,
    package_code: &str,
    channel: &str,
    client_ip: &str,
) -> AppResult<String> {
    user.require_employer()?;
    let pkg = vip_repo::find_package_by_code(state.db.reader(), package_code)
        .await?
        .ok_or_else(|| -> ApiError {
            ApiError::param_invalid(format!("unknown package: {package_code}"))
        })?;
    if pkg.is_active != 1 {
        return Err(ApiError::param_invalid("package_inactive"));
    }
    if pkg.price_cents <= 0 {
        return Err(ApiError::param_invalid("package_inactive"));
    }
    // `target_usertype` is company_rating.type: 1=套餐 / 2=时间会员, not member.usertype.
    if pkg.target_usertype != 1 && pkg.target_usertype != 2 {
        return Err(ApiError::param_invalid("kind"));
    }

    let now = clock::now_ts();
    let order_no = vip_repo::dingdan_id(now);
    vip_repo::create_order(
        state.db.pool(),
        &order_no,
        user.uid,
        &pkg.code,
        pkg.price_cents,
        channel,
        now,
        2,
    )
    .await?;

    let _ = audit::emit(
        state,
        AuditEvent::new("vip.order_create", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("order:{order_no}"))
            .meta(&serde_json::json!({
                "package": pkg.code,
                "amount": pkg.price_cents,
                "channel": channel,
            })),
    )
    .await;

    Ok(order_no)
}

pub struct CreatedVipOrder {
    pub order_no: String,
    pub amount_cents: i32,
    pub subject: String,
}

/// Create an order and return fields needed to jump to a payment gateway.
pub async fn create_order_ex(
    state: &AppState,
    user: &AuthenticatedUser,
    package_code: &str,
    channel: &str,
    client_ip: &str,
) -> AppResult<CreatedVipOrder> {
    if user.usertype == 1 {
        return crate::seeker_vip_service::create_order(state, user, package_code, channel, client_ip)
            .await;
    }
    user.require_employer()?;
    let pkg = vip_repo::find_package_by_code(state.db.reader(), package_code)
        .await?
        .ok_or_else(|| -> ApiError {
            ApiError::param_invalid(format!("unknown package: {package_code}"))
        })?;
    let order_no = create_order(state, user, package_code, channel, client_ip).await?;
    Ok(CreatedVipOrder {
        order_no,
        amount_cents: pkg.price_cents,
        subject: pkg.name,
    })
}

/// Mark order as paid + activate VIP.
///
/// **Security contract**: this function does **not** verify caller identity
/// (it only checks order-status idempotency). The caller MUST have completed authorization:
///   - production: the payment gateway callback handler **must** verify the signature first
///     (see `pay_callback.rs`)
///   - dev: the `mock_paid` handler **must** first check `order.uid == authenticated_user.uid`
///
/// Any new entry point that bypasses the above and calls this function directly is a security hole.
pub async fn mark_paid(state: &AppState, order_no: &str, pay_tx_id: &str) -> AppResult<()> {
    let order = vip_repo::find_order_by_no(state.db.reader(), order_no)
        .await?
        .ok_or_else(|| -> ApiError { ApiError::param_invalid("order_not_found") })?;
    if order.status != 0 {
        return Err(ApiError::param_invalid("order_not_pending"));
    }

    let pkg = vip_repo::find_package_by_code(state.db.reader(), &order.package_code)
        .await?
        .ok_or_else(|| -> ApiError { ApiError::internal(std::io::Error::other("package gone")) })?;

    let now = clock::now_ts();
    // 1. Update order status
    let affected = vip_repo::mark_order_paid(state.db.pool(), order_no, pay_tx_id, now).await?;
    if affected == 0 {
        return Err(ApiError::param_invalid("order_already_processed"));
    }
    // 2. PHP ratingInfo → company_statis + company + company_job.rating
    let rating_id = i32::try_from(pkg.id).unwrap_or(0);
    crate::rating_info_service::apply_rating(state, order.uid, rating_id, None).await?;

    // 3. Audit + event bus
    let _ = audit::emit(
        state,
        AuditEvent::new("vip.order_paid", Actor::uid(order.uid))
            .target(format!("order:{order_no}"))
            .meta(&serde_json::json!({
                "package": pkg.code,
                "amount": pkg.price_cents,
                "pay_tx_id": pay_tx_id,
            })),
    )
    .await;

    let _ = state
        .events
        .publish_json(
            "vip.activated",
            &serde_json::json!({
                "uid": order.uid,
                "package": pkg.code,
                "duration_days": pkg.duration_days,
            }),
        )
        .await;
    Ok(())
}

pub struct BankPayInput<'a> {
    pub order_no: &'a str,
    pub bank_name: &'a str,
    pub bank_number: &'a str,
    pub bank_price: &'a str,
    pub bank_time: i64,
    pub order_remark: &'a str,
    pub order_pic: Option<&'a str>,
}

/// PHP `payComOrderByBank` — submit bank-transfer voucher for a pending VIP order.
pub async fn submit_bank_pay(
    state: &AppState,
    user: &AuthenticatedUser,
    input: BankPayInput<'_>,
    client_ip: &str,
) -> AppResult<()> {
    if input.bank_name.trim().is_empty() {
        return Err(ApiError::business("model_00022"));
    }
    if input.bank_number.trim().is_empty() {
        return Err(ApiError::business("model_00023"));
    }
    if input.bank_price.trim().is_empty() {
        return Err(ApiError::business("model_00024"));
    }
    if input.bank_time <= 0 {
        return Err(ApiError::business("wap_js_00127"));
    }
    let order = if let Some(o) =
        vip_repo::find_order_by_no_and_type(state.db.reader(), input.order_no, 2).await?
    {
        o
    } else if let Some(o) = vip_repo::find_order_by_no(state.db.reader(), input.order_no).await? {
        o
    } else {
        vip_repo::find_order_by_no_and_type(
            state.db.reader(),
            input.order_no,
            vip_repo::SEEKER_VIP_ORDER_TYPE,
        )
        .await?
        .ok_or_else(|| ApiError::business("order_not_found"))?
    };
    if order.uid != user.uid {
        return Err(ApiError::business("order_not_owned"));
    }
    if order.status != 0 && !(order.channel == "bank" && order.status == 3) {
        return Err(ApiError::business("order_not_pending"));
    }
    let order_bank = format!(
        "{}@%{}@%{}",
        input.bank_name.trim(),
        input.bank_number.trim(),
        input.bank_price.trim()
    );
    let remark_json = if input.order_remark.trim().is_empty() {
        None
    } else {
        Some(serde_json::json!({ "remark": input.order_remark.trim() }).to_string())
    };
    let pic = input.order_pic.map(str::trim).filter(|s| !s.is_empty());
    let affected = vip_repo::submit_bank_pay(
        state.db.pool(),
        input.order_no,
        user.uid,
        &order_bank,
        input.bank_time,
        pic,
        remark_json.as_deref(),
    )
    .await?;
    if affected == 0 {
        return Err(ApiError::business("order_not_pending"));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("vip.order_bank", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("order:{}", input.order_no)),
    )
    .await;
    Ok(())
}

pub async fn list_bank_accounts(
    state: &AppState,
    _user: &AuthenticatedUser,
) -> AppResult<Vec<phpyun_models::bank::entity::BankAccount>> {
    Ok(phpyun_models::bank::repo::list_all(state.db.reader()).await?)
}

pub struct OrderPage {
    pub list: Vec<PayOrder>,
    pub total: u64,
}

pub async fn list_orders(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<OrderPage> {
    let (total, list) = tokio::join!(
        vip_repo::count_user_orders(state.db.reader(), user.uid),
        vip_repo::list_user_orders(state.db.reader(), user.uid, page.offset, page.limit),
    );
    Ok(OrderPage {
        total: total?,
        list: list?,
    })
}

/// Cancel an unpaid order (only callable by the order owner).
pub async fn cancel_order(
    state: &AppState,
    user: &AuthenticatedUser,
    order_no: &str,
) -> AppResult<()> {
    let affected = vip_repo::cancel_order(state.db.pool(), order_no, user.uid).await?;
    if affected == 0 {
        return Err(ApiError::param_invalid("order_not_cancellable"));
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("vip.order_cancelled", Actor::uid(user.uid))
            .target(format!("order:{order_no}")),
    )
    .await;
    Ok(())
}

pub async fn get_current_vip(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<Option<UserVip>> {
    if user.usertype == 1 {
        return Ok(crate::seeker_vip_service::active_for(state, user.uid)
            .await?
            .map(|(v, _)| v));
    }
    use phpyun_models::company_statis::repo as statis_repo;
    let now = clock::now_ts();
    let Some(st) = statis_repo::find_admin(state.db.reader(), user.uid).await? else {
        return Ok(None);
    };
    if st.rating <= 0 && st.vip_etime == 0 {
        return Ok(None);
    }
    Ok(Some(UserVip {
        uid: user.uid,
        package_code: format!("pkg_{}", st.rating),
        started_at: st.vip_stime,
        expires_at: st.vip_etime,
        updated_at: now,
    }))
}

pub async fn buy_with_integral(
    state: &AppState,
    user: &AuthenticatedUser,
    package_code: &str,
    client_ip: &str,
) -> AppResult<String> {
    user.require_employer()?;
    let pkg = vip_repo::find_package_by_code(state.db.reader(), package_code)
        .await?
        .ok_or_else(|| ApiError::param_invalid("unknown package"))?;
    let q = quote_package_price(state, user, u64::from(pkg.id), "vip").await?;
    if q.style != 2 {
        return Err(ApiError::business("integral_insufficient"));
    }
    let pts = q.price.round() as i64;
    if pts > 0 {
        let n = phpyun_models::company_statis::repo::try_deduct_integral(
            state.db.pool(),
            user.uid,
            pts,
        )
        .await?;
        if n == 0 {
            return Err(ApiError::business("integral_insufficient"));
        }
    }
    let order_no = create_order(state, user, package_code, "integral", client_ip).await?;
    mark_paid(state, &order_no, "integral").await?;
    Ok(order_no)
}

// ==================== Pricing quote (PHPYun `getVipPrice` / `getPackPrice`) ====================
//
// Computes the effective price for a package. Mirrors PHP rules:
// - Active promo window (`time_start < now < time_end`) → use `yh_price`
// - Apply rating-tier discount (`service_discount`, percent value 0..=100)
// - When `com_integral_online == 3` and the package is integral-eligible:
//   integral may substitute for cash, multiplied by `integral_proportion`.
//   Compare the integral-priced amount to the user's integral balance to
//   decide `style`:
//     * 1 = cash-only path (integral mode disabled or excluded)
//     * 2 = integral covers it (user has enough)
//     * 3 = integral mode but balance insufficient — frontend should fall
//           back to cash with the discount still applied.

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PriceQuote {
    pub id: u64,
    pub name: String,
    /// Original (un-discounted) price in yuan.
    pub service_price: f64,
    /// Discounted yuan price. Falls back to `service_price` when no discount applies.
    pub yh_price: f64,
    /// Effective price the user will pay. Yuan when `style == 1`, integral
    /// units when `style == 2`, yuan when `style == 3` (frontend should
    /// route to cash payment).
    pub price: f64,
    /// 1=cash, 2=integral-pay, 3=insufficient-integral-fallback-to-cash.
    pub style: i32,
    /// Whether the promo window is currently active.
    pub promo_active: bool,
    /// User integral balance — let the client display "余额 X 不足".
    pub user_integral: i64,
}

/// `kind`:
///   * `pack` — company-package buy (PHP `getPackPrice_action`)
///   * `vip`  — individual VIP buy (PHP `getVipPrice_action`)
pub async fn quote_package_price(
    state: &AppState,
    user: &AuthenticatedUser,
    package_id: u64,
    kind: &str,
) -> AppResult<PriceQuote> {
    user.require_employer()?;
    let reader = state.db.reader();

    let pkg = phpyun_models::vip::repo::find_package_pricing(reader, package_id)
        .await?
        .ok_or_else(|| phpyun_core::ApiError::param_invalid("package_not_found"))?;

    // Read site config (`com_integral_online`, `integral_proportion`) and
    // `sy_only_price` (CSV of kinds that opt out of integral payment).
    let online_mode = read_int_setting(state, "com_integral_online")
        .await?
        .unwrap_or(0);
    let proportion = read_int_setting(state, "integral_proportion")
        .await?
        .unwrap_or(0);
    let only_price = read_str_setting(state, "sy_only_price")
        .await?
        .unwrap_or_default();
    let only_price_csv: Vec<&str> = only_price.split(',').filter(|s| !s.is_empty()).collect();

    let now = phpyun_core::clock::now_ts();
    let promo_active = pkg.time_start < now && pkg.time_end > now;

    let user_integral = phpyun_models::vip::repo::read_company_integral(reader, user.uid).await?;
    let discount = phpyun_models::vip::repo::read_company_rating_discount(reader, user.uid).await?;

    // PHP `service_discount` is a percent (e.g. 80 = 80%); divide by 100.
    let discount_factor = f64::from(discount) / 100.0;
    let pro = phpyun_core::numeric::finite_to_f64_db(
        phpyun_core::numeric::i64_to_f64(proportion.max(0)),
        "site_setting.integral_proportion",
    )?; // multiplier between yuan and integral

    let effective_yh = if pkg.yh_price > 0.0 {
        pkg.yh_price
    } else {
        pkg.service_price
    };

    // PHP separates `pack` and `vip` paths but the math is the same once
    // discount + window are folded in. The only difference: `pack` always
    // applies the rating discount; `vip` only applies the promo-window
    // discount. We stay faithful to that here.
    let (display_yh, display_service) = if kind == "pack" {
        (effective_yh * discount_factor, pkg.service_price)
    } else if kind == "vip" {
        if promo_active {
            (pkg.yh_price, pkg.service_price)
        } else {
            (pkg.service_price, pkg.service_price)
        }
    } else {
        return Err(phpyun_core::ApiError::param_invalid("kind"));
    };

    let integral_excluded = only_price_csv.contains(&kind);
    let integral_path = online_mode == 3 && !integral_excluded && pro > 0.0;

    let (price, style) = if !integral_path {
        (display_yh, 1)
    } else {
        // When paying with integral, the user pays `display_yh * pro` integral.
        let integral_needed = phpyun_core::numeric::finite_f64_to_i64_db(
            display_yh * pro,
            phpyun_core::numeric::FloatRounding::Truncate,
            "vip.integral_needed",
        )?;
        if integral_needed <= user_integral {
            (display_yh * pro, 2)
        } else {
            (display_yh, 3)
        }
    };

    Ok(PriceQuote {
        id: pkg.id,
        name: pkg.name,
        service_price: display_service,
        yh_price: display_yh,
        price,
        style,
        promo_active,
        user_integral,
    })
}

async fn read_int_setting(state: &AppState, key: &'static str) -> Result<Option<i64>, sqlx::Error> {
    let Some(row) = phpyun_models::site_setting::repo::find(state.db.reader(), key).await? else {
        return Ok(None);
    };
    let value = row.value.trim();
    if value.is_empty() {
        return Ok(None);
    }
    value
        .parse::<i64>()
        .map(Some)
        .map_err(|error| phpyun_core::numeric::db_conversion_error::<i64>(key, value, error))
}

async fn read_str_setting(state: &AppState, key: &str) -> Result<Option<String>, sqlx::Error> {
    Ok(
        phpyun_models::site_setting::repo::find(state.db.reader(), key)
            .await?
            .map(|row| row.value),
    )
}

pub struct IntegralClassView {
    pub id: u64,
    pub integral: i32,
    pub discount: i32,
}

pub struct IntegralClassPack {
    pub list: Vec<IntegralClassView>,
    pub min_recharge: i64,
    pub proportion: i64,
    pub pricename: String,
    pub priceunit: String,
    pub balance: i64,
}

fn member_usertype(user: &AuthenticatedUser) -> i32 {
    if user.usertype == 1 { 1 } else { 2 }
}

fn order_usertype(order: &PayOrder) -> i32 {
    if order.usertype == 1 { 1 } else { 2 }
}

async fn integral_balance(state: &AppState, uid: u64, usertype: i32) -> i64 {
    if usertype == 1 {
        phpyun_models::member_statis::repo::get_balance(state.db.reader(), uid)
            .await
            .map(|u| u.balance)
            .unwrap_or(0)
    } else {
        phpyun_models::company_statis::repo::read_integral(state.db.reader(), uid)
            .await
            .unwrap_or(0)
    }
}

async fn credit_integral(
    state: &AppState,
    uid: u64,
    usertype: i32,
    pts: i64,
    now: i64,
) -> AppResult<()> {
    if pts <= 0 {
        return Ok(());
    }
    if usertype == 1 {
        phpyun_models::member_statis::repo::add_balance(state.db.pool(), uid, pts, now).await?;
    } else {
        phpyun_models::company_statis::repo::add_integral(state.db.pool(), uid, pts).await?;
    }
    Ok(())
}

pub async fn list_integral_classes(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<IntegralClassPack> {
    let list = phpyun_models::integral::repo::list_active_classes(state.db.reader())
        .await?
        .into_iter()
        .map(|c| IntegralClassView {
            id: c.id,
            integral: c.integral,
            discount: c.discount,
        })
        .collect();
    let min_recharge = read_int_setting(state, "integral_min_recharge")
        .await?
        .unwrap_or(0)
        .max(0);
    let proportion = read_int_setting(state, "integral_proportion")
        .await?
        .unwrap_or(1)
        .max(1);
    let pricename = read_str_setting(state, "integral_pricename")
        .await?
        .unwrap_or_default();
    let priceunit = read_str_setting(state, "integral_priceunit")
        .await?
        .unwrap_or_default();
    let ut = member_usertype(user);
    let balance = integral_balance(state, user.uid, ut).await;
    Ok(IntegralClassPack {
        list,
        min_recharge,
        proportion,
        pricename,
        priceunit,
        balance,
    })
}

pub struct CreatedRechargeOrder {
    pub order_no: String,
    pub amount_cents: i32,
    pub integral: i64,
    pub subject: String,
}

fn recharge_price_yuan(pts: i64, proportion: i64, discount: i32) -> AppResult<(f64, i32)> {
    let pro = phpyun_core::numeric::finite_to_f64_db(
        phpyun_core::numeric::i64_to_f64(proportion.max(1)),
        "site_setting.integral_proportion",
    )?;
    let pts_f = phpyun_core::numeric::finite_to_f64_db(
        phpyun_core::numeric::i64_to_f64(pts),
        "company_order.integral",
    )?;
    let mut price = pts_f / pro;
    if discount > 0 {
        price *= f64::from(discount) / 100.0;
    }
    let price = (price * 100.0).round() / 100.0;
    let amount_cents = (price * 100.0).round() as i32;
    if amount_cents < 1 {
        return Err(ApiError::business("common_00644"));
    }
    Ok((price, amount_cents))
}

pub async fn create_recharge(
    state: &AppState,
    user: &AuthenticatedUser,
    price_int: i64,
    integralid: u64,
    channel: &str,
    remark: &str,
    client_ip: &str,
) -> AppResult<CreatedRechargeOrder> {
    if price_int > 10_000_000 {
        return Err(ApiError::business("common_00644"));
    }
    let min_recharge = read_int_setting(state, "integral_min_recharge")
        .await?
        .unwrap_or(0)
        .max(0);
    let mut pts = price_int.max(0);
    if min_recharge > 0 && pts < min_recharge {
        pts = min_recharge;
    }
    if pts < 1 {
        return Err(ApiError::business("common_00644"));
    }
    let proportion = read_int_setting(state, "integral_proportion")
        .await?
        .unwrap_or(1)
        .max(1);
    let mut discount = 0i32;
    let mut rating = 0i32;
    if integralid > 0 {
        if let Some(cls) = phpyun_models::integral::repo::find_class(state.db.reader(), integralid)
            .await?
        {
            if cls.state == 1 && pts >= i64::from(cls.integral) {
                discount = cls.discount;
                rating = i32::try_from(cls.id).unwrap_or(0);
            }
        }
    }
    let (price_yuan, amount_cents) = recharge_price_yuan(pts, proportion, discount)?;
    let now = clock::now_ts();
    let note = {
        let t = remark.trim();
        if t.is_empty() {
            "recharge".to_string()
        } else {
            t.to_string()
        }
    };
    let order_no = vip_repo::create_recharge_order(
        state.db.pool(),
        user.uid,
        user.did,
        member_usertype(user),
        channel,
        price_yuan,
        pts,
        rating,
        &note,
        now,
    )
    .await?;
    let pricename = read_str_setting(state, "integral_pricename")
        .await?
        .unwrap_or_default();
    let _ = audit::emit(
        state,
        AuditEvent::new("vip.recharge_create", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("order:{order_no}"))
            .meta(&serde_json::json!({
                "integral": pts,
                "amount_cents": amount_cents,
                "channel": channel,
            })),
    )
    .await;
    Ok(CreatedRechargeOrder {
        order_no,
        amount_cents,
        integral: pts,
        subject: format!("common_01946{pricename}"),
    })
}

pub async fn mark_recharge_paid(state: &AppState, order_no: &str, pay_tx_id: &str) -> AppResult<()> {
    let order = vip_repo::find_order_by_no_and_type(state.db.reader(), order_no, 2)
        .await?
        .ok_or_else(|| ApiError::param_invalid("order_not_found"))?;
    if order.status == 1 {
        return Ok(());
    }
    if order.status != 0 {
        return Err(ApiError::param_invalid("order_not_pending"));
    }
    let now = clock::now_ts();
    let affected = vip_repo::mark_order_paid(state.db.pool(), order_no, pay_tx_id, now).await?;
    if affected == 0 {
        return Ok(());
    }
    let pts = i64::from(order.integral.max(0));
    let ut = order_usertype(&order);
    if pts > 0 {
        credit_integral(state, order.uid, ut, pts, now).await?;
        let pricename = read_str_setting(state, "integral_pricename")
            .await?
            .unwrap_or_default();
        let remark = format!("member_user_00285{pricename}");
        let _ = phpyun_models::integral_transfer::repo::php_insert_pay_typed(
            state.db.pool(),
            order_no,
            &pts.to_string(),
            now,
            order.uid,
            &remark,
            phpyun_models::integral_transfer::repo::LEDGER_KIND_INTEGRAL,
            ut,
            2,
        )
        .await;
    }
    let _ = audit::emit(
        state,
        AuditEvent::new("vip.recharge_paid", Actor::uid(order.uid))
            .target(format!("order:{order_no}"))
            .meta(&serde_json::json!({ "integral": pts, "pay_tx_id": pay_tx_id })),
    )
    .await;
    Ok(())
}

fn table_missing(err: &sqlx::Error) -> bool {
    let s = err.to_string();
    s.contains("1146") || s.contains("doesn't exist")
}

pub async fn redeem_card(
    state: &AppState,
    user: &AuthenticatedUser,
    card: &str,
    password: &str,
    client_ip: &str,
) -> AppResult<i32> {
    let card = card.trim();
    let password = password.trim();
    if card.is_empty() || password.is_empty() {
        return Err(ApiError::param_invalid("card"));
    }
    if !card.bytes().all(|b| b.is_ascii_digit()) || !password.bytes().all(|b| b.is_ascii_digit()) {
        return Err(ApiError::param_invalid("card"));
    }
    let row = match phpyun_models::integral::repo::find_prepaid_card(state.db.reader(), card).await {
        Ok(v) => v,
        Err(e) if table_missing(&e) => return Err(ApiError::business("member_com_00645")),
        Err(e) => return Err(e.into()),
    };
    let Some(row) = row else {
        return Err(ApiError::business("member_com_00645"));
    };
    if row.password != password {
        return Err(ApiError::business("member_com_00645"));
    }
    if row.uid > 0 {
        return Err(ApiError::business("member_com_00645"));
    }
    if row.quota <= 0 {
        return Err(ApiError::business("common_00644"));
    }
    let username = phpyun_models::user::repo::find_by_uid(state.db.reader(), user.uid)
        .await?
        .map(|m| m.username)
        .unwrap_or_default();
    let now = clock::now_ts();
    let n = phpyun_models::integral::repo::claim_prepaid_card(
        state.db.pool(),
        row.id,
        user.uid,
        &username,
        now,
    )
    .await?;
    if n == 0 {
        return Err(ApiError::business("member_com_00645"));
    }
    let pts = i64::from(row.quota);
    let ut = member_usertype(user);
    credit_integral(state, user.uid, ut, pts, now).await?;
    let pricename = read_str_setting(state, "integral_pricename")
        .await?
        .unwrap_or_default();
    let remark = format!("member_com_00645{pricename}");
    let order_id = format!("{now}{}", user.uid % 90_000 + 10_000);
    let _ = phpyun_models::integral_transfer::repo::php_insert_pay_typed(
        state.db.pool(),
        &order_id,
        &pts.to_string(),
        now,
        user.uid,
        &remark,
        phpyun_models::integral_transfer::repo::LEDGER_KIND_INTEGRAL,
        ut,
        2,
    )
    .await;
    let _ = audit::emit(
        state,
        AuditEvent::new("vip.card_redeem", Actor::uid(user.uid).with_ip(client_ip))
            .meta(&serde_json::json!({ "card": card, "quota": pts })),
    )
    .await;
    Ok(row.quota)
}
