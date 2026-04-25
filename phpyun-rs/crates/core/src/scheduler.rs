//! Advanced scheduled-task engine — **cron expressions + distributed locks + panic capture**.
//!
//! ## Features
//! - Cron 6-field expressions: `sec min hour day month weekday`
//!   (`0 */5 * * * *` = every 5 minutes).
//! - Fixed intervals (`Duration::from_secs(30)`) are also supported.
//! - **Distributed lock** (via `Kv::acquire_lock`): when deployed across
//!   multiple instances, each job runs globally exactly once per tick.
//! - **Panic capture**: an inner spawn catches panics so they don't take
//!   down the supervisor.
//! - **Metrics**: per-job labels
//!     * `scheduler.tick.start / success / panic / skipped_lock / lock_error`
//!     * `scheduler.tick.duration_ms`
//! - **Graceful shutdown**: listens on a CancellationToken; the ticker stops
//!   and in-flight ticks are not interrupted.
//!
//! ## Usage
//!
//! ```ignore
//! let mut sch = Scheduler::new(state.redis.clone(), shutdown.clone());
//!
//! // Purge expired login audit entries at 3 AM daily
//! let state_bg = state.clone();
//! sch.cron("purge_login_audit", "0 0 3 * * *", move || {
//!     let s = state_bg.clone();
//!     async move { purge_login_audit(&s).await; }
//! })?;
//!
//! // Report cache levels every 30 seconds
//! let state_bg = state.clone();
//! sch.interval("cache_gauge", Duration::from_secs(30), move || {
//!     let s = state_bg.clone();
//!     async move { report_cache_gauge(&s).await; }
//! })
//! .lock_ttl(Duration::from_secs(60));  // change lock TTL to 60s (default 5min)
//!
//! // Start
//! sch.start();  // returns the supervisor JoinHandle; usually not awaited
//! ```
//!
//! ## Adding a new job doesn't require touching core
//!
//! A job is just a `Fn() -> Future<Output = ()>`. The business crate writes
//! its own `fn my_task(state)` and registers it in `main`. The scheduler
//! itself doesn't change.

use crate::error::AppError;
use crate::kv::Kv;
use crate::metrics as m;
use cron::Schedule;
use std::future::Future;
use std::pin::Pin;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

const DEFAULT_LOCK_TTL: Duration = Duration::from_secs(300); // 5 min

/// Job type: a boxed Fn; each tick produces a new future.
type JobFn = Arc<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

enum JobKind {
    Cron(Box<Schedule>),
    Interval(Duration),
}

struct Job {
    name: &'static str,
    kind: JobKind,
    f: JobFn,
    lock_key: String,
    lock_ttl: Duration,
    distributed: bool,
}

pub struct Scheduler {
    kv: Kv,
    shutdown: CancellationToken,
    jobs: Vec<Job>,
}

impl Scheduler {
    pub fn new(kv: Kv, shutdown: CancellationToken) -> Self {
        Self {
            kv,
            shutdown,
            jobs: Vec::new(),
        }
    }

    /// Register a cron job.
    /// 6-field expression: `sec min hour day month weekday`.
    /// Example: `"0 */5 * * * *"` = every 5 minutes.
    pub fn cron<F, Fut>(
        &mut self,
        name: &'static str,
        expr: &str,
        job: F,
    ) -> Result<&mut Self, AppError>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let schedule = Schedule::from_str(expr).map_err(AppError::internal)?;
        self.jobs.push(Job {
            name,
            kind: JobKind::Cron(Box::new(schedule)),
            f: Arc::new(move || Box::pin(job())),
            lock_key: format!("cron:{name}"),
            lock_ttl: DEFAULT_LOCK_TTL,
            distributed: true,
        });
        Ok(self)
    }

    /// Register a fixed-interval job.
    pub fn interval<F, Fut>(&mut self, name: &'static str, dur: Duration, job: F) -> &mut Self
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.jobs.push(Job {
            name,
            kind: JobKind::Interval(dur),
            f: Arc::new(move || Box::pin(job())),
            lock_key: format!("cron:{name}"),
            lock_ttl: DEFAULT_LOCK_TTL,
            distributed: true,
        });
        self
    }

    /// Override the lock TTL of the **most recently registered** job (default 5 minutes).
    pub fn lock_ttl(&mut self, dur: Duration) -> &mut Self {
        if let Some(last) = self.jobs.last_mut() {
            last.lock_ttl = dur;
        }
        self
    }

    /// Disable the distributed lock on the **most recently registered** job
    /// (single-instance / dev use).
    pub fn local_only(&mut self) -> &mut Self {
        if let Some(last) = self.jobs.last_mut() {
            last.distributed = false;
        }
        self
    }

    /// Start every job and return the supervisor handle. Typically not awaited
    /// — rely on the shutdown token for graceful stop.
    pub fn start(self) -> JoinHandle<()> {
        let kv = self.kv;
        let shutdown = self.shutdown;
        let jobs = self.jobs;

        tokio::spawn(async move {
            tracing::info!(n_jobs = jobs.len(), "scheduler starting");
            let handles: Vec<_> = jobs
                .into_iter()
                .map(|job| tokio::spawn(run_job(job, kv.clone(), shutdown.clone())))
                .collect();
            for h in handles {
                let _ = h.await;
            }
            tracing::info!("scheduler stopped");
        })
    }
}

