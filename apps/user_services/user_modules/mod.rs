use actix_web::{
    body::BoxBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::{from_fn, Next},
    web::{self, Json},
    Error, HttpMessage, HttpResponse,
};
use serde_json::{json, Value};
use validator::{Validate, ValidationErrors};

use crate::user_modules::dto::CreateUserDto;

pub mod dto;
pub mod handler;
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

async fn validate_json<T>(
    mut req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error>
where
    T: serde::de::DeserializeOwned + Validate + std::fmt::Debug + 'static,
{
    let payload = match req.extract::<Json<T>>().await {
        Ok(data) => data,
        Err(e) => {
            let res = HttpResponse::BadRequest().json(json!({
                "error": "invalid_json",
                "message": e.to_string()
            }));
            return Ok(req.into_response(res));
        }
    };

    if let Err(e) = payload.validate() {
        let res = HttpResponse::BadRequest().json(json!({
            "error": "validation_failed",
            "messages": extract_errors(e)
        }));
        return Ok(req.into_response(res));
    }

    req.extensions_mut().insert(payload.into_inner());
    next.call(req).await
}

pub fn extract_errors(err: ValidationErrors) -> Vec<Value> {
    let mut errors = Vec::new();

    for (field, field_errors) in err.field_errors() {
        if let Some(err) = field_errors.first() {
            errors.push(json!({
                "field": field,
                "message": err.message
            }));
        }
    }

    errors
}
