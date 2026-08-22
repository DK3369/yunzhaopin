use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Zph {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub banner: String,
    pub city_id: i32,
    pub address: String,
    pub start_at: i64,
    pub end_at: i64,
    pub status: i32,
    pub created_at: i64,
    // ---- Extra fields from PHPYun `phpyun_zhaopinhui` ----
    /// Job-fair sub-site sid (aligned with phpyun_zhaopinhui.sid)
    #[sqlx(default)]
    pub sid: i32,
    /// Hero image (PHPYun `pic`)
    #[sqlx(default)]
    pub pic: String,
    /// Province id
    #[sqlx(default)]
    pub province_id: i32,
    /// Transportation notes
    #[sqlx(default)]
    pub traffic: String,
    /// Contact phone
    #[sqlx(default)]
    pub phone: String,
    /// Organizer
    #[sqlx(default)]
    pub organizers: String,
    /// Attending contact person
    #[sqlx(default)]
    pub user: String,
    /// Official website URL
    #[sqlx(default)]
    pub weburl: String,
    /// Media support (rich text / JSON)
    #[sqlx(default)]
    pub media: String,
    /// Sponsorship packages (rich text / JSON)
    #[sqlx(default)]
    pub packages: String,
    /// Booth description (rich text / JSON)
    #[sqlx(default)]
    pub booth: String,
    /// Attendance notice
    #[sqlx(default)]
    pub participate: String,
    /// Booth layout image
    #[sqlx(default)]
    pub zwpic: String,
    /// Reserved field
    #[sqlx(default)]
    pub reserved: String,
    /// Mobile hero image
    #[sqlx(default)]
    pub is_themb_wap: String,
    /// Mobile banner
    #[sqlx(default)]
    pub banner_wap: String,
    /// Sort order
    #[sqlx(default)]
    pub sort: i32,
    /// Listing flag: 1=listed / 0=unlisted
    #[sqlx(default)]
    pub is_open: i32,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ZphCompany {
    pub id: u64,
    pub zid: u64,
    pub uid: u64,
    pub sort: i32,
    pub status: i32,
    pub created_at: i64,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ZphReservation {
    pub id: u64,
    pub zid: u64,
    pub uid: u64,
    pub job_ids: String,
    pub name: String,
    pub mobile: String,
    pub status: i32,
    pub created_at: i64,
}
