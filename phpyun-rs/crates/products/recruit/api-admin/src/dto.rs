//! PHP admin list envelope extras: `perPage` / `pageSizes` plus snake aliases.

use phpyun_core::Paged;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// PHP Vue posts mixed string/number keys. Wrapper runs `Validate` without
/// dropping extra fields (they stay in `fields` for the service).
#[derive(Debug, Clone, Default, Deserialize, Validate, ToSchema)]
pub struct PhpLooseBody {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub fields: serde_json::Map<String, serde_json::Value>,
}

impl PhpLooseBody {
    pub fn as_value(&self) -> serde_json::Value {
        serde_json::Value::Object(self.fields.clone())
    }
}

#[derive(Debug, Serialize)]
pub struct AdminPaged<T: Serialize> {
    pub list: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
    #[serde(rename = "perPage")]
    pub per_page: u32,
    #[serde(rename = "pageSize")]
    pub page_size_php: u32,
    pub limit: u32,
    #[serde(rename = "pageSizes")]
    pub page_sizes_camel: Vec<u32>,
    pub page_sizes: Vec<u32>,
}

impl<T: Serialize> From<Paged<T>> for AdminPaged<T> {
    fn from(p: Paged<T>) -> Self {
        let per_page = p.page_size;
        let sizes = vec![10, 20, 50, 100];
        Self {
            list: p.list,
            total: p.total,
            page: p.page,
            page_size: p.page_size,
            per_page,
            page_size_php: per_page,
            limit: per_page,
            page_sizes_camel: sizes.clone(),
            page_sizes: sizes,
        }
    }
}

/// List envelope plus tab counts. `statist` shape matches the sibling `/statist` API.
#[derive(Debug, Serialize, ToSchema)]
pub struct ListWithStat<T: Serialize, S: Serialize> {
    #[serde(flatten)]
    #[schema(value_type = Object)]
    pub page: AdminPaged<T>,
    #[schema(value_type = Object)]
    pub statist: S,
}
