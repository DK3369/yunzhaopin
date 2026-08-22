//! Declarative access policy, attached to an operation and enforced in one
//! place.
//!
//! Today authorization is 482 hand-written `user.require_admin()?` calls spread
//! across handlers; whether an endpoint is guarded depends on whether somebody
//! remembered the line. A [`Policy`] is a `const` on the operation, so the
//! requirement is visible at the definition site, enforced by
//! [`enforce`] before the handler body runs, and readable by tooling.

use phpyun_core::ApiError;

use crate::caller::{Caller, Role};

/// Whether the caller must be authenticated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthMode {
    /// No credentials needed. Credentials, if present, are still resolved so
    /// the handler can personalize the response.
    Public,
    /// Same as [`Self::Public`] at the gate; the distinction exists so an
    /// endpoint can document that it behaves differently when signed in.
    Optional,
    /// Anonymous callers are rejected with 401.
    Required,
}

/// Which rate-limit bucket the operation draws from. The transport maps these
/// onto concrete limits; the kernel only carries the intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RateTier {
    /// The global per-IP bucket.
    Default,
    /// Tighter bucket for expensive or abuse-prone operations: login, SMS
    /// sending, password reset.
    Strict,
    /// Looser bucket for cheap reads that clients poll.
    Relaxed,
    /// Exempt. Reserve for ops probes.
    Unlimited,
}

/// The access requirements of one operation.
#[derive(Debug, Clone, Copy)]
pub struct Policy {
    pub auth: AuthMode,
    /// Allowed human roles. Empty means "any authenticated user"; non-empty
    /// implies authentication even if [`Self::auth`] says otherwise.
    pub roles: &'static [Role],
    /// Scopes a machine client must hold. Empty means the operation is not
    /// exposed to open-platform clients at all.
    pub scopes: &'static [&'static str],
    pub rate: RateTier,
    /// Whether a repeated call carrying the same `Idempotency-Key` should
    /// replay the first response instead of executing again.
    pub idempotent: bool,
}

impl Policy {
    /// Reachable without credentials.
    pub const fn public() -> Self {
        Self {
            auth: AuthMode::Public,
            roles: &[],
            scopes: &[],
            rate: RateTier::Default,
            idempotent: false,
        }
    }

    /// Reachable without credentials, but behaves differently when signed in.
    pub const fn optional_auth() -> Self {
        Self {
            auth: AuthMode::Optional,
            ..Self::public()
        }
    }

    /// Requires any authenticated user.
    pub const fn authenticated() -> Self {
        Self {
            auth: AuthMode::Required,
            ..Self::public()
        }
    }

    /// Requires an authenticated user holding one of `roles`.
    pub const fn roles(roles: &'static [Role]) -> Self {
        Self {
            auth: AuthMode::Required,
            roles,
            ..Self::public()
        }
    }

    /// Shorthand for the most common guard.
    pub const fn admin() -> Self {
        Self::roles(&[Role::Admin])
    }

    #[must_use]
    pub const fn with_scopes(mut self, scopes: &'static [&'static str]) -> Self {
        self.scopes = scopes;
        self
    }

    #[must_use]
    pub const fn with_rate(mut self, rate: RateTier) -> Self {
        self.rate = rate;
        self
    }

    #[must_use]
    pub const fn idempotent(mut self) -> Self {
        self.idempotent = true;
        self
    }

    /// Whether a caller must present credentials to get past [`enforce`].
    pub const fn requires_credentials(&self) -> bool {
        matches!(self.auth, AuthMode::Required) || !self.roles.is_empty()
    }
}

