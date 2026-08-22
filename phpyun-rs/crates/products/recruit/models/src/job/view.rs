//! Plain-data view types projected from `Job` rows.
//!
//! These structs intentionally hold dictionary-translated names (`job_one_n`,
//! `job_city_one`, etc.) and time-derived booleans (`is_rec`, `newtime`) but
//! the *constructors that resolve those names* live in the handler crate
//! because they need `phpyun_services::dict_service::LocalizedDicts`, which
//! depends on `phpyun_models` (cyclic if pushed here).
//!
//! Callers in handlers / services should prefer the free helpers
//! `job_summary_from_dict` / `job_summary_from_dict_fav` (defined in
//! `phpyun_handlers::v1::wap::jobs`) over `JobSummary::from(_)` — `From<Job>`
//! is provided here only for legacy / non-i18n call sites and leaves all
//! `*_n` fields empty.

use serde::Serialize;
use utoipa::ToSchema;

use super::entity::Job;

/// Job list item — aligned with the field set returned by PHPYun
/// `JobM::getList()`. **Shape is shared across `wap`, `mcenter` and `admin`**
/// so the front-end card component is identical regardless of the listing
/// context.
#[derive(Debug, Serialize, ToSchema)]
pub struct JobSummary {
    // Basics
    pub id: u64,
    pub uid: u64,
    pub name: String,
    pub com_name: Option<String>,
    pub com_logo: Option<String>,

    // Category ids
    pub job1: i32,
    pub job1_son: i32,
    pub job_post: i32,
    pub hy: i32,

    // Category names (PHP `job_one_n / job_two_n / job_three_n / job_hy`).
    // Filled by `job_summary_from_dict*`; left empty by `From<Job>`.
    pub job_one_n: String,
    pub job_two_n: String,
    pub job_three_n: String,
    pub job_hy: String,
    /// Three-level job category joined as "Frontend / Web Frontend / React Developer" (PHP `jobname`)
    pub jobname: String,

    // Location
    pub province_id: i32,
    pub city_id: i32,
    pub three_city_id: i32,
    /// Province name (PHP `job_city_one`)
    pub job_city_one: String,
    /// City name (PHP `job_city_two`)
    pub job_city_two: String,

    // Salary
    pub salary: i32,
    pub min_salary: i32,
    pub max_salary: i32,

    // Requirements
    pub exp: i32,
    /// Experience-tier name (PHP `comclass` dict, e.g. "5-10 年") —
    /// filled by `job_summary_from_dict*`; empty for `From<Job>`.
    pub exp_n: String,
    pub edu: i32,
    /// Education-level name (PHP `comclass` dict, e.g. "本科") —
    /// filled by `job_summary_from_dict*`; empty for `From<Job>`.
    pub edu_n: String,

    // Promotion status (computed from rec_time / urgent_time vs. now)
    pub rec: i32,
    pub urgent: i32,
    pub is_rec: bool,
    pub is_urgent: bool,
    pub rec_time: i64,
    pub urgent_time: i64,

    // Time
    pub sdate: i64,
    pub lastupdate: i64,
    /// Posted within the last 2 days (PHP `newtime`); requires `now` ts to
    /// derive — `From<Job>` leaves this `false`.
    pub newtime: bool,

    // Stats
    pub jobhits: i32,

    /// Whether the *current* user has favorited this job. Always `false` for
    /// unauthenticated requests / non-favorite contexts.
    pub is_favorited: bool,
}

/// Legacy / non-i18n constructor: leaves dictionary-translated fields empty
/// (`job_one_n`, `jobname`, `job_city_one`, ...) and time-derived booleans
/// `false`. Use `phpyun_handlers::v1::wap::jobs::job_summary_from_dict` for
/// the dictionary-aware build.
impl From<Job> for JobSummary {
    fn from(j: Job) -> Self {
        Self {
            id: j.id,
            uid: j.uid,
            name: j.name,
            com_name: j.com_name,
            com_logo: j.com_logo,

            job1: j.job1,
            job1_son: j.job1_son,
            job_post: j.job_post,
            hy: j.hy,

            job_one_n: String::new(),
            job_two_n: String::new(),
            job_three_n: String::new(),
            job_hy: String::new(),
            jobname: String::new(),

            province_id: j.provinceid,
            city_id: j.cityid,
            three_city_id: j.three_cityid,
            job_city_one: String::new(),
            job_city_two: String::new(),

            salary: (j.minsalary + j.maxsalary) / 2,
            min_salary: j.minsalary,
            max_salary: j.maxsalary,

            exp: j.exp,
            exp_n: String::new(),
            edu: j.edu,
            edu_n: String::new(),

            rec: j.rec,
            urgent: j.urgent,
            is_rec: false,
            is_urgent: false,
            rec_time: j.rec_time,
            urgent_time: j.urgent_time,

            sdate: j.sdate,
            lastupdate: j.lastupdate,
            newtime: false,

            jobhits: j.jobhits,
            is_favorited: false,
        }
    }
}
