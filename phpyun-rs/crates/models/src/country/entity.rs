use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Curated major-country lookup row (`phpyun_country`).
///
/// Distinct from the global `phpyun_region` tree — this is a flat,
/// denormalized table used by UI selectors that need rich per-country
/// metadata (name_zh / phone_code / currency / flag) in one round trip.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Country {
    pub id: u64,
    pub code: String,
    pub code3: String,
    /// Decoded as i32 then narrowed — the source row may be a `phpyun_region`
    /// projection where this field doesn't exist and we synthesize `0`.
    #[sqlx(try_from = "i32")]
    pub numeric_code: u16,
    pub name_en: String,
    pub name_zh: String,
    pub continent: String,
    pub phone_code: String,
    pub currency: String,
    pub flag: String,
    pub sort: i32,
    pub status: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

pub const STATUS_DELETED: i32 = 2;
