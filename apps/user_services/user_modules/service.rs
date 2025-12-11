use std::sync::Arc;

use actix_web::{HttpResponse, ResponseError};
use sqlx::PgPool;

use crate::{
    common::utils::error::{AppError, AppErrorTrait},
    user_modules::{
        dto::{CreateUserDto, CreateUserResponse},
        repo::UserRepoTrait,
    },
};

#[async_trait::async_trait]
pub trait UserServiceTrait {
    async fn create_user(
        &self,
        pg_pool: &PgPool,
        user: CreateUserDto,
    ) -> Result<CreateUserResponse, HttpResponse>;
}

pub struct UserServices {
    user_repo: Arc<dyn UserRepoTrait + Send + Sync>,
}

#[async_trait::async_trait]
impl UserServiceTrait for UserServices {
    async fn create_user(
        &self,
        pg_pool: &PgPool,
        user: CreateUserDto,
    ) -> Result<CreateUserResponse, HttpResponse> {
        let db_response: Result<CreateUserResponse, sqlx::Error> =
            self.user_repo.create_user(pg_pool, user).await;
        match db_response {
            Ok(response) => Ok(response),
            Err(e) => {
                let error = AppError::map_db_error(e);
                Err(error.error_response())
            }
        }
    }
}

impl UserServices {
    pub fn new(user_repo: Arc<dyn UserRepoTrait + Send + Sync>) -> UserServices {
        UserServices { user_repo }
    }
}
