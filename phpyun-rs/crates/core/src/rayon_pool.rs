//! Rayon global thread pool initialization.
//!
//! **Important**: by default Rayon spawns N workers (N = number of CPU cores), which
//! would contend with Tokio's workers for CPU. We explicitly cap it at
//! `max(2, cpu/2)`, leaving headroom for the I/O-heavy Tokio runtime.
//!
//! Also, in async contexts you must **never** call Rayon directly (`par_iter`, etc.);
//! always wrap it in `tokio::task::spawn_blocking(|| rayon::...)` so the synchronous
//! blocking work is offloaded to the blocking pool.

use std::sync::OnceLock;

static INIT: OnceLock<()> = OnceLock::new();

pub fn init(threads: usize) {
    INIT.get_or_init(|| {
        let n = if threads == 0 {
            (std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4)
                / 2)
                .max(2)
        } else {
            threads
        };

        match rayon::ThreadPoolBuilder::new()
            .num_threads(n)
            .thread_name(|i| format!("rayon-{i}"))
            .build_global()
        {
            Ok(_) => tracing::info!(threads = n, "rayon global pool initialized"),
            Err(e) => tracing::warn!(error = %e, "rayon already initialized"),
        }
    });
}
