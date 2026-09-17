//! Generic single-page CMS (aligned with PHPYun `description` + `desc_class`).
//!
//! Complements `site_page_service` (fixed about/privacy/contact codes).
//! Public reads use on-disk `cms-pages/{lang}/` (PHP `desc.cache.php` analog).
//! Admin still writes the Chinese row, then regenerates those files.

use crate::cms_page_files::{self, CmsClass, CmsPageMeta};
use phpyun_core::audit::{self, Actor, AuditEvent};
use phpyun_core::cache::SimpleCache;
use phpyun_core::i18n::{current_lang, Lang};
use phpyun_core::{clock, ApiError, AppResult, AppState, AuthenticatedUser, Paged, Pagination};
use phpyun_models::description::{
    entity::{DescClass, Description},
    repo as desc_repo,
};

static CLASSES_CACHE: std::sync::OnceLock<SimpleCache<(), Vec<DescClass>>> =
    std::sync::OnceLock::new();

fn classes_cache() -> &'static SimpleCache<(), Vec<DescClass>> {
    CLASSES_CACHE.get_or_init(|| SimpleCache::new(1, std::time::Duration::from_secs(60)))
}

pub async fn invalidate_classes_cache() {
    if let Some(c) = CLASSES_CACHE.get() {
        c.invalidate(&()).await;
    }
}

pub async fn list_classes(state: &AppState) -> AppResult<std::sync::Arc<Vec<DescClass>>> {
    let cache = classes_cache();
    let db = state.db.reader().clone();
    cache
        .get_or_load((), move || async move {
            Ok(desc_repo::list_classes(&db).await?)
        })
        .await
}

pub async fn public_list_classes(state: &AppState) -> AppResult<Vec<DescClass>> {
    let lang = current_lang();
    if let Some(rows) = cms_page_files::classes_from_files(lang).await {
        return Ok(rows);
    }
    let raw = list_classes(state).await?;
    Ok(cms_page_files::overlay_classes(lang, raw.as_ref().clone()).await)
}

pub async fn create_class(
    state: &AppState,
    admin: &AuthenticatedUser,
    name: &str,
    sort: i32,
) -> AppResult<u64> {
    let id = desc_repo::insert_class(state.db.pool(), name, sort, clock::now_ts()).await?;
    cms_page_files::upsert_class_meta(
        Lang::ZhCN,
        CmsClass {
            id,
            name: name.to_string(),
            sort,
        },
    )
    .await?;
    invalidate_classes_cache().await;
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.desc_class.create", Actor::uid(admin.uid))
            .target(format!("desc_class:{id}")),
    )
    .await;
    Ok(id)
}

pub async fn update_class_sort(
    state: &AppState,
    admin: &AuthenticatedUser,
    id: u64,
    sort: i32,
) -> AppResult<()> {
    let n = desc_repo::update_class_sort(state.db.pool(), id, sort).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("class_not_found"));
    }
    cms_page_files::upsert_class_meta(
        Lang::ZhCN,
        CmsClass {
            id,
            name: String::new(),
            sort,
        },
    )
    .await?;
    invalidate_classes_cache().await;
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.desc_class.update_sort", Actor::uid(admin.uid))
            .target(format!("desc_class:{id}"))
            .meta(&serde_json::json!({ "sort": sort })),
    )
    .await;
    Ok(())
}

pub async fn delete_class(state: &AppState, admin: &AuthenticatedUser, id: u64) -> AppResult<()> {
    let n = desc_repo::delete_class(state.db.pool(), id).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("class_not_found"));
    }
    cms_page_files::remove_class(id).await?;
    invalidate_classes_cache().await;
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.desc_class.delete", Actor::uid(admin.uid))
            .target(format!("desc_class:{id}")),
    )
    .await;
    Ok(())
}

pub async fn list(
    state: &AppState,
    class_id: Option<u64>,
    only_visible: bool,
    page: Pagination,
) -> AppResult<Paged<Description>> {
    let db = state.db.reader();
    let (list, total) = tokio::join!(
        desc_repo::list(db, class_id, only_visible, page.offset, page.limit),
        desc_repo::count(db, class_id, only_visible),
    );
    Ok(Paged::new(list?, total?, page.page, page.page_size))
}

pub async fn public_list(
    state: &AppState,
    class_id: Option<u64>,
    only_visible: bool,
    page: Pagination,
) -> AppResult<Paged<Description>> {
    if let Some(mut rows) = cms_page_files::pages_from_files(current_lang(), class_id, only_visible).await
    {
        let total = rows.len() as u64;
        let start = page.offset.min(total) as usize;
        let end = (start + page.limit as usize).min(rows.len());
        rows = rows.drain(start..end).collect();
        return Ok(Paged::new(rows, total, page.page, page.page_size));
    }
    list(state, class_id, only_visible, page).await
}

