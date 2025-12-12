use actix_web::{App, HttpServer, middleware::Logger};
use rust_forge_boilerplate::{
    common::infrastructure::{self, redis::RedisClient},
    healthcheck_modules::{self, repo::HealthCheckRepo},
};
use std::{env, sync::Arc};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let health_check_repo = Arc::new(HealthCheckRepo {});
    let health_check_service = Arc::new(healthcheck_modules::service::HealthCheckService {
        repo: health_check_repo.clone(),
    });
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable is required");
    let database_max_connections: u32 = env::var("DATABASE_MAX_CONNECTIONS")
        .unwrap_or_else(|_| "10".to_string())
        .parse()
        .expect("Invalid DATABASE_MAX_CONNECTIONS");
    print!("{}", database_url);
    let db_pool = infrastructure::database::create_pool(&database_url, database_max_connections)
        .await
        .expect("Failed to create database pool");

    let redis_url = env::var("REDIS_URL").expect("REDIS_URL environment variable is required");
    let redis_conn = infrastructure::redis::RedisClientImpl::create_connection(&redis_url)
        .await
        .expect("Failed to create Redis connection");

    let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let server_port: u16 = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("Invalid SERVER_PORT");

    let bind_address = format!("{}:{}", server_host, server_port);
    tracing::info!("Starting server at {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(actix_web::web::Data::new(health_check_service.clone()))
            .app_data(actix_web::web::Data::new(db_pool.clone()))
            .app_data(actix_web::web::Data::new(redis_conn.clone()))
            .configure(healthcheck_modules::configure_routes)
    })
    .bind(&bind_address)?
    .run()
    .await
}
