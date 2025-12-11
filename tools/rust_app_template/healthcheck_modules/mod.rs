pub mod dto;
pub mod handler;
use actix_web::web;
pub mod repo;
pub mod service;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", web::get().to(handler::health_check))
            .route("/ready", web::get().to(handler::readiness_check)),
    );
}
