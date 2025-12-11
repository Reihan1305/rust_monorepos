use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::Error;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub enum AppError {
    InternalError(String),
    BadRequest(String),
    NotFound(String),
    Unauthorized(String),
    ValidationError(String),
    DatabaseError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

pub trait AppErrorTrait {
    fn map_db_error(err: Error) -> AppError;
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InternalError(msg) => write!(f, "Internal Error: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database Error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::ValidationError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            error: self.status_code().to_string(),
            message: self.to_string(),
        };
        HttpResponse::build(self.status_code()).json(error_response)
    }
}

impl AppErrorTrait for AppError {
    fn map_db_error(err: Error) -> Self {
        if let Error::Database(db_err) = &err {
            let code = db_err.code().unwrap_or_default();
            let constraint = db_err.constraint().unwrap_or_default();
            if code == "23505" {
                let parts: Vec<&str> = constraint.split('_').collect();
                let field: String;
                if parts.len() < 3 {
                    field = constraint.to_string();
                } else {
                    let field_parts = &parts[1..parts.len() - 1];

                    field = field_parts.join(" ");
                }

                return AppError::BadRequest(format!("{} already exists", field).into());
            }
        }

        return AppError::InternalError(err.to_string());
    }
}

pub type AppResult<T> = Result<T, AppError>;
