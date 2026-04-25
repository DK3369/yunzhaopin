//! Resume sub-table services (job intentions / education / work).
//!
//! - Every operation enforces `require_jobseeker()`
//! - Owner check is implicit: update/delete SQL already carries `uid = ?`
//! - Every mutation emits a `resume.child_*` audit event
//! - TODO: after writing any child table we could call `state.cache.user.invalidate(uid)` to
//!   invalidate the /me cache. Not urgent — /me cache only stores main profile fields.

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, AppResult, AppState, AuthenticatedUser};
use phpyun_models::resume::{cert, edu, expect, language, other, project, skill, training, work};

use crate::domain_errors::ResumeError;

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

    pub async fn create(
        state: &AppState,
        user: &AuthenticatedUser,
        input: expect::ExpectInput<'_>,
        client_ip: &str,
    ) -> AppResult<u64> {
        user.require_jobseeker()?;
        let id = expect::create(state.db.pool(), user.uid, &input, clock::now_ts()).await?;
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
        let id = edu::create(state.db.pool(), user.uid, &input).await?;
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
        let affected = edu::update(state.db.pool(), id, user.uid, &input).await?;
        if affected == 0 {
            return Err(ResumeError::NotFound.into());
        }
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
        let affected = edu::delete(state.db.pool(), id, user.uid).await?;
        if affected == 0 {
            return Err(ResumeError::NotFound.into());
        }
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
        let id = work::create(state.db.pool(), user.uid, &input).await?;
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
        let affected = work::update(state.db.pool(), id, user.uid, &input).await?;
        if affected == 0 {
            return Err(ResumeError::NotFound.into());
        }
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
        let affected = work::delete(state.db.pool(), id, user.uid).await?;
        if affected == 0 {
            return Err(ResumeError::NotFound.into());
        }
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
        let id = project::create(state.db.pool(), user.uid, &input).await?;
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
        let affected = project::update(state.db.pool(), id, user.uid, &input).await?;
        if affected == 0 {
            return Err(ResumeError::NotFound.into());
        }
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
        let affected = project::delete(state.db.pool(), id, user.uid).await?;
        if affected == 0 {
            return Err(ResumeError::NotFound.into());
        }
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
        let id = skill::create(state.db.pool(), user.uid, &input).await?;
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
        let affected = skill::update(state.db.pool(), id, user.uid, &input).await?;
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
        let affected = skill::delete(state.db.pool(), id, user.uid).await?;
        if affected == 0 {
            return Err(ResumeError::NotFound.into());
        }
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
