//! Cash single-purchase orders (PHP `company_order.type`):
//! - 10 job top / 12 rec / 11 urgent
//! - 16 job refresh / 17 part refresh
//! - 19 resume download / 23 interview invite
//!
//! Cash `confirm=true` only creates a pending `order_state=0` row. Unlock /
//! apply happens in the matching `settle_*` after the payment callback.

use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::company_statis::repo as statis_repo;
use phpyun_models::job::repo as job_repo;
use phpyun_models::part::repo as part_repo;
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

pub fn is_single_kind(kind: i32) -> bool {
    matches!(kind, 10 | 11 | 12 | 16 | 17 | 19 | 23)
}

fn csv_has(raw: &str, key: &str) -> bool {
    raw.split(',').any(|s| s.trim() == key)
}

fn cents_from_yuan(yuan: i64) -> i32 {
    i32::try_from(yuan.saturating_mul(100)).unwrap_or(i32::MAX)
}

fn cents_from_yuan_f64(yuan: f64) -> i32 {
    if !yuan.is_finite() || yuan <= 0.0 {
        return 0;
    }
    (yuan * 100.0).round().clamp(0.0, f64::from(i32::MAX)) as i32
}

#[derive(Debug, Clone, PartialEq)]
pub enum SinglePurchase {
    Free,
    NeedConfirm { price: f64, integral: i64 },
    Integral { integral: i64 },
    Cash,
}

#[derive(Debug, Clone)]
pub struct PurchaseCtx {
    pub online: i32,
    pub single_can: String,
    pub only_price: String,
    pub proportion: f64,
}

impl PurchaseCtx {
    pub fn allows(&self, key: &str) -> bool {
        csv_has(&self.single_can, key)
    }

    pub fn decide(
        &self,
        key: &str,
        price_yuan: f64,
        confirm: bool,
        deny: &str,
    ) -> AppResult<SinglePurchase> {
        SinglePurchase::decide(
            self.online,
            &self.single_can,
            &self.only_price,
            key,
            price_yuan,
            self.proportion,
            confirm,
            deny,
        )
    }
}

impl SinglePurchase {
    pub fn decide(
        online: i32,
        single_can: &str,
        only_price: &str,
        key: &str,
        price_yuan: f64,
        proportion: f64,
        confirm: bool,
        deny: &str,
    ) -> AppResult<Self> {
        let price = if price_yuan.is_finite() {
            price_yuan
        } else {
            0.0
        };
        if price <= 0.0 {
            return Ok(Self::Free);
        }
        let pro = if proportion.is_finite() && proportion > 0.0 {
            proportion
        } else {
            0.0
        };
        let integral = (price * pro).round() as i64;
        if !confirm {
            return Ok(Self::NeedConfirm {
                price,
                integral,
            });
        }
        let integral_mode = online == 3 && !csv_has(only_price, key);
        if integral_mode {
            if integral <= 0 {
                return Err(ApiError::business("integral_insufficient"));
            }
            return Ok(Self::Integral { integral });
        }
        if csv_has(single_can, key) {
            return Ok(Self::Cash);
        }
        Err(ApiError::business(deny.to_string()))
    }
}

pub async fn load_ctx(state: &AppState) -> PurchaseCtx {
    let online = crate::site_gate_service::config_str(state, "com_integral_online")
        .await
        .trim()
        .parse()
        .unwrap_or(0);
    let proportion = crate::site_gate_service::config_str(state, "integral_proportion")
        .await
        .trim()
        .parse()
        .unwrap_or(0.0);
    PurchaseCtx {
        online,
        single_can: crate::site_gate_service::config_str(state, "com_single_can").await,
        only_price: crate::site_gate_service::config_str(state, "sy_only_price").await,
        proportion,
    }
}

pub async fn unit_price(state: &AppState, key: &str) -> f64 {
    crate::site_gate_service::config_str(state, key)
        .await
        .trim()
        .parse()
        .ok()
        .filter(|v: &f64| v.is_finite())
        .unwrap_or(0.0)
        .max(0.0)
}

