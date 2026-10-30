use std::io;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logger() -> io::Result<(WorkerGuard, WorkerGuard)> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,swampnet=debug,tokio=warn"));

    let (stdout_writer, stdout_guard) = tracing_appender::non_blocking(std::io::stdout());
    let stdout_layer = tracing_subscriber::fmt::layer()
        .json()
        .flatten_event(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_writer(stdout_writer)
        .with_filter(env_filter);

    let (file_writer, file_guard) =
        tracing_appender::non_blocking(tracing_appender::rolling::daily("./logs", "server.log"));

    let file_env_filter = EnvFilter::new("info,swampnet=trace");
    let file_layer = tracing_subscriber::fmt::layer()
        .json()
        .flatten_event(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_writer(file_writer)
        .with_filter(file_env_filter);

    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(file_layer)
        .init();

    Ok((stdout_guard, file_guard))
}
