use std::env;

use sqlx::migrate::MigrateDatabase;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use user_services::common::infrastructure;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Running migrations...");

    let database_url: String =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable is required");
    let database_max_connections: u32 = env::var("DATABASE_MAX_CONNECTIONS")
        .unwrap_or_else(|_| "10".to_string())
        .parse()
        .expect("Invalid DATABASE_MAX_CONNECTIONS");

    if !sqlx::Postgres::database_exists(&database_url).await? {
        tracing::info!("Database does not exist, creating...");
        sqlx::Postgres::create_database(&database_url).await?;
    }

    let pool =
        infrastructure::database::create_pool(&database_url, database_max_connections).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    tracing::info!("Migrations completed successfully");

    Ok(())
}
