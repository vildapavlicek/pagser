use super::{customer_service::CustomerService, customer_service_server::CustomerServiceServer};
use std::net::SocketAddr;
use tonic::transport::Server;

pub async fn run() {
    let addr = std::env::var("PAGSER_ADDR")
        .unwrap_or_else(|_| "[::1]:50051".into())
        .parse::<SocketAddr>()
        .expect("failed parse address to bind to");

    Server::builder()
        .add_service(CustomerServiceServer::new(CustomerService))
        .serve(addr)
        .await
        .expect("failed to run server");
}
