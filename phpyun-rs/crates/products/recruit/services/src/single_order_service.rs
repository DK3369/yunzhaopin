//! Cash single-purchase orders for resume download (PHP `type=19`) and
//! interview invite quota (PHP `type=23`). Cash `confirm=true` only creates a
//! pending order; unlock / quota happens in [`settle_download`] / [`settle_invite`]
//! after the payment callback.

use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::company_statis::repo as statis_repo;
use phpyun_models::resume::expect;
use phpyun_models::resume_download::repo as download_repo;
use phpyun_models::vip::repo::{self as vip_repo, AnyOrder};
use serde::Deserialize;

pub fn pay_channel(raw: Option<&str>) -> Result<&'static str, ApiError> {
    match raw.unwrap_or("").trim() {
        "" | "wechat" => Ok("wechat"),
        "alipay" => Ok("alipay"),
        _ => Err(ApiError::param_invalid("channel")),
    }
}

fn cents_from_yuan(yuan: i64) -> i32 {
    i32::try_from(yuan.saturating_mul(100)).unwrap_or(i32::MAX)
}

pub async fn create_download_order(
    state: &AppState,
    user: &AuthenticatedUser,
    eid: u64,
    seeker_uid: u64,
    channel: &str,
) -> AppResult<String> {
    user.require_employer()?;
    let yuan = crate::resume_download_service::resume_day_price(state, eid, false).await?;
    let info = serde_json::json!({ "eid": eid, "uid": seeker_uid }).to_string();
    let now = clock::now_ts();
    Ok(vip_repo::create_single_order(
        state.db.pool(),
        user.uid,
        i32::from(user.usertype),
        19,
        eid,
        "wap_00451",
        cents_from_yuan(yuan),
        channel,
        &info,
        now,
    )
    .await?)
}

pub async fn create_invite_order(
    state: &AppState,
    user: &AuthenticatedUser,
    channel: &str,
) -> AppResult<String> {
    user.require_employer()?;
    let raw = crate::site_gate_service::config_str(state, "integral_interview").await;
    let yuan: i64 = raw.trim().parse().unwrap_or(0).max(0);
    let info = serde_json::json!({ "uid": user.uid }).to_string();
    let now = clock::now_ts();
    Ok(vip_repo::create_single_order(
        state.db.pool(),
        user.uid,
        i32::from(user.usertype),
        23,
        0,
        "wap_com_00046",
        cents_from_yuan(yuan),
        channel,
        &info,
        now,
    )
    .await?)
}

#[derive(Deserialize)]
struct DownloadInfo {
    eid: u64,
    #[serde(default)]
    uid: u64,
}

pub async fn settle_download(
    state: &AppState,
    order: &AnyOrder,
    pay_tx_id: &str,
) -> AppResult<()> {
    let now = clock::now_ts();
    let n = vip_repo::mark_single_paid(state.db.pool(), &order.order_no, pay_tx_id, now).await?;
    if n == 0 {
        return Ok(());
    }
    let info: DownloadInfo = serde_json::from_str(&order.order_info)
        .map_err(|_| ApiError::param_invalid("order_info"))?;
    if info.eid == 0 {
        return Err(ApiError::param_invalid("order_info"));
    }
    let seeker = if info.uid > 0 {
        info.uid
    } else {
        expect::find_by_id(state.db.reader(), info.eid)
            .await?
            .map(|e| e.uid)
            .unwrap_or(0)
    };
    if seeker == 0 {
        return Err(ApiError::param_invalid("order_info"));
    }
    let already =
        download_repo::already_downloaded_eid(state.db.reader(), order.uid, info.eid).await?;
    if !already {
        download_repo::record(state.db.pool(), order.uid, seeker, info.eid, now).await?;
        crate::resume_download_service::notify_first(state, order.uid, seeker, now, true).await?;
    }
    Ok(())
}

pub async fn settle_invite(state: &AppState, order: &AnyOrder, pay_tx_id: &str) -> AppResult<()> {
    let now = clock::now_ts();
    let n = vip_repo::mark_single_paid(state.db.pool(), &order.order_no, pay_tx_id, now).await?;
    if n == 0 {
        return Ok(());
    }
    let _ = statis_repo::add_invite_resume(state.db.pool(), order.uid).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::pay_channel;

    #[test]
    fn channel_whitelist() {
        assert_eq!(pay_channel(None).unwrap(), "wechat");
        assert_eq!(pay_channel(Some("")).unwrap(), "wechat");
        assert_eq!(pay_channel(Some("wechat")).unwrap(), "wechat");
        assert_eq!(pay_channel(Some("alipay")).unwrap(), "alipay");
        assert!(pay_channel(Some("bank")).is_err());
    }
}
