pub mod customer_service {
    tonic::include_proto!("customer_service");
}

use customer_service::{
    customer_service_server::CustomerService, Customer, CustomerDetailsRequest,
    CustomerDetailsResponse, CustomersRequest, CustomersResponse,
};

use tokio_stream::wrappers::ReceiverStream;
use tonic::{codegen::futures_core, Request, Response, Status};

pub struct PagserCustomerService;

#[tonic::async_trait]
impl CustomerService for PagserCustomerService {
    type SelectCustomersStream = ReceiverStream<Result<Customer, Status>>;

    #[tracing::instrument(skip(self, request))]
    async fn select_customers(
        &self,
        request: Request<CustomersRequest>,
    ) -> Result<Response<Self::SelectCustomersStream>, Status> {
        tracing::debug!(?request, "received request");
        Err(Status::unimplemented("Not yet implemented"))
    }

    #[tracing::instrument(skip(self, request))]
    async fn select_newest_customers(
        &self,
        request: Request<CustomersRequest>,
    ) -> Result<Response<CustomersResponse>, Status> {
        tracing::debug!(?request, "received request");
        Err(Status::unimplemented("Not yet implemented"))
    }

    #[tracing::instrument(skip(self, request))]
    async fn customer_details(
        &self,
        request: Request<CustomerDetailsRequest>,
    ) -> Result<Response<CustomerDetailsResponse>, tonic::Status> {
        tracing::debug!(?request, "received request");
        Err(Status::unimplemented("Not yet implemented"))
    }
}
