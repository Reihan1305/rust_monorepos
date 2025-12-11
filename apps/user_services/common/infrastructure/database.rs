use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};

pub trait DatabaseTrait<T>
where
    T: sqlx::Database,
{
    async fn create_pool(database_url: &str, max_connections: u32) -> Result<Pool<T>, sqlx::Error>;
}

pub async fn create_pool(database_url: &str, max_connections: u32) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(database_url)
        .await
}

pub struct PostgresDatabase;

impl DatabaseTrait<Postgres> for PostgresDatabase {
    async fn create_pool(
        database_url: &str,
        max_connections: u32,
    ) -> Result<Pool<Postgres>, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(database_url)
            .await
    }
}
