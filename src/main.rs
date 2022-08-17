use tracing::info;

mod db;
mod grpc;
mod logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = logger::init();
    info!(
        version = env!("CARGO_PKG_VERSION"),
        git_hash = env!("GIT_HASH"),
        "Starting server"
    );

    let db =
        db::DB::connect_lazy("postgres://pagser:pagser1234@127.0.0.1:5432/pagila?sslmode=disable");

    grpc::server::run(db?).await;
    Ok(())
}
