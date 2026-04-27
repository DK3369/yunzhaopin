//! Account split (only available for personal users).
//!
//! Aligns with PHPYun `wap/member::transfer_action` + `member/user/transfer::save_action`.
//!
//! `POST /v1/mcenter/account/split` — body carries old_password / new_username / new_password,
//! returns the new uid on success; the client should discard the current token (the old account's
//! token is still valid but the resume data has already been migrated away).

use axum::{
    extract::State,
    Router,
    routing::post,
};
use phpyun_core::{ApiJson, AppResult, AppState, AuthenticatedUser, ClientIp, ValidatedJson};
use phpyun_services::transfer_service::{self, TransferInput};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub fn routes() -> Router<AppState> {
    Router::new().route("/account/split", post(split))
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SplitForm {
    #[validate(length(min = 6, max = 20))]
    pub new_username: String,
    #[validate(length(min = 6, max = 64))]
    pub new_password: String,
    #[validate(length(min = 4, max = 64))]
    pub old_password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SplitResult {
    pub old_uid: u64,
    pub new_uid: u64,
}

/// Splits the current personal account into an independent new account — all jobseeker data
/// (resumes/favorites/applications/registrations etc.) have their uid batch-updated to the new
/// account's uid; if the original account is also a company, the original account's usertype is
/// changed to 2 to retain the company identity.
///
/// This operation runs inside a single MySQL transaction; if any sub-UPDATE fails everything is rolled back.
#[utoipa::path(
    post,
    path = "/v1/mcenter/account/split",
    tag = "mcenter",
    security(("bearer" = [])),
    request_body = SplitForm,
    responses(
        (status = 200, description = "ok", body = SplitResult),
        (status = 400, description = "Invalid username/password format / username taken"),
        (status = 401, description = "Old password is incorrect"),
        (status = 403, description = "Only jobseekers can detach the account"),
    )
)]
pub async fn split(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    ClientIp(ip): ClientIp,
    ValidatedJson(f): ValidatedJson<SplitForm>,
) -> AppResult<ApiJson<SplitResult>> {
    let input = TransferInput {
        new_username: &f.new_username,
        new_password: &f.new_password,
        old_password: &f.old_password,
    };
    let r = transfer_service::split_account(&state, &user, &input, &ip).await?;
    Ok(ApiJson(SplitResult {
        old_uid: r.old_uid,
        new_uid: r.new_uid,
    }))
}
