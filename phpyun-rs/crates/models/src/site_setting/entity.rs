use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SiteSetting {
    pub key_name: String,
    pub value: String,
    pub description: String,
    pub is_public: i32,
    pub updated_at: i64,
}
