use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SubAccountRow {
    pub uid: u64,
    pub username: String,
    pub status: i32,
    pub login_date: i64,
}
