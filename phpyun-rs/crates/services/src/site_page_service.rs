//! Site static pages (about / privacy / protocol / contact / appDown).

use phpyun_core::{AppResult, AppState};
use phpyun_models::site_page::{entity::SitePage, repo as site_page_repo};

pub async fn get(state: &AppState, code: &str) -> AppResult<Option<SitePage>> {
    Ok(site_page_repo::find_by_code(state.db.reader(), code).await?)
}
