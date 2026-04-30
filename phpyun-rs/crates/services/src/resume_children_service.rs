//! Resume sub-table services (job intentions / education / work).
//!
//! - Every operation enforces `require_jobseeker()`
//! - Owner check is implicit: update/delete SQL already carries `uid = ?`
//! - Every mutation emits a `resume.child_*` audit event
//! - TODO: after writing any child table we could call `state.cache.user.invalidate(uid)` to
//!   invalidate the /me cache. Not urgent — /me cache only stores main profile fields.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, AppResult, AppState, AuthenticatedUser};
use phpyun_models::resume::{
    cert, edu, expect, language, other, project,
    repo as resume_repo, skill, training, user_resume, work,
};

use crate::domain_errors::ResumeError;

/// Which kind of child-row write happened — drives the side-effect mix.
#[derive(Debug, Clone, Copy)]
enum ChildOp {
    Create,
    Update,
    Delete,
}

/// Apply the side-effects PHP runs around every resume-child write. Mirror
/// of `expect.class.php::saveall_action` (insert/update path) +
/// `resume.model.php::getFbReturn` (delete path):
///
/// | step                                  | Create | Update | Delete |
/// |---------------------------------------|--------|--------|--------|
/// | `phpyun_resume.lastupdate = now`      |   ✔    |   ✔    |   ✘ (PHP `getFbReturn` skips) |
/// | `phpyun_user_resume.<section>`        |  +1    |   0    |  -1    |
/// | recompute `whour/avghour` (work only) |   ✔    |   ✔    |   ✘    |
///
/// All three are best-effort — a counter or whour misfire shouldn't roll
/// back the child write that already succeeded. PHP behaves the same way
/// (no transaction wraps these around the child INSERT).
async fn after_child(
    state: &AppState,
    uid: u64,
    eid: u64,
    section: user_resume::Section,
    op: ChildOp,
) {
    let now = clock::now_ts();
    let pool = state.db.pool();
    if matches!(op, ChildOp::Create | ChildOp::Update) {
        let _ = resume_repo::touch_lastupdate(pool, uid, now).await;
    }
    let delta = match op {
        ChildOp::Create => 1,
        ChildOp::Delete => -1,
        ChildOp::Update => 0,
    };
    if delta != 0 {
        let _ = user_resume::bump(pool, uid, eid, section, delta).await;
    }
    if matches!(section, user_resume::Section::Work)
        && matches!(op, ChildOp::Create | ChildOp::Update)
    {
        let _ = expect::recompute_whour(pool, eid, uid, now).await;
    }
}

/// Resolve the eid to attach a child row (work / edu / project / skill / ...)
/// to. PHPYun fans every child off `phpyun_resume_expect.id`, so this looks
/// up the user's default expect (or most-recent fallback). If the user has no
/// expect yet — i.e. wizard skipped step 2 — return `ResumeError::NotFound`
/// so the caller surfaces a clear "请先创建求职意向" instead of writing an
/// orphan row that no read endpoint would ever surface.
async fn resolve_default_eid(state: &AppState, uid: u64) -> AppResult<u64> {
    expect::find_default_id_by_uid(state.db.reader(), uid)
        .await?
        .ok_or_else(|| ResumeError::NotFound.into())
}

// ==================== Job intentions ====================

pub mod expect_svc {
    use super::*;

    pub async fn list(
        state: &AppState,
        user: &AuthenticatedUser,
    ) -> AppResult<Vec<expect::Expect>> {
        user.require_jobseeker()?;
        Ok(expect::list_by_uid(state.db.reader(), user.uid).await?)
    }

