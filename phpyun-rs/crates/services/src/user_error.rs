//! User-domain errors. **Fully independent of core** — implementing
//! `phpyun_core::error::ApiError` is enough for `?` to flow into the HTTP response.
//! This is the canonical example of the "pluggable" architecture: future
//! `phpyun_payment::PaymentError` / `phpyun_order::OrderError` follow the same pattern.

use std::borrow::Cow;
use thiserror::Error;

use phpyun_core::error::ApiError;

#[derive(Error, Debug, Clone)]
pub enum UserError {
    #[error("user not found")]
    NotFound,

    #[error("mobile already registered")]
    MobileDup,
}

impl ApiError for UserError {
    fn code(&self) -> u16 {
        match self {
            Self::NotFound => 404,
            Self::MobileDup => 409,
        }
    }

    fn tag(&self) -> Cow<'static, str> {
        match self {
            Self::NotFound => "user_not_found".into(),
            Self::MobileDup => "mobile_dup".into(),
        }
    }
}
