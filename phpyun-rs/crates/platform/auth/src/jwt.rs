//! Legacy API compatibility: re-export the JWT module from core.

pub use phpyun_core::jwt::{
    issue_pair, issue_pair_ex, verify, verify_access, verify_refresh, Claims, JwtIssued,
};
