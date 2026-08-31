//! HTTP server binary for the PHPYun Rust backend.
//!
//! The workspace is otherwise library-only. This crate wires:
//! `Config::load` → telemetry → tokio runtime → `AppState` →
//! `assemble` (handlers + api-admin) → graceful shutdown.

use std::net::SocketAddr;
use std::time::Duration;

use anyhow::Context;
use phpyun_core::db::run_migrations;
use phpyun_core::scheduler::Scheduler;
use phpyun_core::shutdown::{wait_for_signal, CancellationToken};
use phpyun_core::{metrics, telemetry, AppState, Config};
use phpyun_handlers::assemble;

fn main() -> anyhow::Result<()> {
    let config = Config::load().context("load configuration")?;
    telemetry::init(&config.log_level, config.env);

    let mut builder = tokio::runtime::Builder::new_multi_thread();
    builder.enable_all().thread_name("phpyun-worker");
    if config.worker_threads > 0 {
        builder.worker_threads(config.worker_threads);
    }
    if config.thread_stack_mb > 0 {
        builder.thread_stack_size(config.thread_stack_mb.saturating_mul(1024 * 1024));
    }
    if config.max_blocking_threads > 0 {
        builder.max_blocking_threads(config.max_blocking_threads);
    }

    builder
        .build()
        .context("build tokio runtime")?
        .block_on(serve(config))
}

async fn serve(config: Config) -> anyhow::Result<()> {
    if let Err(e) = metrics::install_prometheus(&config.metrics_bind) {
        tracing::warn!(error = %e, bind = %config.metrics_bind, "prometheus exporter not started");
    }

    let shutdown = CancellationToken::new();
    let signal_token = shutdown.clone();
    tokio::spawn(async move {
        wait_for_signal(signal_token).await;
    });

    let state = AppState::build(config.clone(), shutdown.clone())
        .await
        .context("build AppState (MySQL / Redis / storage)")?;

    phpyun_core::dev_token::init(&config, state.db.pool(), &state.redis).await;

    if config.run_migrations_on_boot {
        run_migrations(&state.db)
            .await
            .map_err(|e| anyhow::anyhow!("sqlx migrations failed: {e}"))?;
    }

    start_scheduler(&state);

    let bind = config.bind.clone();
    let addr: SocketAddr = bind.parse().with_context(|| format!("parse BIND={bind}"))?;
    let admin_docs = phpyun_api_admin::openapi();
    let app = assemble(
        &config,
        phpyun_api_admin::router(state.clone()),
        Some(("/api-docs/admin/openapi.json", admin_docs)),
        phpyun_api_admin::get_allowed_paths(),
    )
    .with_state(state);

    tracing::info!(%addr, env = %config.env, "listening");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("bind {addr}"))?;

    let shutdown_for_serve = shutdown.clone();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(async move {
        shutdown_for_serve.cancelled().await;
        tracing::info!("shutdown signal received, draining connections");
    })
    .await
    .context("http server")?;

    tracing::info!("server stopped");
    Ok(())
}

fn start_scheduler(state: &AppState) {
    let mut sch = Scheduler::new(state.redis.clone(), state.shutdown.clone());

    let s = state.clone();
    if let Err(e) = sch.cron("expire_jobs", "0 0 * * * *", move || {
        let s = s.clone();
        async move {
            phpyun_services::maintenance::expire_jobs(&s).await;
        }
    }) {
        tracing::warn!(error = %e, "register expire_jobs cron failed");
    }

    let s = state.clone();
    if let Err(e) = sch.cron("purge_share_tokens", "0 15 3 * * *", move || {
        let s = s.clone();
        async move {
            phpyun_services::maintenance::purge_expired_share_tokens(&s).await;
        }
    }) {
        tracing::warn!(error = %e, "register purge_share_tokens cron failed");
    }

    let s = state.clone();
    if let Err(e) = sch.cron("rotate_audit_log", "0 30 3 * * *", move || {
        let s = s.clone();
        async move {
            phpyun_services::maintenance::rotate_audit_log(&s).await;
        }
    }) {
        tracing::warn!(error = %e, "register rotate_audit_log cron failed");
    }

    let s = state.clone();
    if let Err(e) = sch.cron("purge_recycle_bin", "0 45 3 * * *", move || {
        let s = s.clone();
        async move {
            phpyun_services::maintenance::purge_recycle_bin(&s).await;
        }
    }) {
        tracing::warn!(error = %e, "register purge_recycle_bin cron failed");
    }

    let s = state.clone();
    sch.interval("db_pool_metrics", Duration::from_secs(30), move || {
        let s = s.clone();
        async move {
            s.db.record_pool_metrics();
        }
    })
    .local_only();

    let _handle = sch.start();
}
