pub mod dto;
pub mod handler;
pub mod repo;
use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(handler::health_check))
            .route("/ready", web::get().to(handler::readiness_check)),
    );
}
