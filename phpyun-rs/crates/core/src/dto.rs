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

// ==================== Auth ====================

/// Login / refresh / oauth-login response. Returns only the fields the client
/// needs — `uid`, `usertype`, and the bearer token. The token's `exp` is
/// already encoded in the JWT payload (base64-decodable), so exposing it
/// separately just bloats the response. The refresh-token concept is
/// internal: the server still tracks a paired refresh jti in the session
/// table for "kick device" semantics, but the client never sees it.
#[derive(Debug, Serialize, ToSchema)]
pub struct AuthTokenData {
    pub uid: u64,
    pub usertype: u8,
    pub access_token: String,
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

/// `{ id, created }` — upsert-result envelope (create-or-update endpoints).
/// `created=true` when a new row was inserted, `false` when an existing row
/// was updated.
#[derive(Debug, Serialize, ToSchema)]
pub struct UpsertCreated {
    pub id: u64,
    pub created: bool,
}

/// `{ requested, affected }` — bulk-action result envelope. Used by every
/// admin / employer batch-update / batch-delete endpoint to report how many
/// items were *requested* (input length) vs. how many were *affected*
/// (rows the DB actually touched, after auth-scoping and stale filters).
#[derive(Debug, Serialize, ToSchema)]
pub struct BatchResult {
    pub requested: usize,
    pub affected: u64,
}

/// `{ unread }` — unread badge response. Used by every notification channel
/// (broadcasts / chat / warnings / messages) to drive the bell icon counter.
#[derive(Debug, Serialize, ToSchema)]
pub struct UnreadCount {
    pub unread: u64,
}

/// `{ exists }` — boolean probe response. Used by frontend to render a
/// "follow / favorite / already-applied" button state without leaking the
/// underlying record id.
#[derive(Debug, Serialize, ToSchema)]
pub struct ExistsResp {
    pub exists: bool,
}

/// `{ removed }` — clear-history result envelope. Used by endpoints that wipe
/// a user's owned list (search history / blacklist / notification feed) and
/// report how many rows were dropped.
#[derive(Debug, Serialize, ToSchema)]
pub struct ClearResult {
    pub removed: u64,
}

/// `{ ok }` — generic boolean-success response. Used by claim / oauth-bind /
/// payment-callback endpoints that just want to signal completion. Prefer
/// `ApiOk` for the full message-envelope; this struct is for cases where the
/// frontend explicitly reads `data.ok`.
#[derive(Debug, Serialize, ToSchema)]
pub struct OkResp {
    pub ok: bool,
}

/// `{ hits }` — view-count response. Used by job-detail / article-detail
/// "increment + return new value" endpoints.
#[derive(Debug, Serialize, ToSchema)]
pub struct HitsResp {
    pub hits: u64,
}

/// `{ authorize_url, state }` — third-party OAuth start-flow response.
/// Used identically by every provider (wechat / qq / weibo) to ship the user
/// off to the provider's consent page.
#[derive(Debug, Serialize, ToSchema)]
pub struct OAuthAuthorizeData {
    pub authorize_url: String,
    pub state: String,
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
