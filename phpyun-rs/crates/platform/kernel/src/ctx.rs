//! The protocol-agnostic request context handed to every operation.

use phpyun_core::{ApiError, AppState};

use crate::caller::{Caller, ProductId, UserCaller};

/// Which transport delivered the call. Business code should almost never
/// branch on this; it exists for logging, metrics, and the rare handler that
/// genuinely cannot be driven from a queue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transport {
    Http,
    Mq,
    WebSocket,
}

impl Transport {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Http => "http",
            Self::Mq => "mq",
            Self::WebSocket => "ws",
        }
    }
}

/// Ambient per-request facts that are not business input.
#[derive(Debug, Clone, Default)]
pub struct RequestMeta {
    pub request_id: Option<String>,
    pub client_ip: Option<String>,
    pub user_agent: Option<String>,
    pub idempotency_key: Option<String>,
    pub transport: Option<Transport>,
}

impl RequestMeta {
    pub fn new(transport: Transport) -> Self {
        Self {
            transport: Some(transport),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn with_request_id(mut self, id: Option<String>) -> Self {
        self.request_id = id;
        self
    }

    #[must_use]
    pub fn with_client_ip(mut self, ip: Option<String>) -> Self {
        self.client_ip = ip;
        self
    }

    #[must_use]
    pub fn with_user_agent(mut self, ua: Option<String>) -> Self {
        self.user_agent = ua;
        self
    }

    #[must_use]
    pub fn with_idempotency_key(mut self, key: Option<String>) -> Self {
        self.idempotency_key = key;
        self
    }
}

/// Everything an operation needs besides its typed input.
///
/// Cloning is cheap: [`AppState`] is a bundle of `Arc`-backed facades, and the
/// rest is small owned data.
#[derive(Clone)]
pub struct Ctx {
    pub state: AppState,
    pub product: ProductId,
    pub caller: Caller,
    pub meta: RequestMeta,
}

impl Ctx {
    pub fn new(state: AppState, product: ProductId, caller: Caller, meta: RequestMeta) -> Self {
        Self {
            state,
            product,
            caller,
            meta,
        }
    }

    /// Build a context for work we originate ourselves — scheduler ticks and
    /// event-bus consumers. See [`Caller::System`] for why this bypasses policy
    /// checks and must never be reachable from external input.
    pub fn system(state: AppState, product: ProductId, transport: Transport) -> Self {
        Self::new(state, product, Caller::System, RequestMeta::new(transport))
    }

    pub fn uid(&self) -> Option<u64> {
        self.caller.uid()
    }

    /// The authenticated user, or 401.
    ///
    /// Handlers should rarely need this: if the operation's [`Policy`] requires
    /// authentication, the dispatcher has already rejected anonymous callers,
    /// and this just narrows the type.
    ///
    /// [`Policy`]: crate::policy::Policy
    pub fn require_user(&self) -> Result<&UserCaller, ApiError> {
        self.caller.user().ok_or_else(ApiError::unauth)
    }
}

impl std::fmt::Debug for Ctx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // AppState has no useful Debug and would dump connection pools.
        f.debug_struct("Ctx")
            .field("product", &self.product)
            .field("caller", &self.caller.kind())
            .field("meta", &self.meta)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transport_labels_are_stable() {
        assert_eq!(Transport::Http.as_str(), "http");
        assert_eq!(Transport::Mq.as_str(), "mq");
        assert_eq!(Transport::WebSocket.as_str(), "ws");
    }

    #[test]
    fn meta_builders_compose() {
        let meta = RequestMeta::new(Transport::Http)
            .with_request_id(Some("req-1".into()))
            .with_client_ip(Some("10.0.0.1".into()))
            .with_idempotency_key(Some("key-1".into()));
        assert_eq!(meta.transport, Some(Transport::Http));
        assert_eq!(meta.request_id.as_deref(), Some("req-1"));
        assert_eq!(meta.client_ip.as_deref(), Some("10.0.0.1"));
        assert_eq!(meta.idempotency_key.as_deref(), Some("key-1"));
        assert_eq!(meta.user_agent, None);
    }
}
