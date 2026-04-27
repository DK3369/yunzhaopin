//! Plain-data view types for `PartJob`.
//!
//! Dict-aware constructor `part_summary_from_dict` lives in
//! `phpyun_handlers::v1::wap::part`.

use serde::Serialize;
use utoipa::ToSchema;

use super::entity::PartJob;

/// Part-time list item — aligned with PHP `wap/part::index_action` full-field
/// output + dict translation + time formatting. Shape is shared between
/// `wap/part` and `mcenter/part::ComPartSummary`.
#[derive(Debug, Serialize, ToSchema)]
pub struct PartSummary {
    // ==== Full 33 columns of master table phpyun_partjob ====
    pub id: u64,
    pub uid: u64,
    pub name: String,
    pub com_name: Option<String>,
    /// Part-time category id (PHP `type`)
    pub part_type: i32,
    /// Part-time category name (dict resolve_part)
    pub part_type_n: String,

    pub province_id: i32,
    pub province_name: String,
    pub city_id: i32,
    pub city_name: String,
    pub three_city_id: i32,
    pub three_city_name: String,
    pub address: Option<String>,

    pub number: i32,
    pub sex: i32,
    pub salary: i32,
    pub salary_type: i32,
    pub salary_type_n: String,
    pub billing_cycle: i32,
    pub billing_cycle_n: String,
    pub worktime: Option<String>,

    pub sdate: i64,
    pub sdate_n: String,
    pub edate: i64,
    pub edate_n: String,
    pub addtime: i64,
    pub addtime_n: String,
    pub lastupdate: i64,
    pub lastupdate_n: String,
    pub deadline: i64,
    pub deadline_n: String,
    pub upstatus_time: i64,
    pub upstatus_count: i32,

    pub content: Option<String>,
    pub linkman: Option<String>,
    pub linktel: Option<String>,

    pub state: i32,
    pub status: i32,
    pub r_status: i32,
    pub rec_time: i64,
    pub did: u32,

    pub x: Option<String>,
    pub y: Option<String>,
    pub hits: i64,

    /// 1 means currently within the pinned period (`rec_time > now`)
    pub is_rec: bool,
    /// 0 = long-term recruitment (`edate == 0`)
    pub is_long_term: bool,
}

/// Backward-compatible / no-i18n constructor — leaves dict-translated names empty.
impl From<PartJob> for PartSummary {
    fn from(j: PartJob) -> Self {
        Self {
            id: j.id,
            uid: j.uid,
            name: j.name,
            com_name: j.com_name,
            part_type: j.r#type,
            part_type_n: String::new(),
            province_id: j.provinceid,
            province_name: String::new(),
            city_id: j.cityid,
            city_name: String::new(),
            three_city_id: j.three_cityid,
            three_city_name: String::new(),
            address: j.address,
            number: j.number,
            sex: j.sex,
            salary: j.salary,
            salary_type: j.salary_type,
            salary_type_n: String::new(),
            billing_cycle: j.billing_cycle,
            billing_cycle_n: String::new(),
            worktime: j.worktime,
            sdate_n: fmt_date(j.sdate),
            sdate: j.sdate,
            edate_n: fmt_date(j.edate),
            edate: j.edate,
            addtime_n: fmt_dt(j.addtime),
            addtime: j.addtime,
            lastupdate_n: fmt_dt(j.lastupdate),
            lastupdate: j.lastupdate,
            deadline_n: fmt_dt(j.deadline),
            deadline: j.deadline,
            upstatus_time: j.upstatus_time,
            upstatus_count: j.upstatus_count,
            content: j.content,
            linkman: j.linkman,
            linktel: j.linktel,
            state: j.state,
            status: j.status,
            r_status: j.r_status,
            is_rec: false,
            rec_time: j.rec_time,
            did: j.did,
            x: j.x,
            y: j.y,
            hits: j.hits,
            is_long_term: j.edate == 0,
        }
    }
}

fn fmt_date(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_default()
}
fn fmt_dt(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}
