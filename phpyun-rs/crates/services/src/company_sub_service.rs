//! Company sub-pages (products / news).
//!
//! - Public access: only `status=1` rows are visible.
//! - Company self-management: list/create/update/delete, requires `user.uid == uid` (enforced by `require_employer()` at the handler layer).

use phpyun_core::error::InfraError;
use phpyun_core::{background, clock, AppError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::company_sub::{
    entity::{CompanyNews, CompanyProduct},
    repo as sub_repo,
};

// ---------- Products ----------

pub async fn list_products(
    state: &AppState,
    com_uid: u64,
    page: Pagination,
) -> AppResult<Paged<CompanyProduct>> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        sub_repo::list_products_public(db, com_uid, page.offset, page.limit),
        sub_repo::count_products_public(db, com_uid),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

pub async fn get_product(
    state: &AppState,
    com_uid: u64,
    id: u64,
) -> AppResult<CompanyProduct> {
    sub_repo::find_product_public(state.db.reader(), com_uid, id)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("product_not_found".into())))
}

pub async fn list_own_products(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<Paged<CompanyProduct>> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        sub_repo::list_products_own(db, user.uid, page.offset, page.limit),
        sub_repo::count_products_own(db, user.uid),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

pub struct ProductInput<'a> {
    pub title: &'a str,
    pub cover: &'a str,
    pub body: &'a str,
    pub sort: i32,
}

pub async fn create_product(
    state: &AppState,
    user: &AuthenticatedUser,
    input: ProductInput<'_>,
) -> AppResult<u64> {
    let id = sub_repo::create_product(
        state.db.pool(),
        sub_repo::ProductCreate {
            uid: user.uid,
            title: input.title,
            cover: input.cover,
            body: input.body,
            sort: input.sort,
        },
        clock::now_ts(),
    )
    .await?;
    Ok(id)
}

pub struct ProductUpdateInput<'a> {
    pub title: Option<&'a str>,
    pub cover: Option<&'a str>,
    pub body: Option<&'a str>,
    pub sort: Option<i32>,
    pub status: Option<i32>,
}

pub async fn update_product(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
    input: ProductUpdateInput<'_>,
) -> AppResult<()> {
    let affected = sub_repo::update_product(
        state.db.pool(),
        id,
        user.uid,
        sub_repo::ProductUpdate {
            title: input.title,
            cover: input.cover,
            body: input.body,
            sort: input.sort,
            status: input.status,
        },
        clock::now_ts(),
    )
    .await?;
    if affected == 0 {
        return Err(AppError::new(InfraError::Forbidden));
    }
    Ok(())
}

pub async fn delete_product(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
) -> AppResult<()> {
    let affected = sub_repo::delete_product(state.db.pool(), id, user.uid).await?;
    if affected == 0 {
        return Err(AppError::new(InfraError::Forbidden));
    }
    Ok(())
}

// ---------- News ----------

pub async fn list_news(
    state: &AppState,
    com_uid: u64,
    page: Pagination,
) -> AppResult<Paged<CompanyNews>> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        sub_repo::list_news_public(db, com_uid, page.offset, page.limit),
        sub_repo::count_news_public(db, com_uid),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

pub async fn get_news(
    state: &AppState,
    com_uid: u64,
    id: u64,
) -> AppResult<CompanyNews> {
    let n = sub_repo::find_news_public(state.db.reader(), com_uid, id)
        .await?
        .ok_or_else(|| AppError::new(InfraError::InvalidParam("news_not_found".into())))?;
    // hits +1 asynchronously
    let pool = state.db.pool().clone();
    let id_bg = id;
    background::spawn_best_effort("company_news.hit", async move {
        let _ = sub_repo::incr_news_hit(&pool, id_bg).await;
    });
    Ok(n)
}

pub async fn list_own_news(
    state: &AppState,
    user: &AuthenticatedUser,
    page: Pagination,
) -> AppResult<Paged<CompanyNews>> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        sub_repo::list_news_own(db, user.uid, page.offset, page.limit),
        sub_repo::count_news_own(db, user.uid),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

pub struct NewsInput<'a> {
    pub title: &'a str,
    pub summary: &'a str,
    pub body: &'a str,
}

pub async fn create_news(
    state: &AppState,
    user: &AuthenticatedUser,
    input: NewsInput<'_>,
) -> AppResult<u64> {
    let id = sub_repo::create_news(
        state.db.pool(),
        sub_repo::NewsCreate {
            uid: user.uid,
            title: input.title,
            summary: input.summary,
            body: input.body,
        },
        clock::now_ts(),
    )
    .await?;
    Ok(id)
}

pub struct NewsUpdateInput<'a> {
    pub title: Option<&'a str>,
    pub summary: Option<&'a str>,
    pub body: Option<&'a str>,
    pub status: Option<i32>,
}

pub async fn update_news(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
    input: NewsUpdateInput<'_>,
) -> AppResult<()> {
    let affected = sub_repo::update_news(
        state.db.pool(),
        id,
        user.uid,
        sub_repo::NewsUpdate {
            title: input.title,
            summary: input.summary,
            body: input.body,
            status: input.status,
        },
        clock::now_ts(),
    )
    .await?;
    if affected == 0 {
        return Err(AppError::new(InfraError::Forbidden));
    }
    Ok(())
}

pub async fn delete_news(
    state: &AppState,
    user: &AuthenticatedUser,
    id: u64,
) -> AppResult<()> {
    let affected = sub_repo::delete_news(state.db.pool(), id, user.uid).await?;
    if affected == 0 {
        return Err(AppError::new(InfraError::Forbidden));
    }
    Ok(())
}
