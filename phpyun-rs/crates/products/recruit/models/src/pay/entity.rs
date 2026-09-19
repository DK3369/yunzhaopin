//! Pay gateway rows: merchant, method, order.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PayMerchant {
    pub id: u64,
    pub code: String,
    pub name: String,
    pub api_key: String,
    pub api_secret: String,
    pub notify_url: String,
    pub return_url: String,
    pub status: String,
    pub ctime: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PayMethod {
    pub id: u64,
    pub merchant_id: u64,
    pub code: String,
    pub name: String,
    pub status: String,
    pub config_json: String,
    pub sort: i32,
    pub ctime: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PayOrder {
    pub id: u64,
    pub pay_no: String,
    pub merchant_id: u64,
    pub merchant_order_no: String,
    pub method_code: String,
    pub amount_cents: i32,
    pub currency: String,
    pub status: String,
    pub channel_ref: String,
    pub pay_url: String,
    pub subject: String,
    pub paid_at: i64,
    pub ctime: i64,
    pub updated_at: i64,
    pub extra_json: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PayOrderListRow {
    pub id: u64,
    pub pay_no: String,
    pub merchant_id: u64,
    pub merchant_code: String,
    pub merchant_name: String,
    pub merchant_order_no: String,
    pub method_code: String,
    pub amount_cents: i32,
    pub currency: String,
    pub status: String,
    pub channel_ref: String,
    pub subject: String,
    pub paid_at: i64,
    pub ctime: i64,
}

pub struct MerchantWrite<'a> {
    pub code: &'a str,
    pub name: &'a str,
    pub api_key: &'a str,
    pub api_secret: &'a str,
    pub notify_url: &'a str,
    pub return_url: &'a str,
    pub status: &'a str,
}

pub struct MethodWrite<'a> {
    pub merchant_id: u64,
    pub code: &'a str,
    pub name: &'a str,
    pub status: &'a str,
    pub config_json: &'a str,
    pub sort: i32,
}

pub struct OrderInsert<'a> {
    pub pay_no: &'a str,
    pub merchant_id: u64,
    pub merchant_order_no: &'a str,
    pub method_code: &'a str,
    pub amount_cents: i32,
    pub currency: &'a str,
    pub subject: &'a str,
}
