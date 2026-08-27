//! PHP `adminCommon::admin_get_user_login` + `getPower` + `getAdminNavList`.

use std::collections::HashSet;

use phpyun_auth::md5_hex;
use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::jwt::{issue_pair, JwtIssued};
use phpyun_core::{clock, numeric, rate_limit, ApiError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::admin_rbac::repo::{self as rbac_repo, AdminNavRow};
use serde::Serialize;

use crate::user_service::LoginContext;
use crate::user_session_service::{self, LoginRecord};

#[derive(Debug, Clone, Serialize)]
pub struct AdminLoginResult {
    pub uid: u64,
    pub usertype: u8,
    pub username: String,
    pub name: String,
    pub group_name: String,
    pub path: String,
    pub access: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AdminMe {
    pub uid: u64,
    pub usertype: u8,
    pub username: String,
    pub name: String,
    pub group_name: String,
    pub m_id: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct AdminMenuItem {
    pub id: i64,
    pub keyid: i64,
    pub name: String,
    pub url: String,
    pub path: String,
    pub classname: String,
    pub menu: i32,
    pub sort: i32,
    pub route: String,
}

pub async fn login(
    state: &AppState,
    username: &str,
    password: &str,
    ctx: LoginContext<'_>,
) -> AppResult<AdminLoginResult> {
    let account = username.replace(' ', "");
    let account_key = format!("rl:login:fail:{account}");
    if rate_limit::check_and_incr(
        &state.redis,
        &account_key,
        rate_limit::LimitRule {
            max: 5,
            window: std::time::Duration::from_secs(900),
        },
    )
    .await
    .is_err()
    {
        return Err(ApiError::rate_limit());
    }

    let user = rbac_repo::find_login_user(state.db.reader(), &account)
        .await?
        .ok_or_else(ApiError::bad_credentials)?;
    if user.status != 1 {
        return Err(ApiError::locked());
    }

    let hashed = md5_hex(&md5_hex(password));
    if !hashed.eq_ignore_ascii_case(&user.password) {
        return Err(ApiError::bad_credentials());
    }
    rate_limit::clear_login_fail(&state.redis, &account).await;

    let did = numeric::checked_db(user.did, "phpyun_admin_user.did")?;
    let JwtIssued {
        access,
        refresh: _,
        access_exp,
        refresh_exp,
        jti_access,
        jti_refresh,
    } = issue_pair(&state.config, user.uid, 3, did)?;

    let _ = user_session_service::record_login(
        state,
        LoginRecord {
            uid: user.uid,
            usertype: 3,
            jti_access: &jti_access,
            jti_refresh: &jti_refresh,
            access_exp,
            refresh_exp,
            ip: ctx.ip,
            ua: ctx.ua,
        },
    )
    .await;

    let now = clock::now_ts();
    let _ = rbac_repo::touch_lasttime(state.db.pool(), user.uid, now).await;
    let group_name = rbac_repo::group_name(state.db.reader(), user.m_id)
        .await
        .unwrap_or_default();
    let power = rbac_repo::group_power_ids(state.db.reader(), user.m_id)
        .await
        .unwrap_or_default();
    // PHP adminCommon::admin_get_user_login
    let path = if power.iter().any(|id| *id == 216) && !power.iter().any(|id| *id == 226) {
        "/jobtai".to_string()
    } else {
        "/index".to_string()
    };

    let _ = audit::emit(
        state,
        AuditEvent::new("admin.login", Actor::uid(user.uid))
            .target(format!("admin:{}", user.uid))
            .success(true),
    )
    .await;

    Ok(AdminLoginResult {
        uid: user.uid,
        usertype: 3,
        username: user.username,
        name: user.name,
        group_name,
        path,
        access,
    })
}

pub async fn me(state: &AppState, user: &AuthenticatedUser) -> AppResult<AdminMe> {
    user.require_admin()?;
    let row = rbac_repo::find_by_uid(state.db.reader(), user.uid)
        .await?
        .ok_or_else(ApiError::unauth)?;
    if row.status != 1 {
        return Err(ApiError::locked());
    }
    let group_name = rbac_repo::group_name(state.db.reader(), row.m_id)
        .await
        .unwrap_or_default();
    Ok(AdminMe {
        uid: row.uid,
        usertype: 3,
        username: row.username,
        name: row.name,
        group_name,
        m_id: row.m_id,
    })
}

pub async fn menu(state: &AppState, user: &AuthenticatedUser) -> AppResult<Vec<AdminMenuItem>> {
    user.require_admin()?;
    let row = rbac_repo::find_by_uid(state.db.reader(), user.uid)
        .await?
        .ok_or_else(ApiError::unauth)?;
    let power: HashSet<i64> = rbac_repo::group_power_ids(state.db.reader(), row.m_id)
        .await?
        .into_iter()
        .collect();
    let rows = rbac_repo::list_navigation(state.db.reader()).await?;
    Ok(rows
        .into_iter()
        .filter(|n| power.is_empty() || power.contains(&n.id))
        .map(nav_item)
        .collect())
}

fn nav_item(n: AdminNavRow) -> AdminMenuItem {
    let route = map_php_path(&n.path, &n.url);
    AdminMenuItem {
        id: n.id,
        keyid: n.keyid,
        name: n.name,
        url: n.url,
        path: n.path.clone(),
        classname: n.classname,
        menu: n.menu,
        sort: n.sort,
        route,
    }
}

fn map_php_path(path: &str, _url: &str) -> String {
    let p = path.trim();
    if p.is_empty() || p == "/" {
        String::new()
    } else if p.starts_with('/') {
        p.to_string()
    } else {
        format!("/{p}")
    }
}