    /// **Idempotent on default expect.** PHPYun lets a user own multiple
    /// `phpyun_resume_expect` rows (different job intents = different
    /// "resume copies"), but the H5 wizard only ever wants the "primary"
    /// one. Without idempotence, every wizard re-entry created a fresh
    /// expect → multiple "简历" piling up. We collapse that:
    ///   - if the user has no expect yet → INSERT a fresh default
    ///   - if the user has at least one expect → UPDATE the default in place
    /// Returns the id of the row that was written.
    pub async fn create(
        state: &AppState,
        user: &AuthenticatedUser,
        input: expect::ExpectInput<'_>,
        client_ip: &str,
    ) -> AppResult<u64> {
        user.require_jobseeker()?;
        let pool = state.db.pool();
        let now = clock::now_ts();

        if let Some(existing_id) = expect::find_default_id_by_uid(pool, user.uid).await? {
            let _ = expect::update(pool, existing_id, user.uid, &input, now).await?;
            tracing::info!(
                op = "expect.upsert", uid = user.uid, eid = existing_id,
                ip = client_ip, name = ?input.name,
                "wizard write"
            );
            let _ = audit::emit(
                state,
                AuditEvent::new(
                    "resume.expect_upsert",
                    Actor::uid(user.uid).with_ip(client_ip),
                )
                .target(format!("expect:{existing_id}")),
            )
            .await;
            return Ok(existing_id);
        }

        let id = expect::create(pool, user.uid, &input, now).await?;
        tracing::info!(
            op = "expect.create", uid = user.uid, eid = id,
            ip = client_ip, name = ?input.name,
            "wizard write"
        );
        let _ = audit::emit(
            state,
            AuditEvent::new(
                "resume.expect_add",
                Actor::uid(user.uid).with_ip(client_ip),
            )
            .target(format!("expect:{id}")),
        )
        .await;
        Ok(id)
    }

    pub async fn update(
        state: &AppState,
        user: &AuthenticatedUser,
        id: u64,
        input: expect::ExpectInput<'_>,
        client_ip: &str,
    ) -> AppResult<()> {
        user.require_jobseeker()?;
        let affected =
            expect::update(state.db.pool(), id, user.uid, &input, clock::now_ts()).await?;
        if affected == 0 {
            return Err(ResumeError::NotFound.into());
        }
        let _ = audit::emit(
            state,
            AuditEvent::new(
                "resume.expect_update",
                Actor::uid(user.uid).with_ip(client_ip),
            )
            .target(format!("expect:{id}")),
        )
        .await;
        Ok(())
    }

    pub async fn delete(
        state: &AppState,
        user: &AuthenticatedUser,
        id: u64,
        client_ip: &str,
    ) -> AppResult<()> {
        user.require_jobseeker()?;
        let affected = expect::delete(state.db.pool(), id, user.uid).await?;
        if affected == 0 {
            return Err(ResumeError::NotFound.into());
        }
        let _ = audit::emit(
            state,
            AuditEvent::new(
                "resume.expect_delete",
                Actor::uid(user.uid).with_ip(client_ip),
            )
            .target(format!("expect:{id}")),
        )
        .await;
        Ok(())
    }
}

// ==================== Education history ====================

pub mod edu_svc {
    use super::*;

    pub async fn list(state: &AppState, user: &AuthenticatedUser) -> AppResult<Vec<edu::Edu>> {
        user.require_jobseeker()?;
        Ok(edu::list_by_uid(state.db.reader(), user.uid).await?)
    }

    pub async fn create(
        state: &AppState,
        user: &AuthenticatedUser,
        input: edu::EduInput<'_>,
        client_ip: &str,
    ) -> AppResult<u64> {
        user.require_jobseeker()?;
        let eid = super::resolve_default_eid(state, user.uid).await?;
        let id = edu::create(state.db.pool(), user.uid, eid, &input).await?;
        tracing::info!(
            op = "edu.create", uid = user.uid, eid, id,
            ip = client_ip, name = input.name, education = input.education,
            "wizard write"
        );
        super::after_child(state, user.uid, eid, user_resume::Section::Edu, ChildOp::Create).await;
        let _ = audit::emit(
            state,
            AuditEvent::new("resume.edu_add", Actor::uid(user.uid).with_ip(client_ip))
                .target(format!("edu:{id}")),
        )
        .await;
        Ok(id)
    }

    pub async fn update(
        state: &AppState,
        user: &AuthenticatedUser,
        id: u64,
        input: edu::EduInput<'_>,
        client_ip: &str,
    ) -> AppResult<()> {
        user.require_jobseeker()?;
        let pool = state.db.pool();
        // Look up the row's eid first so we can target the correct expect on
        // the side-effects below; doubles as the existence + ownership check.
        let eid = user_resume::fetch_eid(pool, user_resume::Section::Edu, id, user.uid)
            .await?
            .ok_or(ResumeError::NotFound)?;
        let affected = edu::update(pool, id, user.uid, &input).await?;
        if affected == 0 {
            return Err(ResumeError::NotFound.into());
        }
        super::after_child(state, user.uid, eid, user_resume::Section::Edu, ChildOp::Update).await;
        let _ = audit::emit(
            state,
            AuditEvent::new(
                "resume.edu_update",
                Actor::uid(user.uid).with_ip(client_ip),
            )
            .target(format!("edu:{id}")),
        )
        .await;
        Ok(())
    }

