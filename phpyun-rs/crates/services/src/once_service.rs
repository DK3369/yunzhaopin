//! One-off shop hiring (once) service.
//!
//! Mirrors PHPYun `app/model/once.model.php` + `app/controller/once/index.class.php`.
//!
//! Semantics: a company posts a single job without registering, managed via mobile+password(md5).
//! Related configuration:
//! - `sy_once`          per-IP daily limit
//! - `sy_once_totalnum` site-wide daily limit (0 = unlimited)
//! - `sy_once_web`      switch: 2 = disabled

use phpyun_auth::md5_hex;
use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, AppError, AppResult, AppState, InfraError, Pagination};
use phpyun_models::once_job::entity::OnceJob;
use phpyun_models::once_job::repo as once_repo;

use crate::domain_errors::TinyError;

// ==================== Public browsing ====================

#[derive(Debug, Clone, Default)]
pub struct OnceSearch {
    pub keyword: Option<String>,
    pub province_id: Option<i32>,
    pub city_id: Option<i32>,
    pub three_city_id: Option<i32>,
    pub exp: Option<i32>,
    pub edu: Option<i32>,
    pub did: u32,
}

pub struct OncePage {
    pub list: Vec<OnceJob>,
    pub total: u64,
}

pub async fn list_public(
    state: &AppState,
    search: &OnceSearch,
    page: Pagination,
) -> AppResult<OncePage> {
    let now = clock::now_ts();
    let filter = once_repo::Filter {
        keyword: search.keyword.as_deref(),
        province_id: search.province_id,
        city_id: search.city_id,
        three_city_id: search.three_city_id,
        exp: search.exp,
        edu: search.edu,
        did: if search.did == 0 { 1 } else { search.did },
    };
    let (total, list) = tokio::join!(
        once_repo::count_public(state.db.reader(), &filter, now),
        once_repo::list_public(state.db.reader(), &filter, page.offset, page.limit, now),
    );
    Ok(OncePage {
        total: total?,
        list: list?,
    })
}

/// Detail page + asynchronously increment hits
pub async fn show(state: &AppState, id: u64) -> AppResult<OnceJob> {
    let item = once_repo::find_by_id(state.db.reader(), id)
        .await?
        .ok_or(TinyError::NotFound)?;

    let pool = state.db.pool().clone();
    tokio::spawn(async move {
        let _ = once_repo::incr_hits(&pool, id).await;
    });
    Ok(item)
}

// ==================== Create / edit ====================

#[derive(Debug, Clone)]
pub struct UpsertInput {
    pub id: Option<u64>,
    pub companyname: String,
    pub linkman: String,
    pub linktel: String,
    pub password: String,
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    pub number: i32,
    pub job_type: i32,
    pub salary: i32,
    pub exp: i32,
    pub edu: i32,
    pub require: String,
    pub pic: String,
    pub yyzz: String,
    /// Default status (matches PHP `user_wjl`; 0 = pending review / 1 = approved)
    pub default_status: i32,
    pub valid_days: i64,
    pub today_by_ip: u64,
    pub today_total: u64,
    /// `sy_once_totalnum` (0 = unlimited)
    pub daily_total_limit: u64,
    /// `sy_once` (0 = unlimited)
    pub daily_ip_limit: u64,
    pub did: u32,
    pub login_ip: String,
}

#[derive(Debug)]
pub struct UpsertResult {
    pub id: u64,
    pub created: bool,
}

