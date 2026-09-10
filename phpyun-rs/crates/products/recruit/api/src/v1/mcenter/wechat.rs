//! Member Official Account follow status (PHP WAP `isgzh_action`).

use axum::{extract::State, routing::post, Router};
use phpyun_core::{clock, ApiResponse, AppResult, AppState, AuthenticatedUser};
use phpyun_models::user::repo as user_repo;
use phpyun_services::wechat_api_service;
use serde::Serialize;
use utoipa::ToSchema;

pub fn routes() -> Router<AppState> {
    Router::new().route("/wechat/subscribe", post(subscribe_status))
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SubscribeView {
    /// 0 = not following / unbound, 1 = following, 2 = bound to another WeChat.
    pub subscribe: i32,
}

/// PHP `wap/member/index::isgzh_action` (same logic on company home).
#[utoipa::path(
    post,
    path = "/v1/mcenter/wechat/subscribe",
    tag = "mcenter",
    security(("bearer" = [])),
    responses((status = 200, description = "ok", body = SubscribeView))
)]
pub async fn subscribe_status(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> AppResult<ApiResponse<SubscribeView>> {
    let row = user_repo::find_wx_subscribe(state.db.reader(), user.uid).await?;
    let (wxid, db_sub) = match row {
        Some(r) => (r.wxid, r.subscribe),
        None => (String::new(), 0),
    };
    if wxid.is_empty() {
        return Ok(ApiResponse::data(SubscribeView { subscribe: 0 }));
    }

    let login_id = format!("weixin_gzhid_{}", user.uid);
    if let Some(scan) =
        user_repo::latest_gzh_scan_wxid(state.db.reader(), &login_id, clock::start_of_today())
            .await?
    {
        if scan != wxid {
            return Ok(ApiResponse::data(SubscribeView { subscribe: 2 }));
        }
    }

    if state.config.wechat_appid.is_none() {
        return Ok(ApiResponse::data(SubscribeView { subscribe: db_sub }));
    }

    match wechat_api_service::get_subscribe(&state, &wxid).await {
        Ok(sub) => {
            if db_sub == 2 {
                let _ = user_repo::set_subscribe(state.db.pool(), user.uid, sub).await;
            }
            Ok(ApiResponse::data(SubscribeView { subscribe: sub }))
        }
        Err(_) => Ok(ApiResponse::data(SubscribeView { subscribe: db_sub })),
    }
}
