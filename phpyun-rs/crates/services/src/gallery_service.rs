//! Company workplace showcase / personal portfolio (gallery) service.
//!
//! Aligned with PHPYun `member/com/show` + `member/user/show`: CRUD on image galleries.
//! `kind = company`: workplace gallery (usertype=2 only)
//! `kind = resume`: personal portfolio (usertype=1 only)

use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::{AppResult, AppState, AuthenticatedUser, InfraError, Pagination};
use phpyun_models::gallery::entity::{GalleryItem, GalleryKind};
use phpyun_models::gallery::repo as gallery_repo;

pub struct GalleryPage {
    pub list: Vec<GalleryItem>,
    pub total: u64,
}

fn check_role(user: &AuthenticatedUser, kind: GalleryKind) -> AppResult<()> {
    match kind {
        GalleryKind::Company => user.require_employer()?,
        GalleryKind::Resume => user.require_jobseeker()?,
    };
    Ok(())
}

pub async fn list_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    kind: GalleryKind,
    page: Pagination,
) -> AppResult<GalleryPage> {
    check_role(user, kind)?;
    let (total, list) = tokio::join!(
        gallery_repo::count_by_uid(state.db.reader(), kind, user.uid),
        gallery_repo::list_by_uid(state.db.reader(), kind, user.uid, page.offset, page.limit),
    );
    Ok(GalleryPage {
        total: total?,
        list: list?,
    })
}

pub async fn create(
    state: &AppState,
    user: &AuthenticatedUser,
    kind: GalleryKind,
    title: &str,
    picurl: &str,
    sort: i32,
    client_ip: &str,
) -> AppResult<u64> {
    check_role(user, kind)?;
    if picurl.trim().is_empty() {
        return Err(InfraError::InvalidParam("picurl".into()).into());
    }
    let id = gallery_repo::create(state.db.pool(), kind, user.uid, title, picurl, sort).await?;
    let _ = audit::emit(
        state,
        AuditEvent::new("gallery.add", Actor::uid(user.uid).with_ip(client_ip))
            .target(format!("{}:{id}", kind.table())),
    )
    .await;
    Ok(id)
}

pub async fn update(
    state: &AppState,
    user: &AuthenticatedUser,
    kind: GalleryKind,
    id: u64,
    title: Option<&str>,
    picurl: Option<&str>,
    sort: Option<i32>,
) -> AppResult<u64> {
    check_role(user, kind)?;
    Ok(gallery_repo::update(state.db.pool(), kind, id, user.uid, title, picurl, sort).await?)
}

pub async fn delete_mine(
    state: &AppState,
    user: &AuthenticatedUser,
    kind: GalleryKind,
    ids: &[u64],
) -> AppResult<u64> {
    check_role(user, kind)?;
    Ok(gallery_repo::delete_by_ids(state.db.pool(), kind, ids, user.uid).await?)
}
