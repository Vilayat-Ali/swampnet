use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, prelude::*};

pub fn setup_logger() {
    let timer = fmt::time::UtcTime::rfc_3339();
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            fmt::layer()
                .with_timer(timer)
                .with_target(true)
                .with_ansi(true)
                .with_thread_ids(true),
        )
        .init();
}
