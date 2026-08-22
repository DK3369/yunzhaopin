//! Chat status and content type, as numbers all the way to the database.
//!
//! JSON, the event bus, and `phpyun_rs_chat` share the same `u8` values.
//! Zero is the default and is omitted on the wire so a text unread message
//! does not pay for two extra keys.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Whether the message has been seen, or a live event that is not a row.
///
/// JSON / column / field: `cs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToSchema)]
#[repr(u8)]
#[schema(example = 0)]
pub enum CStatus {
    /// Unread / a newly arrived message. Default; omitted on the wire.
    Unread = 0,
    /// Read, or a read-receipt event (payload has `u`, no `b`).
    Read = 1,
    /// Soft-deleted.
    Deleted = 2,
}

impl CStatus {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    /// Unknown numbers stay unknown rather than becoming Unread, so a future
    /// value is not silently rewritten in the database.
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Unread),
            1 => Some(Self::Read),
            2 => Some(Self::Deleted),
            _ => None,
        }
    }
}

impl Serialize for CStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u8(self.as_u8())
    }
}

impl<'de> Deserialize<'de> for CStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v = u8::deserialize(d)?;
        Self::from_u8(v).ok_or_else(|| serde::de::Error::custom(format!("unknown cs {v}")))
    }
}

/// What the message contains. Numbers only grow at the end; never reorder.
///
/// JSON / column / field: `ctype`. This version only writes [`CType::Text`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, ToSchema)]
#[repr(u8)]
#[schema(example = 0)]
pub enum CType {
    Text = 0,
    Image = 1,
    File = 2,
    Voice = 3,
    Video = 4,
    RedPacket = 5,
    Location = 6,
    /// Job or resume card.
    Card = 7,
}

impl CType {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Text),
            1 => Some(Self::Image),
            2 => Some(Self::File),
            3 => Some(Self::Voice),
            4 => Some(Self::Video),
            5 => Some(Self::RedPacket),
            6 => Some(Self::Location),
            7 => Some(Self::Card),
            _ => None,
        }
    }
}

impl Serialize for CType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_u8(self.as_u8())
    }
}

impl<'de> Deserialize<'de> for CType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v = u8::deserialize(d)?;
        Self::from_u8(v).ok_or_else(|| serde::de::Error::custom(format!("unknown ctype {v}")))
    }
}

/// `skip_serializing_if` for `cs` / `ctype` stored as `u8`.
pub fn is_zero(v: &u8) -> bool {
    *v == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn status_and_content_are_bare_numbers() {
        assert_eq!(serde_json::to_value(CStatus::Read).unwrap(), json!(1));
        assert_eq!(serde_json::to_value(CType::Location).unwrap(), json!(6));
        assert_eq!(serde_json::from_value::<CStatus>(json!(2)).unwrap(), CStatus::Deleted);
        assert_eq!(serde_json::from_value::<CType>(json!(5)).unwrap(), CType::RedPacket);
    }

    #[test]
    fn unknown_numbers_are_not_silently_rewritten() {
        assert_eq!(CStatus::from_u8(9), None);
        assert_eq!(CType::from_u8(99), None);
        assert!(serde_json::from_value::<CStatus>(json!(9)).is_err());
        assert!(serde_json::from_value::<CType>(json!(99)).is_err());
    }

    #[test]
    fn numbers_never_move() {
        assert_eq!(CStatus::Unread as u8, 0);
        assert_eq!(CStatus::Read as u8, 1);
        assert_eq!(CStatus::Deleted as u8, 2);
        assert_eq!(CType::Text as u8, 0);
        assert_eq!(CType::Image as u8, 1);
        assert_eq!(CType::File as u8, 2);
        assert_eq!(CType::Voice as u8, 3);
        assert_eq!(CType::Video as u8, 4);
        assert_eq!(CType::RedPacket as u8, 5);
        assert_eq!(CType::Location as u8, 6);
        assert_eq!(CType::Card as u8, 7);
    }
}
