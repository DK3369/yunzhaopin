use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Ad {
    pub id: u64,
    pub slot: String,
    pub title: String,
    pub image: String,
    pub link: String,
    pub weight: i32,
    pub start_at: i64,
    pub end_at: i64,
    pub status: i32,
    pub created_at: i64,
    /// phpyun_ad.target: 1=current window / 2=new window
    pub target: i32,
    /// phpyun_ad.pic_width (string; passed through to frontend as-is by PHP)
    pub pic_width: String,
    /// phpyun_ad.pic_height
    pub pic_height: String,
    /// phpyun_ad.pic_content (image alt text / alt attribute)
    pub pic_content: String,
}
