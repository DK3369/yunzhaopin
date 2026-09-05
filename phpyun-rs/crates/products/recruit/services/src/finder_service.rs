//! PHP `finder.model.php` — saved job/resume searcher on `phpyun_finder`.

use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser, Pagination};
use phpyun_models::finder::{self, Finder};
use phpyun_models::site_setting::repo as setting_repo;

pub struct FinderPage {
    pub list: Vec<Finder>,
    pub total: u64,
}

pub struct FinderInput<'a> {
    pub name: &'a str,
    pub keyword: &'a str,
    pub cityid: i32,
    pub minsalary: &'a str,
    pub maxsalary: &'a str,
}

pub async fn list(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<FinderPage> {
    let (total, list) = tokio::join!(
        finder::count_by_uid(state.db.reader(), user.uid),
        finder::list_by_uid(state.db.reader(), user.uid, page.offset, page.limit),
    );
    Ok(FinderPage {
        total: total?,
        list: list?,
    })
}

pub async fn create(
    state: &AppState,
    user: &AuthenticatedUser,
    input: FinderInput<'_>,
) -> AppResult<u64> {
    let name = input.name.trim();
    if name.is_empty() {
        return Err(ApiError::param_invalid("name"));
    }
    let mut parts: Vec<String> = Vec::new();
    if !input.keyword.trim().is_empty() {
        parts.push(format!("keyword={}", input.keyword.trim()));
    }
    if input.cityid > 0 {
        parts.push(format!("cityid={}", input.cityid));
    }
    if !input.minsalary.trim().is_empty() {
        parts.push(format!("minsalary={}", input.minsalary.trim()));
    }
    if !input.maxsalary.trim().is_empty() {
        parts.push(format!("maxsalary={}", input.maxsalary.trim()));
    }
    if parts.is_empty() {
        return Err(ApiError::business("common_00535"));
    }
    let cap_key = if user.usertype == 2 {
        "com_finder"
    } else {
        "user_finder"
    };
    let cap = setting_repo::find(state.db.reader(), cap_key)
        .await?
        .and_then(|r| r.value.trim().parse::<u64>().ok())
        .unwrap_or(0);
    if cap > 0 {
        let n = finder::count_by_uid(state.db.reader(), user.uid).await?;
        if n >= cap {
            return Err(ApiError::business("common_00874"));
        }
    }
    Ok(finder::insert(
        state.db.pool(),
        user.uid,
        i32::from(user.usertype),
        name,
        &parts.join("##"),
        clock::now_ts(),
    )
    .await?)
}

pub async fn delete(state: &AppState, user: &AuthenticatedUser, id: u64) -> AppResult<u64> {
    let n = finder::delete_by_uid(state.db.pool(), id, user.uid).await?;
    if n == 0 {
        return Err(ApiError::business("not_found"));
    }
    Ok(n)
}
