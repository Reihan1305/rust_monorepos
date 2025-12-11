use rust_forge_boilerplate::common::infrastructure;
use std::env;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url: String =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable is required");
    let database_max_connections: u32 = env::var("DATABASE_MAX_CONNECTIONS")
        .unwrap_or_else(|_| "10".to_string())
        .parse()
        .expect("Invalid DATABASE_MAX_CONNECTIONS");

    let _db_pool =
        infrastructure::database::create_pool(&database_url, database_max_connections).await?;

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