    pub async fn delete(
        state: &AppState,
        user: &AuthenticatedUser,
        id: u64,
        client_ip: &str,
    ) -> AppResult<()> {
        user.require_jobseeker()?;
        let pool = state.db.pool();
        let eid = user_resume::fetch_eid(pool, user_resume::Section::Edu, id, user.uid)
            .await?
            .ok_or(ResumeError::NotFound)?;
        let affected = edu::delete(pool, id, user.uid).await?;
        if affected == 0 {
            return Err(ResumeError::NotFound.into());
        }
        super::after_child(state, user.uid, eid, user_resume::Section::Edu, ChildOp::Delete).await;
        let _ = audit::emit(
            state,
            AuditEvent::new(
                "resume.edu_delete",
                Actor::uid(user.uid).with_ip(client_ip),
            )
            .target(format!("edu:{id}")),
        )
        .await;
        Ok(())
    }
}

// ==================== Work history ====================

pub mod work_svc {
    use super::*;

    pub async fn list(state: &AppState, user: &AuthenticatedUser) -> AppResult<Vec<work::Work>> {
        user.require_jobseeker()?;
        Ok(work::list_by_uid(state.db.reader(), user.uid).await?)
    }

    pub async fn create(
        state: &AppState,
        user: &AuthenticatedUser,
        input: work::WorkInput<'_>,
        client_ip: &str,
    ) -> AppResult<u64> {
        user.require_jobseeker()?;
        let eid = super::resolve_default_eid(state, user.uid).await?;
        let id = work::create(state.db.pool(), user.uid, eid, &input).await?;
        tracing::info!(
            op = "work.create", uid = user.uid, eid, id,
            ip = client_ip, name = input.name, title = input.title,
            "wizard write"
        );
        super::after_child(state, user.uid, eid, user_resume::Section::Work, ChildOp::Create).await;
        let _ = audit::emit(
            state,
            AuditEvent::new("resume.work_add", Actor::uid(user.uid).with_ip(client_ip))
                .target(format!("work:{id}")),
        )
        .await;
        Ok(id)
    }

    pub async fn update(
        state: &AppState,
        user: &AuthenticatedUser,
        id: u64,
        input: work::WorkInput<'_>,
        client_ip: &str,
    ) -> AppResult<()> {
        user.require_jobseeker()?;
        let pool = state.db.pool();
        let eid = user_resume::fetch_eid(pool, user_resume::Section::Work, id, user.uid)
            .await?
            .ok_or(ResumeError::NotFound)?;
        let affected = work::update(pool, id, user.uid, &input).await?;
        if affected == 0 {
            return Err(ResumeError::NotFound.into());
        }
        super::after_child(state, user.uid, eid, user_resume::Section::Work, ChildOp::Update).await;
        let _ = audit::emit(
            state,
            AuditEvent::new(
                "resume.work_update",
                Actor::uid(user.uid).with_ip(client_ip),
            )
            .target(format!("work:{id}")),
        )
        .await;
        Ok(())
    }

    pub async fn delete(
        state: &AppState,
        user: &AuthenticatedUser,
        id: u64,
        client_ip: &str,
    ) -> AppResult<()> {
        user.require_jobseeker()?;
        let pool = state.db.pool();
        let eid = user_resume::fetch_eid(pool, user_resume::Section::Work, id, user.uid)
            .await?
            .ok_or(ResumeError::NotFound)?;
        let affected = work::delete(pool, id, user.uid).await?;
        if affected == 0 {
            return Err(ResumeError::NotFound.into());
        }
        super::after_child(state, user.uid, eid, user_resume::Section::Work, ChildOp::Delete).await;
        let _ = audit::emit(
            state,
            AuditEvent::new(
                "resume.work_delete",
                Actor::uid(user.uid).with_ip(client_ip),
            )
            .target(format!("work:{id}")),
        )
        .await;
        Ok(())
    }
}

