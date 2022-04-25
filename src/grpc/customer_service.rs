use super::*;
use tonic::{Request, Response, Status};

pub struct CustomerService;

#[tonic::async_trait]
impl customer_service_server::CustomerService for CustomerService {
    #[doc = "Server streaming response type for the SelectCustomers method."]
        type SelectCustomersStream = tokio_stream::wrappers::ReceiverStream<Result<Customer, Status>>;
        #[doc = " Returns stream of all the customers stored in DB"]
        async fn select_customers(
            &self,
            request: Request<CustomersRequest>,
        ) -> Result<Response<Self::SelectCustomersStream>, Status> {
            Err(Status::unimplemented("Not yet implemented".to_string()))
        }

        #[doc = " Returns 10 customers with newest created_date"]
        async fn select_newest_customers(
            &self,
            request: Request<CustomersRequest>,
        ) -> Result<Response<NewestCustomersResponse>, Status> {Err(Status::unimplemented("Not yet implemented".to_string()))}

        #[doc = " Returns single customer's details"]
        async fn customer_details(
            &self,
            request: Request<CustomerDetailsRequest>,
        ) -> Result<Response<CustomerDetailsResponse>, Status> {Err(Status::unimplemented("Not yet implemented".to_string()))}
}