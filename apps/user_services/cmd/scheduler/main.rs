use tokio_cron_scheduler::{Job, JobScheduler};
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

    tracing::info!("Scheduler started");

    let scheduler = JobScheduler::new().await?;

    scheduler
        .add(Job::new_async("0 * * * * *", |_uuid, _l| {
            Box::pin(async move {
                tracing::info!("Scheduled job executed");
            })
        })?)
        .await?;

    scheduler.start().await?;

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
