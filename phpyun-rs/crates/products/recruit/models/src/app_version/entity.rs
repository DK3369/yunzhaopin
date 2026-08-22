use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AppVersion {
    pub id: u64,
    pub platform: String,
    pub version: String,
    pub version_code: u32,
    pub is_force: i32,
    pub download_url: String,
    pub changelog: String,
    pub status: i32,
    pub released_at: i64,
    pub created_at: i64,
}
