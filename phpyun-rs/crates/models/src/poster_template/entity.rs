//! `phpyun_admin_jobwhb` -- poster templates (job / company / invite-register /
//! public recruitment).
//!
//! Aligns with PHPYun `whb.model`. The PHP side uses server-side GD to render
//! PNGs; the Rust side returns template metadata for the client to compose
//! via Canvas -- more resource-efficient and a more modern architecture.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct PosterTemplate {
    pub id: u64,
    pub title: String,
    /// Template background image URL (relative path or absolute URL).
    pub pic: Option<String>,
    /// Template kind: 1 = job / 2 = company / 3 = invite-register / 4 = public recruitment.
    pub r#type: i32,
    /// 0 = disabled / 1 = enabled.
    pub isopen: i32,
    /// Sort weight (used together with num).
    pub sort: i32,
    /// Usage count (popularity counter, used for sorting).
    #[serde(default)]
    pub num: i64,
    /// JSON config for text and QR-code positioning (passed through to client as-is).
    #[serde(default)]
    pub config_pos: Option<String>,
}

/// Poster kind enum, aligned with the `type` column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PosterKind {
    Job = 1,
    Company = 2,
    InviteReg = 3,
    Gongzhao = 4,
}

impl PosterKind {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "job" => Some(Self::Job),
            "company" => Some(Self::Company),
            "invite-reg" | "invitereg" => Some(Self::InviteReg),
            "gongzhao" => Some(Self::Gongzhao),
            _ => None,
        }
    }

    pub fn as_i8(self) -> i32 {
        self as i32
    }
}
