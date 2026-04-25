//! Domain errors — resume / company / job. Each domain has its own enum; implementing `ApiError` is
//! enough for `?` to flow into the response pipeline without touching core (showcasing the "pluggable error architecture").

use phpyun_core::error::ApiError;
use std::borrow::Cow;
use thiserror::Error;

// ==================== ResumeError ====================

#[derive(Error, Debug, Clone)]
pub enum ResumeError {
    #[error("resume not found")]
    NotFound,
    #[error("resume hidden by owner")]
    Hidden,
    #[error("resume not allowed in this status: {0}")]
    BadStatus(String),
}

impl ApiError for ResumeError {
    fn code(&self) -> u16 {
        match self {
            Self::NotFound => 404,
            Self::Hidden => 403,
            Self::BadStatus(_) => 400,
        }
    }
    fn tag(&self) -> Cow<'static, str> {
        match self {
            Self::NotFound => "resume_not_found".into(),
            Self::Hidden => "resume_hidden".into(),
            Self::BadStatus(_) => "resume_bad_status".into(),
        }
    }
}

// ==================== CompanyError ====================

#[derive(Error, Debug, Clone)]
pub enum CompanyError {
    #[error("company not found")]
    NotFound,
    #[error("company not verified (r_status != 1)")]
    NotVerified,
    #[error("company locked")]
    Locked,
}

impl ApiError for CompanyError {
    fn code(&self) -> u16 {
        match self {
            Self::NotFound => 404,
            Self::NotVerified => 403,
            Self::Locked => 403,
        }
    }
    fn tag(&self) -> Cow<'static, str> {
        match self {
            Self::NotFound => "company_not_found".into(),
            Self::NotVerified => "company_not_verified".into(),
            Self::Locked => "company_locked".into(),
        }
    }
}

// ==================== ApplyError ====================

#[derive(Error, Debug, Clone)]
pub enum ApplyError {
    #[error("apply not found")]
    NotFound,
    #[error("already applied")]
    Duplicate,
    #[error("not owner")]
    NotOwner,
    #[error("cannot apply to own job")]
    OwnJob,
}

impl ApiError for ApplyError {
    fn code(&self) -> u16 {
        match self {
            Self::NotFound => 404,
            Self::Duplicate => 409,
            Self::NotOwner => 403,
            Self::OwnJob => 400,
        }
    }
    fn tag(&self) -> Cow<'static, str> {
        match self {
            Self::NotFound => "apply_not_found".into(),
            Self::Duplicate => "apply_duplicate".into(),
            Self::NotOwner => "apply_not_owner".into(),
            Self::OwnJob => "apply_own_job".into(),
        }
    }
}

// ==================== CollectError ====================

#[derive(Error, Debug, Clone)]
pub enum CollectError {
    #[error("invalid kind: {0}")]
    InvalidKind(i32),
    #[error("target not found or not collectable")]
    TargetNotFound,
    #[error("already collected")]
    Duplicate,
}

impl ApiError for CollectError {
    fn code(&self) -> u16 {
        match self {
            Self::InvalidKind(_) => 400,
            Self::TargetNotFound => 404,
            Self::Duplicate => 409,
        }
    }
    fn tag(&self) -> Cow<'static, str> {
        match self {
            Self::InvalidKind(_) => "collect_bad_kind".into(),
            Self::TargetNotFound => "collect_target_not_found".into(),
            Self::Duplicate => "collect_duplicate".into(),
        }
    }
}

// ==================== JobError ====================

#[derive(Error, Debug, Clone)]
pub enum JobError {
    #[error("job not found")]
    NotFound,
    #[error("job offline")]
    Offline,
    #[error("job pending review")]
    PendingReview,
    #[error("job expired")]
    Expired,
}

impl ApiError for JobError {
    fn code(&self) -> u16 {
        match self {
            Self::NotFound => 404,
            Self::Offline => 410, // Gone
            Self::PendingReview => 403,
            Self::Expired => 410,
        }
    }
    fn tag(&self) -> Cow<'static, str> {
        match self {
            Self::NotFound => "job_not_found".into(),
            Self::Offline => "job_offline".into(),
            Self::PendingReview => "job_pending".into(),
            Self::Expired => "job_expired".into(),
        }
    }
}

// ==================== TinyError (general-worker resume) ====================

#[derive(Error, Debug, Clone)]
pub enum TinyError {
    #[error("tiny resume not found")]
    NotFound,
    #[error("password mismatch")]
    PasswordMismatch,
    #[error("daily site limit reached")]
    DailySiteLimit,
    #[error("daily IP limit reached")]
    DailyIpLimit,
}

