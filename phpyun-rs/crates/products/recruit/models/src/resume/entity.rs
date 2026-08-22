//! Core fields of the `phpyun_resume` table (PHPYun has 50+ columns in total; this covers the ones used by the WAP detail page).

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Resume {
    #[sqlx(try_from = "i32")]
    pub uid: u64,
    pub name: Option<String>,
    /// 1 = real name public / 2 = real name hidden
    #[sqlx(default)]
    pub nametype: i32,
    /// 0 = unset / 1 = male / 2 = female
    #[sqlx(default)]
    pub sex: i32,
    /// Birthday in YYYY-MM-DD
    pub birthday: Option<String>,
    /// Marital status: 0 = unset / 1 = single / 2 = married
    #[sqlx(default)]
    pub marriage: i32,
    /// Education dictionary id (PHPYun column name is `edu`, mapped via SELECT `edu AS education`)
    #[sqlx(default)]
    pub education: i32,
    /// Contact phone number
    pub telphone: Option<String>,
    /// Home phone number
    #[serde(default)]
    pub telhome: Option<String>,
    pub email: Option<String>,
    /// Avatar (PHPYun uses the `photo` column)
    pub photo: Option<String>,
    /// Avatar type: 0 = not verified / 1 = verified
    #[sqlx(default)]
    pub phototype: i32,
    /// Resume display status: 1 = public / 2 = hidden / 3 = visible only to companies applied to
    #[sqlx(default)]
    pub status: i32,
    /// Account status (review workflow)
    #[sqlx(default)]
    pub r_status: i32,
    /// Default job intent id (points to phpyun_resume_expect.id)
    #[sqlx(default)]
    pub def_job: i32,
    #[sqlx(default)]
    pub lastupdate: i64,

    // ---- New: personal info ----
    /// Height
    #[serde(default)]
    pub height: Option<String>,
    /// Weight
    #[serde(default)]
    pub weight: Option<String>,
    /// Nationality
    #[serde(default)]
    pub nationality: Option<String>,
    /// Current city of residence
    #[serde(default)]
    pub living: Option<String>,
    /// Place of household registration
    #[serde(default)]
    pub domicile: Option<String>,
    /// Personal homepage
    #[serde(default)]
    pub homepage: Option<String>,
    /// Address
    #[serde(default)]
    pub address: Option<String>,
    /// Self-introduction
    #[serde(default)]
    pub description: Option<String>,
    /// ID card number (masking is handled by the service layer)
    #[serde(default)]
    pub idcard: Option<String>,
    /// ID card images
    #[serde(default)]
    pub idcard_pic: Option<String>,
    /// ID card verification: 0 = not reviewed / 1 = approved
    #[sqlx(default)]
    pub idcard_status: i32,
    /// Mobile phone verification status
    #[sqlx(default)]
    pub moblie_status: i32,
    /// Email verification status
    #[sqlx(default)]
    pub email_status: i32,
    /// Total work experience dictionary id
    #[sqlx(default)]
    pub exp: i32,
    /// Profile/portrait photo
    #[serde(default)]
    pub resume_photo: Option<String>,
    /// QQ number
    #[serde(default)]
    pub qq: Option<String>,
    /// WeChat QR code image
    #[serde(default)]
    pub wxewm: Option<String>,
    /// Tags (CSV)
    #[serde(default)]
    pub tag: Option<String>,
    /// Custom tags (free text)
    #[serde(default)]
    pub label: Option<String>,
    /// Retirement information
    #[serde(default)]
    pub retire: Option<String>,
    /// Registration time
    #[sqlx(default)]
    pub resumetime: i64,
    /// Last login time (resume main account)
    #[sqlx(default)]
    pub login_date: i64,

    #[sqlx(try_from = "i32")]
    pub did: u64,
}
