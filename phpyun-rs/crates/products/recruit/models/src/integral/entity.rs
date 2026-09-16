use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct IntegralItem {
    pub id: u64,
    pub name: String,
    pub image: String,
    pub description: String,
    pub cost: u32,
    pub stock: i32,
    pub status: i32,
    pub created_at: i64,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct IntegralExchange {
    pub id: u64,
    pub uid: u64,
    pub item_id: u64,
    pub cost: u32,
    pub status: i32,
    pub created_at: i64,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserIntegral {
    pub uid: u64,
    pub balance: i64,
    pub updated_at: i64,
}

/// `phpyun_admin_integralclass` — points packages (state=1 is on sale).
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct IntegralClass {
    pub id: u64,
    pub integral: i32,
    pub discount: i32,
    pub state: i32,
}

/// `phpyun_company_card` — prepaid recharge card (PHP `cardOrder`).
#[derive(Debug, Clone, FromRow)]
pub struct PrepaidCard {
    pub id: u64,
    pub password: String,
    pub quota: i32,
    pub uid: u64,
}
