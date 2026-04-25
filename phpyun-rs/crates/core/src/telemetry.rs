use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub fn init(level: &str, env: &str) {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(level));

    let fmt_layer = if env == "prod" {
        fmt::layer().json().with_target(true).boxed()
    } else {
        fmt::layer().pretty().boxed()
    };

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .init();

    tracing::info!(env = env, "telemetry initialized");
}
