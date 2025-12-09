use crate::healthcheck_modules::dto::{HealthResponse, ReadinessResponse};
use actix_web::{web, HttpResponse};
use mongodb::Database;
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
    db_pool: web::Data<PgPool>,
    redis_conn: web::Data<ConnectionManager>,
    mongo_db: web::Data<Database>,
) -> HttpResponse {
    let mut ready = true;

    let db_ready = sqlx::query("SELECT 1")
        .fetch_one(db_pool.get_ref())
        .await
        .is_ok();
    if !db_ready {
        ready = false;
    }

    let mut redis_conn_clone = redis_conn.as_ref().clone();
    let redis_ready = redis::cmd("PING")
        .query_async::<_, String>(&mut redis_conn_clone)
        .await
        .is_ok();
    if !redis_ready {
        ready = false;
    }

    let mongo_ready = mongo_db
        .run_command(mongodb::bson::doc! { "ping": 1 }, None)
        .await
        .is_ok();
    if !mongo_ready {
        ready = false;
    }

    let response = ReadinessResponse {
        ready,
        database: db_ready,
        redis: redis_ready,
        mongodb: mongo_ready,
    };

    if ready {
        HttpResponse::Ok().json(response)
    } else {
        HttpResponse::ServiceUnavailable().json(response)
    }
}
