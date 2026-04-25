use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct HotSearch {
    pub id: u64,
    pub scope: String,
    pub keyword: String,
    pub hits: i32,
    pub last_hit_at: i64,
}