impl ApiError for TinyError {
    fn code(&self) -> u16 {
        match self {
            Self::NotFound => 404,
            Self::PasswordMismatch => 403,
            Self::DailySiteLimit | Self::DailyIpLimit => 429,
        }
    }
    fn tag(&self) -> Cow<'static, str> {
        match self {
            Self::NotFound => "tiny_not_found".into(),
            Self::PasswordMismatch => "tiny_pwd_mismatch".into(),
            Self::DailySiteLimit => "tiny_site_limit".into(),
            Self::DailyIpLimit => "tiny_ip_limit".into(),
        }
    }
}

// ==================== PartError (part-time) ====================
//
// Aligned with the business-error semantics of PHPYun `app/model/part.model.php`.

#[derive(Error, Debug, Clone)]
pub enum PartError {
    #[error("part-time job not found")]
    NotFound,
    #[error("part-time job offline")]
    Offline,
    #[error("part-time job pending review")]
    PendingReview,
    #[error("part-time job expired")]
    Expired,
    #[error("already applied")]
    DuplicateApply,
    #[error("already collected")]
    DuplicateCollect,
    #[error("only jobseekers can apply/collect")]
    RoleNotAllowed,
    #[error("module disabled")]
    Disabled,
}

impl ApiError for PartError {
    fn code(&self) -> u16 {
        match self {
            Self::NotFound => 404,
            Self::Offline => 410,
            Self::PendingReview => 403,
            Self::Expired => 410,
            Self::DuplicateApply | Self::DuplicateCollect => 409,
            Self::RoleNotAllowed => 403,
            Self::Disabled => 503,
        }
    }
    fn tag(&self) -> Cow<'static, str> {
        match self {
            Self::NotFound => "part_not_found".into(),
            Self::Offline => "part_offline".into(),
            Self::PendingReview => "part_pending".into(),
            Self::Expired => "part_expired".into(),
            Self::DuplicateApply => "part_apply_duplicate".into(),
            Self::DuplicateCollect => "part_collect_duplicate".into(),
            Self::RoleNotAllowed => "part_role_not_allowed".into(),
            Self::Disabled => "part_disabled".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resume_error_codes() {
        assert_eq!(ResumeError::NotFound.code(), 404);
        assert_eq!(ResumeError::Hidden.code(), 403);
        assert_eq!(ResumeError::BadStatus("x".into()).code(), 400);
    }

    #[test]
    fn company_error_codes() {
        assert_eq!(CompanyError::NotFound.code(), 404);
        assert_eq!(CompanyError::Locked.code(), 403);
    }

    #[test]
    fn job_error_codes() {
        assert_eq!(JobError::NotFound.code(), 404);
        assert_eq!(JobError::Offline.code(), 410);
        assert_eq!(JobError::Expired.code(), 410);
    }

    #[test]
    fn part_error_codes() {
        assert_eq!(PartError::NotFound.code(), 404);
        assert_eq!(PartError::Offline.code(), 410);
        assert_eq!(PartError::Expired.code(), 410);
        assert_eq!(PartError::PendingReview.code(), 403);
        assert_eq!(PartError::DuplicateApply.code(), 409);
        assert_eq!(PartError::DuplicateCollect.code(), 409);
        assert_eq!(PartError::RoleNotAllowed.code(), 403);
        assert_eq!(PartError::Disabled.code(), 503);
    }

    #[test]
    fn part_error_tags_ascii() {
        for t in [
            PartError::NotFound.tag(),
            PartError::Offline.tag(),
            PartError::Expired.tag(),
            PartError::PendingReview.tag(),
            PartError::DuplicateApply.tag(),
            PartError::DuplicateCollect.tag(),
            PartError::RoleNotAllowed.tag(),
            PartError::Disabled.tag(),
        ] {
            assert!(t.is_ascii(), "tag must be ASCII: {t}");
            assert!(t.len() <= 32, "tag too long: {t}");
            assert!(t.starts_with("part_"), "tag must be namespaced: {t}");
        }
    }

    #[test]
    fn all_tags_ascii_and_short() {
        for e in [
            ResumeError::NotFound.tag(),
            ResumeError::Hidden.tag(),
            CompanyError::NotFound.tag(),
            CompanyError::NotVerified.tag(),
            JobError::NotFound.tag(),
            JobError::Offline.tag(),
            JobError::Expired.tag(),
        ] {
            assert!(e.is_ascii(), "tag must be ASCII: {e}");
            assert!(e.len() <= 32, "tag too long: {e}");
        }
    }
}
