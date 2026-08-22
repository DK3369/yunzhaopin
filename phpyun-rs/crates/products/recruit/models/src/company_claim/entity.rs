use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CompanyClaim {
    pub id: u64,
    pub uid: u64,
    pub claimer_uid: u64,
    pub client_ip: String,
    pub created_at: i64,
}