pub async fn upsert(state: &AppState, input: &UpsertInput) -> AppResult<UpsertResult> {
    validate_fields(input)?;
    let pwd_md5 = if input.password.is_empty() {
        String::new()
    } else {
        md5_hex(&input.password)
    };

    if let Some(id) = input.id {
        if pwd_md5.is_empty() {
            return Err(InfraError::InvalidParam("password_required".into()).into());
        }
        let upd = once_repo::Update {
            companyname: &input.companyname,
            linkman: &input.linkman,
            linktel: &input.linktel,
            provinceid: input.provinceid,
            cityid: input.cityid,
            three_cityid: input.three_cityid,
            number: input.number,
            job_type: input.job_type,
            salary: input.salary,
            exp: input.exp,
            edu: input.edu,
            require: &input.require,
        };
        let n = once_repo::update_with_password_check(state.db.pool(), id, &pwd_md5, &upd).await?;
        if n == 0 {
            return Err(TinyError::PasswordMismatch.into());
        }
        let _ = audit::emit(
            state,
            AuditEvent::new("once.update", Actor::anonymous().with_ip(input.login_ip.clone()))
                .target(format!("once:{id}")),
        )
        .await;
        return Ok(UpsertResult { id, created: false });
    }

    // Quota check
    if input.daily_total_limit > 0 && input.today_total >= input.daily_total_limit {
        return Err(TinyError::DailySiteLimit.into());
    }
    if input.daily_ip_limit > 0 && input.today_by_ip >= input.daily_ip_limit {
        return Err(TinyError::DailyIpLimit.into());
    }

    if pwd_md5.is_empty() {
        return Err(InfraError::InvalidParam("password_required".into()).into());
    }

    let now = clock::now_ts();
    let edate = if input.valid_days > 0 {
        now + input.valid_days * 86400
    } else {
        0
    };
    let create = once_repo::Create {
        companyname: &input.companyname,
        linkman: &input.linkman,
        linktel: &input.linktel,
        provinceid: input.provinceid,
        cityid: input.cityid,
        three_cityid: input.three_cityid,
        number: input.number,
        job_type: input.job_type,
        salary: input.salary,
        exp: input.exp,
        edu: input.edu,
        require: &input.require,
        pic: &input.pic,
        yyzz: &input.yyzz,
        password_md5: &pwd_md5,
        login_ip: &input.login_ip,
        status: input.default_status,
        edate,
        did: if input.did == 0 { 1 } else { input.did },
        now,
    };
    let id = once_repo::create(state.db.pool(), &create).await?;
    let _ = audit::emit(
        state,
        AuditEvent::new("once.create", Actor::anonymous().with_ip(input.login_ip.clone()))
            .target(format!("once:{id}")),
    )
    .await;
    Ok(UpsertResult { id, created: true })
}

fn validate_fields(input: &UpsertInput) -> AppResult<()> {
    if input.companyname.trim().is_empty() {
        return Err(InfraError::InvalidParam("companyname".into()).into());
    }
    if input.linkman.trim().is_empty() {
        return Err(InfraError::InvalidParam("linkman".into()).into());
    }
    if input.linktel.trim().is_empty() {
        return Err(InfraError::InvalidParam("linktel".into()).into());
    }
    if input.provinceid == 0 && input.cityid == 0 {
        return Err(InfraError::InvalidParam("city".into()).into());
    }
    if input.require.trim().is_empty() {
        return Err(InfraError::InvalidParam("require".into()).into());
    }
    Ok(())
}

// ==================== Password management ====================

#[derive(Debug, Clone, Copy)]
pub enum ManageOp {
    Verify,
    Refresh,
    Delete,
}

pub async fn manage(
    state: &AppState,
    id: u64,
    password: &str,
    op: ManageOp,
) -> AppResult<()> {
    if password.is_empty() {
        return Err(InfraError::InvalidParam("password".into()).into());
    }
    let pwd_md5 = md5_hex(password);
    match op {
        ManageOp::Verify => {
            let ok = once_repo::verify_password(state.db.reader(), id, &pwd_md5).await?;
            if !ok {
                return Err(TinyError::PasswordMismatch.into());
            }
        }
        ManageOp::Refresh => {
            let n = once_repo::refresh_with_password(
                state.db.pool(),
                id,
                &pwd_md5,
                clock::now_ts(),
            )
            .await?;
            if n == 0 {
                return Err(TinyError::PasswordMismatch.into());
            }
        }
        ManageOp::Delete => {
            let n = once_repo::delete_with_password(state.db.pool(), id, &pwd_md5).await?;
            if n == 0 {
                return Err(TinyError::PasswordMismatch.into());
            }
        }
    }
    Ok(())
}

pub async fn usage_today(state: &AppState, login_ip: &str) -> AppResult<(u64, u64)> {
    let begin = crate::tiny_service::today_begin_ts(clock::now_ts());
    let (by_ip, total) = tokio::join!(
        once_repo::count_today_by_ip(state.db.reader(), login_ip, begin),
        once_repo::count_today_total(state.db.reader(), begin),
    );
    Ok((by_ip?, total?))
}

