use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SearchHistory {
    pub id: u64,
    pub uid: u64,
    pub scope: String,
    pub keyword: String,
    pub created_at: i64,
}
