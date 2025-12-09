use actix_web::{middleware::Logger, App, HttpServer};
use rust_forge_boilerplate::{
    common::{config::AppConfig, infrastructure},
    healthcheck_modules,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = AppConfig::new().expect("Failed to load configuration");

    // Initialize database pool
    let db_pool = infrastructure::database::create_pool(
        &config.database.url,
        config.database.max_connections,
    )
    .await
    .expect("Failed to create database pool");

    // Initialize Redis connection
    let redis_conn = infrastructure::redis::create_connection(&config.redis.url)
        .await
        .expect("Failed to create Redis connection");

    // Initialize MongoDB
    let mongo_db = infrastructure::mongodb::create_client(&config.mongodb.url, &config.mongodb.database)
        .await
        .expect("Failed to create MongoDB client");

    let bind_address = format!("{}:{}", config.server.host, config.server.port);
    tracing::info!("Starting server at {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(actix_web::web::Data::new(db_pool.clone()))
            .app_data(actix_web::web::Data::new(redis_conn.clone()))
            .app_data(actix_web::web::Data::new(mongo_db.clone()))
            .configure(healthcheck_modules::configure_routes)
    })
    .bind(&bind_address)?
    .run()
    .await
}
