//! Plain-data view types for `Resume` and its child tables.
//!
//! `from_with_dict` constructors live in `phpyun_handlers::v1::wap::resumes`
//! since they need `LocalizedDicts` (which depends on `phpyun_models`). These
//! plain views give other crates a single source of truth for the response
//! shape without forcing a circular dep.

use serde::Serialize;
use utoipa::ToSchema;

use super::cert::Cert;
use super::edu::Edu;
use super::expect::Expect;
use super::project::Project;
use super::skill::Skill;
use super::training::Training;
use super::work::Work;

// ==================== Job-expectation item ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct ResumeExpectItem {
    pub id: u64,
    pub uid: u64,
    pub name: Option<String>,
    pub job_classid: i64,
    /// Job class name (dict resolve_job; takes the first numeric id)
    pub job_class_n: String,
    pub city_classid: i64,
    /// City name (dict resolve_city)
    pub city_class_n: String,
    pub salary: i32,
    /// Salary dictionary name
    pub salary_n: String,
    /// 1 public / 2 hidden
    pub status: i32,
    pub r_status: i32,
    /// 0 pending / 1 approved / 3 rejected
    pub state: i32,
    pub lastupdate: i64,
    pub lastupdate_n: String,
}

// ==================== Education item ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct ResumeEduItem {
    pub id: u64,
    pub uid: u64,
    pub eid: u64,
    pub name: String,
    pub sdate: i64,
    pub sdate_n: String,
    pub edate: i64,
    pub edate_n: String,
    pub specialty: Option<String>,
    /// Education-level dict id (PHPYun column `education`).
    pub education: i32,
    /// Localized education-level name (e.g. "本科" / "硕士").
    pub education_n: String,
}

// ==================== Work-experience item ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct ResumeWorkItem {
    pub id: u64,
    pub uid: u64,
    pub eid: u64,
    pub name: String,
    pub sdate: i64,
    pub sdate_n: String,
    pub edate: i64,
    pub edate_n: String,
    pub department: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
}

impl From<Work> for ResumeWorkItem {
    fn from(w: Work) -> Self {
        Self {
            id: w.id,
            uid: w.uid,
            eid: w.eid,
            name: w.name,
            sdate_n: fmt_date(w.sdate),
            sdate: w.sdate,
            edate_n: fmt_date(w.edate),
            edate: w.edate,
            department: w.department,
            title: w.title,
            content: w.content,
        }
    }
}

// ==================== Project-experience item ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct ResumeProjectItem {
    pub id: u64,
    pub uid: u64,
    pub eid: u64,
    pub name: String,
    pub sdate: i64,
    pub sdate_n: String,
    pub edate: i64,
    pub edate_n: String,
    pub role: Option<String>,
    pub content: Option<String>,
}

impl From<Project> for ResumeProjectItem {
    fn from(p: Project) -> Self {
        Self {
            id: p.id,
            uid: p.uid,
            eid: p.eid,
            name: p.name,
            sdate_n: fmt_date(p.sdate),
            sdate: p.sdate,
            edate_n: fmt_date(p.edate),
            edate: p.edate,
            role: p.role,
            content: p.content,
        }
    }
}

// ==================== Skill item ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct ResumeSkillItem {
    pub id: u64,
    pub uid: u64,
    pub eid: u64,
    pub name: String,
    /// PHP `skill` column: proficiency dictionary id
    pub level: i32,
    pub level_n: String,
    /// PHP `longtime` column
    pub years: i32,
}

// ==================== Training item ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct ResumeTrainingItem {
    pub id: u64,
    pub uid: u64,
    pub eid: u64,
    pub name: String,
    pub sdate: i64,
    pub sdate_n: String,
    pub edate: i64,
    pub edate_n: String,
    pub title: Option<String>,
    pub content: Option<String>,
}

impl From<Training> for ResumeTrainingItem {
    fn from(t: Training) -> Self {
        Self {
            id: t.id,
            uid: t.uid,
            eid: t.eid,
            name: t.name,
            sdate_n: fmt_date(t.sdate),
            sdate: t.sdate,
            edate_n: fmt_date(t.edate),
            edate: t.edate,
            title: t.title,
            content: t.content,
        }
    }
}

// ==================== Certificate item ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct ResumeCertItem {
    pub id: u64,
    pub uid: u64,
    pub eid: u64,
    pub name: String,
    pub sdate: i64,
    pub sdate_n: String,
    pub edate: i64,
    pub edate_n: String,
    pub title: Option<String>,
    pub content: Option<String>,
}

impl From<Cert> for ResumeCertItem {
    fn from(c: Cert) -> Self {
        Self {
            id: c.id,
            uid: c.uid,
            eid: c.eid,
            name: c.name,
            sdate_n: fmt_date(c.sdate),
            sdate: c.sdate,
            edate_n: fmt_date(c.edate),
            edate: c.edate,
            title: c.title,
            content: c.content,
        }
    }
}

// ==================== "Other" / freeform item ====================

#[derive(Debug, Serialize, ToSchema)]
pub struct ResumeOtherItem {
    pub id: u64,
    pub uid: u64,
    pub eid: u64,
    pub name: String,
    pub content: Option<String>,
}

// ==================== Stub From impls for dict-aware types ====================
// Plain (no-dict) From for use by legacy callers — leaves `*_n` fields empty.

impl From<Expect> for ResumeExpectItem {
    fn from(e: Expect) -> Self {
        Self {
            id: e.id,
            uid: e.uid,
            name: e.name,
            job_classid: e.job_classid,
            job_class_n: String::new(),
            city_classid: e.city_classid,
            city_class_n: String::new(),
            salary: e.salary,
            salary_n: String::new(),
            status: e.status,
            r_status: e.r_status,
            state: e.state,
            lastupdate_n: fmt_dt(e.lastupdate),
            lastupdate: e.lastupdate,
        }
    }
}

impl From<Edu> for ResumeEduItem {
    fn from(e: Edu) -> Self {
        Self {
            id: e.id,
            uid: e.uid,
            eid: e.eid,
            name: e.name,
            sdate_n: fmt_date(e.sdate),
            sdate: e.sdate,
            edate_n: fmt_date(e.edate),
            edate: e.edate,
            specialty: e.specialty,
            education: e.education,
            education_n: String::new(),
        }
    }
}

impl From<Skill> for ResumeSkillItem {
    fn from(s: Skill) -> Self {
        Self {
            id: s.id,
            uid: s.uid,
            eid: s.eid,
            name: s.name,
            level: s.level,
            level_n: String::new(),
            years: s.years,
        }
    }
}

// ==================== Helpers ====================

fn fmt_dt(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}
fn fmt_date(ts: i64) -> String {
    if ts <= 0 { return String::new(); }
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_default()
}