pub async fn create_kind_order(
    state: &AppState,
    user: &AuthenticatedUser,
    kind: i32,
    sid: u64,
    remark: &str,
    yuan: f64,
    channel: &str,
    order_info_json: &str,
) -> AppResult<String> {
    user.require_employer()?;
    if !is_single_kind(kind) {
        return Err(ApiError::param_invalid("type"));
    }
    let now = clock::now_ts();
    Ok(vip_repo::create_single_order(
        state.db.pool(),
        user.uid,
        i32::from(user.usertype),
        kind,
        sid,
        remark,
        cents_from_yuan_f64(yuan),
        channel,
        order_info_json,
        now,
    )
    .await?)
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

#[derive(Deserialize)]
struct PromoteInfo {
    jobid: u64,
    days: i32,
    kind: String,
}

pub async fn settle_promote(state: &AppState, order: &AnyOrder, pay_tx_id: &str) -> AppResult<()> {
    let now = clock::now_ts();
    let n = vip_repo::mark_single_paid(state.db.pool(), &order.order_no, pay_tx_id, now).await?;
    if n == 0 {
        return Ok(());
    }
    let info: PromoteInfo = serde_json::from_str(&order.order_info)
        .map_err(|_| ApiError::param_invalid("order_info"))?;
    let kind = match info.kind.as_str() {
        "top" | "rec" | "urgent" => info.kind.as_str(),
        _ => return Err(ApiError::param_invalid("order_info")),
    };
    if info.jobid == 0 || !(1..=365).contains(&info.days) {
        return Err(ApiError::param_invalid("order_info"));
    }
    let _ = job_repo::apply_member_promote(
        state.db.pool(),
        info.jobid,
        order.uid,
        kind,
        info.days,
        now,
    )
    .await?;
    crate::job_service::invalidate_job(state, info.jobid).await;
    Ok(())
}

#[derive(Deserialize)]
struct RefreshJobsInfo {
    #[serde(default)]
    job_ids: Vec<u64>,
}

pub async fn settle_refresh_jobs(
    state: &AppState,
    order: &AnyOrder,
    pay_tx_id: &str,
) -> AppResult<()> {
    let now = clock::now_ts();
    let n = vip_repo::mark_single_paid(state.db.pool(), &order.order_no, pay_tx_id, now).await?;
    if n == 0 {
        return Ok(());
    }
    let info: RefreshJobsInfo = serde_json::from_str(&order.order_info)
        .map_err(|_| ApiError::param_invalid("order_info"))?;
    let ids: Vec<u64> = info
        .job_ids
        .into_iter()
        .filter(|id| *id > 0)
        .take(100)
        .collect();
    if ids.is_empty() {
        return Err(ApiError::param_invalid("order_info"));
    }
    let _ = job_repo::refresh_ids(state.db.pool(), &ids, order.uid, now).await?;
    crate::job_service::invalidate_jobs(state, &ids).await;
    Ok(())
}

#[derive(Deserialize)]
struct RefreshPartInfo {
    #[serde(default)]
    ids: Vec<u64>,
}

pub async fn settle_refresh_part(
    state: &AppState,
    order: &AnyOrder,
    pay_tx_id: &str,
) -> AppResult<()> {
    let now = clock::now_ts();
    let n = vip_repo::mark_single_paid(state.db.pool(), &order.order_no, pay_tx_id, now).await?;
    if n == 0 {
        return Ok(());
    }
    let info: RefreshPartInfo = serde_json::from_str(&order.order_info)
        .map_err(|_| ApiError::param_invalid("order_info"))?;
    let ids: Vec<u64> = info.ids.into_iter().filter(|id| *id > 0).take(100).collect();
    if ids.is_empty() {
        return Err(ApiError::param_invalid("order_info"));
    }
    for id in ids {
        let _ = part_repo::refresh_for_com(state.db.pool(), id, order.uid, now).await?;
    }
    crate::part_service::invalidate_list();
    Ok(())
}

pub fn promote_meta(kind: &str) -> AppResult<(i32, &'static str, &'static str)> {
    match kind {
        "top" => Ok((10, "jobtop", "integral_job_top")),
        "rec" => Ok((12, "jobrec", "com_recjob")),
        "urgent" => Ok((11, "joburgent", "com_urgent")),
        _ => Err(ApiError::param_invalid("kind")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_whitelist() {
        assert_eq!(pay_channel(None).unwrap(), "wechat");
        assert_eq!(pay_channel(Some("")).unwrap(), "wechat");
        assert_eq!(pay_channel(Some("wechat")).unwrap(), "wechat");
        assert_eq!(pay_channel(Some("alipay")).unwrap(), "alipay");
        assert!(pay_channel(Some("bank")).is_err());
    }

    #[test]
    fn decide_branches() {
        let d = |online, can, only, price, pro, confirm| {
            SinglePurchase::decide(
                online, can, only, "sxjob", price, pro, confirm, "need_buy",
            )
        };
        assert_eq!(d(1, "sxjob", "", 0.0, 1.0, true).unwrap(), SinglePurchase::Free);
        assert_eq!(
            d(1, "sxjob", "", 10.0, 2.0, false).unwrap(),
            SinglePurchase::NeedConfirm {
                price: 10.0,
                integral: 20
            }
        );
        assert_eq!(
            d(3, "sxjob", "", 10.0, 1.0, true).unwrap(),
            SinglePurchase::Integral { integral: 10 }
        );
        assert!(d(3, "sxjob", "", 10.0, 0.0, true).is_err());
        assert_eq!(d(1, "sxjob", "", 10.0, 1.0, true).unwrap(), SinglePurchase::Cash);
        assert!(d(1, "", "", 10.0, 1.0, true).is_err());
        assert_eq!(
            d(3, "sxjob", "sxjob", 10.0, 1.0, true).unwrap(),
            SinglePurchase::Cash
        );
        assert!(is_single_kind(16) && is_single_kind(10) && !is_single_kind(5));
    }
}
