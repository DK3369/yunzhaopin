use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

/// PHP `phpyun_moblie_msg` (messagelog). JSON names match table columns (`moblie` spelling kept).
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct MoblieMsg {
    pub id: u64,
    pub uid: i32,
    pub name: String,
    pub cuid: i32,
    pub cname: String,
    pub moblie: String,
    pub content: String,
    pub ctime: i64,
    pub state: i32,
    pub ip: String,
    pub del: i32,
    pub msgtype: i32,
}
