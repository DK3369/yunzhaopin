use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ThirdData {
    pub id: u64,
    pub name: String,
    pub url: String,
    pub api_url: String,
    pub provider: String,
    pub sort: i32,
    pub enabled: i32,
    pub created_at: i64,
    pub updated_at: i64,
}
