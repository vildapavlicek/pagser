use super::{customer_service::CustomerService, customer_service_server::CustomerServiceServer};
use std::{net::SocketAddr, time::Duration};
use tonic::{
    body::BoxBody,
    transport::{Body, Server},
};
use tower_http::classify::GrpcFailureClass;
use tracing::{debug, error, info, Span};

pub async fn run() {
    let addr = std::env::var("PAGSER_ADDR")
        .unwrap_or_else(|_| "[::1]:50051".into())
        .parse::<SocketAddr>()
        .expect("failed parse address to bind to");

    let tracing_layer = tower::ServiceBuilder::new().layer(
        tower_http::trace::TraceLayer::new_for_grpc()
            .make_span_with(|_request: &http::Request<Body>| {
                tracing::info_span!("grpc_request", uuid = %uuid::Uuid::new_v4().to_string())
            })
            .on_request(|request: &http::Request<Body>, _span: &Span| info!(path = %request.uri().path(), "received new request"))
            .on_response(|_response: &http::Response<BoxBody>, latency: Duration, _span: &Span| {
                debug!(?latency, "generated response")
            })
            .on_failure(|failure_class: GrpcFailureClass, latency: Duration, _span: &Span| {
                error!(?latency, ?failure_class, "generated response with error code")
            })
    ).into_inner();

    Server::builder()
        .layer(tracing_layer)
        .add_service(CustomerServiceServer::new(CustomerService))
        .serve(addr)
        .await
        .expect("failed to run server");
}
