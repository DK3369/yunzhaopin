//! Who is making the call, resolved by the transport before dispatch.
//!
//! Every transport answers the same question — "who is this?" — and expresses
//! the answer as a [`Caller`]. HTTP resolves it from a JWT or a signed client
//! credential; the MQ consumer answers [`Caller::System`]. Downstream policy
//! enforcement and business code then work identically regardless of protocol.

use phpyun_core::extractors::{
    AuthenticatedUser, USERTYPE_ADMIN, USERTYPE_EMPLOYER, USERTYPE_JOBSEEKER,
};

/// Which product line an operation belongs to.
///
/// Encoded as a `&'static str` rather than an enum so a new product line does
/// not have to edit the kernel. The value is the first segment of
/// [`Operation::ID`](crate::Operation::ID), e.g. `"recruit"` in
/// `"recruit.job.list"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProductId(&'static str);

impl ProductId {
    pub const fn new(name: &'static str) -> Self {
        Self(name)
    }

    pub const fn as_str(&self) -> &'static str {
        self.0
    }
}

impl std::fmt::Display for ProductId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

/// A human role, as opposed to a machine client. Mirrors PHPYun's `usertype`
/// column, which is the source of truth we cannot change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Jobseeker,
    Employer,
    Admin,
}

impl Role {
    pub const fn usertype(self) -> u8 {
        match self {
            Self::Jobseeker => USERTYPE_JOBSEEKER,
            Self::Employer => USERTYPE_EMPLOYER,
            Self::Admin => USERTYPE_ADMIN,
        }
    }

    pub const fn from_usertype(usertype: u8) -> Option<Self> {
        match usertype {
            USERTYPE_JOBSEEKER => Some(Self::Jobseeker),
            USERTYPE_EMPLOYER => Some(Self::Employer),
            USERTYPE_ADMIN => Some(Self::Admin),
            _ => None,
        }
    }
}

/// An authenticated end user.
#[derive(Debug, Clone)]
pub struct UserCaller {
    pub uid: u64,
    pub usertype: u8,
    /// PHPYun's site/department discriminator, carried in the JWT.
    pub did: u32,
    pub jti: String,
}

impl UserCaller {
    pub fn role(&self) -> Option<Role> {
        Role::from_usertype(self.usertype)
    }
}

impl From<AuthenticatedUser> for UserCaller {
    fn from(u: AuthenticatedUser) -> Self {
        Self {
            uid: u.uid,
            usertype: u.usertype,
            did: u.did,
            jti: u.jti,
        }
    }
}

/// A machine client — a third-party integration on the open platform, holding
/// an `app_id` and a set of granted scopes rather than a user session.
#[derive(Debug, Clone)]
pub struct ClientCaller {
    pub app_id: String,
    pub product: ProductId,
    pub scopes: Vec<String>,
}

impl ClientCaller {
    pub fn has_scope(&self, scope: &str) -> bool {
        self.scopes.iter().any(|s| s == scope)
    }
}

/// The resolved identity behind a request.
#[derive(Debug, Clone)]
pub enum Caller {
    /// No credentials, or credentials that failed to verify on an endpoint that
    /// tolerates that.
    Anonymous,
    User(UserCaller),
    Client(ClientCaller),
    /// Our own infrastructure: scheduler ticks and event-bus messages. These
    /// never crossed a network boundary we do not control, so
    /// [`enforce`](crate::policy::enforce) lets them past authentication,
    /// role, and scope checks.
    System,
}

impl Caller {
    pub fn user(&self) -> Option<&UserCaller> {
        match self {
            Self::User(u) => Some(u),
            _ => None,
        }
    }

    pub fn client(&self) -> Option<&ClientCaller> {
        match self {
            Self::Client(c) => Some(c),
            _ => None,
        }
    }

    pub fn uid(&self) -> Option<u64> {
        self.user().map(|u| u.uid)
    }

    pub fn is_anonymous(&self) -> bool {
        matches!(self, Self::Anonymous)
    }

    /// Short label for logs and metrics. Never includes the uid or app_id, so
    /// it is safe to use as a metric dimension.
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Anonymous => "anonymous",
            Self::User(_) => "user",
            Self::Client(_) => "client",
            Self::System => "system",
        }
    }
}

impl From<Option<AuthenticatedUser>> for Caller {
    fn from(user: Option<AuthenticatedUser>) -> Self {
        match user {
            Some(u) => Self::User(u.into()),
            None => Self::Anonymous,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn role_roundtrips_through_phpyun_usertype() {
        for role in [Role::Jobseeker, Role::Employer, Role::Admin] {
            assert_eq!(Role::from_usertype(role.usertype()), Some(role));
        }
        assert_eq!(Role::from_usertype(0), None);
        assert_eq!(Role::from_usertype(9), None);
    }

    #[test]
    fn caller_kind_is_safe_as_a_metric_label() {
        let user = Caller::User(UserCaller {
            uid: 42,
            usertype: USERTYPE_ADMIN,
            did: 0,
            jti: "secret-jti".into(),
        });
        assert_eq!(user.kind(), "user");
        assert!(!user.kind().contains("42"));
        assert_eq!(Caller::Anonymous.kind(), "anonymous");
        assert_eq!(Caller::System.kind(), "system");
    }

    #[test]
    fn anonymous_has_no_uid() {
        assert_eq!(Caller::Anonymous.uid(), None);
        assert_eq!(Caller::System.uid(), None);
    }

    #[test]
    fn scope_matching_is_exact() {
        let client = ClientCaller {
            app_id: "acme".into(),
            product: ProductId::new("recruit"),
            scopes: vec!["job.read".into()],
        };
        assert!(client.has_scope("job.read"));
        assert!(!client.has_scope("job"));
        assert!(!client.has_scope("job.read.all"));
    }
}
