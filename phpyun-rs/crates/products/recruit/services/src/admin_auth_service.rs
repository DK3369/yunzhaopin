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
    /// PHP `$admin_lasttime` formatted `Y-m-d H:i:s`.
    pub last_login: String,
    /// PHP `$power` — admin_navigation ids this group may use.
    pub power: Vec<i64>,
    /// PHP `getMenu` `customizeIds` (custom table, else `menu=2` defaults).
    pub customize_ids: Vec<i64>,
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
    // Namespace so a member account named `admin` cannot lock the admin user.
    let rl_account = format!("admin:{account}");
    // APP_ENV=dev/test skips this lockout (same policy as Governor). Prod keeps
    // 5 failures / 15 minutes. PHP admin had no equivalent Redis lock.
    let lock_fails = !state.config.env.is_dev_or_test();
    if lock_fails {
        rate_limit::check_login_fail(&state.redis, &rl_account).await?;
    }

    let user = match rbac_repo::find_login_user(state.db.reader(), &account).await? {
        Some(u) => u,
        None => {
            if lock_fails {
                rate_limit::record_login_fail(&state.redis, &rl_account).await;
            }
            return Err(ApiError::bad_credentials());
        }
    };
    if user.status != 1 {
        return Err(ApiError::locked());
    }

    let hashed = md5_hex(&md5_hex(password));
    if !hashed.eq_ignore_ascii_case(&user.password) {
        if lock_fails {
            rate_limit::record_login_fail(&state.redis, &rl_account).await;
        }
        return Err(ApiError::bad_credentials());
    }
    if lock_fails {
        rate_limit::clear_login_fail(&state.redis, &rl_account).await;
    }

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

/// JWT `usertype=3` plus a live `phpyun_admin_user.status=1` row.
/// Use on destructive admin writes (delete / purge / update-as-delete).
pub async fn require_active_admin(state: &AppState, actor: &AuthenticatedUser) -> AppResult<()> {
    actor.require_admin()?;
    let row = rbac_repo::find_by_uid(state.db.reader(), actor.uid)
        .await?
        .ok_or_else(ApiError::unauth)?;
    if row.status != 1 {
        return Err(ApiError::locked());
    }
    Ok(())
}

pub async fn me(state: &AppState, user: &AuthenticatedUser) -> AppResult<AdminMe> {
    require_active_admin(state, user).await?;
    let row = rbac_repo::find_by_uid(state.db.reader(), user.uid)
        .await?
        .ok_or_else(ApiError::unauth)?;
    let db = state.db.reader();
    let (group_name, power_vec, last_log, lasttime, stored, navs) = tokio::try_join!(
        rbac_repo::group_name(db, row.m_id),
        rbac_repo::group_power_ids(db, row.m_id),
        rbac_repo::latest_login_log_ctime(db, row.uid),
        rbac_repo::user_lasttime(db, row.uid),
        rbac_repo::customize_nav_ids(db, row.uid),
        rbac_repo::list_navigation(db),
    )?;
    let power: HashSet<i64> = power_vec.iter().copied().collect();
    let customize_ids = resolve_customize_ids(&navs, &power, stored);
    let ts = if last_log > 0 {
        last_log
    } else if lasttime > 0 {
        lasttime
    } else {
        clock::now_ts()
    };
    Ok(AdminMe {
        uid: row.uid,
        usertype: 3,
        username: row.username,
        name: row.name,
        group_name,
        m_id: row.m_id,
        last_login: phpyun_core::utils::fmt_ts(ts, "%Y-%m-%d %H:%M:%S"),
        power: power_vec,
        customize_ids,
    })
}

/// PHP `index::shortcut_menu_action` + `navigation::setCustomizeNav`.
pub async fn save_shortcut_menu(
    state: &AppState,
    user: &AuthenticatedUser,
    chk_value: &[i64],
) -> AppResult<()> {
    require_active_admin(state, user).await?;
    let ids: Vec<i64> = chk_value.iter().copied().filter(|id| *id > 0).collect();
    if ids.is_empty() {
        return Err(ApiError::business("wap_com_00228"));
    }
    let json = serde_json::to_string(&ids).unwrap_or_else(|_| "[]".to_string());
    rbac_repo::upsert_customize_nav(state.db.pool(), user.uid, &json, clock::now_ts()).await?;
    Ok(())
}

/// PHP `getMenu`: custom `nav_ids` in power, else default `menu==2` rows.
fn resolve_customize_ids(
    navs: &[AdminNavRow],
    power: &HashSet<i64>,
    stored: Option<Vec<i64>>,
) -> Vec<i64> {
    let in_power = |id: i64| power.is_empty() || power.contains(&id);
    let default_ids: Vec<i64> = navs
        .iter()
        .filter(|n| n.menu == 2 && in_power(n.id))
        .map(|n| n.id)
        .collect();
    let Some(raw) = stored else {
        return default_ids;
    };
    let allowed: HashSet<i64> = raw.into_iter().filter(|id| in_power(*id)).collect();
    let filtered: Vec<i64> = navs
        .iter()
        .filter(|n| allowed.contains(&n.id))
        .map(|n| n.id)
        .collect();
    if filtered.is_empty() {
        default_ids
    } else {
        filtered
    }
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
