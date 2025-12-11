use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use user_services::common::{config::AppConfig, infrastructure};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = AppConfig::new()?;

    let _db_pool = infrastructure::database::create_pool(
        &config.database.url,
        config.database.max_connections,
    )
    .await?;

    let _redis_conn = infrastructure::redis::create_connection(&config.redis.url).await?;

    tracing::info!("Worker started");

    loop {
        // TODO: Implement job processing logic
        // Example: fetch jobs from Redis queue, process them
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        tracing::debug!("Worker tick...");
    }
}
