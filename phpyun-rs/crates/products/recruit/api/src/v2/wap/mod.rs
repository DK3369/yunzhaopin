//! v2 WAP endpoints — only the diffs (login); everything else is reused from v1.

pub mod login;

use axum::Router;
use phpyun_core::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        // v2-only handler
        .merge(login::routes())
        // Unchanged endpoints reuse v1 directly (/v2/wap/logout, /v2/wap/refresh, /v2/wap/me)
        .merge(crate::v1::wap::auth::routes())
}
