use sqlx::{
    postgres::{PgArguments, PgRow},
    FromRow, PgPool,
};

#[derive(Clone)]
pub struct DB {
    inner: PgPool,
}

impl DB {
    #[tracing::instrument]
    pub fn connect_lazy(dsn: &str) -> Result<Self, sqlx::Error> {
        tracing::trace!("creating DB connection pool");
        PgPool::connect_lazy(dsn).map(|pool| DB { inner: pool })
    }

    pub async fn fetch_customer_details(&self, id: i32) -> Result<Option<PgRow>, sqlx::Error> {
        let mut args = PgArguments::default();
        sqlx::Arguments::add(&mut args, id);
        sqlx::query_with(
            r#"SELECT first_name, last_name, create_date, address, address2, district, phone 
        FROM customer c
        INNER JOIN address a
        ON c.address_id = a.address_id
        where c.customer_id = $1"#,
            args,
        )
        .fetch_optional(&mut self.inner.acquire().await?)
        .await
    }

    #[tracing::instrument(skip(self, query, arguments))]
    pub async fn fetch_optional_as<O, A>(
        &self,
        query: &str,
        arguments: A,
    ) -> Result<Option<O>, sqlx::Error>
    where
        O: Send + Unpin + for<'r> FromRow<'r, PgRow>,
        A: Into<PgArguments>,
    {
        sqlx::query_as_with::<_, O, PgArguments>(query, arguments.into())
            .fetch_optional(&mut self.inner.acquire().await?)
            .await
    }

    #[tracing::instrument(skip(self, query, arguments))]
    pub async fn fetch_all_as<O, A>(&self, query: &str, arguments: A) -> Result<Vec<O>, sqlx::Error>
    where
        O: Send + Unpin + for<'r> FromRow<'r, PgRow>,
        A: Into<PgArguments>,
    {
        sqlx::query_as_with::<_, O, PgArguments>(query, arguments.into())
            .fetch_all(&mut self.inner.acquire().await?)
            .await
    }

    #[tracing::instrument(skip(self, query))]
    pub async fn fetch_optional_scalar<O>(&self, query: &str) -> Result<Vec<O>, sqlx::Error>
    where
        O: for<'r> sqlx::Decode<'r, sqlx::Postgres> + sqlx::Type<sqlx::Postgres> + Send + Unpin,
    {
        sqlx::query_scalar::<_, O>(query)
            .fetch_all(&mut self.inner.acquire().await?)
            .await
    }
}