/// Enforce `policy` against `caller`.
///
/// [`Caller::System`] bypasses every check: those calls originate from our own
/// scheduler and event bus, never from a network peer, and there is no user
/// identity to check roles against. Transports must therefore never construct a
/// `System` caller from anything an external client can influence.
pub fn enforce(policy: &Policy, caller: &Caller) -> Result<(), ApiError> {
    if matches!(caller, Caller::System) {
        return Ok(());
    }

    if policy.requires_credentials() && caller.is_anonymous() {
        return Err(ApiError::unauth());
    }

    if !policy.roles.is_empty() {
        let Some(user) = caller.user() else {
            // A machine client has no `usertype`, so it can never satisfy a
            // role requirement — that is a scope question, not a role one.
            return Err(ApiError::role_mismatch());
        };
        if !policy.roles.iter().any(|r| r.usertype() == user.usertype) {
            return Err(ApiError::role_mismatch());
        }
    }

    if let Caller::Client(client) = caller {
        if policy.scopes.is_empty() {
            // The operation was never opened to the platform.
            return Err(ApiError::forbidden());
        }
        if !policy.scopes.iter().all(|s| client.has_scope(s)) {
            return Err(ApiError::forbidden());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::caller::{ClientCaller, ProductId, UserCaller};
    use phpyun_core::extractors::{USERTYPE_ADMIN, USERTYPE_EMPLOYER, USERTYPE_JOBSEEKER};

    fn user(usertype: u8) -> Caller {
        Caller::User(UserCaller {
            uid: 1,
            usertype,
            did: 0,
            jti: "j".into(),
        })
    }

    fn client(scopes: &[&str]) -> Caller {
        Caller::Client(ClientCaller {
            app_id: "acme".into(),
            product: ProductId::new("recruit"),
            scopes: scopes.iter().map(|s| (*s).to_owned()).collect(),
        })
    }

    #[test]
    fn public_operations_admit_anonymous_callers() {
        assert!(enforce(&Policy::public(), &Caller::Anonymous).is_ok());
        assert!(enforce(&Policy::optional_auth(), &Caller::Anonymous).is_ok());
    }

    #[test]
    fn authenticated_operations_reject_anonymous_with_401() {
        let err = enforce(&Policy::authenticated(), &Caller::Anonymous).unwrap_err();
        assert_eq!(err.code(), 401);
        assert!(enforce(&Policy::authenticated(), &user(USERTYPE_JOBSEEKER)).is_ok());
    }

    #[test]
    fn role_requirement_implies_authentication() {
        let policy = Policy::admin();
        assert!(policy.requires_credentials());
        assert_eq!(
            enforce(&policy, &Caller::Anonymous).unwrap_err().code(),
            401
        );
    }

    #[test]
    fn wrong_role_is_403_not_401() {
        let err = enforce(&Policy::admin(), &user(USERTYPE_JOBSEEKER)).unwrap_err();
        assert_eq!(err.code(), 403);
        assert_eq!(err.key(), "role_mismatch");
    }

    #[test]
    fn multiple_allowed_roles_are_or_ed() {
        let policy = Policy::roles(&[Role::Employer, Role::Admin]);
        assert!(enforce(&policy, &user(USERTYPE_EMPLOYER)).is_ok());
        assert!(enforce(&policy, &user(USERTYPE_ADMIN)).is_ok());
        assert!(enforce(&policy, &user(USERTYPE_JOBSEEKER)).is_err());
    }

    #[test]
    fn clients_are_locked_out_unless_the_operation_declares_scopes() {
        // Default-deny: forgetting `with_scopes` must not silently expose an
        // endpoint to the open platform.
        let err = enforce(&Policy::authenticated(), &client(&["job.read"])).unwrap_err();
        assert_eq!(err.code(), 403);
    }

    #[test]
    fn clients_need_every_declared_scope() {
        let policy = Policy::authenticated().with_scopes(&["job.read", "job.write"]);
        assert!(enforce(&policy, &client(&["job.read", "job.write"])).is_ok());
        assert!(enforce(&policy, &client(&["job.read"])).is_err());
    }

    #[test]
    fn clients_cannot_satisfy_a_human_role_requirement() {
        let policy = Policy::admin().with_scopes(&["admin.all"]);
        let err = enforce(&policy, &client(&["admin.all"])).unwrap_err();
        assert_eq!(err.key(), "role_mismatch");
    }

    #[test]
    fn system_callers_bypass_every_check() {
        let policy = Policy::admin().with_scopes(&["nobody.has.this"]);
        assert!(enforce(&policy, &Caller::System).is_ok());
    }
}
