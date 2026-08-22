//! What a connection may subscribe to.
//!
//! Two separate questions, and conflating them is how push channels leak other
//! people's data:
//!
//! - *Whose* messages? Never asked. A session only ever receives pushes
//!   addressed to the uid that authenticated it — the hub routes by uid, so
//!   there is no topic string a client could send to reach another account.
//! - *Which kind* of messages? That is this module: a fixed catalogue, with a
//!   role requirement per entry.

use phpyun_core::ApiError;
use phpyun_kernel::{Caller, Role};

/// Channel number on the wire (`tp`). Subscribe still uses the catalogue
/// string (`chat`, `notifications`, …); this is what a push JSON carries
/// so a client can `switch` without reading SSE `event:` or a string topic.
///
/// Numbers only grow at the end; never reorder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Tp {
    Chat = 0,
    Notifications = 1,
    AdminOps = 2,
}

impl Tp {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Chat),
            1 => Some(Self::Notifications),
            2 => Some(Self::AdminOps),
            _ => None,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::Chat => "chat",
            Self::Notifications => "notifications",
            Self::AdminOps => "admin.ops",
        }
    }
}

/// A subscribable channel of the caller's own events.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Topic(&'static str);

impl Topic {
    pub fn as_str(&self) -> &'static str {
        self.0
    }
}

impl std::fmt::Display for Topic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

/// The catalogue. `None` means any authenticated user; a role means only that
/// role. Adding a channel is one line here — and deliberately not something a
/// client can do by inventing a topic string.
const CATALOGUE: &[(Tp, Option<Role>)] = &[
    (Tp::Chat, None),
    (Tp::Notifications, None),
    (Tp::AdminOps, Some(Role::Admin)),
];

/// Resolve a client-supplied topic name, checking that it exists and that this
/// caller may have it.
///
/// Unknown and forbidden are reported differently on purpose: the catalogue is
/// public knowledge, so hiding it buys nothing, while a clear 403 saves a
/// support ticket.
pub fn resolve(name: &str, caller: &Caller) -> Result<Topic, ApiError> {
    let Some((tp, required)) = CATALOGUE.iter().find(|(tp, _)| tp.name() == name) else {
        return Err(ApiError::param_invalid(format!("unknown topic {name:?}")));
    };

    match required {
        None => Ok(Topic(tp.name())),
        Some(role) => match caller.user() {
            Some(user) if user.usertype == role.usertype() => Ok(Topic(tp.name())),
            // A machine client has no `usertype` and so can never hold a human
            // role; same answer either way.
            _ => Err(ApiError::forbidden()),
        },
    }
}

/// Every topic this caller is allowed to subscribe to, for the welcome frame.
pub fn available(caller: &Caller) -> Vec<&'static str> {
    CATALOGUE
        .iter()
        .filter(|(tp, _)| resolve(tp.name(), caller).is_ok())
        .map(|(tp, _)| tp.name())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use phpyun_core::extractors::{USERTYPE_ADMIN, USERTYPE_JOBSEEKER};
    use phpyun_kernel::{ClientCaller, ProductId, UserCaller};

    fn user(usertype: u8) -> Caller {
        Caller::User(UserCaller {
            uid: 7,
            usertype,
            did: 0,
            jti: "j".into(),
        })
    }

    #[test]
    fn ordinary_channels_are_open_to_any_authenticated_user() {
        let caller = user(USERTYPE_JOBSEEKER);
        assert_eq!(resolve("chat", &caller).unwrap().as_str(), "chat");
        assert!(resolve("notifications", &caller).is_ok());
    }

    #[test]
    fn a_restricted_channel_checks_the_role() {
        assert_eq!(
            resolve("admin.ops", &user(USERTYPE_JOBSEEKER))
                .unwrap_err()
                .code(),
            403
        );
        assert!(resolve("admin.ops", &user(USERTYPE_ADMIN)).is_ok());
    }

    #[test]
    fn an_invented_topic_is_rejected_rather_than_created() {
        let err = resolve("chat:9999", &user(USERTYPE_JOBSEEKER)).unwrap_err();
        assert_eq!(err.code(), 400);
        assert_eq!(err.key(), "param_invalid");
    }

    /// Matching is exact: no prefix, suffix, or case games.
    #[test]
    fn topic_matching_is_exact() {
        let caller = user(USERTYPE_ADMIN);
        for name in ["Chat", "chat ", "chat.", "cha", "chatx", "admin.ops.all"] {
            assert!(
                resolve(name, &caller).is_err(),
                "{name:?} must not resolve to a real topic"
            );
        }
    }

    #[test]
    fn a_machine_client_cannot_satisfy_a_role_requirement() {
        let client = Caller::Client(ClientCaller {
            app_id: "acme".into(),
            product: ProductId::new("recruit"),
            scopes: vec!["admin.all".into()],
        });
        assert_eq!(resolve("admin.ops", &client).unwrap_err().code(), 403);
    }

    #[test]
    fn the_advertised_list_matches_what_resolve_accepts() {
        let jobseeker = available(&user(USERTYPE_JOBSEEKER));
        assert_eq!(jobseeker, vec!["chat", "notifications"]);
        assert!(available(&user(USERTYPE_ADMIN)).contains(&"admin.ops"));
    }

    #[test]
    fn wire_numbers_never_move() {
        assert_eq!(Tp::Chat.as_u8(), 0);
        assert_eq!(Tp::Notifications.as_u8(), 1);
        assert_eq!(Tp::AdminOps.as_u8(), 2);
        assert_eq!(Tp::from_u8(0).unwrap().name(), "chat");
        assert_eq!(Tp::from_u8(9), None);
    }
}
