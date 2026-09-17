//! Classic company sub-accounts (`phpyun_member.pid`).

use phpyun_auth::argon2_hash_async;
use phpyun_core::{clock, validators, ApiError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::company_statis::repo as statis_repo;
use phpyun_models::sub_account::{entity::SubAccountRow, repo as sub_repo};
use phpyun_models::user::repo as user_repo;
use uuid::Uuid;

fn require_parent(user: &AuthenticatedUser) -> AppResult<u64> {
    user.require_employer()?;
    if user.is_sub_account() {
        return Err(ApiError::business("sub_account_forbidden"));
    }
    Ok(user.uid)
}

fn gen_salt() -> String {
    Uuid::now_v7().simple().to_string().chars().take(16).collect()
}

pub async fn list(state: &AppState, user: &AuthenticatedUser) -> AppResult<Vec<SubAccountRow>> {
    let uid = require_parent(user)?;
    Ok(sub_repo::list_by_parent(state.db.reader(), uid).await?)
}

pub async fn create(
    state: &AppState,
    user: &AuthenticatedUser,
    username: &str,
    password: &str,
) -> AppResult<u64> {
    let uid = require_parent(user)?;
    let name = username.trim();
    if name.len() < 3 || name.len() > 20 {
        return Err(ApiError::param_invalid("validation.username.length"));
    }
    if user_repo::exists_username(state.db.reader(), name).await? {
        return Err(ApiError::param_invalid("username_taken"));
    }
    if validators::strong_password(password).is_err() {
        return Err(ApiError::param_invalid("password_weak"));
    }
    let st = statis_repo::find_admin(state.db.reader(), uid)
        .await?
        .ok_or_else(|| ApiError::param_invalid("statis_not_found"))?;
    if st.rating_type == 1 {
        let n = statis_repo::dec_sons_num(state.db.pool(), uid).await?;
        if n == 0 {
            return Err(ApiError::business("sub_account_quota"));
        }
    }
    let salt = gen_salt();
    let hash = argon2_hash_async(format!("{password}{salt}")).await?;
    let now = clock::now_ts();
    Ok(sub_repo::insert(state.db.pool(), uid, user.did, name, &hash, &salt, now).await?)
}

pub async fn update(
    state: &AppState,
    user: &AuthenticatedUser,
    uid: u64,
    password: Option<&str>,
    status: Option<i32>,
) -> AppResult<()> {
    let parent = require_parent(user)?;
    if uid == 0 || uid == parent {
        return Err(ApiError::param_invalid("uid"));
    }
    let row = sub_repo::find_owned(state.db.reader(), parent, uid)
        .await?
        .ok_or_else(|| ApiError::business("sub_account_not_found"))?;
    if let Some(pw) = password.map(str::trim).filter(|s| !s.is_empty()) {
        if validators::strong_password(pw).is_err() {
            return Err(ApiError::param_invalid("password_weak"));
        }
        let salt = gen_salt();
        let hash = argon2_hash_async(format!("{pw}{salt}")).await?;
        user_repo::update_password_with_salt(state.db.pool(), row.uid, &hash, &salt).await?;
        let _ = phpyun_core::jwt_blacklist::bump_pw_epoch(
            &state.redis,
            row.uid,
            state.config.pw_epoch_ttl_secs(),
        )
        .await;
    }
    if let Some(st) = status {
        if st != 0 && st != 1 && st != 2 {
            return Err(ApiError::param_invalid("status"));
        }
        let n = sub_repo::update_status(state.db.pool(), parent, uid, st).await?;
        if n == 0 {
            return Err(ApiError::business("sub_account_not_found"));
        }
    }
    Ok(())
}

pub async fn delete(state: &AppState, user: &AuthenticatedUser, uid: u64) -> AppResult<()> {
    let parent = require_parent(user)?;
    if uid == 0 || uid == parent {
        return Err(ApiError::param_invalid("uid"));
    }
    let n = sub_repo::delete_owned(state.db.pool(), parent, uid).await?;
    if n == 0 {
        return Err(ApiError::business("sub_account_not_found"));
    }
    let _ = phpyun_core::jwt_blacklist::bump_pw_epoch(
        &state.redis,
        uid,
        state.config.pw_epoch_ttl_secs(),
    )
    .await;
    Ok(())
}
