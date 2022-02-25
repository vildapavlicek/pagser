use http::{Request, Response};
use std::net::SocketAddr;
use std::time::Duration;
use tonic::{body::BoxBody, transport::Body};
use tower_http::classify::GrpcFailureClass;
use tracing::{debug, error, Span};

mod services;

pub async fn start_server() {
    let addr: SocketAddr = "[::1]:50051".parse().expect("invalid addr format");

    let layer = tower::ServiceBuilder::new()
        .layer(
            tower_http::trace::TraceLayer::new_for_grpc()
                .on_request(|request: &Request<Body>, _span: &Span| {
                    debug!(method = %request.method(), path = request.uri().path(), "processing request")
                })
                .on_response(
                    |response: &Response<BoxBody>,
                     latency: Duration,
                     _span: &Span| {
                        debug!(status = response.status().as_str(), latency_seconds = latency.as_secs(), "request processed")
                    },
                )
                .on_failure(
                    |error: GrpcFailureClass, latency: Duration, _span: &Span| {
                        error!(%error, latency_seconds = latency.as_secs(), "failed to process request")
                    },
                ),
        )
        .into_inner();

    tonic::transport::Server::builder()
        .layer(layer)
        .add_service(
            services::customer_service::customer_service::customer_service_server::CustomerServiceServer::new(
                services::customer_service::PagserCustomerService,
            ),
        )
        .serve(addr).await.expect("failed to start server");
}
