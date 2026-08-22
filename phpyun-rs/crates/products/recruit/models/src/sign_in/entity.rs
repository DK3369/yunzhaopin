use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SignIn {
    pub id: u64,
    pub uid: u64,
    pub date_ymd: u32,
    pub client_ip: String,
    pub reward: u32,
    pub created_at: i64,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserSign {
    pub uid: u64,
    pub signday: u32,
    pub signdays: u32,
    pub last_date_ymd: u32,
    pub updated_at: i64,
}
