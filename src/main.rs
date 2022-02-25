mod grpc;
mod logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = logger::init();

    tracing::debug!(
        name = env!("CARGO_PKG_NAME"),
        version = env!("CARGO_PKG_VERSION"),
        commit = env!("GIT_HASH"),
        "App started",
    );

    grpc::start_server().await;
    Ok(())
}
