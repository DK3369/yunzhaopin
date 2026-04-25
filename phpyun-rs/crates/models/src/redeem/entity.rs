use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct RedeemClass {
    pub id: u64,
    pub parent_id: u64,
    pub name: String,
    pub sort: i32,
    pub created_at: i64,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Reward {
    pub id: u64,
    pub name: String,
    pub pic: String,
    pub content: String,
    pub integral: u32,
    pub stock: u32,
    pub sold: u32,
    pub restriction: u32,
    pub nid: u64,
    pub tnid: u64,
    pub status: i32,
    pub is_rec: i32,
    pub is_hot: i32,
    pub created_at: i64,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct RedeemOrder {
    pub id: u64,
    pub uid: u64,
    pub gid: u64,
    pub name: String,
    pub linkman: String,
    pub linktel: String,
    pub address: String,
    pub integral: u32,
    pub num: u32,
    pub status: i32,
    pub created_at: i64,
}
