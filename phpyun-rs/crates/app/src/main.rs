//! Process entry point.
//!
//! This file does only three things: set up the Tokio runtime, build AppState, and wire the router.
//! All concrete logic goes through the `phpyun_core` facade — we don't directly `use redis::` /
//! `use sqlx::` / `use metrics::` / `use dotenvy::`.

use phpyun_core::{
    background::spawn_periodic, config::Config, db, idempotency, metrics::install_prometheus,
    rayon_pool, shutdown, telemetry, AppState, CancellationToken, Scheduler,
};
use phpyun_handlers::build_router;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    let config = Config::load()?;

    // ---- 1. Initialize the global Rayon thread pool (rate-limited; doesn't steal tokio cores) ----
    rayon_pool::init(0); // 0 = auto: max(2, cpu/2)

    // ---- 2. Explicitly configured Tokio runtime ----
    let worker_threads = if config.worker_threads == 0 {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
    } else {
        config.worker_threads
    };

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(worker_threads)
        .max_blocking_threads(config.max_blocking_threads)
        .thread_stack_size(config.thread_stack_mb * 1024 * 1024)
        .thread_name("phpyun-worker")
        .enable_all()
        .build()?;

    runtime.block_on(async_main(config, worker_threads))
}

async fn async_main(config: Config, worker_threads: usize) -> anyhow::Result<()> {
    telemetry::init(&config.log_level, &config.env);
    tracing::info!(
        bind = %config.bind,
        env = %config.env,
        workers = worker_threads,
        "starting phpyun-rs"
    );

    // Prometheus metrics (separate port)
    install_prometheus(&config.metrics_bind)?;

    // Global Shutdown token
    let shutdown = CancellationToken::new();

    // AppState
    let state = AppState::build(config.clone(), shutdown.clone()).await?;

    // Run migrations (recommended false in prod; managed by a separate release pipeline via `sqlx migrate run`)
    if config.run_migrations_on_boot {
        db::run_migrations(&state.db).await.map_err(|e| anyhow::anyhow!(e.to_string()))?;
    }

    // Global cache for the dictionary translation table: synchronously load once into ArcSwap at startup,
    // and spawn a background fallback periodic refresher plus a Redis pubsub subscription that listens
    // for admin-triggered reloads.
    phpyun_services::dict_service::init_and_spawn_refresher(&state).await;

    // Global region tree (countries / states / cities). Same caching pattern as dict_service.
    phpyun_services::region_service::init_and_spawn_refresher(&state).await;

    // Background task: export DB pool metrics every minute (includes writer + reader)
    {
        let state_bg = state.clone();
        spawn_periodic(
            "db-pool-metrics",
            Duration::from_secs(60),
            shutdown.clone(),
            move || {
                let s = state_bg.clone();
                async move {
                    s.db.record_pool_metrics();
                }
            },
        );
    }

    // ---- Scheduled task engine (cron + distributed lock) ----
    //
    // This is the "pluggable" template: business code adds a new job by registering one more entry,
    // and main doesn't care what they actually do.
    // When deployed across multiple instances in production, the same job name runs globally exactly
    // once thanks to a Redis distributed lock.
    {
        let mut sch = Scheduler::new(state.redis.clone(), shutdown.clone());

        // Sweep for expired jobs every hour (state=1 and edate<=now → 2)
        {
            let s = state.clone();
            sch.cron("jobs.expire-overdue", "0 5 * * * *", move || {
                let s = s.clone();
                async move {
                    phpyun_services::maintenance::expire_jobs(&s).await;
                }
            })?
            .lock_ttl(Duration::from_secs(120));
        }

        // Clean up share-tokens every day at 3:10 AM
        {
            let s = state.clone();
            sch.cron("share_tokens.purge", "0 10 3 * * *", move || {
                let s = s.clone();
                async move {
                    phpyun_services::maintenance::purge_expired_share_tokens(&s).await;
                }
            })?;
        }

        // Clean up audit logs every day at 3:30 AM (retain 90 days)
        {
            let s = state.clone();
            sch.cron("audit_log.rotate", "0 30 3 * * *", move || {
                let s = s.clone();
                async move {
                    phpyun_services::maintenance::rotate_audit_log(&s).await;
                }
            })?;
        }

        // Clean up the recycle bin every day at 3:45 AM (retain 30 days)
        {
            let s = state.clone();
            sch.cron("recycle_bin.purge", "0 45 3 * * *", move || {
                let s = s.clone();
                async move {
                    phpyun_services::maintenance::purge_recycle_bin(&s).await;
                }
            })?;
        }

        sch.start();
    }

    // ---- Event-bus consumers (apply.created / vip.activated / chat.sent) ----
    phpyun_services::notification_consumers::start_all(&state);

    // HTTP service
    //
    // Router + cross-cutting middleware + state injection, then append the idempotency middleware
    // (idempotency needs AppState to attach, so it goes after with_state)
    let app = build_router(&config)
        .with_state(state.clone())
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            idempotency::layer,
        ));

    let listener = tokio::net::TcpListener::bind(&config.bind).await?;
    tracing::info!("listening on {}", config.bind);

    // Signal listener -> cancel the shutdown token -> axum stops accepting new connections and waits on in-flight ones
    let signal_token = shutdown.clone();
    tokio::spawn(async move {
        shutdown::wait_for_signal(signal_token).await;
    });

    let graceful_token = shutdown.clone();
    // with_connect_info: injects peer_addr into request extensions so the ClientIp extractor can read it
    if let Err(e) = axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .with_graceful_shutdown(async move {
        graceful_token.cancelled().await;
    })
    .await
    {
        tracing::error!(error = %e, "server error");
    }

    tracing::info!("server stopped; finalizing…");
    tokio::time::sleep(Duration::from_millis(500)).await;
    Ok(())
}