// ==================== Project history ====================

pub mod project_svc {
    use super::*;

    pub async fn list(
        state: &AppState,
        user: &AuthenticatedUser,
    ) -> AppResult<Vec<project::Project>> {
        user.require_jobseeker()?;
        Ok(project::list_by_uid(state.db.reader(), user.uid).await?)
    }

    pub async fn create(
        state: &AppState,
        user: &AuthenticatedUser,
        input: project::ProjectInput<'_>,
        client_ip: &str,
    ) -> AppResult<u64> {
        user.require_jobseeker()?;
        let eid = super::resolve_default_eid(state, user.uid).await?;
        let id = project::create(state.db.pool(), user.uid, eid, &input).await?;
        super::after_child(state, user.uid, eid, user_resume::Section::Project, ChildOp::Create).await;
        let _ = audit::emit(
            state,
            AuditEvent::new(
                "resume.project_add",
                Actor::uid(user.uid).with_ip(client_ip),
            )
            .target(format!("project:{id}")),
        )
        .await;
        Ok(id)
    }

    pub async fn update(
        state: &AppState,
        user: &AuthenticatedUser,
        id: u64,
        input: project::ProjectInput<'_>,
        client_ip: &str,
    ) -> AppResult<()> {
        user.require_jobseeker()?;
        let pool = state.db.pool();
        let eid = user_resume::fetch_eid(pool, user_resume::Section::Project, id, user.uid)
            .await?
            .ok_or(ResumeError::NotFound)?;
        let affected = project::update(pool, id, user.uid, &input).await?;
        if affected == 0 {
            return Err(ResumeError::NotFound.into());
        }
        super::after_child(state, user.uid, eid, user_resume::Section::Project, ChildOp::Update).await;
        let _ = audit::emit(
            state,
            AuditEvent::new(
                "resume.project_update",
                Actor::uid(user.uid).with_ip(client_ip),
            )
            .target(format!("project:{id}")),
        )
        .await;
        Ok(())
    }

    pub async fn delete(
        state: &AppState,
        user: &AuthenticatedUser,
        id: u64,
        client_ip: &str,
    ) -> AppResult<()> {
        user.require_jobseeker()?;
        let pool = state.db.pool();
        let eid = user_resume::fetch_eid(pool, user_resume::Section::Project, id, user.uid)
            .await?
            .ok_or(ResumeError::NotFound)?;
        let affected = project::delete(pool, id, user.uid).await?;
        if affected == 0 {
            return Err(ResumeError::NotFound.into());
        }
        super::after_child(state, user.uid, eid, user_resume::Section::Project, ChildOp::Delete).await;
        let _ = audit::emit(
            state,
            AuditEvent::new(
                "resume.project_delete",
                Actor::uid(user.uid).with_ip(client_ip),
            )
            .target(format!("project:{id}")),
        )
        .await;
        Ok(())
    }
}

// ==================== Skills ====================

pub mod skill_svc {
    use super::*;

    pub async fn list(
        state: &AppState,
        user: &AuthenticatedUser,
    ) -> AppResult<Vec<skill::Skill>> {
        user.require_jobseeker()?;
        Ok(skill::list_by_uid(state.db.reader(), user.uid).await?)
    }

    pub async fn create(
        state: &AppState,
        user: &AuthenticatedUser,
        input: skill::SkillInput<'_>,
        client_ip: &str,
    ) -> AppResult<u64> {
        user.require_jobseeker()?;
        let eid = super::resolve_default_eid(state, user.uid).await?;
        let id = skill::create(state.db.pool(), user.uid, eid, &input).await?;
        super::after_child(state, user.uid, eid, user_resume::Section::Skill, ChildOp::Create).await;
        let _ = audit::emit(
            state,
            AuditEvent::new(
                "resume.skill_add",
                Actor::uid(user.uid).with_ip(client_ip),
            )
            .target(format!("skill:{id}")),
        )
        .await;
        Ok(id)
    }

