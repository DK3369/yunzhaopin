//! On-disk CMS cache (PHP `desc.cache.php`, plus HTML bodies).
//!
//! Public reads open files. Writes happen only from admin save / generate-cache.
//! Not Vue `zh.json` / `en.json` and not Rust `locales/`.

use phpyun_core::i18n::Lang;
use phpyun_core::{ApiError, AppResult};
use phpyun_models::description::entity::{DescClass, Description};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmsClass {
    pub id: u64,
    pub name: String,
    pub sort: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CmsPageMeta {
    pub id: u64,
    pub class_id: u64,
    pub name: String,
    pub title: String,
    pub url: String,
    pub is_type: i32,
    pub is_nav: i32,
    pub sort: i32,
    pub status: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CmsMeta {
    #[serde(default)]
    pub classes: Vec<CmsClass>,
    #[serde(default)]
    pub pages: Vec<CmsPageMeta>,
}

pub fn root() -> PathBuf {
    if let Ok(p) = std::env::var("CMS_PAGES_DIR") {
        let t = p.trim();
        if !t.is_empty() {
            return PathBuf::from(t);
        }
    }
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let cand = cwd.join("cms-pages");
    if cand.is_dir() {
        return cand;
    }
    PathBuf::from("/www/wwwroot/zzzz.com/phpyun-rs/cms-pages")
}

fn lang_dir(lang: Lang) -> PathBuf {
    root().join(lang.as_str())
}

fn meta_path(lang: Lang) -> PathBuf {
    lang_dir(lang).join("meta.json")
}

fn html_path(lang: Lang, id: u64) -> PathBuf {
    lang_dir(lang).join(format!("{id}.html"))
}

async fn atomic_write(path: &Path, bytes: &[u8]) -> AppResult<()> {
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(ApiError::internal)?;
    }
    let tmp = path.with_extension("tmp");
    tokio::fs::write(&tmp, bytes)
        .await
        .map_err(ApiError::internal)?;
    tokio::fs::rename(&tmp, path)
        .await
        .map_err(ApiError::internal)?;
    Ok(())
}

pub async fn read_meta(lang: Lang) -> Option<CmsMeta> {
    let text = tokio::fs::read_to_string(meta_path(lang)).await.ok()?;
    serde_json::from_str(&text).ok()
}

pub async fn write_meta(lang: Lang, meta: &CmsMeta) -> AppResult<()> {
    let body = serde_json::to_vec_pretty(meta).map_err(ApiError::internal)?;
    atomic_write(&meta_path(lang), &body).await
}

pub async fn read_html(lang: Lang, id: u64) -> Option<String> {
    tokio::fs::read_to_string(html_path(lang, id)).await.ok()
}

pub async fn write_html(lang: Lang, id: u64, html: &str) -> AppResult<()> {
    atomic_write(&html_path(lang, id), html.as_bytes()).await
}

async fn load_or_clone_zh(lang: Lang) -> CmsMeta {
    if let Some(m) = read_meta(lang).await {
        return m;
    }
    if lang != Lang::ZhCN {
        if let Some(m) = read_meta(Lang::ZhCN).await {
            return m;
        }
    }
    CmsMeta::default()
}

pub async fn upsert_page_meta(lang: Lang, page: CmsPageMeta) -> AppResult<()> {
    let mut meta = load_or_clone_zh(lang).await;
    if let Some(row) = meta.pages.iter_mut().find(|p| p.id == page.id) {
        *row = page;
    } else {
        meta.pages.push(page);
    }
    write_meta(lang, &meta).await
}

pub async fn upsert_class_meta(lang: Lang, class: CmsClass) -> AppResult<()> {
    let mut meta = load_or_clone_zh(lang).await;
    if let Some(row) = meta.classes.iter_mut().find(|c| c.id == class.id) {
        if !class.name.is_empty() {
            row.name = class.name;
        }
        row.sort = class.sort;
    } else {
        meta.classes.push(class);
    }
    write_meta(lang, &meta).await
}

pub async fn set_page_sort(id: u64, sort: i32) -> AppResult<()> {
    for lang in [Lang::ZhCN, Lang::En] {
        let Some(mut meta) = read_meta(lang).await else {
            continue;
        };
        let Some(row) = meta.pages.iter_mut().find(|p| p.id == id) else {
            continue;
        };
        row.sort = sort;
        write_meta(lang, &meta).await?;
    }
    Ok(())
}

pub async fn remove_page(id: u64) -> AppResult<()> {
    for lang in [Lang::ZhCN, Lang::En] {
        let html = html_path(lang, id);
        let _ = tokio::fs::remove_file(&html).await;
        if let Some(mut meta) = read_meta(lang).await {
            meta.pages.retain(|p| p.id != id);
            write_meta(lang, &meta).await?;
        }
    }
    Ok(())
}

pub async fn remove_class(id: u64) -> AppResult<()> {
    for lang in [Lang::ZhCN, Lang::En] {
        if let Some(mut meta) = read_meta(lang).await {
            meta.classes.retain(|c| c.id != id);
            write_meta(lang, &meta).await?;
        }
    }
    Ok(())
}

fn pick<'a>(a: &'a str, b: &'a str) -> &'a str {
    if !a.is_empty() {
        a
    } else {
        b
    }
}

fn page_from_meta(p: &CmsPageMeta, content: String) -> Description {
    Description {
        id: p.id,
        class_id: p.class_id,
        name: p.name.clone(),
        title: p.title.clone(),
        content,
        is_nav: p.is_nav,
        is_type: p.is_type,
        link_url: p.url.clone(),
        sort: p.sort,
        status: p.status,
        created_at: 0,
        updated_at: 0,
    }
}

pub async fn load_page(lang: Lang, id: u64) -> Option<Description> {
    let mut structural: Option<CmsPageMeta> = None;
    let mut name = String::new();
    let mut title = String::new();
    let mut content = String::new();
    for l in lang.fallback_chain() {
        if let Some(meta) = read_meta(*l).await {
            if let Some(p) = meta.pages.iter().find(|p| p.id == id) {
                if structural.is_none() {
                    structural = Some(p.clone());
                }
                if name.is_empty() && !p.name.is_empty() {
                    name = p.name.clone();
                }
                if title.is_empty() && !p.title.is_empty() {
                    title = p.title.clone();
                }
            }
        }
        if content.is_empty() {
            if let Some(html) = read_html(*l, id).await {
                if !html.trim().is_empty() {
                    content = html;
                }
            }
        }
    }
    let mut p = structural?;
    p.name = pick(&name, &p.name).to_string();
    p.title = pick(&title, &p.title).to_string();
    Some(page_from_meta(&p, content))
}

pub async fn overlay_classes(lang: Lang, base: Vec<DescClass>) -> Vec<DescClass> {
    let Some(meta) = read_meta(lang).await else {
        return base;
    };
    base.into_iter()
        .map(|mut c| {
            if let Some(row) = meta.classes.iter().find(|x| x.id == c.id) {
                if !row.name.is_empty() {
                    c.name = row.name.clone();
                }
            }
            c
        })
        .collect()
}

pub async fn classes_from_files(lang: Lang) -> Option<Vec<DescClass>> {
    let mut classes = read_meta(Lang::ZhCN).await?.classes;
    if lang != Lang::ZhCN {
        if let Some(en) = read_meta(lang).await {
            for c in &mut classes {
                if let Some(row) = en.classes.iter().find(|x| x.id == c.id) {
                    if !row.name.is_empty() {
                        c.name = row.name.clone();
                    }
                }
            }
        }
    }
    classes.sort_by(|a, b| a.sort.cmp(&b.sort).then(a.id.cmp(&b.id)));
    Some(
        classes
            .into_iter()
            .map(|c| DescClass {
                id: c.id,
                name: c.name,
                sort: c.sort,
                created_at: 0,
            })
            .collect(),
    )
}

pub async fn pages_from_files(
    lang: Lang,
    class_id: Option<u64>,
    only_visible: bool,
) -> Option<Vec<Description>> {
    let mut pages = read_meta(Lang::ZhCN).await?.pages;
    if lang != Lang::ZhCN {
        if let Some(en) = read_meta(lang).await {
            for p in &mut pages {
                if let Some(row) = en.pages.iter().find(|x| x.id == p.id) {
                    if !row.name.is_empty() {
                        p.name = row.name.clone();
                    }
                    if !row.title.is_empty() {
                        p.title = row.title.clone();
                    }
                }
            }
        }
    }
    pages.retain(|p| {
        if only_visible && p.is_nav != 1 {
            return false;
        }
        if let Some(cid) = class_id {
            return p.class_id == cid;
        }
        true
    });
    pages.sort_by(|a, b| a.sort.cmp(&b.sort).then(b.id.cmp(&a.id)));
    Some(
        pages
            .into_iter()
            .map(|p| page_from_meta(&p, String::new()))
            .collect(),
    )
}

pub async fn find_id_by_name(name: &str) -> Option<u64> {
    let needle = name.trim();
    if needle.is_empty() {
        return None;
    }
    for lang in [Lang::En, Lang::ZhCN] {
        if let Some(meta) = read_meta(lang).await {
            if let Some(p) = meta.pages.iter().find(|p| p.name == needle) {
                return Some(p.id);
            }
        }
    }
    None
}

pub async fn class_name(lang: Lang, id: u64) -> String {
    if let Some(meta) = read_meta(lang).await {
        if let Some(c) = meta.classes.iter().find(|c| c.id == id) {
            return c.name.clone();
        }
    }
    String::new()
}

pub async fn page_triple(lang: Lang, id: u64) -> Option<(String, String, String)> {
    let meta = read_meta(lang).await?;
    let p = meta.pages.iter().find(|p| p.id == id)?;
    let html = read_html(lang, id).await.unwrap_or_default();
    Some((p.name.clone(), p.title.clone(), html))
}
