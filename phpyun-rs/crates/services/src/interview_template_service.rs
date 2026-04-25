//! Interview-invitation templates (aligned with PHPYun `yqmb`).
//!
//! Companies prepare N templates and reuse them when creating interviews. `MAX_PER_EMPLOYER` = 10
//! (PHPYun reads `com_yqmb_num` from config; the migrated version uses a hard-coded value
//! for now and will switch to the config facade later).

use phpyun_core::error::InfraError;
use phpyun_core::{clock, AppError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::interview_template::{entity::InterviewTemplate, repo as tpl_repo};

const MAX_PER_EMPLOYER: u64 = 10;

pub struct TplInput<'a> {
    pub name: &'a str,
    pub content: &'a str,
    pub address: &'a str,
    pub linkman: &'a str,
    pub linktel: &'a str,
    pub intertime: i64,
}

pub async fn list(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<Vec<InterviewTemplate>> {
    Ok(tpl_repo::list_by_uid(state.db.reader(), user.uid).await?)
}

pub async fn create(
    state: &AppState,
    user: &AuthenticatedUser,
    input: TplInput<'_>,
) -> AppResult<u64> {
    let now = clock::now_ts();
    if input.intertime != 0 && input.intertime < now {
        return Err(AppError::new(InfraError::InvalidParam("intertime_past".into())));
    }
    let used = tpl_repo::count_by_uid(state.db.reader(), user.uid).await?;
    if used >= MAX_PER_EMPLOYER {
        return Err(AppError::new(InfraError::InvalidParam("tpl_limit_reached".into())));
    }
    let id = tpl_repo::create(
        state.db.pool(),
        tpl_repo::TplCreate {
            uid: user.uid,
            name: input.name,
            content: input.content,
            address: input.address,
            linkman: input.linkman,
            linktel: input.linktel,
            intertime: input.intertime,
        },
        now,
    )
    .await?;
    Ok(id)
}

pub struct TplPatch<'a> {
    pub name: Option<&'a str>,
    pub content: Option<&'a str>,
    pub address: Option<&'a str>,
    pub linkman: Option<&'a str>,
    pub linktel: Option<&'a str>,
    pub intertime: Option<i64>,
    pub status: Option<i32>,
}

pub async fn update(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
    patch: TplPatch<'_>,
) -> AppResult<()> {
    if let Some(t) = patch.intertime {
        if t != 0 && t < clock::now_ts() {
            return Err(AppError::new(InfraError::InvalidParam("intertime_past".into())));
        }
    }
    let affected = tpl_repo::update(
        state.db.pool(),
        id,
        user.uid,
        tpl_repo::TplUpdate {
            name: patch.name,
            content: patch.content,
            address: patch.address,
            linkman: patch.linkman,
            linktel: patch.linktel,
            intertime: patch.intertime,
            status: patch.status,
        },
        clock::now_ts(),
    )
    .await?;
    if affected == 0 {
        return Err(AppError::new(InfraError::Forbidden));
    }
    Ok(())
}

pub async fn delete(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
) -> AppResult<()> {
    let affected = tpl_repo::delete(state.db.pool(), id, user.uid).await?;
    if affected == 0 {
        return Err(AppError::new(InfraError::Forbidden));
    }
    Ok(())
}
