use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

pub fn init() -> tracing_appender::non_blocking::WorkerGuard {
    let app_name = env!("CARGO_PKG_NAME");
    let file_writer = tracing_appender::rolling::daily("./logs", format!("{app_name}.log"));

    let (non_blocking_appender, guard) = tracing_appender::non_blocking(file_writer);

    tracing_subscriber::Registry::default()
        .with(EnvFilter::new(format!("{app_name}=trace")))
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_writer(non_blocking_appender),
        )
        .with(tracing_subscriber::fmt::layer().with_ansi(true).pretty())
        .init();

    guard
}