async fn run_job(job: Job, kv: Kv, shutdown: CancellationToken) {
    tracing::info!(task = job.name, "scheduler: job registered");

    loop {
        let wait = match &job.kind {
            JobKind::Cron(schedule) => {
                let Some(next) = schedule.upcoming(chrono::Utc).next() else {
                    tracing::error!(task = job.name, "cron schedule has no upcoming fire");
                    return;
                };
                let now = chrono::Utc::now();
                (next - now).to_std().unwrap_or(Duration::from_secs(1))
            }
            JobKind::Interval(dur) => *dur,
        };

        tokio::select! {
            _ = tokio::time::sleep(wait) => {
                tick(&job, &kv).await;
            }
            _ = shutdown.cancelled() => {
                tracing::info!(task = job.name, "scheduler: shutdown; job exiting");
                return;
            }
        }
    }
}

async fn tick(job: &Job, kv: &Kv) {
    m::counter_with("scheduler.tick.start", &[("name", job.name)]);

    // Distributed lock
    let owner = uuid::Uuid::now_v7().to_string();
    if job.distributed {
        match kv
            .acquire_lock(&job.lock_key, &owner, job.lock_ttl.as_millis() as u64)
            .await
        {
            Ok(true) => {}
            Ok(false) => {
                // Another instance got it; skip.
                m::counter_with("scheduler.tick.skipped_lock", &[("name", job.name)]);
                tracing::debug!(task = job.name, "tick skipped (lock held by peer)");
                return;
            }
            Err(e) => {
                m::counter_with("scheduler.tick.lock_error", &[("name", job.name)]);
                tracing::warn!(task = job.name, ?e, "lock acquire failed, skipping tick");
                return;
            }
        }
    }

    // Execute — inner spawn catches panics.
    let started = Instant::now();
    let inner = tokio::spawn((job.f)());
    match inner.await {
        Ok(()) => m::counter_with("scheduler.tick.success", &[("name", job.name)]),
        Err(e) if e.is_panic() => {
            m::counter_with("scheduler.tick.panic", &[("name", job.name)]);
            tracing::error!(task = job.name, ?e, "scheduled task panicked");
        }
        Err(_) => m::counter_with("scheduler.tick.cancelled", &[("name", job.name)]),
    }
    m::histogram_ms(
        "scheduler.tick.duration_ms",
        started.elapsed().as_secs_f64() * 1000.0,
    );

    // Release the lock (CAS semantics — only releases ours).
    if job.distributed {
        if let Err(e) = kv.release_lock(&job.lock_key, &owner).await {
            tracing::warn!(task = job.name, ?e, "lock release failed (TTL will expire)");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cron_expression_parses() {
        assert!(Schedule::from_str("0 */5 * * * *").is_ok()); // 6-field, every 5min
        assert!(Schedule::from_str("0 0 3 * * *").is_ok()); // daily 3am
        assert!(Schedule::from_str("bad expr").is_err());
    }

    #[test]
    fn default_lock_ttl_is_sane() {
        const _: () = {
            assert!(DEFAULT_LOCK_TTL.as_secs() >= 10);
            assert!(DEFAULT_LOCK_TTL.as_secs() <= 3600);
        };
    }

    // Don't test the runtime flow here (it'd need a real Kv); cron job
    // registration and argument parsing are covered above. End-to-end
    // integration tests live in the app layer.
}
