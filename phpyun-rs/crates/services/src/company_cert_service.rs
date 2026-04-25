//! Company verification (aligned with PHPYun's `upComYyzz` / `cert` flow).
//!
//! Business flow:
//!   1) The company uploads URLs for the business license / legal-rep ID in the member center (URLs are produced by `/v1/wap/upload/*`).
//!   2) Write to `phpyun_company_cert` with status=1 (under review).
//!   3) Backend admin reviews -> status=2 (approved) / 3 (rejected).
//!   4) On approval, also flip `phpyun_company.r_status` to 1 (since the Rust side does not modify the company table directly,
//!      we only update the cert here; the `company.r_status` sync is performed by a backend job or review SQL trigger).

use phpyun_core::error::InfraError;
use phpyun_core::{audit, clock, AppError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::company_cert::{
    entity::{CompanyCert, STATUS_APPROVED, STATUS_REJECTED},
    repo as cert_repo,
};

pub async fn get_mine(
    state: &AppState,
    user: &AuthenticatedUser,
) -> AppResult<Option<CompanyCert>> {
    user.require_employer()?;
    Ok(cert_repo::find(state.db.reader(), user.uid).await?)
}

pub async fn submit(
    state: &AppState,
    user: &AuthenticatedUser,
    license_photo: &str,
    id_photo: &str,
) -> AppResult<()> {
    user.require_employer()?;
    if license_photo.is_empty() || id_photo.is_empty() {
        return Err(AppError::new(InfraError::InvalidParam("photos_required".into())));
    }
    let now = clock::now_ts();
    cert_repo::upsert(state.db.pool(), user.uid, license_photo, id_photo, now).await?;
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("company_cert.submit", audit::Actor::uid(user.uid)),
    )
    .await;
    Ok(())
}

// ---------- admin ----------

pub async fn list_pending(
    state: &AppState,
    page: Pagination,
) -> AppResult<Paged<CompanyCert>> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        cert_repo::list_pending(db, page.offset, page.limit),
        cert_repo::count_pending(db),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

pub async fn review(
    state: &AppState,
    admin: &AuthenticatedUser,
    target_uid: u64,
    approve: bool,
    note: &str,
) -> AppResult<()> {
    admin.require_admin()?;
    let status = if approve { STATUS_APPROVED } else { STATUS_REJECTED };
    let now = clock::now_ts();
    let affected =
        cert_repo::review(state.db.pool(), target_uid, status, note, admin.uid, now).await?;
    if affected == 0 {
        return Err(AppError::new(InfraError::InvalidParam("cert_not_pending".into())));
    }
    let _ = audit::emit(
        state,
        audit::AuditEvent::new("admin.company_cert.review", audit::Actor::uid(admin.uid))
            .target(format!("uid:{target_uid}"))
            .meta(&serde_json::json!({ "approve": approve, "note": note })),
    )
    .await;
    Ok(())
}
