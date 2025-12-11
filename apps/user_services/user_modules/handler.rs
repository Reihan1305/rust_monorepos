use std::sync::Arc;

use actix_web::{web, HttpMessage, HttpResponse, Responder};
use sqlx::PgPool;

use crate::{
    common::utils::hash::CryptoUtils,
    user_modules::{
        dto::CreateUserDto,
        service::{UserServiceTrait, UserServices},
    },
};

pub async fn register_user(
    user_services: web::Data<Arc<UserServices>>,
    pg_pool: web::Data<PgPool>,
    req: actix_web::HttpRequest,
) -> impl Responder {
    let mut data: CreateUserDto = match req.extensions_mut().remove::<CreateUserDto>() {
        Some(d) => d,
        None => {
            return HttpResponse::BadRequest().json("Missing validated payload");
        }
    };

    if let Ok(hash_password) = CryptoUtils::hash_password(&data.password) {
        data.password = hash_password;
    } else {
        return HttpResponse::InternalServerError().json("Password hash error");
    }
    let new_user = user_services.create_user(&pg_pool, data).await;

    match new_user {
        Ok(user) => return HttpResponse::Ok().json(user),
        Err(e) => return e,
    }
}
