//! `phpyun_company_pay` — actual PHP integral / payment ledger.
//!
//! Schema (PHP truth):
//!   id, order_id, order_price, pay_time, pay_state, com_id, pay_remark,
//!   type, pay_type, did, eid, usertype, coupon_id
//!
//! `com_id` despite the name is the affected uid (jobseeker uid for
//! usertype=1, employer uid for usertype=2). Balance changes are tracked
//! by `phpyun_member_statis.integral` (not by accumulating this ledger).

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct IntegralTransfer {
    pub id: u64,
    /// Generated 18-char order id.
    pub order_id: String,
    /// Price of the transaction (PHPYun stores integral as a decimal). Use
    /// i64 cents-equivalent semantics is wrong here — keep as-is.
    pub order_price: f64,
    pub pay_time: i64,
    /// 1=pending, 2=success, ... (PHP `pay_state`).
    pub pay_state: i32,
    /// Affected uid (PHP `com_id`).
    pub com_id: u64,
    pub pay_remark: String,
    /// 1=integral, 2=packpay (PHP `type`).
    #[sqlx(rename = "type")]
    pub kind: i32,
    /// PHP `pay_type` — see integral.model.php docblock for the 1-100 mapping.
    pub pay_type: i32,
    pub did: i32,
    /// Optional resume-id for resume-download charges.
    pub eid: u64,
    /// 1=jobseeker, 2=employer.
    pub usertype: i32,
    pub coupon_id: u64,
}
