//! PHP admin list envelope extras: `perPage` / `pageSizes` (additive).

use phpyun_core::Paged;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AdminPaged<T: Serialize> {
    pub list: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
    #[serde(rename = "perPage")]
    pub per_page: u32,
    #[serde(rename = "pageSizes")]
    pub page_sizes: Vec<u32>,
}

impl<T: Serialize> From<Paged<T>> for AdminPaged<T> {
    fn from(p: Paged<T>) -> Self {
        let per_page = p.page_size;
        Self {
            list: p.list,
            total: p.total,
            page: p.page,
            page_size: p.page_size,
            per_page,
            page_sizes: vec![10, 20, 50, 100],
        }
    }
}
