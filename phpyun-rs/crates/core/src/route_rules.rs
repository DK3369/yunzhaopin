//! HTTP policy that varies per route, declared by the modules that own those
//! routes instead of hardcoded in the middleware.
//!
//! The middleware stack needs to answer two questions about a request path:
//!
//! 1. *Is this a business API path?* — those are POST-only and every framework
//!    rejection on them is rewritten into the JSON envelope. Ops probes and the
//!    Swagger UI are not.
//! 2. *Is GET allowed here anyway?* — a handful of endpoints implement
//!    third-party protocols (OAuth callbacks, webhook handshakes) whose verb we
//!    do not control.
//!
//! Both used to be `if path.starts_with("/v1/")` and a literal match on two
//! paths inside `middleware.rs`. That meant every new API namespace had to edit
//! the middleware, and the middleware — which is generic infrastructure — had
//! to know product-specific URLs. [`RouteRules`] inverts that: the router
//! assembles the rules from its namespaces and hands them to `install`.
//!
//! ```
//! # use phpyun_core::route_rules::RouteRules;
//! let rules = RouteRules::new()
//!     .api_namespace("/v1")
//!     .api_namespace("/v2")
//!     .allow_get("/v1/wap/wechat/callback");
//!
//! assert!(rules.is_api_path("/v1/wap/jobs"));
//! assert!(!rules.is_api_path("/health"));
//! assert!(rules.allows_get("/v1/wap/wechat/callback"));
//! assert!(!rules.allows_get("/v1/wap/jobs"));
//! ```

use std::collections::HashSet;

/// Per-path HTTP policy for the middleware stack. Cheap to clone at build time,
/// then shared behind an `Arc` for the process lifetime.
#[derive(Clone, Debug, Default)]
pub struct RouteRules {
    /// Normalized to always carry a trailing `/` so `"/v1"` cannot match
    /// `"/v1beta/x"`.
    api_prefixes: Vec<String>,
    get_allowed: HashSet<String>,
}

impl RouteRules {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a namespace whose routes follow the business-API contract:
    /// POST-only, and framework rejections normalized into the JSON envelope.
    #[must_use]
    pub fn api_namespace(mut self, prefix: impl AsRef<str>) -> Self {
        let prefix = prefix.as_ref().trim_end_matches('/');
        if !prefix.is_empty() {
            self.api_prefixes.push(format!("{prefix}/"));
        }
        self
    }

    /// Exempt one exact path from the POST-only rule.
    ///
    /// Reserve this for protocols we do not control — WeChat's verification
    /// handshake mandates `GET` with a query string, for example. It is not an
    /// escape hatch for new endpoints that would rather be `GET`.
    #[must_use]
    pub fn allow_get(mut self, path: impl Into<String>) -> Self {
        self.get_allowed.insert(path.into());
        self
    }

    /// Bulk form of [`Self::allow_get`], for namespaces that collect the
    /// exemptions of their submodules.
    #[must_use]
    pub fn allow_get_all<I, S>(mut self, paths: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.get_allowed.extend(paths.into_iter().map(Into::into));
        self
    }

    /// Whether `path` belongs to a registered business-API namespace.
    pub fn is_api_path(&self, path: &str) -> bool {
        self.api_prefixes
            .iter()
            .any(|p| path.starts_with(p.as_str()))
    }

    /// Whether `path` was explicitly exempted from the POST-only rule.
    pub fn allows_get(&self, path: &str) -> bool {
        self.get_allowed.contains(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn namespace_matching_is_prefix_bounded() {
        let rules = RouteRules::new().api_namespace("/v1");
        assert!(rules.is_api_path("/v1/wap/jobs"));
        assert!(rules.is_api_path("/v1/"));
        // A namespace must not swallow a sibling that merely shares a prefix.
        assert!(!rules.is_api_path("/v1beta/wap/jobs"));
        assert!(!rules.is_api_path("/v1"));
        assert!(!rules.is_api_path("/health"));
    }

    #[test]
    fn trailing_slash_in_registration_is_normalized() {
        let with = RouteRules::new().api_namespace("/v2/");
        let without = RouteRules::new().api_namespace("/v2");
        assert!(with.is_api_path("/v2/wap/login"));
        assert!(without.is_api_path("/v2/wap/login"));
    }

    #[test]
    fn empty_namespace_is_ignored_rather_than_matching_everything() {
        let rules = RouteRules::new().api_namespace("/");
        assert!(!rules.is_api_path("/health"));
    }

    #[test]
    fn get_exemptions_are_exact_matches() {
        let rules = RouteRules::new()
            .api_namespace("/v1")
            .allow_get_all(["/v1/wap/wechat/callback", "/v1/wap/dict/industries"]);
        assert!(rules.allows_get("/v1/wap/wechat/callback"));
        assert!(rules.allows_get("/v1/wap/dict/industries"));
        assert!(!rules.allows_get("/v1/wap/wechat/callback/extra"));
        assert!(!rules.allows_get("/v1/wap/jobs"));
    }
}
