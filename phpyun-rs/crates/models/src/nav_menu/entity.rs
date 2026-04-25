use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct NavMenu {
    pub id: u64,
    pub position: String,
    pub label: String,
    pub url: String,
    pub icon: String,
    pub parent_id: u64,
    pub sort: i32,
    pub status: i32,
    pub updated_at: i64,
}
