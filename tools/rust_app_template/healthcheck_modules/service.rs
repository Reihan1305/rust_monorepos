use redis::aio::ConnectionManager;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

use crate::healthcheck_modules::repo::HealthCheckRepoTrait;

#[async_trait::async_trait]
pub trait HealthCheckServicesTrait: Send + Sync {
    async fn ping_db(&self, pg_pool: &Pool<Postgres>) -> bool;
    async fn ping_redis(&self, redis_conn: &mut ConnectionManager) -> bool;
}

pub struct HealthCheckService {
    pub repo: Arc<dyn HealthCheckRepoTrait>,
}

#[async_trait::async_trait]
impl HealthCheckServicesTrait for HealthCheckService {
    async fn ping_db(&self, pg_pool: &Pool<Postgres>) -> bool {
        self.repo.ping(pg_pool).await.is_ok()
    }

    async fn ping_redis(&self, redis_conn: &mut ConnectionManager) -> bool {
        redis::cmd("PING")
            .query_async::<_, String>(redis_conn)
            .await
            .is_ok()
    }
}
