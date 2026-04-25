//! Audit log — **typed events + DB persistence + event-bus dual write**.
//!
//! ## Architecture
//! ```text
//!  Business handler
//!     │  audit::emit(state, AuditEvent { ... })
//!     ▼
//!  audit facade  ──┬──► DB (synchronous write to yun_rs_audit_log)   ← query index
//!                  │
//!                  └──► EventBus topic="audit.log"                    ← real-time consumers
//!                        (asynchronous spawn_best_effort; failures don't block business)
//! ```
//!
//! ## Design points
//! - **Type safety**: `AuditEvent` requires actor + action; `meta` is arbitrary serde JSON.
//! - **Dual-write resilience**: synchronous DB write ensures persistence; an async
//!   bus-publish failure doesn't disturb the main flow.
//! - **Queryable**: `action / actor_uid / actor_ip / created_at` are all indexed.
//! - **Zero coupling**: business code calling `audit::emit(state, ...)` doesn't
//!   know whether the backend is DB or Kafka.
//!
//! ## Typical events
//! - `user.login`, `user.logout`, `user.register`, `user.forget_password`
//! - `resume.create`, `resume.update`, `resume.publish`
//! - `job.create`, `job.close`
//! - `admin.*` (admin-side operations)

use crate::background;
use crate::clock;
use crate::error::AppResult;
use crate::events::EventBus;
use crate::json;
use crate::state::AppState;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Actor {
    pub uid: Option<u64>,
    pub ip: Option<String>,
    pub ua: Option<String>,
}

impl Actor {
    pub fn anonymous() -> Self {
        Self { uid: None, ip: None, ua: None }
    }

    pub fn uid(uid: u64) -> Self {
        Self { uid: Some(uid), ip: None, ua: None }
    }

    pub fn with_ip(mut self, ip: impl Into<String>) -> Self {
        self.ip = Some(ip.into());
        self
    }

    pub fn with_ua(mut self, ua: impl Into<String>) -> Self {
        self.ua = Some(ua.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub actor: Actor,
    pub action: &'static str,
    #[serde(default)]
    pub target: String,
    #[serde(default = "default_success")]
    pub success: bool,
    #[serde(default)]
    pub meta: Option<json::Value>,
}

fn default_success() -> bool {
    true
}

impl AuditEvent {
    pub fn new(action: &'static str, actor: Actor) -> Self {
        Self {
            actor,
            action,
            target: String::new(),
            success: true,
            meta: None,
        }
    }

    pub fn success(mut self, ok: bool) -> Self {
        self.success = ok;
        self
    }

    pub fn target(mut self, t: impl Into<String>) -> Self {
        self.target = t.into();
        self
    }

    pub fn meta<T: Serialize>(mut self, m: &T) -> Self {
        self.meta = json::to_value(m).ok();
        self
    }
}

/// Write an audit event: synchronous DB + asynchronous event-bus (best-effort).
///
/// DB-write failures are returned to the caller; bus-publish failures only log a warn.
pub async fn emit(state: &AppState, event: AuditEvent) -> AppResult<()> {
    let created_at = clock::now_ts();

    // 1. Synchronous DB write (uses the writer pool).
    insert_db(state.db.pool(), &event, created_at).await?;

    // 2. Asynchronous event-bus publish (best-effort).
    let bus = state.events.clone();
    let event_c = event.clone();
    background::spawn_best_effort("audit.publish", async move {
        if let Err(e) = bus.publish_json("audit.log", &event_c).await {
            tracing::warn!(?e, action = event_c.action, "audit bus publish failed");
        }
    });

    Ok(())
}

async fn insert_db(pool: &MySqlPool, e: &AuditEvent, created_at: i64) -> AppResult<()> {
    let meta_s = e.meta.as_ref().map(json::to_string).transpose()?;
    sqlx::query(
        r"INSERT INTO yun_rs_audit_log
          (actor_uid, actor_ip, actor_ua, action, target, success, meta, created_at)
          VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(e.actor.uid)
    .bind(e.actor.ip.as_deref().unwrap_or(""))
    .bind(e.actor.ua.as_deref().unwrap_or(""))
    .bind(e.action)
    .bind(&e.target)
    .bind(if e.success { 1 } else { 0 })
    .bind(meta_s)
    .bind(created_at)
    .execute(pool)
    .await?;
    Ok(())
}

/// Bus-only variant that doesn't write the DB — used for events that don't need
/// strong persistence (e.g. triggering email sends). Not strictly an audit event,
/// but kept here so we can reuse the `EventBus` wrapper.
#[allow(dead_code)]
pub async fn emit_bus_only<T: Serialize + ?Sized>(
    bus: &EventBus,
    topic: &str,
    payload: &T,
) -> AppResult<String> {
    bus.publish_json(topic, payload).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actor_builder() {
        let a = Actor::uid(42).with_ip("1.2.3.4").with_ua("curl/8.0");
        assert_eq!(a.uid, Some(42));
        assert_eq!(a.ip.as_deref(), Some("1.2.3.4"));
        assert_eq!(a.ua.as_deref(), Some("curl/8.0"));
    }

    #[test]
    fn event_builder() {
        let e = AuditEvent::new("user.login", Actor::uid(1))
            .success(false)
            .target("uid:1")
            .meta(&serde_json::json!({"reason": "bad_password"}));
        assert_eq!(e.action, "user.login");
        assert!(!e.success);
        assert_eq!(e.target, "uid:1");
        assert_eq!(e.meta.as_ref().unwrap()["reason"], "bad_password");
    }

    #[test]
    fn event_serializes() {
        let e = AuditEvent::new("user.login", Actor::uid(1));
        let s = serde_json::to_string(&e).unwrap();
        assert!(s.contains(r#""action":"user.login""#));
    }
}
