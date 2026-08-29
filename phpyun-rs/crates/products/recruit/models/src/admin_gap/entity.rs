//! Admin rows for PHP tables that did not yet have an admin repo.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserPhotoRow {
    pub uid: u64,
    pub name: String,
    pub username: String,
    pub sex: i32,
    pub photo: String,
    pub photo_status: i32,
    #[sqlx(skip)]
    #[serde(default)]
    pub username_n: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserCertRow {
    pub uid: u64,
    pub name: String,
    pub idcard: String,
    pub idcard_pic: String,
    pub idcard_status: i32,
    pub cert_time: i64,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserMsgRow {
    pub id: u64,
    pub uid: u64,
    pub username: String,
    pub job_name: String,
    pub com_name: String,
    pub content: String,
    pub reply: String,
    pub datetime: i64,
    pub reply_time: i64,
    pub status: i32,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct MemberLogRow {
    pub id: u64,
    pub uid: u64,
    pub opera: i32,
    pub r#type: i32,
    pub usertype: i32,
    pub content: String,
    pub ip: String,
    pub ctime: i64,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CompanyPhotoRow {
    pub uid: u64,
    pub name: String,
    pub logo: String,
    pub logo_status: i32,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct GalleryAdminRow {
    pub id: u64,
    pub uid: u64,
    pub title: String,
    pub picurl: String,
    pub status: i32,
    pub sort: i32,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CompanyContentAdminRow {
    pub id: u64,
    pub uid: u64,
    pub title: String,
    pub status: i32,
    pub statusbody: String,
    pub ctime: i64,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct InterviewAdminRow {
    pub id: u64,
    pub uid: u64,
    pub title: String,
    pub fname: String,
    pub jobname: String,
    pub content: String,
    pub datetime: i64,
    pub is_browse: i32,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CompanyStatisAdminRow {
    pub uid: u64,
    pub com_name: String,
    pub rating: i32,
    pub rating_name: String,
    pub integral: String,
    pub vip_stime: i64,
    pub vip_etime: i64,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct JobRefreshLogRow {
    pub id: u64,
    pub uid: u64,
    pub jobid: u64,
    pub usertype: i32,
    pub r#type: i32,
    pub r_time: String,
    pub ip: String,
    pub remark: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct HotKeyAdminRow {
    pub id: u64,
    pub key_name: String,
    pub num: i32,
    pub r#type: i32,
    pub check: i32,
    pub bold: i32,
    pub tuijian: i32,
    pub color: String,
    pub size: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CronRow {
    pub id: u64,
    pub name: String,
    pub dir: String,
    pub r#type: i32,
    pub week: i32,
    pub month: i32,
    pub hour: i32,
    pub minute: i32,
    pub display: i32,
    pub nowtime: i64,
    pub nexttime: i64,
    #[sqlx(skip)]
    #[serde(default)]
    pub nowtime_n: String,
    #[sqlx(skip)]
    #[serde(default)]
    pub nexttime_n: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ErrorLogRow {
    pub id: u64,
    pub uid: i64,
    pub r#type: i32,
    pub content: String,
    pub ctime: i64,
    pub isread: i32,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SysmsgAdminRow {
    pub id: u64,
    pub fa_uid: u64,
    pub username: String,
    pub content: String,
    pub usertype: i32,
    pub ctime: i64,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct NavmapRow {
    pub id: u64,
    pub nid: i32,
    pub name: String,
    pub url: String,
    pub sort: i32,
    pub display: i32,
    pub eject: i32,
    pub r#type: i32,
    pub furl: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SpecialComAdminRow {
    pub id: u64,
    pub sid: u64,
    pub uid: u64,
    pub integral: i32,
    pub status: i32,
    pub statusbody: String,
    pub sort: i32,
    pub famous: i32,
    pub created_at: i64,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct WxQrcodeRow {
    pub id: u64,
    pub wxloginid: String,
    pub ticket: String,
    pub time: i64,
    pub status: i32,
    pub wxid: String,
    pub uid: u64,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct WxpubTempRow {
    pub id: u64,
    pub title: String,
    pub header: String,
    pub body: String,
    pub footer: String,
    pub r#type: String,
    pub temptype: i32,
    pub time: i64,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct OutsideRow {
    pub id: u64,
    pub name: String,
    pub r#type: String,
    pub titlelen: i32,
    pub infolen: i32,
    pub num: i32,
    pub code: String,
    pub lasttime: i64,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct HrLogRow {
    pub id: u64,
    pub uid: u64,
    pub job: i32,
    pub lookjob: i32,
    pub lookresume: i32,
    pub sqjob: i32,
    pub yq: i32,
    pub login: i32,
    pub ctime: i64,
    pub uptime: i64,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DomainAdminRow {
    pub id: u64,
    pub title: String,
    pub domain: String,
    pub fz_type: i32,
    pub mode: i32,
    pub web_title: String,
    pub indexdir: String,
    pub style: String,
    pub hy: i32,
    pub cityid: i32,
    pub province: i32,
    pub tpl: String,
    #[sqlx(skip)]
    #[serde(default)]
    pub name: String,
    #[sqlx(skip)]
    #[serde(default)]
    pub city: String,
    #[sqlx(skip)]
    #[serde(default)]
    pub hy_n: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DomainAdminUserRow {
    pub uid: u64,
    pub username: String,
    pub name: String,
    pub m_id: i32,
    pub did: u64,
    pub status: i32,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct LastMsgAt {
    pub title: String,
    pub ctime: i64,
}

/// PHP `phpyun_company_rating` admin row (VIP package / 套餐大表单).
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct RatingPackageRow {
    pub id: u64,
    pub name: String,
    pub service_price: String,
    pub integral_buy: String,
    pub yh_price: String,
    pub yh_integral: String,
    pub time_start: i64,
    pub time_end: i64,
    pub resume: i32,
    pub job_num: i32,
    pub interview: i32,
    pub editjob_num: i32,
    pub breakjob_num: i32,
    pub sort: i32,
    pub display: i32,
    pub explains: String,
    pub com_pic: String,
    #[sqlx(rename = "type")]
    #[serde(rename = "type")]
    pub r#type: i32,
    pub category: i32,
    pub service_time: i32,
    pub zph_num: i32,
    pub service_discount: i32,
    pub top_num: i32,
    pub urgent_num: i32,
    pub rec_num: i32,
    pub freelook_num: i32,
    pub freerefresh_num: i32,
    pub suspend_num: i32,
    pub max_time: i32,
    #[sqlx(skip)]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub time: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoStat {
    #[serde(rename = "numAll")]
    pub num_all: u64,
    #[serde(rename = "numAudited")]
    pub num_audited: u64,
    #[serde(rename = "numUnaudited")]
    pub num_unaudited: u64,
    #[serde(rename = "numFailed", skip_serializing_if = "Option::is_none")]
    pub num_failed: Option<u64>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct BannerAdminRow {
    pub id: u64,
    pub uid: u64,
    pub pic: String,
    pub status: i32,
    pub statusbody: String,
    pub name: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct BizLogRow {
    pub id: u64,
    pub uid: u64,
    pub comid: u64,
    pub eid: u64,
    pub jobid: u64,
    pub username: String,
    pub com_name: String,
    pub com_username: String,
    pub job_name: String,
    pub telphone: String,
    pub datetime: i64,
    pub is_browse: i32,
    pub isdel_n: String,
    pub status: i32,
    pub title: String,
    pub ip: String,
    pub remark: String,
    pub pic: String,
    #[sqlx(skip)]
    #[serde(default)]
    pub username_n: String,
    #[sqlx(skip)]
    #[serde(default)]
    pub datetime_n: String,
    #[sqlx(skip)]
    #[serde(default)]
    pub datetime_n_n: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct RatingServiceRow {
    pub id: u64,
    pub name: String,
    pub display: i32,
    pub sort: i32,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct RatingServiceDetailRow {
    pub id: u64,
    pub service_price: String,
    pub resume: i32,
    pub interview: i32,
    pub job_num: i32,
    pub breakjob_num: i32,
    pub part_num: i32,
    pub breakpart_num: i32,
    pub lt_job_num: i32,
    pub lt_breakjob_num: i32,
    pub lt_resume: i32,
    #[sqlx(rename = "type")]
    #[serde(rename = "type")]
    pub r#type: i32,
    pub sort: i32,
    pub zph_num: i32,
    pub top_num: i32,
    pub rec_num: i32,
    pub urgent_num: i32,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CronLogRow {
    pub id: u64,
    pub cid: String,
    pub ctime: i64,
    pub name: String,
    #[sqlx(skip)]
    #[serde(default)]
    pub ctime_n: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct MarketingExportRow {
    pub uid: u64,
    pub username: String,
    pub email: String,
    pub moblie: String,
}
