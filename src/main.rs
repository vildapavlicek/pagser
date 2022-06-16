use tracing::info;

mod grpc;
mod logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = logger::init();
    info!(
        version = env!("CARGO_PKG_VERSION"),
        git_hash = env!("GIT_HASH"),
        "Starting server..."
    );

    grpc::server::run().await;
    Ok(())
}
