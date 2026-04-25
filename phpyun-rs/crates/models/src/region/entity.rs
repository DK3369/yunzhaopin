use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// One node in the global region tree.
///
/// Levels:
/// - `0` = country (parent_id NULL, code = ISO 3166-1 alpha-2)
/// - `1` = state / province / prefecture (code = ISO 3166-2, e.g. `CN-BJ` / `US-CA`)
/// - `2` = city
/// - `3` = district / county / ward
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Region {
    pub id: u64,
    pub parent_id: Option<u64>,
    pub country_code: String,
    pub code: String,
    pub level: i32,
    pub name: String,
    pub continent: Option<String>,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub sort: i32,
    pub status: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Levels expressed as named constants so callers don't sprinkle magic numbers.
pub const LEVEL_COUNTRY: i32 = 0;
pub const LEVEL_STATE: i32 = 1;
pub const LEVEL_CITY: i32 = 2;
pub const LEVEL_DISTRICT: i32 = 3;
