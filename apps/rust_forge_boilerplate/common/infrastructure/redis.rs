use redis::{aio::ConnectionManager, Client};

pub async fn create_connection(redis_url: &str) -> Result<ConnectionManager, redis::RedisError> {
    let client = Client::open(redis_url)?;
    ConnectionManager::new(client).await
}
