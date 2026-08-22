use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Full set of fields from `phpyun_news_base` + JOIN `phpyun_news_group`
/// + JOIN `phpyun_news_content`.
///
/// A single PHPYun news row consists of:
/// - `phpyun_news_base`: 18 columns (metadata)
/// - `phpyun_news_content`: one column for the body (linked by `nbid`)
/// - `phpyun_news_group`: category dictionary (linked by `nid`)
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Article {
    // ==== phpyun_news_base main table ====
    pub id: u64,
    /// PHPYun `nid`: category foreign key (-> phpyun_news_group.id)
    pub nid: i32,
    /// Category name (JOIN news_group.name; "" when absent)
    #[sqlx(default)]
    pub category: String,
    /// PHPYun `did`: sub-site id (0 = global, -1 = cross-site push)
    #[sqlx(default)]
    pub did: u64,
    pub title: String,
    /// PHPYun `color`: foreground color used in list rendering
    #[sqlx(default)]
    pub color: Option<String>,
    /// PHPYun `keyword`: SEO keywords (CSV)
    #[sqlx(default)]
    pub keyword: String,
    pub author: String,
    /// PHPYun `datetime`: publish time (unix)
    pub published_at: i64,
    pub hits: i64,
    /// PHPYun `describe`: attribute CSV (e.g., "1,2,3" -> bitmask for
    /// recommended/headline/sticky etc.)
    #[sqlx(default)]
    pub describe: String,
    /// PHPYun `description`: summary (<=255)
    pub summary: String,
    /// PHPYun `newsphoto`: cover image relative path
    pub cover: String,
    /// PHPYun `s_thumb`: small thumbnail (used to speed up list loading)
    #[sqlx(default)]
    pub s_thumb: Option<String>,
    /// PHPYun `source`: source attribution
    #[sqlx(default)]
    pub source: Option<String>,
    /// PHPYun `sort`: sort weight
    #[sqlx(default)]
    pub sort: i32,
    /// PHPYun `lastupdate`: last update time (unix)
    #[sqlx(default)]
    pub lastupdate: i64,
    /// PHPYun `starttime`: publish-on time
    #[sqlx(default)]
    pub starttime: i64,
    /// PHPYun `endtime`: publish-off time
    #[sqlx(default)]
    pub endtime: i64,
    /// Body content joined in for detail queries
    pub content: Option<String>,
    /// Compatibility for legacy DTO: keep `rec` / `status` fields, derived
    /// from the describe CSV. `rec=1` means describe contains "1".
    #[sqlx(default)]
    pub rec: i32,
    /// PHPYun news_base has no status column; always 1.
    #[sqlx(default)]
    pub status: i32,
}
