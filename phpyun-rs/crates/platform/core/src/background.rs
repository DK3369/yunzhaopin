//! Background-task facade. Business code should not call `tokio::spawn` directly —
//! go through this module instead:
//!
//! - `spawn_best_effort(name, fut)` — "fire and forget":
//!     * **Panic capture**: panics inside the task are caught by a watchdog and
//!       counted as `tasks.panicked{name}`.
//!     * **Global concurrency gate**: prevents fire-and-forget tasks from piling
//!       up and killing the process (default cap of 1024 concurrent tasks).
//!     * Metrics: `tasks.spawned{name}` / `tasks.dropped{name}` /
//!       `tasks.panicked{name}` / `tasks.duration_ms{name}`.
//! - `spawn_periodic(name, interval, shutdown, job)` — periodic task that respects
//!   the shutdown token.
//!
//! Why we don't let business code call `tokio::spawn` directly:
//! 1. Panics aren't tracked — Tokio only writes them to stderr; they're invisible
//!    in Grafana.
//! 2. Task counts are unbounded — a slow Redis/DB can spawn tens of thousands of
//!    tasks instantly and crash the scheduler.
//! 3. Swapping the runtime, adding a budget, or attaching a tracing span would
//!    require changing N files.
//!
//! Funnelling everything through this one module means the three points above
//! are fixed in a single place.

use crate::metrics as m;
use std::future::Future;
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

const DEFAULT_SPAWN_CAP: usize = 1024;
static SPAWN_SEM: OnceLock<Arc<Semaphore>> = OnceLock::new();

fn spawn_sem() -> &'static Arc<Semaphore> {
    SPAWN_SEM.get_or_init(|| Arc::new(Semaphore::new(DEFAULT_SPAWN_CAP)))
}

/// One-shot background task. Panics are caught by the watchdog and logged as
/// errors without disturbing the main flow.
/// When the concurrency cap is full this spawn is dropped and counted in metrics —
/// the caller is never blocked.
pub fn spawn_best_effort<Fut>(name: &'static str, fut: Fut) -> JoinHandle<()>
where
    Fut: Future<Output = ()> + Send + 'static,
{
    m::counter_with("tasks.spawned", &[("name", name)]);

    let Ok(permit) = spawn_sem().clone().try_acquire_owned() else {
        m::counter_with("tasks.dropped", &[("name", name)]);
        tracing::warn!(task = name, "best-effort task dropped (sem full)");
        // Return an already-completed handle so the API stays consistent.
        return tokio::spawn(async {});
    };

    // Watchdog: the outer spawn holds the permit, times the run, and catches panics;
    // the inner spawn runs the real future.
    tokio::spawn(async move {
        let _permit = permit;
        let started = Instant::now();
        let inner = tokio::spawn(fut);
        let res = inner.await;
        let elapsed_ms = started.elapsed().as_secs_f64() * 1000.0;
        m::histogram_ms("tasks.duration_ms", elapsed_ms);
        match res {
            Ok(()) => {
                tracing::debug!(task = name, took_ms = elapsed_ms as u64, "bg done");
            }
            Err(e) if e.is_panic() => {
                m::counter_with("tasks.panicked", &[("name", name)]);
                tracing::error!(task = name, ?e, "bg task panicked");
            }
            Err(_cancelled) => {
                tracing::warn!(task = name, "bg task cancelled");
            }
        }
    })
}

/// Periodic task that respects the shutdown token. Panics inside a tick are caught
/// by the inner spawn, so the ticker keeps running.
pub fn spawn_periodic<F, Fut>(
    name: &'static str,
    interval: Duration,
    shutdown: CancellationToken,
    mut job: F,
) -> JoinHandle<()>
where
    F: FnMut() -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    tokio::spawn(async move {
        tracing::info!(task = name, "background task started");
        let mut ticker = tokio::time::interval(interval);
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);

        loop {
            tokio::select! {
                _ = ticker.tick() => {
                    let started = Instant::now();
                    // Spawn each tick separately to run the job so panics don't propagate to the loop.
                    let inner = tokio::spawn(job());
                    if let Err(e) = inner.await {
                        if e.is_panic() {
                            m::counter_with("tasks.panicked", &[("name", name)]);
                            tracing::error!(task = name, ?e, "periodic tick panicked");
                        }
                    }
                    tracing::debug!(
                        task = name,
                        took_ms = started.elapsed().as_millis() as u64,
                        "tick"
                    );
                }
                _ = shutdown.cancelled() => {
                    tracing::info!(task = name, "shutdown; exiting");
                    break;
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[tokio::test]
    async fn spawn_best_effort_runs_to_completion() {
        let counter = Arc::new(AtomicU32::new(0));
        let c = counter.clone();
        let handle = spawn_best_effort("test_ok", async move {
            c.fetch_add(1, Ordering::SeqCst);
        });
        handle.await.unwrap();
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn spawn_best_effort_captures_panic() {
        // The watchdog catches the panic, so the outer handle should await
        // normally (the inner spawn fails, but the outer one doesn't propagate).
        let handle = spawn_best_effort("test_panic", async {
            panic!("intentional");
        });
        // No panic here means the watchdog caught it.
        handle.await.unwrap();
    }
}
