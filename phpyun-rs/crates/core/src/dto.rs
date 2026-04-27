//! Shared request/response DTOs reused across handlers.
//!
//! Phase A consolidates the dozens of `IdBody { id: u64 }` / `UidBody { uid: u64 }` /
//! `EidBody` / ... structs that were copy-pasted into every handler file. Each
//! type carries the same validation envelope (range capped at 10^8, matching DB
//! `int(11)` reality) so loosening any of them only happens here.
//!
//! Handlers should `use phpyun_core::dto::*;` instead of redefining locals.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// ==================== Single-id bodies ====================

/// Body carrying just `{ id }`. Use for any "act on one resource by id" call.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct IdBody {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,
}

/// Body carrying just `{ uid }`. Use for endpoints whose primary subject is a
/// member uid (resume / company profile / favorited target).
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UidBody {
    #[validate(range(min = 1, max = 99_999_999))]
    pub uid: u64,
}

/// Body carrying just `{ eid }` (resume-expect id; PHPYun calls this `eid`).
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct EidBody {
    #[validate(range(min = 1, max = 99_999_999))]
    pub eid: u64,
}

/// Body carrying just `{ aid }` (Q&A answer id).
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AidBody {
    #[validate(range(min = 1, max = 99_999_999))]
    pub aid: u64,
}

/// Body carrying just `{ mid }` (job-page message id).
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct MidBody {
    #[validate(range(min = 1, max = 99_999_999))]
    pub mid: u64,
}

/// Body carrying just `{ peer }` (chat peer uid).
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PeerBody {
    #[validate(range(min = 1, max = 99_999_999))]
    pub peer: u64,
}

/// Body carrying an opaque token (resume-share, password-reset, etc.).
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct TokenBody {
    #[validate(length(min = 1, max = 128))]
    pub token: String,
}

/// Body carrying an order_no string (vip / once payments).
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct OrderNoBody {
    #[validate(length(min = 1, max = 64))]
    pub order_no: String,
}

/// Body carrying a third-party `provider` slug (oauth bind/unbind).
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ProviderBody {
    #[validate(length(min = 1, max = 32))]
    pub provider: String,
}

// ==================== Two-field bodies ====================

/// Body carrying `{ uid, id }` (e.g. company sub-page detail).
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UidIdBody {
    #[validate(range(min = 1, max = 99_999_999))]
    pub uid: u64,
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,
}

/// Body carrying `{ kind, target_id }` for favorites / generic kind+id pairs.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct KindTargetIdBody {
    #[validate(range(min = 1, max = 99))]
    pub kind: i32,
    #[validate(range(min = 1, max = 99_999_999))]
    pub target_id: u64,
}

/// Body carrying `{ kind, target_uid }` for follows / ratings / remarks.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct KindTargetUidBody {
    #[validate(range(min = 1, max = 99))]
    pub kind: i32,
    #[validate(range(min = 1, max = 99_999_999))]
    pub target_uid: u64,
}

// ==================== Common responses ====================

/// `{ id }` — used as the create-result envelope across CRUD endpoints.
#[derive(Debug, Serialize, ToSchema)]
pub struct CreatedId {
    pub id: u64,
}

/// `{ on }` — used by toggle endpoints (favorite/follow/upvote) to report new state.
#[derive(Debug, Serialize, ToSchema)]
pub struct Toggled {
    pub on: bool,
}

// ==================== Bulk-id bodies ====================

/// Body carrying `{ ids: Vec<u64> }`. Capped at 200 items, each id 1..=10^8.
/// Used for batch-delete / batch-update endpoints.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct IdsBody {
    #[validate(length(min = 1, max = 200), custom(function = "validate_id_items"))]
    pub ids: Vec<u64>,
}

fn validate_id_items(ids: &[u64]) -> Result<(), validator::ValidationError> {
    for id in ids {
        if *id == 0 || *id > 99_999_999 {
            return Err(validator::ValidationError::new("id_out_of_range"));
        }
    }
    Ok(())
}

// ==================== Limit / password / status-filter bodies ====================
//
// Note: `{ id, status }` and `{ ids, status }` are *not* extracted into shared
// types — each handler bounds `status` to its actual contract (e.g. 1..=2 for
// approve/reject vs. 1..=1 for resolved-only). A wide shared range would silently
// loosen validation, so those locals stay.

/// `{ status: Option<i32> }` — admin list filter envelope. The 0..=99 bound is
/// permissive on purpose; handlers further interpret the value (0=pending,
/// 1=approved, ...) via service logic.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct StatusFilterBody {
    #[validate(range(min = 0, max = 99))]
    pub status: Option<i32>,
}

/// `{ id, password }` — password-required action (verify / refresh / delete a
/// short-link or one-shot job by id, where the owner re-enters the password
/// they set when creating the resource).
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct IdPasswordBody {
    #[validate(range(min = 1, max = 99_999_999))]
    pub id: u64,
    #[validate(length(min = 4, max = 64))]
    pub password: String,
}