pub async fn get(state: &AppState, id: u64) -> AppResult<Description> {
    let mut d = if let Some(d) = cms_page_files::load_page(current_lang(), id).await {
        d
    } else {
        load_page(state, id).await?
    };
    d.content = phpyun_core::html::sanitize_html(&d.content);
    Ok(d)
}

pub async fn public_get_by_name(state: &AppState, name: &str) -> AppResult<Description> {
    if let Some(id) = cms_page_files::find_id_by_name(name).await {
        return get(state, id).await;
    }
    let row = desc_repo::find_by_name(state.db.reader(), name)
        .await?
        .ok_or_else(|| ApiError::param_invalid("description_not_found"))?;
    if let Some(d) = cms_page_files::load_page(current_lang(), row.id).await {
        let mut d = d;
        d.content = phpyun_core::html::sanitize_html(&d.content);
        return Ok(d);
    }
    let mut row = row;
    row.content = phpyun_core::html::sanitize_html(&row.content);
    Ok(row)
}

pub async fn public_legal(state: &AppState, slug: &str) -> AppResult<Description> {
    let names: &[&str] = match slug {
        "about" => &["关于我们", "About Us", "About"],
        "contact" => &["联系我们", "Contact Us", "Contact"],
        "privacy" => &["隐私政策", "Privacy Policy", "Privacy"],
        "protocol" => &["注册协议", "Registration Agreement", "Terms"],
        _ => return Err(ApiError::param_invalid(format!("slug: {slug}"))),
    };
    for name in names {
        if let Ok(d) = public_get_by_name(state, name).await {
            return Ok(d);
        }
    }
    Err(ApiError::param_invalid("description_not_found"))
}

async fn load_page(state: &AppState, id: u64) -> AppResult<Description> {
    desc_repo::get(state.db.reader(), id)
        .await?
        .ok_or_else(|| ApiError::param_invalid("description_not_found"))
}

pub struct UpsertForm<'a> {
    pub id: Option<u64>,
    pub class_id: u64,
    pub title: &'a str,
    pub content: &'a str,
    pub is_type: i32,
    pub link_url: &'a str,
    pub sort: i32,
    pub status: i32,
}

pub async fn upsert(
    state: &AppState,
    admin: &AuthenticatedUser,
    f: &UpsertForm<'_>,
) -> AppResult<u64> {
    phpyun_core::validators::ensure_http_or_site_url(f.link_url)?;
    let content = phpyun_core::html::sanitize_html(f.content);
    let id = desc_repo::upsert(
        state.db.pool(),
        &desc_repo::UpsertDesc {
            id: f.id,
            class_id: f.class_id,
            title: f.title,
            content: &content,
            is_type: f.is_type,
            link_url: f.link_url,
            sort: f.sort,
            status: f.status,
        },
        clock::now_ts(),
    )
    .await?;
    write_page_zh(
        id,
        CmsPageMeta {
            id,
            class_id: f.class_id,
            name: f.title.to_string(),
            title: f.title.to_string(),
            url: f.link_url.to_string(),
            is_type: f.is_type,
            is_nav: 1,
            sort: f.sort,
            status: f.status,
        },
        f.content,
    )
    .await?;
    invalidate_classes_cache().await;
    let _ = audit::emit(
        state,
        AuditEvent::new(
            if f.id.is_some() {
                "admin.description.update"
            } else {
                "admin.description.create"
            },
            Actor::uid(admin.uid),
        )
        .target(format!("description:{id}")),
    )
    .await;
    Ok(id)
}

pub async fn delete(state: &AppState, admin: &AuthenticatedUser, id: u64) -> AppResult<()> {
    let n = desc_repo::delete(state.db.pool(), id).await?;
    if n == 0 {
        return Err(ApiError::param_invalid("description_not_found"));
    }
    cms_page_files::remove_page(id).await?;
    invalidate_classes_cache().await;
    let _ = audit::emit(
        state,
        AuditEvent::new("admin.description.delete", Actor::uid(admin.uid))
            .target(format!("description:{id}")),
    )
    .await;
    Ok(())
}

pub struct PageFileWrite<'a> {
    pub name: &'a str,
    pub title: &'a str,
    pub content: &'a str,
    pub class_id: u64,
    pub url: &'a str,
    pub is_type: i32,
    pub is_nav: i32,
    pub sort: i32,
}

pub async fn write_page_zh(id: u64, meta: CmsPageMeta, html: &str) -> AppResult<()> {
    cms_page_files::write_html(Lang::ZhCN, id, html).await?;
    cms_page_files::upsert_page_meta(Lang::ZhCN, meta).await
}

