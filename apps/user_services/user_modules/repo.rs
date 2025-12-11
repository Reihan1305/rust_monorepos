use sqlx::PgPool;

use crate::user_modules::dto::{CreateUserDto, CreateUserResponse};

pub struct UserRepo;

#[async_trait::async_trait]
pub trait UserRepoTrait {
    async fn create_user(
        &self,
        pg_pool: &PgPool,
        data: CreateUserDto,
    ) -> Result<CreateUserResponse, sqlx::Error>;
}

#[async_trait::async_trait]
impl UserRepoTrait for UserRepo {
    async fn create_user(
        &self,
        pg_pool: &PgPool,
        data: CreateUserDto,
    ) -> Result<CreateUserResponse, sqlx::Error> {
        let query = "INSERT INTO users (
            first_name,
            last_name,
            email,
            phone_number,
            password
        ) VALUES ( $1, $2, $3, $4, $5)
        RETURNING id, first_name, last_name, email, phone_number;";
        match sqlx::query_as::<_, CreateUserResponse>(query)
            .bind(&data.first_name)
            .bind(&data.last_name)
            .bind(&data.email)
            .bind(&data.phone_number)
            .bind(&data.password)
            .fetch_one(pg_pool)
            .await
        {
            Ok(response) => return Ok(response),
            Err(e) => {
                print!("here the error:{}", &e);
                return Err(e);
            }
        }
    }
}

impl UserRepo {
    pub fn new() -> UserRepo {
        UserRepo {}
    }
}