// ==================== Pay flow ====================

pub struct PayResult {
    pub order_id: String,
    pub price: f64,
    pub days: i32,
    /// 1 = pending payment (gateway redirect needed), 2 = already paid
    /// (gear price was 0, no gateway round-trip required).
    pub state: i32,
    pub fast: String,
}

#[derive(Debug, Clone)]
pub struct PayInput<'a> {
    pub once_id: u64,
    pub password: &'a str,
    pub pay_type: &'a str,
    pub gear_id: i32,
    pub did: i32,
}

/// Create a payment order for a once-job posting.
/// Counterpart of PHP `wap/once::getOrder_action` + `once.model.php::payOnce`.
///
/// Caller authenticates via the once_job's posting password (md5), so this is
/// a public endpoint — no JWT required (mirrors PHP).
pub async fn create_pay_order(state: &AppState, input: PayInput<'_>) -> AppResult<PayResult> {
    if input.once_id == 0 {
        return Err(InfraError::InvalidParam("once_id".into()).into());
    }
    if input.password.is_empty() {
        return Err(InfraError::InvalidParam("password".into()).into());
    }
    if input.gear_id == 0 {
        return Err(InfraError::InvalidParam("oncepricegear".into()).into());
    }

    let pool = state.db.pool();
    let pwd_md5 = md5_hex(input.password);
    if !once_repo::verify_password(pool, input.once_id, &pwd_md5).await? {
        return Err(TinyError::PasswordMismatch.into());
    }

    let (days, price) = once_repo::find_price_gear(pool, input.gear_id)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("gear_not_found".into())))?;

    // Wipe stale pending orders on the same once_job before creating a new one.
    let _ = once_repo::delete_pending_orders_for_once(pool, input.once_id).await;

    let now = clock::now_ts();
    let suffix1 = uuid::Uuid::new_v4().as_u128() as u32 % 100_000;
    let suffix2 = uuid::Uuid::new_v4().as_u128() as u32 % 100_000;
    let order_id = format!("{}{:05}", now, suffix1);
    let fast = format!("{}{:05}", now, suffix2);

    let state_code = if price <= 0.0 { 2 } else { 1 };
    let _ = once_repo::insert_once_order(
        pool,
        once_repo::OrderInsert {
            uid: 0, // PHP doesn't fill uid for once orders (anonymous posters)
            order_id: &order_id,
            pay_type: input.pay_type,
            price,
            now,
            state: state_code,
            did: input.did,
            once_id: input.once_id,
            fast: &fast,
        },
    )
    .await?;

    if state_code == 2 {
        let _ = once_repo::mark_once_paid(pool, input.once_id).await;
    }

    let _ = audit::emit(
        state,
        AuditEvent::new("once.pay_order_created", Actor::anonymous()).target(&order_id),
    )
    .await;

    Ok(PayResult {
        order_id,
        price,
        days,
        state: state_code,
        fast,
    })
}

pub struct PendingOrdersPage {
    pub list: Vec<phpyun_models::once_job::repo::OnceOrder>,
    pub total: u64,
}

pub async fn list_my_pending_orders(
    state: &AppState,
    user: &phpyun_core::AuthenticatedUser,
    page: Pagination,
) -> AppResult<PendingOrdersPage> {
    user.require_employer()?;
    let pool = state.db.reader();
    let (list, total) = tokio::join!(
        once_repo::list_pending_once_orders(pool, user.uid, page.offset, page.limit),
        once_repo::count_pending_once_orders(pool, user.uid),
    );
    Ok(PendingOrdersPage {
        list: list?,
        total: total?,
    })
}

pub async fn cancel_pending_order(
    state: &AppState,
    user: &phpyun_core::AuthenticatedUser,
    order_id: u64,
) -> AppResult<()> {
    user.require_employer()?;
    let n = once_repo::cancel_pending_once_order(state.db.pool(), user.uid, order_id).await?;
    if n == 0 {
        return Err(InfraError::InvalidParam("order_not_cancellable".into()).into());
    }
    Ok(())
}
