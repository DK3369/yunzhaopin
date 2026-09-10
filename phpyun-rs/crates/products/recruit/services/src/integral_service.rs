//! Points-mall service (mirrors PHPYun `wap/redeem`).
//!
//! Redemption flow (atomic):
//!   1) Deduct stock (atomic UPDATE WHERE stock>0) — failure means sold out
//!   2) Deduct user points (atomic UPDATE WHERE balance>=cost) — failure means insufficient balance
//!   3) Write the exchange record
//!   4) If any of the above fails, roll back what was already deducted (using a reverse UPDATE)
//!
//! No database transaction is used here (transactions across the writer pool are expensive);
//! we use compensation (transaction-script pattern) instead. The PHPYun original only does naive
//! checks at the PHP layer — not atomic, so it has oversell / double-deduct issues. The migrated
//! version fixes that along the way.

use phpyun_core::{
    audit, clock, ApiError, AppResult, AppState, AuthenticatedUser, Paged, Pagination,
};
use phpyun_models::integral::{
    entity::{IntegralExchange, IntegralItem, UserIntegral},
    repo as integral_repo,
};

pub async fn list_items(state: &AppState, page: Pagination) -> AppResult<Paged<IntegralItem>> {
    let db = state.db.reader();
    let list = integral_repo::list_items(db, page.offset, page.limit).await?;
    let total = integral_repo::count_items(db).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

pub async fn get_item(state: &AppState, id: u64) -> AppResult<IntegralItem> {
    integral_repo::find_item(state.db.reader(), id)
        .await?
        .ok_or_else(|| ApiError::param_invalid("item_not_found"))
}

pub async fn balance(state: &AppState, user: &AuthenticatedUser) -> AppResult<UserIntegral> {
    Ok(integral_repo::get_balance(state.db.reader(), user.uid).await?)
}

pub async fn exchange(
    state: &AppState,
    user: &AuthenticatedUser,
    item_id: u64,
    client_ip: &str,
) -> AppResult<u64> {
    let item = integral_repo::find_item(state.db.reader(), item_id)
        .await?
        .ok_or_else(|| ApiError::param_invalid("item_not_found"))?;
    if item.status != 1 {
        return Err(ApiError::param_invalid("item_unavailable"));
    }

    let pool = state.db.pool();
    let now = clock::now_ts();

    // 1) Deduct stock
    let stock_affected = integral_repo::try_consume_stock(pool, item_id).await?;
    if stock_affected == 0 {
        return Err(ApiError::param_invalid("item_sold_out"));
    }

    // 2) Deduct points
    let deduct_affected = integral_repo::try_deduct(pool, user.uid, item.cost, now).await?;
    if deduct_affected == 0 {
        // Roll back stock
        integral_repo::rollback_stock(pool, item_id).await?;
        return Err(ApiError::param_invalid("insufficient_balance"));
    }

    // 3) Write the exchange record
    let exchange_id =
        match integral_repo::create_exchange(pool, user.uid, item_id, item.cost, now).await {
            Ok(id) => id,
            Err(e) => {
                // Roll back points + stock
                let _ = integral_repo::add_balance(pool, user.uid, i64::from(item.cost), now).await;
                let _ = integral_repo::rollback_stock(pool, item_id).await;
                return Err(e.into());
            }
        };

    let _ = audit::emit(
        state,
        audit::AuditEvent::new(
            "integral.exchange",
            audit::Actor::uid(user.uid).with_ip(client_ip),
        )
        .target(format!("item:{item_id}"))
        .meta(&serde_json::json!({ "cost": item.cost, "exchange_id": exchange_id })),
    )
    .await;

    Ok(exchange_id)
}

pub async fn list_history(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<Paged<IntegralExchange>> {
    let db = state.db.reader();
    let list = integral_repo::list_exchanges_by_user(db, user.uid, page.offset, page.limit).await?;
    let total = integral_repo::count_exchanges_by_user(db, user.uid).await?;
    Ok(Paged::new(list, total, page.page, page.page_size))
}

// ---------- Points transfer ----------

use phpyun_models::integral_transfer::{entity::IntegralTransfer, repo as transfer_repo};

fn validate_transfer(from_uid: u64, to_uid: u64, points: u32) -> AppResult<()> {
    if to_uid == from_uid {
        return Err(ApiError::param_invalid("cannot_transfer_to_self"));
    }
    if points == 0 {
        return Err(ApiError::param_invalid("bad_points"));
    }
    Ok(())
}

pub async fn transfer(
    state: &AppState,
    user: &AuthenticatedUser,
    to_uid: u64,
    points: u32,
    note: &str,
) -> AppResult<u64> {
    validate_transfer(user.uid, to_uid, points)?;
    let now = clock::now_ts();
    let id = transfer_repo::execute(state.db.pool(), user.uid, to_uid, points, note, now)
        .await?
        .ok_or_else(|| ApiError::param_invalid("insufficient_balance"))?;
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("integral.transfer", audit::Actor::uid(user.uid))
            .target(format!("to:{to_uid}"))
            .meta(&serde_json::json!({ "points": points, "transfer_id": id })),
    )
    .await;
    Ok(id)
}

pub async fn list_transfers(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<Paged<IntegralTransfer>> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        transfer_repo::list_by_user(db, user.uid, page.offset, page.limit),
        transfer_repo::count_by_user(db, user.uid),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

pub async fn list_consumes(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<Paged<IntegralTransfer>> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        transfer_repo::list_ledger_by_uid(db, user.uid, page.offset, page.limit),
        transfer_repo::count_ledger_by_uid(db, user.uid),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

// ---------- Daily mission completion (PHP `integral.model::integralMission`) ----------

use phpyun_models::company::repo as company_repo;
use phpyun_models::company_banner::repo as banner_repo;
use phpyun_models::resume::expect as expect_repo;
use phpyun_models::resume::repo as resume_repo;
use phpyun_models::sign_in::repo as sign_repo;
use phpyun_models::user::repo as user_repo;

#[derive(Debug, Clone)]
pub struct IntegralMission {
    pub base_info: bool,
    pub photo: bool,
    pub logo: bool,
    pub signin: bool,
    pub email_checked: bool,
    pub phone_checked: bool,
    pub identification: bool,
    pub weixin_bind: bool,
    pub map: bool,
    pub banner: bool,
    pub yyzz: bool,
    pub question: bool,
    pub answer: bool,
    pub answerpl: bool,
    pub resume: bool,
}

fn filled(s: Option<&str>) -> bool {
    s.map(|v| !v.trim().is_empty()).unwrap_or(false)
}

fn coord_set(s: Option<&str>) -> bool {
    match s.map(str::trim) {
        Some(v) if !v.is_empty() && v != "0" && v != "0.0" => true,
        _ => false,
    }
}

pub async fn mission(state: &AppState, user: &AuthenticatedUser) -> AppResult<IntegralMission> {
    let db = state.db.reader();
    let usertype = i32::from(user.usertype);
    let today = super::sign_service::ymd_of(clock::now_ts())?;
    let mut out = IntegralMission {
        base_info: false,
        photo: false,
        logo: false,
        signin: false,
        email_checked: false,
        phone_checked: false,
        identification: false,
        weixin_bind: false,
        map: false,
        banner: false,
        yyzz: false,
        question: false,
        answer: false,
        answerpl: false,
        resume: false,
    };

    if user.usertype == 2 {
        if let Some(c) = company_repo::find_by_uid(db, user.uid).await? {
            out.base_info = filled(c.name.as_deref()) && c.hy != 0;
            out.logo = filled(c.logo.as_deref());
            out.email_checked = c.email_status != 0;
            out.phone_checked = c.moblie_status != 0;
            out.map = coord_set(c.x.as_deref()) && coord_set(c.y.as_deref());
            out.yyzz = c.yyzz_status != 0;
        }
        out.banner = banner_repo::count_by_uid(db, user.uid).await? > 0;
    } else {
        if let Some(r) = resume_repo::find_by_uid(db, user.uid).await? {
            out.base_info = filled(r.name.as_deref())
                && r.sex != 0
                && filled(r.birthday.as_deref())
                && filled(r.telphone.as_deref())
                && r.education != 0
                && r.exp != 0
                && filled(r.living.as_deref());
            out.photo = filled(r.photo.as_deref()) && r.defphoto == 1;
            out.email_checked = r.email_status != 0;
            out.phone_checked = r.moblie_status != 0;
            out.identification = r.idcard_status != 0;
        }
        out.resume = expect_repo::count_by_uid(db, user.uid).await? > 0;
    }

    out.weixin_bind = user_repo::weixin_bound(db, user.uid).await?;
    let lock_key = format!("sign:lock:{}:{}", user.uid, today);
    out.signin = sign_repo::exists_reg_today(db, user.uid, usertype, today).await?
        || state.redis.exists(&lock_key).await;
    out.question =
        transfer_repo::count_remark_today(db, user.uid, usertype, "wap_user_00112").await? > 0;
    out.answer =
        transfer_repo::count_remark_today(db, user.uid, usertype, "wap_user_00113").await? > 0;
    out.answerpl =
        transfer_repo::count_remark_today(db, user.uid, usertype, "common_06374").await? > 0;
    Ok(out)
}

#[cfg(test)]
mod transfer_tests {
    use super::validate_transfer;

    #[test]
    fn rejects_self_transfer_and_zero_points() {
        assert_eq!(validate_transfer(7, 7, 1).unwrap_err().code(), 400);
        assert_eq!(validate_transfer(7, 8, 0).unwrap_err().code(), 400);
        assert!(validate_transfer(7, 8, 1).is_ok());
    }
}
