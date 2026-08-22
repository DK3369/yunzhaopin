//! `phpyun_company_job_link` -- company work addresses (multi-location supported).
//!
//! Used when posting jobs at different locations: pick one of these rows
//! as the contact/address for the job; also used to label "work location"
//! on the company home page.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CompanyAddress {
    pub id: u64,
    pub uid: u64,
    pub link_man: String,
    pub link_moblie: String,
    pub link_phone: Option<String>,
    pub email: Option<String>,
    pub link_address: Option<String>,
    pub provinceid: i32,
    pub cityid: i32,
    pub three_cityid: i32,
    /// Longitude
    pub x: Option<String>,
    /// Latitude
    pub y: Option<String>,
}
