use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{prelude::*, EnvFilter};

pub fn init() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::daily(".", env!("CARGO_PKG_NAME"));
    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::Registry::default()
        .with(EnvFilter::new(format!("{}=trace", env!("CARGO_PKG_NAME"))))
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(file_writer)
                .with_ansi(false),
        )
        .with(tracing_subscriber::fmt::layer().with_ansi(true).pretty())
        .init();

    guard
}
