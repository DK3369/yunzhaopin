//! PC left-nav customize (`phpyun_company_nav` JSON + `company.is_nav`).

use phpyun_core::json;
use phpyun_core::{ApiError, AppResult, AppState, AuthenticatedUser};
use phpyun_models::company::repo as company_repo;
use phpyun_models::company_nav::repo as nav_repo;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavItem {
    pub key: String,
    pub label_key: String,
    pub to: String,
    pub sort: i32,
    pub show: bool,
    pub target: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct NavPack {
    pub is_nav: i32,
    pub items: Vec<NavItem>,
}

const ALLOWED: &[&str] = &[
    "/com/jobs",
    "/com/applications",
    "/com/interviews",
    "/com/talent-search",
    "/com/fairs",
    "/com/member-right",
    "/com/profile",
    "/com/binding",
];

fn defaults() -> Vec<NavItem> {
    [
        ("job", "wap_com_00106", "/com/jobs"),
        ("hr", "wap_com_00105", "/com/applications"),
        ("invite", "member_com_00213", "/com/interviews"),
        ("resume", "member_com_00597", "/com/talent-search"),
        ("zhaopinhui", "member_com_00293", "/com/fairs"),
        ("right", "wap_com_00097", "/com/member-right"),
        ("info", "wap_com_00096", "/com/profile"),
        ("binding", "member_user_00059", "/com/binding"),
    ]
    .into_iter()
    .enumerate()
    .map(|(i, (key, label_key, to))| NavItem {
        key: key.into(),
        label_key: label_key.into(),
        to: to.into(),
        sort: i as i32,
        show: true,
        target: "_self".into(),
    })
    .collect()
}

fn sanitize(items: &[NavItem]) -> AppResult<Vec<NavItem>> {
    let mut out = defaults();
    for it in items {
        if let Some(dst) = out.iter_mut().find(|d| d.key == it.key || d.to == it.to) {
            if !ALLOWED.contains(&dst.to.as_str()) {
                return Err(ApiError::param_invalid("to"));
            }
            dst.sort = it.sort;
            dst.show = it.show;
            dst.target = if it.target == "_blank" {
                "_blank".into()
            } else {
                "_self".into()
            };
        }
    }
    out.sort_by_key(|i| i.sort);
    for (i, it) in out.iter_mut().enumerate() {
        it.sort = i as i32;
        if !ALLOWED.contains(&it.to.as_str()) {
            return Err(ApiError::param_invalid("to"));
        }
    }
    Ok(out)
}

pub async fn get(state: &AppState, user: &AuthenticatedUser) -> AppResult<NavPack> {
    user.require_employer()?;
    let is_nav = company_repo::read_is_nav(state.db.reader(), user.uid).await?;
    if is_nav != 2 {
        return Ok(NavPack {
            is_nav: if is_nav == 0 { 1 } else { is_nav },
            items: defaults(),
        });
    }
    let items = match nav_repo::find_nav_info(state.db.reader(), user.uid).await? {
        Some(raw) => {
            let parsed: Vec<NavItem> = json::from_str(&raw).unwrap_or_default();
            sanitize(&parsed)?
        }
        None => defaults(),
    };
    Ok(NavPack { is_nav: 2, items })
}

pub async fn save(state: &AppState, user: &AuthenticatedUser, items: &[NavItem]) -> AppResult<()> {
    user.require_employer()?;
    if user.is_sub_account() {
        return Err(ApiError::business("sub_account_forbidden"));
    }
    let items = sanitize(items)?;
    let raw = json::to_string(&items)?;
    nav_repo::upsert_nav_info(state.db.pool(), user.uid, &raw).await?;
    company_repo::set_is_nav(state.db.pool(), user.uid, 2).await?;
    Ok(())
}

pub async fn reset(state: &AppState, user: &AuthenticatedUser) -> AppResult<()> {
    user.require_employer()?;
    if user.is_sub_account() {
        return Err(ApiError::business("sub_account_forbidden"));
    }
    nav_repo::delete_nav(state.db.pool(), user.uid).await?;
    company_repo::set_is_nav(state.db.pool(), user.uid, 1).await?;
    Ok(())
}
