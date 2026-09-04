//! `phpyun_userid_msg` — PHP interview invitations (`job.model.php::getYqmsList`).

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UseridMsg {
    #[sqlx(try_from = "i32")]
    pub id: u64,
    #[sqlx(try_from = "i32")]
    pub uid: u64,
    pub title: String,
    pub content: String,
    #[sqlx(try_from = "i32")]
    pub fid: u64,
    pub fname: String,
    #[sqlx(rename = "type")]
    pub r#type: i32,
    pub datetime: i64,
    pub is_browse: i32,
    pub address: String,
    pub intertime: String,
    pub linkman: String,
    pub linktel: String,
    #[sqlx(try_from = "i32")]
    pub jobid: u64,
    pub jobname: String,
    #[sqlx(try_from = "i32")]
    pub did: u32,
    pub x: String,
    pub y: String,
    pub mappic: String,
    pub isdel: i32,
    pub remark: String,
}
