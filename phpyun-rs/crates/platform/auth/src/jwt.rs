//! Legacy API compatibility: re-export the JWT module from core.

pub use phpyun_core::jwt::{issue_pair, verify, verify_access, verify_refresh, Claims, JwtIssued};
