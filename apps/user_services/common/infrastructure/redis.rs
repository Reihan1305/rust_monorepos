use redis::{aio::ConnectionManager, Client};
use std::future::Future;

pub trait RedisClient {
    fn create_connection(redis_url: &str) -> impl Future<Output = Result<ConnectionManager, redis::RedisError>> + Send;
}

pub struct RedisClientImpl;

impl RedisClient for RedisClientImpl {
    fn create_connection(redis_url: &str) -> impl Future<Output = Result<ConnectionManager, redis::RedisError>> + Send {
        async move {
            let client = Client::open(redis_url)?;
            ConnectionManager::new(client).await
        }
    }
}
