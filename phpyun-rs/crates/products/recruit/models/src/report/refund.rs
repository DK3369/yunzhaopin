//! Refunds for upheld resume reports (PHP `yunying/report_resume`).
//!
//! When an employer downloads a resume, reports it, and the admin upholds the
//! report with 返还 (`datafh = 1`), PHP hands back whatever that download had
//! cost. The download charge is written by `compay.model::buyDownresume`, so
//! that function is the source of truth for how to find it again:
//!
//! - **Cash** → `phpyun_company_order` with `type = 19` ("下载简历"),
//!   `sid = <resume eid>`, `uid = <employer>`, and `order_remark` containing
//!   `wap_00451`. Refunded into `phpyun_company_statis.packpay`.
//! - **Points** → `phpyun_company_pay` with `type = 1`, `pay_type = 12`,
//!   `com_id = <employer>`, `eid = <resume eid>`. Refunded into
//!   `phpyun_company_statis.integral`.
//! - **Neither, but a download exists** → the download came out of the
//!   employer's quota, so `down_resume` gets one back.
//!
//! PHP asks "was this resume downloaded?" three different ways depending on
//! which button was pressed — `eid + uid + comid` for a single report,
//! `eid + comid` for 同步处理, and `eid + uid + comid + usertype = 2` for the
//! checkbox batch. We use the single-report form everywhere: it is the
//! narrowest of the three that all real download rows satisfy, since
//! `_downResume` writes `usertype = comusertype` (2 for an employer) and free
//! VIP views land in `freedown_resume` instead, where nothing was ever paid.
//!
//! Deliberate divergence from PHP on the points lookup: PHP additionally
//! requires `pay_remark LIKE 'wap_00451'` (single) or `LIKE 'wap_com_00042'`
//! (batch), but `buyDownresume` actually writes the remark as
//! `common_06449 + {integral_pricename} + common_01992`. Neither pattern can
//! ever match, so in PHP the points half of the refund is silently dead. We
//! drop the remark predicate — `type` + `pay_type` + `com_id` + `eid` already
//! identify the charge uniquely — so points are genuinely returned. The cash
//! lookup keeps its `order_remark` predicate, which does match what
//! `buyDownresume` writes.

use sqlx::MySqlPool;

/// PHP `company_order.type` for a resume download.
const ORDER_TYPE_DOWN_RESUME: i32 = 19;
/// PHP `company_pay.pay_type` for points spent on a download.
const PAY_TYPE_DOWN_RESUME: i32 = 12;

/// PHP `downresume.model::getDownResumeInfo(eid, uid, comid)` — did this
/// employer download this specific resume of this jobseeker?
pub async fn downloaded(
    pool: &MySqlPool,
    eid: u64,
    uid: u64,
    com_uid: u64,
) -> Result<bool, sqlx::Error> {
    let row: Option<(u64,)> = sqlx::query_as(
        "SELECT CAST(id AS UNSIGNED) FROM phpyun_down_resume \
         WHERE eid = ? AND uid = ? AND comid = ? LIMIT 1",
    )
    .bind(eid)
    .bind(uid)
    .bind(com_uid)
    .fetch_optional(pool)
    .await?;
    Ok(row.is_some())
}

/// PHP `downresume.model::getSimpleList(eid, comid IN (...))` — of these
/// employers, which ones downloaded this resume?
pub async fn downloaders_of(
    pool: &MySqlPool,
    eid: u64,
    com_uids: &[u64],
) -> Result<Vec<u64>, sqlx::Error> {
    if com_uids.is_empty() {
        return Ok(Vec::new());
    }
    let ph = vec!["?"; com_uids.len()].join(",");
    let sql = format!(
        "SELECT DISTINCT CAST(COALESCE(comid, 0) AS UNSIGNED) FROM phpyun_down_resume \
         WHERE eid = ? AND comid IN ({ph})"
    );
    let mut q = sqlx::query_as::<_, (u64,)>(&sql).bind(eid);
    for id in com_uids {
        q = q.bind(*id);
    }
    Ok(q.fetch_all(pool).await?.into_iter().map(|r| r.0).collect())
}

/// Cash paid for the download, if any.
pub async fn cash_paid(
    pool: &MySqlPool,
    com_uid: u64,
    eid: u64,
) -> Result<Option<f64>, sqlx::Error> {
    let row: Option<(f64,)> = sqlx::query_as(
        "SELECT COALESCE(order_price, 0) FROM phpyun_company_order \
         WHERE `type` = ? AND sid = ? AND uid = ? AND order_remark LIKE '%wap_00451%' \
         ORDER BY id DESC LIMIT 1",
    )
    .bind(ORDER_TYPE_DOWN_RESUME)
    .bind(eid)
    .bind(com_uid)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| r.0).filter(|p| *p > 0.0))
}

/// Points spent on the download, if any. Stored negative by
/// `company_invtal(..., auto: false)`, so we take the absolute value the way
/// PHP's `abs($v['order_price'])` does.
///
/// `order_price` is DECIMAL here (it is DOUBLE over in `company_order`), and
/// sqlx won't decode DECIMAL into `f64`, so the rounding happens in SQL and
/// comes back as a plain integer — which is what points are anyway.
pub async fn points_paid(
    pool: &MySqlPool,
    com_uid: u64,
    eid: u64,
) -> Result<Option<i64>, sqlx::Error> {
    let row: Option<(i64,)> = sqlx::query_as(
        "SELECT CAST(ROUND(ABS(COALESCE(order_price, 0))) AS SIGNED) \
         FROM phpyun_company_pay \
         WHERE `type` = 1 AND pay_type = ? AND com_id = ? AND eid = ? \
         ORDER BY id DESC LIMIT 1",
    )
    .bind(PAY_TYPE_DOWN_RESUME)
    .bind(com_uid)
    .bind(eid)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| r.0).filter(|p| *p > 0))
}

/// `phpyun_member.did` — the ledger row carries the site id the user belongs to.
pub async fn member_did(pool: &MySqlPool, uid: u64) -> Result<i64, sqlx::Error> {
    let row: Option<(i64,)> =
        sqlx::query_as("SELECT COALESCE(did, 0) FROM phpyun_member WHERE uid = ? LIMIT 1")
            .bind(uid)
            .fetch_optional(pool)
            .await?;
    Ok(row.map(|r| r.0).unwrap_or(0))
}
