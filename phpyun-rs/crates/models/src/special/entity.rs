use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Special {
    pub id: u64,
    pub title: String,
    pub banner: String,
    pub description: String,
    pub body: String,
    pub start_at: i64,
    pub end_at: i64,
    pub status: i32,
    pub view_count: i32,
    pub created_at: i64,
    // ---- Extra fields from PHPYun `phpyun_special` ----
    /// Template name (PHP `tpl`)
    #[sqlx(default)]
    pub tpl: String,
    /// Background image
    #[sqlx(default)]
    pub background: String,
    /// Maximum company count (PHP `limit`)
    #[sqlx(default)]
    pub max_count: i32,
    /// Rating (PHP `rating`)
    #[sqlx(default)]
    pub rating: String,
    /// Company sign-up toggle (PHP `com_bm`)
    #[sqlx(default)]
    pub com_bm: i32,
    /// Required points (PHP `integral`)
    #[sqlx(default)]
    pub integral: i32,
    /// Sort order
    #[sqlx(default)]
    pub sort: i32,
    /// Introduction (PHP `intro`)
    #[sqlx(default)]
    pub intro: String,
    /// Mobile main image (PHP `wappic`)
    #[sqlx(default)]
    pub wappic: String,
    /// Mobile background (PHP `wapback`)
    #[sqlx(default)]
    pub wapback: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SpecialCompany {
    pub id: u64,
    pub sid: u64,
    pub uid: u64,
    pub sort: i32,
    pub status: i32,
    pub created_at: i64,
}
