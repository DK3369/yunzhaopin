//! Account transfer (split) service.
//!
//! Aligned with PHPYun `app/model/transfer.model.php::setTransfer`.
//!
//! Business semantics: "split the current personal account into a brand-new standalone account":
//! 1. Verify the original account password
//! 2. Create a new member record containing only usertype=1 (username/password supplied by user)
//! 3. Re-point the "jobseeker-side data" of the original account (resume, favorites, applications,
//!    and a series of related tables) from the old uid to the new uid in bulk
//! 4. If the original account is also a company (has a row in the company table), set its
//!    usertype to 2; otherwise leave it as is
//!
//! All UPDATEs run inside a single transaction; any failure rolls back the whole thing.
//!
//! ## Account merge
//! `merge_into_company` is the Rust port of PHPYun's `mergeData`: migrate the data of the
//! personal account (uid) under the company account (com_uid), then delete the original
//! personal member record.
//! Preconditions:
//! - The personal account must not already have a company identity (no row in `phpyun_company`)
//! - The company account must not already have a personal identity (no row in `phpyun_resume`)

use phpyun_auth::{argon2_hash_async, verify_password_async};
use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::jwt_blacklist;
use phpyun_core::{clock, AppError, AppResult, AppState, AuthenticatedUser, InfraError};
use phpyun_core::validators;
use phpyun_models::user::repo as user_repo;

/// Tables that can be migrated along with the account split (only have a `uid` column,
/// directly `WHERE uid = old`). Order does not matter; rollback on failure.
///
/// Tables not listed here (`login_log` / `member_log`, etc.) usually need an additional
/// `usertype` predicate and are handled separately.
const TABLES_UID_ONLY: &[&str] = &[
    "phpyun_resume",
    "phpyun_resume_cert",
    "phpyun_resume_edu",
    "phpyun_resume_expect",
    "phpyun_resume_other",
    "phpyun_resume_project",
    "phpyun_resume_show",
    "phpyun_resume_skill",
    "phpyun_resume_training",
    "phpyun_resume_work",
    "phpyun_fav_job",
    "phpyun_part_apply",
    "phpyun_part_collect",
    "phpyun_user_entrust",
    "phpyun_user_entrust_record",
    "phpyun_down_resume",
    "phpyun_talent_pool",
    "phpyun_look_job",
    "phpyun_look_resume",
    "phpyun_userid_job",
    "phpyun_userid_msg",
    "phpyun_entrust",
    "phpyun_user_resume",
    "phpyun_resumeout",
    "phpyun_member_statis",
    "phpyun_msg",
];

/// These tables additionally require the `usertype = 1` predicate
/// (matches PHP's `tableUsertypeList`).
const TABLES_UID_AND_USERTYPE: &[&str] = &[
    "phpyun_atn",
    "phpyun_change",
    "phpyun_company_order",
    "phpyun_evaluate_leave_message",
    "phpyun_finder",
    "phpyun_login_log",
    "phpyun_member_log",
    "phpyun_member_reg",
];

pub struct TransferInput<'a> {
    pub new_username: &'a str,
    pub new_password: &'a str,
    pub old_password: &'a str,
}

pub struct TransferResult {
    pub new_uid: u64,
    pub old_uid: u64,
}

