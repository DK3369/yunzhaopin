use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SeekerVipPack {
    pub id: u32,
    pub code: String,
    pub name: String,
    pub months: i32,
    pub price_cents: i32,
    pub chat: i32,
    pub resume_top: i32,
    pub tpl_all: i32,
    pub refresh_free: i32,
    pub sort: i32,
    pub display: i32,
    pub deleted: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

impl SeekerVipPack {
    pub fn duration_days(&self) -> i32 {
        self.months.saturating_mul(30).max(1)
    }

    pub fn flag(v: i32) -> bool {
        v != 0
    }
}
