use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};
use std::future::Future;

pub trait DatabaseTrait<T>
where
    T: sqlx::Database,
{
    fn create_pool(
        database_url: &str,
        max_connections: u32,
    ) -> impl Future<Output = Result<Pool<T>, sqlx::Error>> + Send;
}

pub async fn create_pool(database_url: &str, max_connections: u32) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(database_url)
        .await
}

pub struct PostgresDatabase;

impl DatabaseTrait<Postgres> for PostgresDatabase {
    fn create_pool(
        database_url: &str,
        max_connections: u32,
    ) -> impl Future<Output = Result<Pool<Postgres>, sqlx::Error>> + Send {
        async move {
            PgPoolOptions::new()
                .max_connections(max_connections)
                .connect(database_url)
                .await
        }
    }
}
