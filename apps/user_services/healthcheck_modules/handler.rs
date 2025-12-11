use std::sync::Arc;

use crate::healthcheck_modules::{
    dto::{HealthResponse, ReadinessResponse},
    service::{HealthCheckService, HealthCheckServicesTrait},
};
use actix_web::{web, HttpResponse};
use redis::aio::ConnectionManager;
use sqlx::PgPool;

pub async fn health_check() -> HttpResponse {
    let response = HealthResponse {
        status: "ok".to_string(),
        service: "rust_forge_boilerplate".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    HttpResponse::Ok().json(response)
}

pub async fn readiness_check(
    service: web::Data<Arc<HealthCheckService>>,
    db_pool: web::Data<PgPool>,
    redis_conn: web::Data<ConnectionManager>,
) -> HttpResponse {
    let mut redis_clone = redis_conn.as_ref().clone();

    let db_fut = async { service.ping_db(db_pool.get_ref()).await };

    let redis_fut = async { service.ping_redis(&mut redis_clone).await };

    let (db_ready, redis_ready) = tokio::join!(db_fut, redis_fut);

    let ready = db_ready && redis_ready;

    let response = ReadinessResponse {
        ready,
        database: db_ready,
        redis: redis_ready,
    };

    if ready {
        HttpResponse::Ok().json(response)
    } else {
        HttpResponse::ServiceUnavailable().json(response)
    }
}