    pub async fn update(
        state: &AppState,
        user: &AuthenticatedUser,
        id: u64,
        input: skill::SkillInput<'_>,
        _client_ip: &str,
    ) -> AppResult<()> {
        user.require_jobseeker()?;
        let pool = state.db.pool();
        let eid = user_resume::fetch_eid(pool, user_resume::Section::Skill, id, user.uid)
            .await?
            .ok_or(ResumeError::NotFound)?;
        let affected = skill::update(pool, id, user.uid, &input).await?;
        if affected == 0 {
            return Err(ResumeError::NotFound.into());
        }
        super::after_child(state, user.uid, eid, user_resume::Section::Skill, ChildOp::Update).await;
        Ok(())
    }

    pub async fn delete(
        state: &AppState,
        user: &AuthenticatedUser,
        id: u64,
        _client_ip: &str,
    ) -> AppResult<()> {
        user.require_jobseeker()?;
        let pool = state.db.pool();
        let eid = user_resume::fetch_eid(pool, user_resume::Section::Skill, id, user.uid)
            .await?
            .ok_or(ResumeError::NotFound)?;
        let affected = skill::delete(pool, id, user.uid).await?;
        if affected == 0 {
            return Err(ResumeError::NotFound.into());
        }
        super::after_child(state, user.uid, eid, user_resume::Section::Skill, ChildOp::Delete).await;
        Ok(())
    }
}

// ==================== Languages ====================

pub mod language_svc {
    use super::*;

    pub async fn list(
        state: &AppState,
        user: &AuthenticatedUser,
    ) -> AppResult<Vec<language::Language>> {
        user.require_jobseeker()?;
        Ok(language::list_by_uid(state.db.reader(), user.uid).await?)
    }

    pub async fn create(
        state: &AppState,
        user: &AuthenticatedUser,
        input: language::LanguageInput<'_>,
        _client_ip: &str,
    ) -> AppResult<u64> {
        user.require_jobseeker()?;
        let id = language::create(state.db.pool(), user.uid, &input).await?;
        Ok(id)
    }

    pub async fn update(
        state: &AppState,
        user: &AuthenticatedUser,
        id: u64,
        input: language::LanguageInput<'_>,
        _client_ip: &str,
    ) -> AppResult<()> {
        user.require_jobseeker()?;
        let affected = language::update(state.db.pool(), id, user.uid, &input).await?;
        if affected == 0 {
            return Err(ResumeError::NotFound.into());
        }
        Ok(())
    }

    pub async fn delete(
        state: &AppState,
        user: &AuthenticatedUser,
        id: u64,
        _client_ip: &str,
    ) -> AppResult<()> {
        user.require_jobseeker()?;
        let affected = language::delete(state.db.pool(), id, user.uid).await?;
        if affected == 0 {
            return Err(ResumeError::NotFound.into());
        }
        Ok(())
    }
}

// ==================== Public reads (for companies viewing a resume) ====================

/// One-shot fetch of the three primary sub-tables for a uid — used by the resume detail page
/// (expect / edu / work).
pub async fn get_public_bundle(
    state: &AppState,
    uid: u64,
) -> AppResult<(Vec<expect::Expect>, Vec<edu::Edu>, Vec<work::Work>)> {
    let (e, ed, w) = tokio::join!(
        expect::list_by_uid(state.db.reader(), uid),
        edu::list_by_uid(state.db.reader(), uid),
        work::list_by_uid(state.db.reader(), uid),
    );
    Ok((e?, ed?, w?))
}

/// Full bundle for the detail page: expectations / education / work / projects / skills /
/// training / certificates / other (8 categories total).
/// All 8 sub-tables are queried in parallel; total wall time ≈ a single SELECT.
#[allow(clippy::type_complexity)]
pub async fn get_full_bundle(
    state: &AppState,
    uid: u64,
) -> AppResult<(
    Vec<expect::Expect>,
    Vec<edu::Edu>,
    Vec<work::Work>,
    Vec<project::Project>,
    Vec<skill::Skill>,
    Vec<training::Training>,
    Vec<cert::Cert>,
    Vec<other::Other>,
)> {
    let db = state.db.reader();
    let (e, ed, w, p, s, tr, c, o) = tokio::join!(
        expect::list_by_uid(db, uid),
        edu::list_by_uid(db, uid),
        work::list_by_uid(db, uid),
        project::list_by_uid(db, uid),
        skill::list_by_uid(db, uid),
        training::list_by_uid(db, uid),
        cert::list_by_uid(db, uid),
        other::list_by_uid(db, uid),
    );
    Ok((e?, ed?, w?, p?, s?, tr?, c?, o?))
}
