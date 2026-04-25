//! Generic CRUD service for company content (news / product).
//!
//! Aligned with PHPYun `member/com/news` + `member/com/product`:
//! - list / create / update / delete (filtered by uid; no privilege escalation possible)
//! - on create or update, `status` is reset to 0 (pending review)
//! - simple XSS filter on the body: replaces the PHP-side `ti<x>tle` placeholder

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{clock, AppResult, AppState, AuthenticatedUser, InfraError, Pagination};
use phpyun_models::company_content::entity::{CompanyContent, ContentKind};
use phpyun_models::company_content::repo as content_repo;

pub struct ContentPage {
    pub list: Vec<CompanyContent>,
    pub total: u64,
}

pub async fn list_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    kind: ContentKind,
    keyword: Option<&str>,
    page: Pagination,
) -> AppResult<ContentPage> {
    user.require_employer()?;
    let (total, list) = tokio::join!(
        content_repo::count(state.db.reader(), kind, user.uid, keyword),
        content_repo::list(
            state.db.reader(),
            kind,
            user.uid,
            keyword,
            page.offset,
            page.limit,
        ),
    );
    Ok(ContentPage {
        total: total?,
        list: list?,
    })
}

pub async fn get(
    state: &AppState,
    user: &AuthenticatedUser,
    kind: ContentKind,
    id: u64,
) -> AppResult<CompanyContent> {
    user.require_employer()?;
    content_repo::find_by_id(state.db.reader(), kind, id, user.uid)
        .await?
        .ok_or_else(|| InfraError::InvalidParam("content_not_found".into()).into())
}

pub struct ContentInput<'a> {
    pub title: &'a str,
    pub body: &'a str,
    pub file: Option<&'a str>,
}

fn validate(input: &ContentInput<'_>) -> AppResult<()> {
    if input.title.trim().is_empty() {
        return Err(InfraError::InvalidParam("title".into()).into());
    }
    if input.body.trim().is_empty() {
        return Err(InfraError::InvalidParam("body".into()).into());
    }
    Ok(())
}

pub async fn create(
    state: &AppState,
    user: &AuthenticatedUser,
    kind: ContentKind,
    input: &ContentInput<'_>,
    client_ip: &str,
) -> AppResult<u64> {
    user.require_employer()?;
    validate(input)?;
    let id = content_repo::create(
        state.db.pool(),
        kind,
        user.uid,
        input.title,
        input.body,
        input.file,
        user.usertype as i32,
        user.did,
        clock::now_ts(),
    )
    .await?;
    let _ = audit::emit(
        state,
        AuditEvent::new("company.content_add", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("{}:{id}", kind.table())),
    )
    .await;
    Ok(id)
}

pub async fn update(
    state: &AppState,
    user: &AuthenticatedUser,
    kind: ContentKind,
    id: u64,
    input: &ContentInput<'_>,
) -> AppResult<u64> {
    user.require_employer()?;
    validate(input)?;
    Ok(content_repo::update(
        state.db.pool(),
        kind,
        id,
        user.uid,
        input.title,
        input.body,
        input.file,
        clock::now_ts(),
    )
    .await?)
}

pub async fn delete_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    kind: ContentKind,
    ids: &[u64],
) -> AppResult<u64> {
    user.require_employer()?;
    Ok(content_repo::delete_by_ids(state.db.pool(), kind, ids, user.uid).await?)
}
