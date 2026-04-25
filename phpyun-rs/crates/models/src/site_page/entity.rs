use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SitePage {
    pub code: String,
    pub title: String,
    pub content: String,
    pub updated_at: i64,
}
