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
    pub statusbody: String,
    pub job_uid: u64,
    pub r#type: i32,
    #[sqlx(skip)]
    #[serde(default)]
    pub datetime_n: String,
    #[sqlx(skip)]
    #[serde(default)]
    pub reply_time_n: String,
    #[sqlx(skip)]
    #[serde(default)]
    pub com_url: String,
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

/// PHP `admin_memberlog::index` row after `getMemlogList(..., utype=admin)`.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PhpMemberLogListRow {
    pub id: u64,
    pub uid: u64,
    pub opera: i32,
    pub r#type: i32,
    pub usertype: i32,
    pub content: String,
    pub ip: String,
    pub ctime: i64,
    pub remoteport: i32,
    pub username: String,
    pub rname: String,
    pub eid: u64,
    pub comname: String,
    pub pid: u64,
    pub sub_n: String,
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

/// PHP `company_job_refresh_log::index` admin row after job/part JOIN.
#[derive(Debug, Clone, FromRow)]
pub struct PhpJobRefreshLogRow {
    pub id: u64,
    pub uid: u64,
    pub jobid: u64,
    pub usertype: i32,
    pub r#type: i32,
    pub r_time: i64,
    pub ip: String,
    pub remark: String,
    pub port: i32,
    pub job_name: String,
    pub com_name: String,
}

/// PHP `set_guanjianci::index_action` JSON: check/bold/tuijian are booleans for el-switch.
fn ser_flag_bool<S: serde::Serializer>(v: &i32, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_bool(*v != 0)
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct HotKeyAdminRow {
    pub id: u64,
    pub key_name: String,
    pub num: i32,
    pub r#type: i32,
    #[serde(serialize_with = "ser_flag_bool")]
    pub check: i32,
    #[serde(serialize_with = "ser_flag_bool")]
    pub bold: i32,
    #[serde(serialize_with = "ser_flag_bool")]
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
    #[sqlx(default)]
    #[serde(default)]
    pub username: String,
    #[sqlx(default)]
    #[serde(default)]
    pub usertype: i32,
    #[sqlx(default)]
    #[serde(default)]
    pub time_n: String,
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
    #[sqlx(default)]
    #[serde(default)]
    pub type_n: String,
    #[sqlx(default)]
    #[serde(default)]
    pub time_n: String,
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
    pub three_cityid: i32,
    pub r#type: i32,
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

/// PHP `company_cert::getCertStatist_action` / `msgNum::comCertNum`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComCertStat {
    #[serde(rename = "comCertAll")]
    pub com_cert_all: u64,
    #[serde(rename = "comCert1")]
    pub com_cert1: u64,
    #[serde(rename = "comCert2")]
    pub com_cert2: u64,
}

/// PHP `partjob::partNum_action` / `msgNum::partNum`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartStat {
    #[serde(rename = "partAllNum")]
    pub part_all_num: u64,
    #[serde(rename = "partStatusNum1")]
    pub part_status_num1: u64,
    #[serde(rename = "partStatusNum2")]
    pub part_status_num2: u64,
    #[serde(rename = "partStatusNum3")]
    pub part_status_num3: u64,
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

/// One `phpyun_userid_job` row for the member CRM "application records" tab.
/// Keeps raw `isdel` because PHP renders it as translated text, not a flag.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct MemberApplyRow {
    pub id: u64,
    pub uid: u64,
    pub com_id: u64,
    pub job_id: u64,
    pub com_name: String,
    pub job_name: String,
    pub datetime: i64,
    pub is_browse: i32,
    pub isdel: i32,
}

/// One `phpyun_userid_msg` row for the member CRM "interview invitations" tab.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct MemberInviteRow {
    pub id: u64,
    pub uid: u64,
    pub fid: u64,
    pub jobid: u64,
    pub fname: String,
    pub jobname: String,
    pub title: String,
    pub content: String,
    pub datetime: i64,
    pub is_browse: i32,
    pub isdel: i32,
}

/// One row of the 预约刷新 admin list: a `company_job` joined onto its
/// `phpyun_reserve_refresh` schedule. Times stay raw so the service can render
/// them the way PHP `subReserveJob` does.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ReserveJobRow {
    pub id: u64,
    pub uid: u64,
    pub name: String,
    pub com_name: String,
    pub reserve_status: i32,
    pub reserve_interval: i32,
    pub start_time: i64,
    pub end_time: i64,
    pub s_time: String,
    pub e_time: String,
}

/// The `phpyun_reserve_refresh` schedule of one job, for the "current setting"
/// dialog (PHP member-side `reserveInfo`).
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ReserveScheduleRow {
    pub status: i32,
    pub interval: i32,
    pub s_time: String,
    pub e_time: String,
    pub end_time: i64,
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

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AdminEmailRow {
    pub id: u64,
    pub smtpserver: String,
    pub smtpuser: String,
    pub smtppass: String,
    pub smtpport: String,
    pub smtpnick: String,
    pub default_flag: i32,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct NewsPropertyRow {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct WxZdKeywordRow {
    pub id: u64,
    pub title: String,
    pub keyword: String,
    pub content: String,
    pub time: i64,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct WxZdConRow {
    pub id: u64,
    pub kid: u64,
    pub msgtype: String,
    pub content: String,
    pub media_id: i32,
    pub sort: i32,
    pub time: i64,
}

/// One `phpyun_company_statis_detail` row — the per-company package ledger
/// behind admin `company::statisDetail`.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CompanyStatisDetailRow {
    pub id: u64,
    pub uid: u64,
    #[sqlx(rename = "type")]
    pub kind: i32,
    pub num: i64,
    pub detail: String,
    pub time: i64,
    pub uri: String,
    pub ip: String,
}

/// One selectable company skin from `phpyun_company_tpl`, for admin
/// `company::mcomtpl`.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CompanyTplRow {
    pub id: u64,
    pub name: String,
    pub url: String,
    pub pic: String,
    pub price: String,
    pub status: i32,
}

/// The `phpyun_company_statis` counters PHP snapshots into
/// `phpyun_company_statis_sub` when an admin suspends a company.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CompanyQuotaRow {
    pub rating: i64,
    pub rating_name: String,
    pub rating_type: i64,
    pub job_num: i64,
    pub breakjob_num: i64,
    pub down_resume: i64,
    pub invite_resume: i64,
    pub zph_num: i64,
    pub top_num: i64,
    pub urgent_num: i64,
    pub rec_num: i64,
    pub vip_stime: i64,
    pub vip_etime: i64,
    pub max_time: i64,
}

/// The subset of a `phpyun_company_statis_sub` row that a resume restores.
/// `sons_num` is deliberately absent: PHP zeroes it on suspend but never
/// snapshots or restores it.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CompanyQuotaSnapshotRow {
    pub id: u64,
    pub job_num: i64,
    pub breakjob_num: i64,
    pub down_resume: i64,
    pub invite_resume: i64,
    pub zph_num: i64,
    pub top_num: i64,
    pub urgent_num: i64,
    pub rec_num: i64,
}

/// The company columns a 推文 task row is built from (PHP
/// `wxpubtemp.model::addTwTask` `type = 2`).
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct TuiWenCompanyRow {
    pub uid: u64,
    pub name: String,
    /// `phpyun_company.lastupdate` is `varchar(10)` even though it holds a unix
    /// timestamp, and it lands in `wxpub_twtask.jobsdate` (`int`).
    pub lastupdate: String,
}