pub async fn write_page_lang(lang: Lang, meta: CmsPageMeta, html: &str) -> AppResult<()> {
    if !html.is_empty() {
        cms_page_files::write_html(lang, id_of(&meta), html).await?;
    }
    cms_page_files::upsert_page_meta(lang, meta).await
}

fn id_of(meta: &CmsPageMeta) -> u64 {
    meta.id
}

pub async fn save_php_page(id: u64, zh: PageFileWrite<'_>, en: Option<PageFileWrite<'_>>) -> AppResult<()> {
    write_page_zh(
        id,
        CmsPageMeta {
            id,
            class_id: zh.class_id,
            name: zh.name.to_string(),
            title: zh.title.to_string(),
            url: zh.url.to_string(),
            is_type: zh.is_type,
            is_nav: zh.is_nav,
            sort: zh.sort,
            status: 1,
        },
        zh.content,
    )
    .await?;
    if let Some(en) = en {
        let has = !en.name.is_empty() || !en.title.is_empty() || !en.content.is_empty();
        if has {
            write_page_lang(
                Lang::En,
                CmsPageMeta {
                    id,
                    class_id: en.class_id,
                    name: en.name.to_string(),
                    title: en.title.to_string(),
                    url: en.url.to_string(),
                    is_type: en.is_type,
                    is_nav: en.is_nav,
                    sort: en.sort,
                    status: 1,
                },
                en.content,
            )
            .await?;
        }
    }
    invalidate_classes_cache().await;
    Ok(())
}

pub async fn save_class_zh(id: u64, name: Option<&str>, sort: Option<i32>) -> AppResult<()> {
    let existing = cms_page_files::read_meta(Lang::ZhCN)
        .await
        .and_then(|m| m.classes.into_iter().find(|c| c.id == id));
    let name = name
        .map(str::to_string)
        .or_else(|| existing.as_ref().map(|c| c.name.clone()))
        .unwrap_or_default();
    let sort = sort.unwrap_or_else(|| existing.map(|c| c.sort).unwrap_or(0));
    cms_page_files::upsert_class_meta(Lang::ZhCN, CmsClass { id, name, sort }).await?;
    invalidate_classes_cache().await;
    Ok(())
}

pub async fn save_class_en(id: u64, name: &str) -> AppResult<()> {
    if name.is_empty() {
        return Ok(());
    }
    let sort = cms_page_files::read_meta(Lang::ZhCN)
        .await
        .and_then(|m| m.classes.into_iter().find(|c| c.id == id).map(|c| c.sort))
        .unwrap_or(0);
    cms_page_files::upsert_class_meta(
        Lang::En,
        CmsClass {
            id,
            name: name.to_string(),
            sort,
        },
    )
    .await?;
    Ok(())
}

pub async fn page_i18n(id: u64, lang: Lang) -> Option<(String, String, String)> {
    cms_page_files::page_triple(lang, id).await
}

pub async fn class_i18n_name(id: u64, lang: Lang) -> String {
    cms_page_files::class_name(lang, id).await
}

pub async fn generate_zh_from_db(state: &AppState) -> AppResult<()> {
    let classes = desc_repo::php_list_all_classes(state.db.reader()).await?;
    let pages = desc_repo::php_list_all(state.db.reader()).await?;
    let meta = cms_page_files::CmsMeta {
        classes: classes
            .into_iter()
            .map(|c| CmsClass {
                id: c.id,
                name: c.name,
                sort: c.sort,
            })
            .collect(),
        pages: pages
            .iter()
            .map(|p| CmsPageMeta {
                id: p.id,
                class_id: p.nid,
                name: p.name.clone(),
                title: p.title.clone(),
                url: p.url.clone(),
                is_type: p.is_type,
                is_nav: p.is_nav,
                sort: p.sort,
                status: 1,
            })
            .collect(),
    };
    for p in &pages {
        cms_page_files::write_html(Lang::ZhCN, p.id, &p.content).await?;
    }
    cms_page_files::write_meta(Lang::ZhCN, &meta).await?;
    invalidate_classes_cache().await;
    Ok(())
}

pub async fn remove_pages(ids: &[u64]) -> AppResult<()> {
    for id in ids {
        cms_page_files::remove_page(*id).await?;
    }
    invalidate_classes_cache().await;
    Ok(())
}

pub async fn remove_class_files(id: u64) -> AppResult<()> {
    cms_page_files::remove_class(id).await?;
    invalidate_classes_cache().await;
    Ok(())
}

pub async fn set_page_sort(id: u64, sort: i32) -> AppResult<()> {
    cms_page_files::set_page_sort(id, sort).await
}

