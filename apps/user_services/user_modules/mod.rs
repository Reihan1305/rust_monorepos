use actix_web::{
    middleware::from_fn,
    web::{self},
};

use crate::user_modules::{dto::CreateUserDto, middleware::validate_json};

pub mod dto;
pub mod handler;
pub mod middleware;
pub mod repo;
pub mod service;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users").route(
            "/",
            web::post()
                .to(handler::register_user)
                .wrap(from_fn(validate_json::<CreateUserDto>)),
        ),
    );
}
