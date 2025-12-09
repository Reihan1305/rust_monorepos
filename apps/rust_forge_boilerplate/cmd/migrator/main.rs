use rust_forge_boilerplate::common::{config::AppConfig, infrastructure};
use sqlx::migrate::MigrateDatabase;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

    tracing::info!("Running migrations...");

    // Create database if it doesn't exist
    if !sqlx::Postgres::database_exists(&config.database.url).await? {
        tracing::info!("Database does not exist, creating...");
        sqlx::Postgres::create_database(&config.database.url).await?;
    }

    // Run migrations
    let pool = infrastructure::database::create_pool(
        &config.database.url,
        config.database.max_connections,
    )
    .await?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    tracing::info!("Migrations completed successfully");

    Ok(())
}
