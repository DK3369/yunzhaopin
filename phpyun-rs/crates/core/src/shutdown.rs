//! Graceful shutdown coordination using `tokio_util::sync::CancellationToken`.
//!
//! Why CancellationToken instead of watch/broadcast:
//! - One-shot semantics (once cancelled, stays cancelled — fits shutdown perfectly)
//! - Cheap to clone (internally an `Arc`)
//! - Child tokens can be created (without affecting the parent)
//! - `.cancelled()` returns a future, very ergonomic inside `tokio::select!`

use tokio::signal;
pub use tokio_util::sync::CancellationToken;

/// Listen for SIGTERM / Ctrl-C and cancel the given token when received.
pub async fn wait_for_signal(token: CancellationToken) {
    let ctrl_c = async {
        signal::ctrl_c().await.ok();
    };

    #[cfg(unix)]
    let terminate = async {
        if let Ok(mut sig) = signal::unix::signal(signal::unix::SignalKind::terminate()) {
            sig.recv().await;
        }
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => tracing::info!("received Ctrl-C"),
        _ = terminate => tracing::info!("received SIGTERM"),
    }
    token.cancel();
}
