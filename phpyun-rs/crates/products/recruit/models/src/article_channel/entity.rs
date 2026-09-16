use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ArticleChannelSub {
    pub uid: u64,
    pub group_ids: String,
    pub updated_at: i64,
}
