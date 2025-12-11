use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::Error;
use std::{collections::HashMap, fmt, sync::OnceLock};
use validator::ValidationErrors;

#[derive(Debug, Serialize, Deserialize)]
pub enum AppError {
    InternalError(String),
    BadRequest(String),
    NotFound(String),
    Unauthorized(String),
    ValidationError(String),
    DatabaseError(String),
    // User-specific errors
    UserNotFound,
    UserAlreadyExists,
    InvalidCredentials,
    EmailAlreadyExists,
    WeakPassword,
    InvalidEmailFormat,
    TokenExpired,
    TokenInvalid,
    PermissionDenied,
    RateLimitExceeded,
}

static ERROR_MESSAGES: OnceLock<HashMap<String, String>> = OnceLock::new();

fn load_error_messages() -> &'static HashMap<String, String> {
    ERROR_MESSAGES.get_or_init(|| {
        let error_json = include_str!("../../error.json");
        serde_json::from_str(error_json).unwrap_or_else(|_| HashMap::new())
    })
}

fn get_error_message(code: u16, fallback: &str) -> String {
    let messages = load_error_messages();
    messages.get(&code.to_string()).cloned().unwrap_or_else(|| fallback.to_string())
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
}

pub trait AppErrorTrait {
    fn map_db_error(err: Error) -> AppError;
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            AppError::InternalError(msg) => format!("Internal Error: {}", msg),
            AppError::BadRequest(msg) => format!("Bad Request: {}", msg),
            AppError::NotFound(msg) => format!("Not Found: {}", msg),
            AppError::Unauthorized(msg) => format!("Unauthorized: {}", msg),
            AppError::ValidationError(msg) => format!("Validation Error: {}", msg),
            AppError::DatabaseError(msg) => format!("Database Error: {}", msg),
            AppError::UserNotFound => get_error_message(2000, "User not found"),
            AppError::UserAlreadyExists => get_error_message(2001, "User already exists"),
            AppError::InvalidCredentials => get_error_message(2002, "Invalid credentials"),
            AppError::EmailAlreadyExists => get_error_message(2003, "Email already exists"),
            AppError::WeakPassword => get_error_message(2004, "Password is too weak"),
            AppError::InvalidEmailFormat => get_error_message(2005, "Invalid email format"),
            AppError::TokenExpired => get_error_message(2006, "Token has expired"),
            AppError::TokenInvalid => get_error_message(2007, "Token is invalid"),
            AppError::PermissionDenied => get_error_message(2008, "Permission denied"),
            AppError::RateLimitExceeded => get_error_message(2009, "Rate limit exceeded"),
        };
        write!(f, "{}", message)
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
            AppError::UserNotFound => StatusCode::NOT_FOUND,
            AppError::UserAlreadyExists => StatusCode::CONFLICT,
            AppError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            AppError::EmailAlreadyExists => StatusCode::CONFLICT,
            AppError::WeakPassword => StatusCode::BAD_REQUEST,
            AppError::InvalidEmailFormat => StatusCode::BAD_REQUEST,
            AppError::TokenExpired => StatusCode::UNAUTHORIZED,
            AppError::TokenInvalid => StatusCode::UNAUTHORIZED,
            AppError::PermissionDenied => StatusCode::FORBIDDEN,
            AppError::RateLimitExceeded => StatusCode::TOO_MANY_REQUESTS,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let (code, message) = match self {
            AppError::InternalError(msg) => (1000, msg.clone()),
            AppError::BadRequest(msg) => (1001, msg.clone()),
            AppError::NotFound(msg) => (1002, msg.clone()),
            AppError::Unauthorized(msg) => (1003, msg.clone()),
            AppError::ValidationError(msg) => (1004, msg.clone()),
            AppError::DatabaseError(msg) => (1005, msg.clone()),
            AppError::UserNotFound => (2000, get_error_message(2000, "User not found")),
            AppError::UserAlreadyExists => (2001, get_error_message(2001, "User already exists")),
            AppError::InvalidCredentials => (2002, get_error_message(2002, "Invalid credentials")),
            AppError::EmailAlreadyExists => (2003, get_error_message(2003, "Email already exists")),
            AppError::WeakPassword => (2004, get_error_message(2004, "Password is too weak")),
            AppError::InvalidEmailFormat => (2005, get_error_message(2005, "Invalid email format")),
            AppError::TokenExpired => (2006, get_error_message(2006, "Token has expired")),
            AppError::TokenInvalid => (2007, get_error_message(2007, "Token is invalid")),
            AppError::PermissionDenied => (2008, get_error_message(2008, "Permission denied")),
            AppError::RateLimitExceeded => (2009, get_error_message(2009, "Rate limit exceeded")),
        };
        
        let error_response = ErrorResponse {
            code,
            message,
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

impl AppError {
    // Convenience constructors for common user errors
    pub fn user_not_found() -> Self {
        AppError::UserNotFound
    }
    
    pub fn user_already_exists() -> Self {
        AppError::UserAlreadyExists
    }
    
    pub fn invalid_credentials() -> Self {
        AppError::InvalidCredentials
    }
    
    pub fn email_already_exists() -> Self {
        AppError::EmailAlreadyExists
    }
    
    pub fn weak_password() -> Self {
        AppError::WeakPassword
    }
    
    pub fn invalid_email_format() -> Self {
        AppError::InvalidEmailFormat
    }
    
    pub fn token_expired() -> Self {
        AppError::TokenExpired
    }
    
    pub fn token_invalid() -> Self {
        AppError::TokenInvalid
    }
    
    pub fn permission_denied() -> Self {
        AppError::PermissionDenied
    }
    
    pub fn rate_limit_exceeded() -> Self {
        AppError::RateLimitExceeded
    }
}

pub type AppResult<T> = Result<T, AppError>;

/// Extract validation errors into a structured format
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
