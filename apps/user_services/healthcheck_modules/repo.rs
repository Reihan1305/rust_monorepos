use sqlx::{Pool, Postgres};

#[async_trait::async_trait]
pub trait HealthCheckRepoTrait: Send + Sync {
    async fn ping(&self, pg_pool: &Pool<Postgres>) -> Result<(), sqlx::Error>;
}

pub struct HealthCheckRepo;

#[async_trait::async_trait]
impl HealthCheckRepoTrait for HealthCheckRepo {
    async fn ping(&self, pg_pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT 1").execute(pg_pool).await.map(|_| ())
    }
}
