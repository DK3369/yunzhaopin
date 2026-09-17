//! Site-config integral grants (PHP `integral.model.php::invtalCheck`).
//! Once-only kinds key off `phpyun_company_pay.pay_remark`.

use phpyun_core::{clock, AppResult, AppState};
use phpyun_models::company_statis::repo as company_statis_repo;
use phpyun_models::integral::repo as integral_repo;
use phpyun_models::integral_transfer::repo as pay_repo;

pub async fn grant(
    state: &AppState,
    uid: u64,
    usertype: i32,
    cfg_key: &str,
    remark: &str,
    _pay_type: i32,
) -> AppResult<()> {
    if uid == 0 {
        return Ok(());
    }
    let raw = crate::site_gate_service::config_str(state, cfg_key).await;
    let pts: i64 = raw.trim().parse().unwrap_or(0);
    if pts <= 0 {
        return Ok(());
    }
    let now = clock::now_ts();
    let db = state.db.pool();
    if usertype == 1 {
        integral_repo::add_balance(db, uid, pts, now).await?;
    } else {
        company_statis_repo::add_integral(db, uid, pts).await?;
    }
    let order_id = format!("{cfg_key}{now}{uid}");
    pay_repo::php_insert_pay(
        db,
        &order_id,
        &pts.to_string(),
        now,
        uid,
        remark,
        pay_repo::LEDGER_KIND_INTEGRAL,
        usertype,
    )
    .await?;
    Ok(())
}

pub async fn grant_once(
    state: &AppState,
    uid: u64,
    usertype: i32,
    cfg_key: &str,
    remark: &str,
    pay_type: i32,
) -> AppResult<()> {
    if uid == 0 {
        return Ok(());
    }
    if pay_repo::exists_pay_remark(state.db.reader(), uid, remark).await? {
        return Ok(());
    }
    grant(state, uid, usertype, cfg_key, remark, pay_type).await
}
