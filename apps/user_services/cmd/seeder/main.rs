use user_services::common::{config::AppConfig, infrastructure};
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

    let _db_pool = infrastructure::database::create_pool(
        &config.database.url,
        config.database.max_connections,
    )
    .await?;

    tracing::info!("Running seeders...");

    // TODO: Add your seeding logic here
    // Example:
    // sqlx::query!("INSERT INTO users (name, email) VALUES ($1, $2)", "Admin", "admin@example.com")
    //     .execute(&db_pool)
    //     .await?;

    tracing::info!("Seeding completed successfully");

    Ok(())
}