pub async fn split_account(
    state: &AppState,
    user: &AuthenticatedUser,
    input: &TransferInput<'_>,
    client_ip: &str,
) -> AppResult<TransferResult> {
    user.require_jobseeker()?;

    // 1. Basic validation
    if validators::username(input.new_username).is_err() {
        return Err(InfraError::InvalidParam("username".into()).into());
    }
    if validators::strong_password(input.new_password).is_err() {
        return Err(InfraError::InvalidParam("password".into()).into());
    }
    if input.old_password.is_empty() {
        return Err(InfraError::InvalidParam("old_password".into()).into());
    }

    // 2. Load the original account and verify the old password
    let member = user_repo::find_by_uid(state.db.reader(), user.uid)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("account_not_found".into())))?;

    let pwd_ok = verify_password_async(
        input.old_password.to_string(),
        member.password.clone(),
        member.salt.clone(),
    )
    .await;
    if !pwd_ok {
        return Err(InfraError::InvalidCredentials.into());
    }

    // 3. Uniqueness of the new username
    if user_repo::exists_username(state.db.reader(), input.new_username).await? {
        return Err(InfraError::InvalidParam("username_taken".into()).into());
    }

    // 4. Generate the argon2 hash for the new password
    let new_pw_hash = argon2_hash_async(input.new_password.to_string())
        .await
        .map_err(|e| AppError::new(InfraError::InvalidParam(format!("hash_failed: {e}"))))?;

    let now = clock::now_ts();
    let old_uid = user.uid;
    let new_username = input.new_username.to_string();

    // 5. Determine whether the original account also has a company identity
    let has_company: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM phpyun_company WHERE uid = ?",
    )
    .bind(old_uid)
    .fetch_one(state.db.reader())
    .await?;

    // 6. Inside a single transaction: create the new member + bulk UID migration
    let new_uid = state
        .db
        .with_tx(|tx| {
            let new_username = new_username.clone();
            let new_pw_hash = new_pw_hash.clone();
            Box::pin(async move {
                // Create the new member
                let new_uid = user_repo::create_member(
                    &mut **tx,
                    &new_username,
                    &new_pw_hash,
                    "", // The salt column is unused with argon2; keep an empty string for legacy column compat
                    None,
                    None,
                    1, // usertype = jobseeker
                    0, // did
                    "",
                    now,
                )
                .await?;

                // Bulk migrate uid-only tables
                for table in TABLES_UID_ONLY {
                    let sql = format!("UPDATE {table} SET uid = ? WHERE uid = ?");
                    let _ = sqlx::query(&sql)
                        .bind(new_uid)
                        .bind(old_uid)
                        .execute(&mut **tx)
                        .await?;
                }

                // Tables filtered by uid + usertype=1
                for table in TABLES_UID_AND_USERTYPE {
                    let sql =
                        format!("UPDATE {table} SET uid = ? WHERE uid = ? AND usertype = 1");
                    let _ = sqlx::query(&sql)
                        .bind(new_uid)
                        .bind(old_uid)
                        .execute(&mut **tx)
                        .await?;
                }

                // Special columns: blacklist.c_uid / company_pay.com_id / report.p_uid / sysmsg.fa_uid, etc.
                let _ = sqlx::query(
                    "UPDATE phpyun_blacklist SET c_uid = ? WHERE c_uid = ? AND usertype = 1",
                )
                .bind(new_uid)
                .bind(old_uid)
                .execute(&mut **tx)
                .await?;

                let _ = sqlx::query(
                    "UPDATE phpyun_report SET p_uid = ? WHERE p_uid = ? AND usertype = 1",
                )
                .bind(new_uid)
                .bind(old_uid)
                .execute(&mut **tx)
                .await?;

                // If the original account has a company identity: set member.usertype to 2 to
                // preserve that identity; otherwise keep the original usertype (still 1, but
                // with no resume data left).
                if has_company > 0 {
                    let _ = sqlx::query(
                        "UPDATE phpyun_member SET usertype = 2 WHERE uid = ?",
                    )
                    .bind(old_uid)
                    .execute(&mut **tx)
                    .await?;
                }

                Ok(new_uid)
            })
        })
        .await?;

    // Splitting changes the password and possibly the role: every existing token tied to
    // the old uid must be invalidated.
    let _ = jwt_blacklist::bump_pw_epoch(&state.redis, old_uid).await;

    let _ = audit::emit(
        state,
        AuditEvent::new(
            "user.transfer_split",
            Actor::uid(old_uid).with_ip(client_ip),
        )
        .target(format!("new_uid:{new_uid}"))
        .meta(&serde_json::json!({
            "new_username": input.new_username,
            "kept_company_identity": has_company > 0,
        })),
    )
    .await;

    Ok(TransferResult {
        new_uid,
        old_uid,
    })
}

