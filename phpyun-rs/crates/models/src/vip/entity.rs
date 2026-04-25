use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct VipPackage {
    pub id: u32,
    pub code: String,
    pub name: String,
    pub target_usertype: i32,
    pub duration_days: i32,
    pub price_cents: i32,
    pub desc_json: Option<serde_json::Value>,
    pub is_active: i32,
    pub sort_order: i32,
    pub created_at: i64,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserVip {
    pub uid: u64,
    pub package_code: String,
    pub started_at: i64,
    pub expires_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PayOrder {
    pub id: u64,
    pub order_no: String,
    pub uid: u64,
    pub package_code: String,
    pub amount_cents: i32,
    pub channel: String,
    pub status: i32,
    pub pay_tx_id: Option<String>,
    pub created_at: i64,
    pub paid_at: i64,
}
