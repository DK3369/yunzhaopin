use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

/// PHP `phpyun_email_msg` (emaillog). JSON names match table columns.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct EmailMsg {
    pub id: u64,
    pub uid: i32,
    pub name: String,
    pub cuid: i32,
    pub cname: String,
    pub email: String,
    pub title: String,
    pub content: String,
    pub ctime: i64,
    pub state: i32,
    pub smtpserver: String,
    pub del: i32,
}
