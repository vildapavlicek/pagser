use super::*;
use crate::db::DB;
use sqlx::postgres::PgArguments;
use tonic::{Request, Response, Status};

pub struct CustomerService {
    db: DB,
}

impl CustomerService {
    pub fn new(db: DB) -> Self {
        CustomerService { db }
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct CustomerDBO {
    #[sqlx(rename = "first_name")]
    name: String,
    last_name: String,
    #[sqlx(rename = "create_date")]
    registration_date: chrono::NaiveDate,
}

impl Into<Customer> for CustomerDBO {
    fn into(self) -> Customer {
        Customer {
            name: self.name,
            last_name: self.last_name,
            registration_date: self.registration_date.format("%Y-%m-%d").to_string(),
        }
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct CustomerDetailsDBO {
    #[sqlx(rename = "first_name")]
    name: String,
    last_name: String,
    #[sqlx(rename = "create_date")]
    registration_date: chrono::NaiveDate,
    address: String,
    address2: Option<String>,
    district: String,
    phone: String,
}

#[tonic::async_trait]
impl customer_service_server::CustomerService for CustomerService {
    #[doc = "Server streaming response type for the SelectCustomers method."]
    type SelectCustomersStream = tokio_stream::wrappers::ReceiverStream<Result<Customer, Status>>;
    #[doc = " Returns stream of all the customers stored in DB"]
    async fn select_customers(
        &self,
        _request: Request<CustomersRequest>,
    ) -> Result<Response<Self::SelectCustomersStream>, Status> {
        Err(Status::unimplemented("Not yet implemented".to_string()))
    }

    #[doc = " Returns 10 customers with newest created_date"]
    async fn select_newest_customers(
        &self,
        _request: Request<CustomersRequest>,
    ) -> Result<Response<NewestCustomersResponse>, Status> {
        self.db
            .fetch_all_as::<CustomerDBO, _>(
                r#"SELECT first_name, last_name, create_date
                FROM customer
                ORDER BY create_date DESC
                LIMIT 10"#,
                PgArguments::default(),
            )
            .await
            .map(|data| data.into_iter().map(|c| c.into()).collect())
            .map(|customer| Response::new(NewestCustomersResponse { customer }))
            .map_err(|err| Status::internal(format!("failed to query data from BD {err}")))
    }

    #[doc = " Returns single customer's details"]
    async fn customer_details(
        &self,
        request: Request<CustomerDetailsRequest>,
    ) -> Result<Response<CustomerDetailsResponse>, Status> {
        let mut args = PgArguments::default();
        sqlx::Arguments::add(&mut args, request.into_inner().id);
        match self
            .db
            .fetch_optional_as::<CustomerDetailsDBO, _>(
                r#"SELECT first_name, last_name, create_date, address, address2, district, phone 
                FROM customer c
                INNER JOIN address a
                ON c.address_id = a.address_id
                where c.customer_id = $1"#,
                args,
            )
            .await
        {
            Ok(Some(customer)) => {
                tracing::debug!(?customer, "got customer from DB");

                Ok(Response::new(CustomerDetailsResponse {
                    name: customer.name,
                    last_name: customer.last_name,
                    registration_date: customer.registration_date.format("%Y-%m-%d").to_string(),
                    address: customer.address,
                    address2: customer.address2,
                    district: customer.district,
                    phone: customer.phone,
                }))
            }
            Ok(None) => Err(Status::invalid_argument(
                "No customer found for requested ID",
            )),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}
