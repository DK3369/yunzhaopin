use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: u64,
    pub actor_uid: Option<u64>,
    pub actor_ip: String,
    pub actor_ua: String,
    pub action: String,
    pub target: String,
    pub success: i32,
    pub meta: Option<String>,
    pub created_at: i64,
}