// ==================== Account merge ====================

pub struct MergeResult {
    pub user_uid: u64,
    pub company_uid: u64,
}

/// Merge the personal account `user_uid` into the company account `company_uid`:
/// 1. Validate both account states (the personal account must not already have a company
///    identity; the company account must not already have a resume)
/// 2. Inside a transaction, change all jobseeker-related tables owned by `user_uid` to
///    `company_uid`
/// 3. Delete the original `user_uid` member record
///
/// Admin-only (the handler layer must call `require_admin`). This function performs no
/// role check and operates purely by uid.
pub async fn merge_into_company(
    state: &AppState,
    admin: &AuthenticatedUser,
    user_uid: u64,
    company_uid: u64,
    client_ip: &str,
) -> AppResult<MergeResult> {
    admin.require_admin()?;
    if user_uid == 0 || company_uid == 0 || user_uid == company_uid {
        return Err(InfraError::InvalidParam("uid".into()).into());
    }

    // Precondition: the personal account must not already have a company identity
    let user_has_company: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM phpyun_company WHERE uid = ?")
            .bind(user_uid)
            .fetch_one(state.db.reader())
            .await?;
    if user_has_company > 0 {
        return Err(InfraError::InvalidParam("user_has_company_identity".into()).into());
    }

    // Precondition: the company account must not already have a personal identity
    let company_has_resume: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM phpyun_resume WHERE uid = ?")
            .bind(company_uid)
            .fetch_one(state.db.reader())
            .await?;
    if company_has_resume > 0 {
        return Err(InfraError::InvalidParam("company_has_resume_identity".into()).into());
    }

    // Transaction: bulk uid update + member deletion
    state
        .db
        .with_tx(|tx| {
            Box::pin(async move {
                for table in TABLES_UID_ONLY {
                    let sql = format!("UPDATE {table} SET uid = ? WHERE uid = ?");
                    let _ = sqlx::query(&sql)
                        .bind(company_uid)
                        .bind(user_uid)
                        .execute(&mut **tx)
                        .await?;
                }
                for table in TABLES_UID_AND_USERTYPE {
                    let sql =
                        format!("UPDATE {table} SET uid = ? WHERE uid = ? AND usertype = 1");
                    let _ = sqlx::query(&sql)
                        .bind(company_uid)
                        .bind(user_uid)
                        .execute(&mut **tx)
                        .await?;
                }
                let _ = sqlx::query(
                    "UPDATE phpyun_blacklist SET c_uid = ? WHERE c_uid = ? AND usertype = 1",
                )
                .bind(company_uid)
                .bind(user_uid)
                .execute(&mut **tx)
                .await?;
                let _ = sqlx::query(
                    "UPDATE phpyun_report SET p_uid = ? WHERE p_uid = ? AND usertype = 1",
                )
                .bind(company_uid)
                .bind(user_uid)
                .execute(&mut **tx)
                .await?;

                // Finally: delete the original user's member record (matches PHP: after the
                // merge, the original personal account no longer exists).
                let _ = sqlx::query("DELETE FROM phpyun_member WHERE uid = ?")
                    .bind(user_uid)
                    .execute(&mut **tx)
                    .await?;

                Ok(())
            })
        })
        .await?;

    let _ = audit::emit(
        state,
        AuditEvent::new(
            "user.transfer_merge",
            Actor::uid(admin.uid).with_ip(client_ip),
        )
        .target(format!("uid:{user_uid} -> uid:{company_uid}"))
        .meta(&serde_json::json!({
            "merged_user_uid": user_uid,
            "target_company_uid": company_uid,
        })),
    )
    .await;

    Ok(MergeResult {
        user_uid,
        company_uid,
    })
}
